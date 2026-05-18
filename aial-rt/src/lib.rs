// aial-rt — AIAL runtime (extern "C" functions for AOT linkage)
// Compiled as a static library (.a), linked with AIAL AOT output (user_code.o)

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock, Arc};
use std::sync::atomic::{AtomicBool, Ordering};

/// Lock a mutex, recovering from poison (thread panic) instead of crashing
#[macro_export]
macro_rules! lock {
    ($m:expr) => {
        match $m.lock() {
            Ok(g) => g,
            Err(e) => {
                eprintln!("[aial-rt] poisoned lock recovered");
                e.into_inner()
            }
        }
    };
}

// (AtomicI64/Ordering removed — using combined (tokens, pos) tuple)

pub struct ContextState {
    system_prompt: String,
    token_budget: i64,
    tokens_used: i64,
    messages: Vec<(String, String)>, // (role, content) pairs
}

static CONTEXTS: OnceLock<Mutex<HashMap<i64, ContextState>>> = OnceLock::new();
pub static NEXT_CTX: Mutex<i64> = Mutex::new(1);
const RUNTIME_ADDR_BASE: i64 = 1_000_000;
static LAST_ERROR: OnceLock<Mutex<String>> = OnceLock::new();

fn set_error(msg: &str) {
    if let Some(e) = LAST_ERROR.get() { *lock!(e) = msg.to_string(); }
    else { LAST_ERROR.get_or_init(|| Mutex::new(msg.to_string())); }
    eprintln!("[aial-rt] {}", msg);
}
static NEXT_ADDR: Mutex<i64> = Mutex::new(RUNTIME_ADDR_BASE);
static HEAP: OnceLock<Mutex<HashMap<i64, i64>>> = OnceLock::new();
static STRINGS: OnceLock<Mutex<HashMap<i64, String>>> = OnceLock::new();
type StreamState = (Arc<Mutex<Vec<String>>>, i64, Arc<AtomicBool>);
static STREAM_TOKENS: OnceLock<Mutex<HashMap<i64, StreamState>>> = OnceLock::new();

fn stream_tokens() -> &'static Mutex<HashMap<i64, StreamState>> {
    STREAM_TOKENS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn ctxs() -> &'static Mutex<HashMap<i64, ContextState>> {
    CONTEXTS.get_or_init(|| Mutex::new(HashMap::new()))
}
pub fn heap() -> &'static Mutex<HashMap<i64, i64>> {
    HEAP.get_or_init(|| Mutex::new(HashMap::new()))
}
pub fn strs() -> &'static Mutex<HashMap<i64, String>> {
    STRINGS.get_or_init(|| Mutex::new(HashMap::new()))
}
pub fn alloc() -> i64 { let mut a = lock!(NEXT_ADDR); let v = *a; *a += 1; v }
pub fn alloc_block(n: usize) -> i64 { let mut a = lock!(NEXT_ADDR); let v = *a; *a += n as i64; v }
pub fn alloc_empty() -> i64 { let ptr = alloc(); lock!(strs()).insert(ptr, String::new()); ptr }

mod ai; mod term; mod string; mod collections; mod io; mod line_editor;
mod actor; mod json; mod http; mod file; mod process; mod time; mod convert;
mod ffi; mod global; mod key; mod context_memory;

// Re-export all extern "C" functions from submodules so they're accessible as aial_rt::aial_rt_*
pub use actor::*;
pub use ai::*;
pub use string::*;
pub use json::*;
pub use collections::*;
pub use global::*;
pub use io::*;
pub use term::*;
pub use time::*;
pub use process::*;
pub use convert::*;
pub use ffi::*;
pub use file::*;
pub use key::*;
pub use context_memory::*;
pub use line_editor::*;
pub use http::*;

#[no_mangle]
pub extern "C" fn aial_rt_print(text_ptr: i64) {
    let text = lock!(strs()).get(&text_ptr).cloned().unwrap_or_default();
    use std::io::Write;
    print!("{}", text);
    std::io::stdout().flush().ok();
}

#[no_mangle]
pub extern "C" fn aial_rt_println(text_ptr: i64) {
    let text = lock!(strs()).get(&text_ptr).cloned().unwrap_or_else(|| "(empty)".to_string());
    println!("{}", text);
}

#[no_mangle]
pub extern "C" fn aial_rt_string_register(idx: i64, text_ptr: *const std::ffi::c_char) {
    let text = unsafe { std::ffi::CStr::from_ptr(text_ptr) }.to_string_lossy().into_owned();
    lock!(strs()).insert(idx, text);
}

#[no_mangle] pub extern "C" fn aial_rt_enum_create(_name_ptr: i64, _variant_ptr: i64) -> i64 { alloc() }
#[no_mangle] pub extern "C" fn aial_rt_privacy_sensitive(_val: i64) -> i64 { 0 }
#[no_mangle] pub extern "C" fn aial_rt_cap_check(_c: i64) -> i64 { 1 }

#[cfg(test)]
mod tests {
    use super::io::crossterm_key_name;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn key_event(code: KeyCode, ctrl: bool) -> KeyEvent {
        KeyEvent::new(code, if ctrl { KeyModifiers::CONTROL } else { KeyModifiers::NONE })
    }

    #[test]
    fn test_key_names() {
        assert_eq!(crossterm_key_name(&key_event(KeyCode::Enter, false)), "ENTER");
        assert_eq!(crossterm_key_name(&key_event(KeyCode::Backspace, false)), "BACKSPACE");
        assert_eq!(crossterm_key_name(&key_event(KeyCode::Up, false)), "UP");
        assert_eq!(crossterm_key_name(&key_event(KeyCode::Down, false)), "DOWN");
        assert_eq!(crossterm_key_name(&key_event(KeyCode::Left, false)), "LEFT");
        assert_eq!(crossterm_key_name(&key_event(KeyCode::Right, false)), "RIGHT");
        assert_eq!(crossterm_key_name(&key_event(KeyCode::Char('q'), true)), "CTRL_Q");
        assert_eq!(crossterm_key_name(&key_event(KeyCode::Char('c'), true)), "CTRL_C");
        assert_eq!(crossterm_key_name(&key_event(KeyCode::Char('中'), false)), "中");
    }
}
