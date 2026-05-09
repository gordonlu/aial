// aial-rt — AIAL runtime (extern "C" functions for AOT linkage)
// Compiled as a static library (.a), linked with AIAL AOT output (user_code.o)

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

struct ContextState {
    token_budget: i64,
    tokens_used: i64,
}

static CONTEXTS: OnceLock<Mutex<HashMap<i64, ContextState>>> = OnceLock::new();
static NEXT_CTX: Mutex<i64> = Mutex::new(1);
static NEXT_ADDR: Mutex<i64> = Mutex::new(1);
static HEAP: OnceLock<Mutex<HashMap<i64, i64>>> = OnceLock::new();
static STRINGS: OnceLock<Mutex<HashMap<i64, String>>> = OnceLock::new();

fn ctxs() -> &'static Mutex<HashMap<i64, ContextState>> {
    CONTEXTS.get_or_init(|| Mutex::new(HashMap::new()))
}
fn heap() -> &'static Mutex<HashMap<i64, i64>> {
    HEAP.get_or_init(|| Mutex::new(HashMap::new()))
}
fn strs() -> &'static Mutex<HashMap<i64, String>> {
    STRINGS.get_or_init(|| Mutex::new(HashMap::new()))
}
fn alloc() -> i64 { let mut a = NEXT_ADDR.lock().unwrap(); let v = *a; *a += 1; v }
fn alloc_block(n: usize) -> i64 { let mut a = NEXT_ADDR.lock().unwrap(); let v = *a; *a += n as i64; v }

#[no_mangle]
pub extern "C" fn aial_rt_ai_call(
    model: i64, ctx_id: i64, prompt_idx: i64,
    _temperature: f64, max_tokens: i64, _format: i64,
) -> i64 {
    let text = if std::env::var("AIAL_MOCK").is_ok() {
        format!("[AIAL mock] model={} tokens={}", model, max_tokens)
    } else {
        "[AIAL AOT] AI call stub".to_string()
    };
    let text_ptr = alloc();
    strs().lock().unwrap().insert(text_ptr, text);

    if let Ok(mut c) = ctxs().lock() {
        if let Some(s) = c.get_mut(&ctx_id) {
            s.tokens_used += max_tokens / 2;
        }
    }

    let resp_ptr = alloc_block(4);
    let mut h = heap().lock().unwrap();
    h.insert(resp_ptr, 0);
    h.insert(resp_ptr + 1, text_ptr);
    h.insert(resp_ptr + 2, 0);
    h.insert(resp_ptr + 3, 0);
    resp_ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_new(_prompt: i64, budget: i64, _strategy: i64, _ws: i64) -> i64 {
    let mut n = NEXT_CTX.lock().unwrap();
    let id = *n; *n += 1;
    ctxs().lock().unwrap().insert(id, ContextState { token_budget: budget, tokens_used: 0 });
    id
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_budget(id: i64) -> i64 {
    ctxs().lock().unwrap().get(&id).map_or(0, |s| s.token_budget - s.tokens_used)
}

#[no_mangle]
pub extern "C" fn aial_rt_extract_ai_text(resp: i64) -> i64 {
    heap().lock().unwrap().get(&(resp + 1)).copied().unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn aial_rt_extract_ai_variant(resp: i64) -> i64 {
    heap().lock().unwrap().get(&resp).copied().unwrap_or(-1)
}

#[no_mangle]
pub extern "C" fn aial_rt_extract_ai_usage(resp: i64) -> i64 {
    heap().lock().unwrap().get(&(resp + 3)).copied().unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn aial_rt_println(text_ptr: i64) {
    let text = strs().lock().unwrap().get(&text_ptr).cloned().unwrap_or_else(|| "(empty)".to_string());
    println!("{}", text);
}

#[no_mangle] pub extern "C" fn aial_rt_tool_dispatch(_n: i64, _a: i64) -> i64 { 0 }
#[no_mangle] pub extern "C" fn aial_rt_cap_check(_c: i64) -> i64 { 1 }
