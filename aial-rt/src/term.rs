use super::*;

#[no_mangle]
pub extern "C" fn aial_rt_term_clear() {
    use std::io::Write;
    let _ = std::io::stdout().write_all(b"\x1b[2J\x1b[H");
    let _ = std::io::stdout().flush();
}

#[no_mangle]
pub extern "C" fn aial_rt_term_height() -> i64 {
    #[cfg(unix)]
    {
        let mut ws: libc::winsize = unsafe { std::mem::zeroed() };
        if unsafe { libc::ioctl(0, libc::TIOCGWINSZ, &mut ws) } == 0 {
            return ws.ws_row as i64;
        }
    }
    30 // fallback
}

#[no_mangle]
pub extern "C" fn aial_rt_term_scroll_region(top: i64, bottom: i64) {
    use std::io::Write;
    let _ = write!(std::io::stdout(), "\x1b[{};{}r", top, bottom);
    let _ = std::io::stdout().flush();
}

#[no_mangle]
pub extern "C" fn aial_rt_term_setup(rows: i64) {
    use std::io::Write;
    let _ = std::io::stdout().write_all(b"\x1b[2J\x1b[H");
    // Set scroll region: rows 1..N-2 scroll, rows N-1 and N are fixed
    if rows > 5 { let _ = write!(std::io::stdout(), "\x1b[1;{}r", rows - 2); }
    let sep = "────────────────────────────────────────────────────────────────────────────────";
    // Draw bottom area OUTSIDE the scroll region
    let _ = write!(std::io::stdout(), "\x1b[{};1H\x1b[2K\x1b[90m{}\x1b[0m", rows - 1, sep);
    let _ = write!(std::io::stdout(), "\x1b[{};1H\x1b[2K Deep TUI \x1b[0m", rows);
    // Position cursor at input line (rows-2 = last line of scroll region)
    let _ = write!(std::io::stdout(), "\x1b[{};1H", rows - 2);
    let _ = std::io::stdout().flush();
}

#[no_mangle]
pub extern "C" fn aial_rt_term_redraw(rows: i64, buf_ptr: i64, editor_ptr: i64, editor_col: i64) {
    use std::io::{Write, stdout};
    let buffer = lock!(strs()).get(&buf_ptr).cloned().unwrap_or_default();
    let editor_buf = lock!(strs()).get(&editor_ptr).cloned().unwrap_or_default();
    let lines: Vec<&str> = buffer.split('\n').collect();

    let mut out = stdout();
    let _ = write!(out, "\x1b[r"); // reset scroll region

    // Draw chat area (rows 1 .. rows-3), clear old lines below
    let chat_max = (rows - 3).max(1) as usize;
    let visible = lines.len().min(chat_max);
    let start = if lines.len() > chat_max { lines.len() - chat_max } else { 0 };
    for i in 0..chat_max {
        let row = 1 + i as i64;
        if i < visible {
            let line = lines[start + i];
            let _ = write!(out, "\x1b[{};1H\x1b[2K{}", row, line);
        } else {
            let _ = write!(out, "\x1b[{};1H\x1b[2K", row);
        }
    }

    // Separator at rows-2
    let sep = "────────────────────────────────────────────────────────────────────────────────";
    let _ = write!(out, "\x1b[{};1H\x1b[2K\x1b[90m{}\x1b[0m", rows - 2, sep);

    // Input line at rows-1: "> editor_buf"
    let mut input = String::from("> ");
    input.push_str(&editor_buf);
    let _ = write!(out, "\x1b[{};1H\x1b[2K{}", rows - 1, input);

    // Cursor at input position (rows-1, col = 3 + editor_col)
    let col = 3 + editor_col;
    let _ = write!(out, "\x1b[{};{}H", rows - 1, col);
    let _ = out.flush();
}

#[no_mangle]
pub extern "C" fn aial_rt_term_display_width(s_ptr: i64) -> i64 {
    use unicode_width::UnicodeWidthStr;
    let s = lock!(strs()).get(&s_ptr).cloned().unwrap_or_default();
    UnicodeWidthStr::width(s.as_str()) as i64
}

#[no_mangle]
pub extern "C" fn aial_rt_term_cursor_goto(row: i64, col: i64) {
    use std::io::Write;
    let _ = write!(std::io::stdout(), "\x1b[{};{}H", row, col);
    let _ = std::io::stdout().flush();
}

#[no_mangle]
pub extern "C" fn aial_rt_term_cursor_row() -> i64 {
    // Returns 0 — cursor position is tracked by the terminal, not easily queried
    0
}

#[no_mangle]
pub extern "C" fn aial_rt_term_draw_text_clipped(row: i64, col: i64, width: i64, text_idx: i64) {
    use std::io::Write;
    let text = lock!(strs()).get(&text_idx).cloned().unwrap_or_default();
    let truncated: String = text.chars().take(width as usize).collect();
    let _ = write!(std::io::stdout(), "\x1b[{};{}H\x1b[2K{}", row, col, truncated);
    let _ = std::io::stdout().flush();
}

#[no_mangle]
pub extern "C" fn aial_rt_term_reset() {
    use std::io::Write;
    let _ = std::io::stdout().write_all(b"\x1b[r");
    let _ = std::io::stdout().flush();
}
