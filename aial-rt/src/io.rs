use super::*;

static SAVED_TERMIOS: OnceLock<Mutex<libc::termios>> = OnceLock::new();
static PASTE_BUF: OnceLock<Mutex<Vec<u8>>> = OnceLock::new();
static PASTE_ACTIVE: OnceLock<Mutex<bool>> = OnceLock::new();

fn paste_buf() -> &'static Mutex<Vec<u8>> { PASTE_BUF.get_or_init(|| Mutex::new(Vec::new())) }
fn paste_active() -> &'static Mutex<bool> { PASTE_ACTIVE.get_or_init(|| Mutex::new(false)) }

// Poll stdin fd 0 for available data. Uses raw libc to avoid Rust BufReader.
fn stdin_poll(ms: i64) -> bool {
    let mut fds = [libc::pollfd { fd: 0, events: libc::POLLIN, revents: 0 }];
    unsafe { libc::poll(fds.as_mut_ptr(), 1, ms as libc::c_int) > 0 }
}

// Read one byte from stdin fd 0. Raw libc — no Rust BufReader.
fn read_stdin_byte() -> u8 {
    let mut buf = [0u8; 1];
    unsafe { libc::read(0, buf.as_mut_ptr() as *mut libc::c_void, 1); }
    buf[0]
}

fn read_byte_timeout(ms: i64) -> Option<u8> {
    if ms >= 0 && !stdin_poll(ms) { return None; }
    Some(read_stdin_byte())
}

fn drain_available() -> Vec<u8> {
    let mut buf = Vec::new();
    loop {
        if !stdin_poll(5) { break; }
        buf.push(read_stdin_byte());
    }
    buf
}

/// Read all immediately-available bytes after ESC or UTF-8 lead byte.
fn read_escape_sequence() -> String {
    let mut seq = vec![0x1bu8];
    seq.extend(drain_available());
    String::from_utf8_lossy(&seq).to_string()
}

fn read_paste_data(ptr: i64, is_start: bool) -> i64 {
    if is_start { lock!(paste_active()).clone_from(&true); }
    let mut data: Vec<u8> = Vec::new();
    loop {
        match read_byte_timeout(100) {
            Some(b) => {
                if b == 0x1b {
                    let rest = read_escape_sequence();
                    if rest == "\x1b[201~" || rest.starts_with("\x1b[201") {
                        lock!(paste_active()).clone_from(&false);
                        break;
                    }
                    data.push(b);
                    data.extend(rest.bytes());
                } else {
                    data.push(b);
                }
            }
            None => break,
        }
    }
    let s = String::from_utf8_lossy(&data).into_owned();
    lock!(strs()).insert(ptr, s);
    ptr
}

/// Map crossterm KeyEvent to named key string
pub(crate) fn crossterm_key_name(event: &crossterm::event::KeyEvent) -> String {
    use crossterm::event::{KeyCode, KeyModifiers};
    let ctrl = event.modifiers.contains(KeyModifiers::CONTROL);
    match event.code {
        KeyCode::Enter => "ENTER".into(),
        KeyCode::Backspace => "BACKSPACE".into(),
        KeyCode::Tab => "TAB".into(),
        KeyCode::Esc => "ESC".into(),
        KeyCode::Up => "UP".into(),
        KeyCode::Down => "DOWN".into(),
        KeyCode::Left => "LEFT".into(),
        KeyCode::Right => "RIGHT".into(),
        KeyCode::Home => "HOME".into(),
        KeyCode::End => "END".into(),
        KeyCode::PageUp => "PAGEUP".into(),
        KeyCode::PageDown => "PAGEDOWN".into(),
        KeyCode::Delete => "DELETE".into(),
        KeyCode::F(1) => "F1".into(),
        KeyCode::F(2) => "F2".into(),
        KeyCode::F(3) => "F3".into(),
        KeyCode::F(4) => "F4".into(),
        KeyCode::Char('q') if ctrl => "CTRL_Q".into(),
        KeyCode::Char('l') if ctrl => "CTRL_L".into(),
        KeyCode::Char('d') if ctrl => "CTRL_D".into(),
        KeyCode::Char(c) if ctrl => format!("CTRL_{}", c.to_uppercase()),
        KeyCode::Char(c) => c.to_string(),
        _ => String::new(),
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_io_readkey() -> i64 {
    let ptr = alloc();
    match crossterm::event::read() {
        Ok(crossterm::event::Event::Key(key)) => {
            lock!(strs()).insert(ptr, crossterm_key_name(&key));
        }
        Ok(crossterm::event::Event::Paste(data)) => {
            lock!(strs()).insert(ptr, data);
        }
        Err(_) => { lock!(strs()).insert(ptr, "EOF".to_string()); }
        Ok(_) => { lock!(strs()).insert(ptr, String::new()); }
    }
    ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_io_readkey_timeout(ms: i64) -> i64 {
    let ptr = alloc();
    if let Ok(true) = crossterm::event::poll(std::time::Duration::from_millis(ms.max(0) as u64)) {
        match crossterm::event::read() {
            Ok(crossterm::event::Event::Key(key)) => {
                lock!(strs()).insert(ptr, crossterm_key_name(&key));
            }
            Ok(crossterm::event::Event::Paste(data)) => {
                lock!(strs()).insert(ptr, data);
            }
            Err(_) => { lock!(strs()).insert(ptr, "EOF".to_string()); }
            Ok(_) => { lock!(strs()).insert(ptr, String::new()); }
        }
    } else {
        lock!(strs()).insert(ptr, String::new());
    }
    ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_io_readln() -> i64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    let ptr = alloc();
    lock!(strs()).insert(ptr, input.trim_end().to_string());
    ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_io_readln_timeout(_ms: i64) -> i64 {
    aial_rt_io_readln() // fallback to blocking for now
}

#[no_mangle]
pub extern "C" fn aial_rt_io_read_multiline() -> i64 {
    let ptr = alloc();
    let mut lines = String::new();
    let mut prev_empty = false;
    loop {
        let key_ptr = aial_rt_io_readkey();
        let key = lock!(strs()).get(&key_ptr).cloned().unwrap_or_default();
        match key.as_str() {
            "ENTER" => {
                if lines.is_empty() && prev_empty { break; } // blank line = done
                prev_empty = lines.is_empty();
                lines.push('\n');
            }
            "ESC" | "CTRL_Q" | "CTRL_C" => break,
            "BACKSPACE" => { if !lines.is_empty() { lines.pop(); } }
            s if !s.is_empty() => {
                prev_empty = false;
                let first = s.as_bytes()[0];
                if first >= 32 || first >= 0xC0 { lines.push_str(s); }
            }
            _ => {}
        }
    }
    lock!(strs()).insert(ptr, lines.trim_end().to_string());
    ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_io_is_tty() -> i64 {
    unsafe { libc::isatty(0) as i64 }
}

#[no_mangle]
pub extern "C" fn aial_rt_io_raw_mode(enable: i64) {
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        let fd = std::io::stdin().as_raw_fd();
        if enable != 0 {
            let mut orig: libc::termios = unsafe { std::mem::zeroed() };
            unsafe { libc::tcgetattr(fd, &mut orig); }
            SAVED_TERMIOS.get_or_init(|| Mutex::new(orig));
            let mut raw = orig;
            raw.c_lflag &= !(libc::ECHO | libc::ICANON | libc::ISIG);
            raw.c_iflag &= !(libc::IXON | libc::ICRNL);  // disable XON/XOFF (^Q/^S) and CR→NL
            raw.c_cc[libc::VMIN] = 1;
            raw.c_cc[libc::VTIME] = 0;
            unsafe { libc::tcsetattr(fd, libc::TCSANOW, &raw); }
            // Enable bracketed paste
            use std::io::Write;
            let _ = std::io::stdout().write_all(b"\x1b[?2004h");
            let _ = std::io::stdout().flush();
        } else {
            // Disable bracketed paste
            use std::io::Write;
            let _ = std::io::stdout().write_all(b"\x1b[?2004l");
            let _ = std::io::stdout().flush();
            if let Some(saved) = SAVED_TERMIOS.get() {
                let orig = lock!(saved);
                unsafe { libc::tcsetattr(fd, libc::TCSANOW, &(*orig) as *const libc::termios); }
            }
        }
    }
}
