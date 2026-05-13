// aial-rt — AIAL runtime (extern "C" functions for AOT linkage)
// Compiled as a static library (.a), linked with AIAL AOT output (user_code.o)

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock, Arc};

/// Lock a mutex, recovering from poison (thread panic) instead of crashing
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

struct ContextState {
    token_budget: i64,
    tokens_used: i64,
    messages: Vec<(String, String)>, // (role, content) pairs
}

static CONTEXTS: OnceLock<Mutex<HashMap<i64, ContextState>>> = OnceLock::new();
static NEXT_CTX: Mutex<i64> = Mutex::new(1);
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
static STREAM_TOKENS: OnceLock<Mutex<HashMap<i64, (Arc<Mutex<Vec<String>>>, i64)>>> = OnceLock::new();

fn stream_tokens() -> &'static Mutex<HashMap<i64, (Arc<Mutex<Vec<String>>>, i64)>> {
    STREAM_TOKENS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn ctxs() -> &'static Mutex<HashMap<i64, ContextState>> {
    CONTEXTS.get_or_init(|| Mutex::new(HashMap::new()))
}
fn heap() -> &'static Mutex<HashMap<i64, i64>> {
    HEAP.get_or_init(|| Mutex::new(HashMap::new()))
}
fn strs() -> &'static Mutex<HashMap<i64, String>> {
    STRINGS.get_or_init(|| Mutex::new(HashMap::new()))
}
fn alloc() -> i64 { let mut a = lock!(NEXT_ADDR); let v = *a; *a += 1; v }
fn alloc_block(n: usize) -> i64 { let mut a = lock!(NEXT_ADDR); let v = *a; *a += n as i64; v }
fn alloc_empty() -> i64 { let ptr = alloc(); lock!(strs()).insert(ptr, String::new()); ptr }

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
    lock!(strs()).insert(text_ptr, text);

    if let Ok(mut c) = ctxs().lock() {
        if let Some(s) = c.get_mut(&ctx_id) {
            s.tokens_used += max_tokens / 2;
        }
    }

    let resp_ptr = alloc_block(4);
    let mut h = lock!(heap());
    h.insert(resp_ptr, 0);
    h.insert(resp_ptr + 1, text_ptr);
    h.insert(resp_ptr + 2, 0);
    h.insert(resp_ptr + 3, 0);
    resp_ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_new(_prompt: i64, budget: i64, _strategy: i64, _ws: i64) -> i64 {
    let mut n = lock!(NEXT_CTX);
    let id = *n; *n += 1;
    lock!(ctxs()).insert(id, ContextState { token_budget: budget, tokens_used: 0, messages: Vec::new() });
    id
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_budget(id: i64) -> i64 {
    lock!(ctxs()).get(&id).map_or(0, |s| s.token_budget - s.tokens_used)
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_add_message(ctx_id: i64, role_ptr: i64, content_ptr: i64) -> i64 {
    let role = {
        let st = lock!(strs());
        st.get(&role_ptr).cloned().unwrap_or_default()
    };
    let content = {
        let st = lock!(strs());
        st.get(&content_ptr).cloned().unwrap_or_default()
    };
    lock!(ctxs()).get_mut(&ctx_id).map(|s| s.messages.push((role, content)));
    ctx_id
}

#[no_mangle]
pub extern "C" fn aial_rt_extract_ai_text(resp: i64) -> i64 {
    lock!(heap()).get(&(resp + 1)).copied().unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn aial_rt_extract_ai_variant(resp: i64) -> i64 {
    lock!(heap()).get(&resp).copied().unwrap_or(-1)
}

#[no_mangle]
pub extern "C" fn aial_rt_extract_ai_usage(resp: i64) -> i64 {
    lock!(heap()).get(&(resp + 3)).copied().unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn aial_rt_println(text_ptr: i64) {
    let text = lock!(strs()).get(&text_ptr).cloned().unwrap_or_else(|| "(empty)".to_string());
    println!("{}", text);
}

#[no_mangle]
pub extern "C" fn aial_rt_strcat(a_ptr: i64, b_ptr: i64) -> i64 {
    let a = lock!(strs()).get(&a_ptr).cloned().unwrap_or_default();
    let b = lock!(strs()).get(&b_ptr).cloned().unwrap_or_default();
    let result = a + &b;
    let addr = alloc();
    lock!(strs()).insert(addr, result);
    addr
}

#[no_mangle]
pub extern "C" fn aial_rt_strlen(ptr: i64) -> i64 {
    lock!(strs()).get(&ptr).map_or(0, |s| s.len() as i64)
}

#[no_mangle]
pub extern "C" fn aial_rt_strslice(ptr: i64, start: i64, len: i64) -> i64 {
    let s = lock!(strs()).get(&ptr).cloned().unwrap_or_default();
    let slice: String = s.chars().skip(start as usize).take(len as usize).collect();
    let addr = alloc();
    lock!(strs()).insert(addr, slice);
    addr
}

#[no_mangle]
pub extern "C" fn aial_rt_strchr(ptr: i64, idx: i64) -> i64 {
    lock!(strs()).get(&ptr)
        .and_then(|s| s.chars().nth(idx as usize))
        .map(|c| c as i64)
        .unwrap_or(-1)
}

#[no_mangle]
pub extern "C" fn aial_rt_str_eq(a_ptr: i64, b_ptr: i64) -> i64 {
    let strs = lock!(strs());
    let a = strs.get(&a_ptr).cloned().unwrap_or_default();
    let b = strs.get(&b_ptr).cloned().unwrap_or_default();
    if a == b { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn aial_rt_starts_with(ptr: i64, prefix_ptr: i64) -> i64 {
    let strs = lock!(strs());
    let s = strs.get(&ptr).cloned().unwrap_or_default();
    let prefix = strs.get(&prefix_ptr).cloned().unwrap_or_default();
    if s.starts_with(&prefix) { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn aial_rt_file_read(path_ptr: i64) -> i64 {
    let path = lock!(strs()).get(&path_ptr).cloned().unwrap_or_default();
    let content = std::fs::read_to_string(&path).unwrap_or_else(|e| format!("[read error: {}]", e));
    let addr = alloc();
    lock!(strs()).insert(addr, content);
    addr
}

#[no_mangle]
pub extern "C" fn aial_rt_string_register(idx: i64, text_ptr: *const std::ffi::c_char) {
    let text = unsafe { std::ffi::CStr::from_ptr(text_ptr) }.to_string_lossy().into_owned();
    lock!(strs()).insert(idx, text);
}

// ── HTTP ──

#[no_mangle]
pub extern "C" fn aial_rt_http_get(url_ptr: i64) -> i64 {
    let url = lock!(strs()).get(&url_ptr).cloned().unwrap_or_default();
    let resp_ptr = alloc_block(3);
    let mut h = lock!(heap());
    match reqwest::blocking::get(&url) {
        Ok(resp) => {
            let status = resp.status().as_u16() as i64;
            let body = resp.text().unwrap_or_default();
            let body_ptr = alloc();
            lock!(strs()).insert(body_ptr, body);
            h.insert(resp_ptr, status);
            h.insert(resp_ptr + 1, body_ptr);
            h.insert(resp_ptr + 2, 0);
        }
        Err(e) => {
            let err_body = format!("[http error: {}]", e);
            let body_ptr = alloc();
            lock!(strs()).insert(body_ptr, err_body);
            h.insert(resp_ptr, 0);
            h.insert(resp_ptr + 1, body_ptr);
            h.insert(resp_ptr + 2, 0);
        }
    }
    resp_ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_http_status(resp: i64) -> i64 {
    lock!(heap()).get(&resp).copied().unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn aial_rt_http_text(resp: i64) -> i64 {
    lock!(heap()).get(&(resp + 1)).copied().unwrap_or(0)
}

// ── JSON ──
// JsonValue heap layout: [type, aux, size/f64_bits, data_ptr, _]
// types: 0=null, 1=bool, 2=number, 3=string, 4=array, 5=object, -1=error

fn json_write(s: &mut HashMap<i64, String>, h: &mut HashMap<i64, i64>, ptr: i64, v: &serde_json::Value) {
    match v {
        serde_json::Value::Null => { h.insert(ptr, 0); }
        serde_json::Value::Bool(b) => { h.insert(ptr, 1); h.insert(ptr + 1, *b as i64); }
        serde_json::Value::Number(n) => {
            h.insert(ptr, 2);
            if let Some(f) = n.as_f64() { h.insert(ptr + 2, f.to_bits() as i64); }
        }
        serde_json::Value::String(t) => {
            h.insert(ptr, 3);
            let s_ptr = alloc(); s.insert(s_ptr, t.clone()); h.insert(ptr + 1, s_ptr);
        }
        serde_json::Value::Array(arr) => {
            h.insert(ptr, 4);
            let arr_ptr = alloc_block(arr.len());
            h.insert(ptr + 3, arr_ptr);
            h.insert(ptr + 2, arr.len() as i64);
            for (i, item) in arr.iter().enumerate() {
                let item_ptr = alloc_block(5);
                json_write(s, h, item_ptr, item);
                h.insert(arr_ptr + i as i64, item_ptr);
            }
        }
        serde_json::Value::Object(obj) => {
            h.insert(ptr, 5);
            let n = obj.len();
            let obj_ptr = alloc_block(n * 2);
            h.insert(ptr + 3, obj_ptr);
            h.insert(ptr + 2, n as i64);
            for (i, (k, val)) in obj.iter().enumerate() {
                let k_ptr = alloc(); s.insert(k_ptr, k.clone());
                h.insert(obj_ptr + (i * 2) as i64, k_ptr);
                let v_ptr = alloc_block(5);
                json_write(s, h, v_ptr, val);
                h.insert(obj_ptr + (i * 2 + 1) as i64, v_ptr);
            }
        }
    }
}

fn json_lookup(ptr: i64, key: &str) -> Option<i64> {
    let h = lock!(heap());
    let s = lock!(strs());
    let tag = h.get(&ptr).copied().unwrap_or(0);
    if tag != 5 { return None; }
    let n = h.get(&(ptr + 2)).copied().unwrap_or(0) as usize;
    let obj_ptr = h.get(&(ptr + 3)).copied().unwrap_or(0);
    for i in 0..n {
        let k_ptr = h.get(&(obj_ptr + (i * 2) as i64)).copied().unwrap_or(0);
        if s.get(&k_ptr).map(|k| k == key).unwrap_or(false) {
            return h.get(&(obj_ptr + (i * 2 + 1) as i64)).copied();
        }
    }
    None
}

#[no_mangle]
pub extern "C" fn aial_rt_json_parse(text_ptr: i64) -> i64 {
    let text = lock!(strs()).get(&text_ptr).cloned().unwrap_or_default();
    let val_ptr = alloc_block(5);
    let mut h = lock!(heap());
    let mut s = lock!(strs());
    match serde_json::from_str::<serde_json::Value>(&text) {
        Ok(v) => { json_write(&mut s, &mut h, val_ptr, &v); val_ptr }
        Err(e) => {
            h.insert(val_ptr, -1); // error tag
            let err_ptr = alloc();
            s.insert(err_ptr, e.to_string());
            h.insert(val_ptr + 1, err_ptr);
            val_ptr
        }
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_json_get(val_ptr: i64, key_ptr: i64) -> i64 {
    let key = lock!(strs()).get(&key_ptr).cloned().unwrap_or_default();
    match json_lookup(val_ptr, &key) {
        Some(r) => r,
        None => {
            let null_ptr = alloc_block(5);
            lock!(heap()).insert(null_ptr, 0);
            null_ptr
        }
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_json_get_or(val_ptr: i64, key_ptr: i64, default_ptr: i64) -> i64 {
    let key = lock!(strs()).get(&key_ptr).cloned().unwrap_or_default();
    json_lookup(val_ptr, &key).unwrap_or(default_ptr)
}

#[no_mangle]
pub extern "C" fn aial_rt_json_type(val_ptr: i64) -> i64 {
    lock!(heap()).get(&val_ptr).copied().unwrap_or(0)
}

// ── JSON helpers ──

fn json_value_to_string_runtime(val_ptr: i64) -> String {
    // Collect child pointers under lock, then release before recursive calls
    let (tag, aux, f64_val, children): (i64, i64, u64, Vec<i64>) = {
        let h = lock!(heap());
        let s = lock!(strs());
        let tag = h.get(&val_ptr).copied().unwrap_or(0);
        match tag {
            3 => {
                let sp = h.get(&(val_ptr + 1)).copied().unwrap_or(0);
                return format!("\"{}\"", s.get(&sp).cloned().unwrap_or_default());
            }
            4 => {
                let n = h.get(&(val_ptr + 2)).copied().unwrap_or(0) as usize;
                let arr = h.get(&(val_ptr + 3)).copied().unwrap_or(0);
                let mut children = Vec::new();
                for i in 0..n {
                    if let Some(ip) = h.get(&(arr + i as i64)).copied() { children.push(ip); }
                }
                (tag, 0, 0, children)
            }
            5 => {
                let n = h.get(&(val_ptr + 2)).copied().unwrap_or(0) as usize;
                let obj = h.get(&(val_ptr + 3)).copied().unwrap_or(0);
                let mut children = Vec::new();
                for i in 0..n {
                    if let Some(kp) = h.get(&(obj + (i * 2) as i64)).copied() { children.push(kp); }
                    if let Some(vp) = h.get(&(obj + (i * 2 + 1) as i64)).copied() { children.push(vp); }
                }
                (tag, n as i64, 0, children)
            }
            _ => (tag, h.get(&(val_ptr + 1)).copied().unwrap_or(0), h.get(&(val_ptr + 2)).copied().unwrap_or(0) as u64, vec![]),
        }
    };
    // Format without holding locks
    match tag {
        0 => "null".to_string(),
        1 => (aux != 0).to_string(),
        2 => format!("{}", f64::from_bits(f64_val)),
        4 => {
            let items: Vec<String> = children.iter().map(|&ip| json_value_to_string_runtime(ip)).collect();
            format!("[{}]", items.join(","))
        }
        5 => {
            let n = aux as usize;
            let mut pairs = Vec::new();
            for i in 0..n {
                let kp = children[i * 2];
                let vp = children[i * 2 + 1];
                let key = lock!(strs()).get(&kp).cloned().unwrap_or_default();
                pairs.push(format!("\"{}\":{}", key, json_value_to_string_runtime(vp)));
            }
            format!("{{{}}}", pairs.join(","))
        }
        _ => "null".to_string(),
    }
}

// ── JSON (more) ──

#[no_mangle]
pub extern "C" fn aial_rt_json_stringify(val_ptr: i64) -> i64 {
    let s = json_value_to_string_runtime(val_ptr);
    let ptr = alloc(); lock!(strs()).insert(ptr, s); ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_json_value_to_string(val_ptr: i64) -> i64 {
    let h = lock!(heap());
    let s = lock!(strs());
    let tag = h.get(&val_ptr).copied().unwrap_or(0);
    let text = match tag {
        3 => s.get(&h.get(&(val_ptr+1)).copied().unwrap_or(0)).cloned().unwrap_or_default(),
        2 => format!("{}", f64::from_bits(h.get(&(val_ptr+2)).copied().unwrap_or(0) as u64)),
        1 => (h.get(&(val_ptr+1)).copied().unwrap_or(0) != 0).to_string(),
        0 => "null".to_string(),
        _ => json_value_to_string_runtime(val_ptr),
    };
    drop(h); drop(s);
    let ptr = alloc(); lock!(strs()).insert(ptr, text); ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_json_to_int(val_ptr: i64) -> i64 {
    let h = lock!(heap());
    let tag = h.get(&val_ptr).copied().unwrap_or(0);
    match tag {
        2 => f64::from_bits(h.get(&(val_ptr+2)).copied().unwrap_or(0) as u64) as i64,
        1 => h.get(&(val_ptr+1)).copied().unwrap_or(0),
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_json_to_float(val_ptr: i64) -> f64 {
    let h = lock!(heap());
    let tag = h.get(&val_ptr).copied().unwrap_or(0);
    match tag {
        2 => f64::from_bits(h.get(&(val_ptr+2)).copied().unwrap_or(0) as u64),
        1 => h.get(&(val_ptr+1)).copied().unwrap_or(0) as f64,
        _ => 0.0,
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_json_array_len(val_ptr: i64) -> i64 {
    let h = lock!(heap());
    if h.get(&val_ptr).copied().unwrap_or(0) == 4 { h.get(&(val_ptr+2)).copied().unwrap_or(0) } else { 0 }
}

#[no_mangle]
pub extern "C" fn aial_rt_json_array_get(val_ptr: i64, idx: i64) -> i64 {
    let h = lock!(heap());
    let arr_ptr = h.get(&(val_ptr + 3)).copied().unwrap_or(0);
    match h.get(&(arr_ptr + idx)).copied() {
        Some(v) => v,
        None => {
            drop(h);
            let null_ptr = alloc_block(5); lock!(heap()).insert(null_ptr, 0); null_ptr
        }
    }
}

// ── HTTP (more) ──

#[no_mangle]
pub extern "C" fn aial_rt_http_post(url_ptr: i64, body_ptr: i64) -> i64 {
    let url = lock!(strs()).get(&url_ptr).cloned().unwrap_or_default();
    let body = lock!(strs()).get(&body_ptr).cloned().unwrap_or_default();
    let resp_ptr = alloc_block(3);
    let mut h = lock!(heap());
    let client = reqwest::blocking::Client::new();
    match client.post(&url).body(body).send() {
        Ok(resp) => {
            let status = resp.status().as_u16() as i64;
            let text = resp.text().unwrap_or_default();
            let bp = alloc(); lock!(strs()).insert(bp, text);
            h.insert(resp_ptr, status); h.insert(resp_ptr + 1, bp); h.insert(resp_ptr + 2, 0);
        }
        Err(e) => {
            let err = format!("[http error: {}]", e);
            let bp = alloc(); lock!(strs()).insert(bp, err);
            h.insert(resp_ptr, 0); h.insert(resp_ptr + 1, bp);
        }
    }
    resp_ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_http_post_json(url_ptr: i64, val_ptr: i64) -> i64 {
    let url = lock!(strs()).get(&url_ptr).cloned().unwrap_or_default();
    let json_str = json_value_to_string_runtime(val_ptr);
    let resp_ptr = alloc_block(3);
    let mut h = lock!(heap());
    let client = reqwest::blocking::Client::new();
    match client.post(&url).header("Content-Type", "application/json").body(json_str).send() {
        Ok(resp) => {
            let status = resp.status().as_u16() as i64;
            let text = resp.text().unwrap_or_default();
            let bp = alloc(); lock!(strs()).insert(bp, text);
            h.insert(resp_ptr, status); h.insert(resp_ptr + 1, bp);
        }
        Err(e) => {
            let err = format!("[http error: {}]", e);
            let bp = alloc(); lock!(strs()).insert(bp, err);
            h.insert(resp_ptr, 0); h.insert(resp_ptr + 1, bp);
        }
    }
    resp_ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_http_header_map() -> i64 {
    let ptr = alloc_block(128);
    lock!(heap()).insert(ptr, 0); // count = 0
    ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_http_header_set(map: i64, key_ptr: i64, val_ptr: i64) -> i64 {
    let key = lock!(strs()).get(&key_ptr).cloned().unwrap_or_default();
    let val = lock!(strs()).get(&val_ptr).cloned().unwrap_or_default();
    let mut h = lock!(heap());
    let n = h.get(&map).copied().unwrap_or(0);
    let idx = n * 2 + 1;
    let kp = alloc(); lock!(strs()).insert(kp, key);
    let vp = alloc(); lock!(strs()).insert(vp, val);
    h.insert(map + idx, kp);
    h.insert(map + idx + 1, vp);
    h.insert(map, n + 1);
    map
}

// ── HTTP server (stubs - needs tiny_http) ──

#[no_mangle]
pub extern "C" fn aial_rt_http_start(port: i64) -> i64 {
    match tiny_http::Server::http(format!("0.0.0.0:{}", port)) {
        Ok(server) => {
            let ptr = alloc_block(2);
            lock!(heap()).insert(ptr, Box::into_raw(Box::new(server)) as i64);
            ptr
        }
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_http_listen(handle: i64, timeout_ms: i64) -> i64 {
    let server_ptr = lock!(heap()).get(&handle).copied().unwrap_or(0);
    if server_ptr == 0 { return -1; }
    let server: &tiny_http::Server = unsafe { &*(server_ptr as *const tiny_http::Server) };
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(if timeout_ms <= 0 { u64::MAX } else { timeout_ms as u64 });
    loop {
        match server.recv_timeout(deadline.saturating_duration_since(std::time::Instant::now())) {
            Ok(Some(mut request)) => {
                let url = format!("{}", request.url());
                let method = request.method().to_string();
                let mut body = String::new();
                { use std::io::Read; let _ = request.as_reader().read_to_string(&mut body); }
                // Collect headers
                let mut headers_str = String::new();
                for h in request.headers() {
                    headers_str.push_str(&format!("{}: {}\n", h.field, h.value));
                }
                let req_ptr = alloc_block(5);
                let mut h = lock!(heap());
                h.insert(req_ptr, Box::into_raw(Box::new(request)) as i64);
                let url_ptr = alloc(); lock!(strs()).insert(url_ptr, url);
                h.insert(req_ptr + 1, url_ptr);
                let method_ptr = alloc(); lock!(strs()).insert(method_ptr, method);
                h.insert(req_ptr + 2, method_ptr);
                let body_ptr = alloc(); lock!(strs()).insert(body_ptr, body);
                h.insert(req_ptr + 3, body_ptr);
                let headers_ptr = alloc(); lock!(strs()).insert(headers_ptr, headers_str);
                h.insert(req_ptr + 4, headers_ptr);
                return req_ptr;
            }
            Ok(None) => return -1, // timeout
            Err(_) => return -1,
        }
        if std::time::Instant::now() >= deadline { return -1; }
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_http_respond(req: i64, body_ptr: i64, ct_ptr: i64) -> i64 {
    let mut h = lock!(heap());
    let server_req_ptr = h.get(&req).copied().unwrap_or(0);
    if server_req_ptr == 0 { return -1; }
    drop(h);
    let body = lock!(strs()).get(&body_ptr).cloned().unwrap_or_default();
    let ct = lock!(strs()).get(&ct_ptr).cloned().unwrap_or_default();
    let request: Box<tiny_http::Request> = unsafe { Box::from_raw(server_req_ptr as *mut tiny_http::Request) };
    let ct_bytes = ct.into_bytes();
    let ct_header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &ct_bytes[..])
        .unwrap_or_else(|_| tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..])
        .unwrap_or_else(|_| tiny_http::Header::from_bytes(&b"X"[..], &b"Y"[..]).unwrap())); // never fails with valid input
    let response = tiny_http::Response::from_string(&body).with_header(ct_header);
    let _ = request.respond(response);
    lock!(heap()).remove(&req); // consume request
    0
}

#[no_mangle]
pub extern "C" fn aial_rt_http_body(req: i64) -> i64 {
    lock!(heap()).get(&(req + 3)).copied().unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn aial_rt_http_method(req: i64) -> i64 {
    lock!(heap()).get(&(req + 2)).copied().unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn aial_rt_http_url(req: i64) -> i64 {
    lock!(heap()).get(&(req + 1)).copied().unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn aial_rt_http_path(req: i64) -> i64 {
    let url_ptr = lock!(heap()).get(&(req + 1)).copied().unwrap_or(0);
    let url = lock!(strs()).get(&url_ptr).cloned().unwrap_or_default();
    // Extract path: strip query string (?...)
    let path = match url.find('?') {
        Some(pos) => &url[..pos],
        None => &url,
    };
    let ptr = alloc();
    lock!(strs()).insert(ptr, path.to_string());
    ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_http_query(req: i64, key_ptr: i64) -> i64 {
    let url_ptr = lock!(heap()).get(&(req + 1)).copied().unwrap_or(0);
    let url = lock!(strs()).get(&url_ptr).cloned().unwrap_or_default();
    let key = lock!(strs()).get(&key_ptr).cloned().unwrap_or_default();
    // Parse ?key=value&...
    let query_start = url.find('?').map(|p| p + 1).unwrap_or(0);
    if query_start == 0 || query_start >= url.len() {
        let ptr = alloc(); lock!(strs()).insert(ptr, String::new()); return ptr;
    }
    let query = &url[query_start..];
    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        if let Some(k) = parts.next() {
            if k == key {
                let v = parts.next().unwrap_or("");
                let decoded = urlencoding(v);
                let ptr = alloc(); lock!(strs()).insert(ptr, decoded); return ptr;
            }
        }
    }
    let ptr = alloc(); lock!(strs()).insert(ptr, String::new()); ptr
}

fn urlencoding(s: &str) -> String {
    s.replace("%20", " ").replace("%22", "\"").replace("%3C", "<").replace("%3E", ">")
        .replace("%2F", "/").replace("%3A", ":").replace("%2C", ",")
}

#[no_mangle]
pub extern "C" fn aial_rt_http_header(req: i64, key_ptr: i64) -> i64 {
    let headers_ptr = lock!(heap()).get(&(req + 4)).copied().unwrap_or(0);
    let headers = lock!(strs()).get(&headers_ptr).cloned().unwrap_or_default();
    let key = lock!(strs()).get(&key_ptr).cloned().unwrap_or_default();
    let search = format!("{}: ", key);
    for line in headers.lines() {
        if line.starts_with(&search) {
            let value = line[search.len()..].trim().to_string();
            let ptr = alloc(); lock!(strs()).insert(ptr, value); return ptr;
        }
    }
    let ptr = alloc(); lock!(strs()).insert(ptr, String::new()); ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_http_status_text(code: i64) -> i64 {
    let text = match code {
        200 => "OK", 201 => "Created", 204 => "No Content",
        301 => "Moved", 302 => "Found", 304 => "Not Modified",
        400 => "Bad Request", 401 => "Unauthorized", 403 => "Forbidden", 404 => "Not Found",
        405 => "Method Not Allowed", 500 => "Internal Server Error", 502 => "Bad Gateway",
        _ => "Unknown",
    };
    let ptr = alloc(); lock!(strs()).insert(ptr, text.to_string()); ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_http_ok(req: i64, body_ptr: i64) { respond_with_type(req, body_ptr, "text/plain"); }
#[no_mangle]
pub extern "C" fn aial_rt_http_json(req: i64, body_ptr: i64) { respond_with_type(req, body_ptr, "application/json"); }
#[no_mangle]
pub extern "C" fn aial_rt_http_html(req: i64, body_ptr: i64) { respond_with_type(req, body_ptr, "text/html"); }

fn respond_with_type(req: i64, body_ptr: i64, ct: &str) {
    let ct_ptr = alloc(); lock!(strs()).insert(ct_ptr, ct.to_string());
    aial_rt_http_respond(req, body_ptr, ct_ptr);
}

#[no_mangle]
pub extern "C" fn aial_rt_http_serve(req: i64, path_ptr: i64) {
    let path = lock!(strs()).get(&path_ptr).cloned().unwrap_or_default();
    let content = std::fs::read_to_string(&path).unwrap_or_else(|_| "404 Not Found".to_string());
    let ext = std::path::Path::new(&path).extension().and_then(|e| e.to_str()).unwrap_or("");
    let mime = match ext {
        "html" => "text/html", "css" => "text/css", "js" => "text/javascript",
        "json" => "application/json", "png" => "image/png", "jpg" | "jpeg" => "image/jpeg",
        "svg" => "image/svg+xml", "ico" => "image/x-icon",
        _ => "text/plain",
    };
    let body_ptr = alloc(); lock!(strs()).insert(body_ptr, content);
    respond_with_type(req, body_ptr, mime);
}

// ── HTML ──

#[no_mangle]
pub extern "C" fn aial_rt_html_escape(text_ptr: i64) -> i64 {
    let text = lock!(strs()).get(&text_ptr).cloned().unwrap_or_default();
    let escaped = text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;");
    let ptr = alloc(); lock!(strs()).insert(ptr, escaped); ptr
}

// ── AI Streaming ──

#[no_mangle]
pub extern "C" fn aial_rt_ai_call_raw(model: i64, prompt_ptr: i64, max_tokens: i64) -> i64 {
    let prompt = lock!(strs()).get(&prompt_ptr).cloned().unwrap_or_default();
    if std::env::var("AIAL_MOCK").is_ok() {
        let text = format!("[mock] {}", prompt);
        let ptr = alloc(); lock!(strs()).insert(ptr, text); return ptr;
    }
    let api_key = std::env::var("AIAL_KEY_DEEPSEEK").ok()
        .or_else(|| std::env::var("DEEPSEEK_API_KEY").ok())
        .unwrap_or_default();
    let client = reqwest::blocking::Client::new();
    let body = serde_json::json!({
        "model": if model == 0 { "deepseek-chat".to_string() } else { format!("model_{}", model) },
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": max_tokens,
        "stream": false
    });
    let api_url = std::env::var("AIAL_API_URL")
        .unwrap_or_else(|_| "https://api.deepseek.com/v1/chat/completions".to_string());
    let result = client.post(&api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send();
    let text = match result {
        Ok(r) => {
            match r.json::<serde_json::Value>() {
                Ok(v) => v["choices"][0]["message"]["content"].as_str().unwrap_or("[no response]").to_string(),
                Err(e) => format!("[parse error: {}]", e),
            }
        }
        Err(e) => format!("[http error: {}]", e),
    };
    let ptr = alloc(); lock!(strs()).insert(ptr, text); ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_ai_stream_start(
    model: i64, ctx_id: i64, prompt_idx: i64,
    temperature: f64, max_tokens: i64, _format: i64,
) -> i64 {
    let api_key = std::env::var("AIAL_KEY_DEEPSEEK").ok()
        .or_else(|| std::env::var("AIAL_KEY_OPENAI").ok())
        .or_else(|| std::env::var("DEEPSEEK_API_KEY").ok());
    let prompt = lock!(strs()).get(&prompt_idx).cloned().unwrap_or_default();
    let handle = alloc();
    let tokens: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let tokens_clone = tokens.clone();
    lock!(stream_tokens()).insert(handle, (tokens, 0));

    let api_key = match api_key {
        Some(k) => k,
        None => {
            lock!(tokens_clone).push("[error: no API key set] ".to_string());
            return handle;
        }
    };

    std::thread::spawn(move || {
        let client = reqwest::blocking::Client::new();
        let body = serde_json::json!({
            "model": if model == 0 { "deepseek-chat".to_string() } else { format!("model_{}", model) },
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": max_tokens,
            "temperature": temperature,
            "stream": false
        });
        let api_url = if model == 0 {
            std::env::var("AIAL_API_URL").unwrap_or_else(|_| "https://api.deepseek.com/v1/chat/completions".to_string())
        } else {
            std::env::var("AIAL_API_URL").unwrap_or_else(|_| "https://api.openai.com/v1/chat/completions".to_string())
        };
        let resp = client.post(&api_url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send();
        match resp {
            Ok(r) => {
                let status = r.status().as_u16();
                if status >= 400 {
                    lock!(tokens_clone).push(format!("[error: HTTP {}] ", status));
                    return;
                }
                match r.json::<serde_json::Value>() {
                    Ok(v) => {
                        let text = v["choices"][0]["message"]["content"].as_str().unwrap_or("[no response]").to_string();
                        for word in text.split_whitespace() {
                            lock!(tokens_clone).push(format!("{} ", word));
                        }
                    }
                    Err(_) => {
                        lock!(tokens_clone).push("[parse error] ".to_string());
                    }
                }
            }
            Err(e) => {
                lock!(tokens_clone).push(format!("[error: {}] ", e));
            }
        }
    });

    handle
}

#[no_mangle]
pub extern "C" fn aial_rt_ai_stream_read(handle: i64) -> i64 {
    let ptr = alloc();
    let token_opt: Option<String> = {
        let mut map = lock!(stream_tokens());
        if let Some((tokens, pos)) = map.get_mut(&handle) {
            if let Ok(guard) = tokens.lock() {
                if (*pos as usize) < guard.len() {
                    let token = guard[*pos as usize].clone();
                    *pos += 1;
                    Some(token)
                } else { None }
            } else { None }
        } else { None }
    };
    match token_opt {
        Some(token) => { lock!(strs()).insert(ptr, token); }
        None => { lock!(strs()).insert(ptr, String::new()); }
    }
    ptr
}

// ── I/O ──

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
pub extern "C" fn aial_rt_print(text_ptr: i64) {
    let text = lock!(strs()).get(&text_ptr).cloned().unwrap_or_default();
    use std::io::Write;
    print!("{}", text);
    std::io::stdout().flush().ok();
}

/// Read a full escape sequence after ESC (0x1b).
/// Returns the complete sequence string (e.g., "\x1b[A", "\x1b[200~").
/// Falls back to "\x1b" if no more bytes arrive.
fn read_escape_sequence(timeout_ms: i64) -> String {
    let mut seq = vec![0x1bu8];
    // Read next byte to determine sequence type
    match read_byte_timeout(timeout_ms) {
        Some(b) => {
            seq.push(b);
            if b == b'[' {
                // CSI sequence — read until terminating char (letter or ~), max 8 more bytes
                for _ in 0..8 {
                    match read_byte_timeout(timeout_ms.min(30)) {
                        Some(c) => {
                            seq.push(c);
                            if (c >= b'A' && c <= b'Z') || (c >= b'a' && c <= b'z') || c == b'~' {
                                break;
                            }
                        }
                        None => break,
                    }
                }
            } else if b == b'O' {
                // SS3 sequence (e.g., F1-F4 on some terminals) — one more byte
                if let Some(c) = read_byte_timeout(timeout_ms.min(30)) { seq.push(c); }
            }
            // For other ESC sequences, just return what we have
        }
        None => {} // bare ESC
    }
    String::from_utf8_lossy(&seq).to_string()
}

fn read_paste_data(ptr: i64, is_start: bool) -> i64 {
    if is_start { lock!(paste_active()).clone_from(&true); }
    let mut data: Vec<u8> = Vec::new();
    loop {
        match read_byte_timeout(100) {
            Some(b) => {
                if b == 0x1b {
                    // Check for paste end marker \x1b[201~
                    let rest = read_escape_sequence(30);
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

/// Map raw bytes to a named key string.
/// Returns names like "ENTER", "UP", "^Q", "A", "中" etc.
fn key_name(bytes: &[u8]) -> &'static str {
    match bytes {
        b"\r" | b"\n" => "ENTER",
        b"\x7f" | b"\x08" => "BACKSPACE",
        b"\t" => "TAB",
        b"\x1b" => "ESC",
        b"\x1b[A" => "UP",
        b"\x1b[B" => "DOWN",
        b"\x1b[C" => "RIGHT",
        b"\x1b[D" => "LEFT",
        b"\x1b[1~" | b"\x1b[H" => "HOME",
        b"\x1b[4~" | b"\x1b[F" => "END",
        b"\x1b[5~" => "PAGEUP",
        b"\x1b[6~" => "PAGEDOWN",
        b"\x1b[3~" => "DELETE",
        b"\x1bOP" | b"\x1b[11~" => "F1",
        b"\x1bOQ" | b"\x1b[12~" => "F2",
        b"\x1bOR" | b"\x1b[13~" => "F3",
        b"\x1bOS" | b"\x1b[14~" => "F4",
        b"\x11" => "CTRL_Q",
        b"\x0c" => "CTRL_L",
        b"\x04" => "CTRL_D",
        _ => "RAW",
    }
}

fn read_key_bytes_blocking() -> Vec<u8> {
    match read_byte_timeout(-1) {
        Some(0x1b) => {
            read_escape_sequence(50).into_bytes()
        }
        Some(b) if b >= 0xC0 => {
            let extra = if b >= 0xF0 { 3 } else if b >= 0xE0 { 2 } else { 1 };
            let mut bytes = vec![b];
            for _ in 0..extra {
                match read_byte_timeout(20) { Some(c) => bytes.push(c), None => break }
            }
            bytes
        }
        Some(b) => vec![b],
        None => vec![],
    }
}

fn read_key_bytes_timeout(ms: i64) -> Vec<u8> {
    match read_byte_timeout(ms) {
        Some(0x1b) => read_escape_sequence(30.min(ms)).into_bytes(),
        Some(b) if b >= 0xC0 => {
            let extra = if b >= 0xF0 { 3 } else if b >= 0xE0 { 2 } else { 1 };
            let mut bytes = vec![b];
            for _ in 0..extra {
                match read_byte_timeout(20) { Some(c) => bytes.push(c), None => break }
            }
            bytes
        }
        Some(b) => vec![b],
        None => vec![],
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_io_readkey() -> i64 {
    let ptr = alloc();
    let bytes = read_key_bytes_blocking();
    if bytes.is_empty() {
        lock!(strs()).insert(ptr, String::new());
        return ptr;
    }
    // Check for paste sequences
    let seq = String::from_utf8_lossy(&bytes);
    if seq.starts_with("\x1b[200") || seq.starts_with("\x1b[201") {
        return read_paste_data(ptr, seq.starts_with("\x1b[200"));
    }
    // Map to named key
    let name = key_name(&bytes);
    if name == "RAW" {
        // For regular characters (including UTF-8), return the string itself
        lock!(strs()).insert(ptr, String::from_utf8_lossy(&bytes).to_string());
    } else {
        lock!(strs()).insert(ptr, name.to_string());
    }
    ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_io_readkey_timeout(ms: i64) -> i64 {
    let ptr = alloc();
    let bytes = read_key_bytes_timeout(ms);
    if bytes.is_empty() {
        lock!(strs()).insert(ptr, String::new());
        return ptr;
    }
    let seq = String::from_utf8_lossy(&bytes);
    if seq.starts_with("\x1b[200") || seq.starts_with("\x1b[201") {
        return read_paste_data(ptr, seq.starts_with("\x1b[200"));
    }
    let name = key_name(&bytes);
    if name == "RAW" {
        lock!(strs()).insert(ptr, String::from_utf8_lossy(&bytes).to_string());
    } else {
        lock!(strs()).insert(ptr, name.to_string());
    }
    ptr
}

static SAVED_TERMIOS: OnceLock<Mutex<libc::termios>> = OnceLock::new();
static PASTE_BUF: OnceLock<Mutex<Vec<u8>>> = OnceLock::new();
static PASTE_ACTIVE: OnceLock<Mutex<bool>> = OnceLock::new();

fn paste_buf() -> &'static Mutex<Vec<u8>> { PASTE_BUF.get_or_init(|| Mutex::new(Vec::new())) }
fn paste_active() -> &'static Mutex<bool> { PASTE_ACTIVE.get_or_init(|| Mutex::new(false)) }

// Try to read one byte from stdin with poll, return 0 if no data after ms
fn read_byte_timeout(ms: i64) -> Option<u8> {
    use std::os::unix::io::AsRawFd;
    let fd = std::io::stdin().as_raw_fd();
    let mut fds = [libc::pollfd { fd, events: libc::POLLIN, revents: 0 }];
    let ret = unsafe { libc::poll(fds.as_mut_ptr(), 1, ms as libc::c_int) };
    if ret > 0 {
        use std::io::Read;
        let mut buf = [0u8; 1];
        if std::io::stdin().read(&mut buf).unwrap_or(0) > 0 { Some(buf[0]) } else { None }
    } else { None }
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

// ── Context Memory (SQLite-backed) ──

static DB_CONNS: OnceLock<Mutex<HashMap<i64, Arc<Mutex<rusqlite::Connection>>>>> = OnceLock::new();
fn db_conns() -> &'static Mutex<HashMap<i64, Arc<Mutex<rusqlite::Connection>>>> {
    DB_CONNS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_open_memory(path_ptr: i64) -> i64 {
    let path = lock!(strs()).get(&path_ptr).cloned().unwrap_or_else(|| ":memory:".to_string());
    let conn = match rusqlite::Connection::open(&path) { Ok(c) => c, Err(e) => { eprintln!("[aial-rt] db open error: {}", e); return -1; } };
    conn.execute("CREATE TABLE IF NOT EXISTS messages (id INTEGER PRIMARY KEY AUTOINCREMENT, session TEXT, role TEXT, content TEXT, ts INTEGER DEFAULT (unixepoch()))", []).ok();
    let handle = alloc();
    let conn_arc = Arc::new(Mutex::new(conn));
    lock!(db_conns()).insert(handle, conn_arc);
    handle
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_save_message(db: i64, session_ptr: i64, role_ptr: i64, content_ptr: i64) {
    let strs = lock!(strs());
    let session = strs.get(&session_ptr).cloned().unwrap_or_default();
    let role = strs.get(&role_ptr).cloned().unwrap_or_default();
    let content = strs.get(&content_ptr).cloned().unwrap_or_default();
    drop(strs);
    if let Some(conn) = lock!(db_conns()).get(&db).cloned() {
        let c = lock!(conn);
        c.execute("INSERT INTO messages (session, role, content) VALUES (?1, ?2, ?3)", rusqlite::params![session, role, content]).ok();
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_load_messages(db: i64, session_ptr: i64, limit: i64) -> i64 {
    let session = lock!(strs()).get(&session_ptr).cloned().unwrap_or_default();
    let json = if let Some(conn) = lock!(db_conns()).get(&db).cloned() {
        let c = lock!(conn);
        let mut stmt = match c.prepare("SELECT role, content, ts FROM messages WHERE session=?1 ORDER BY id ASC LIMIT ?2") { Ok(s) => s, Err(_) => { return alloc_empty(); } };
        let rows: Vec<String> = stmt.query_map(rusqlite::params![session, limit], |row| {
            let r: String = row.get(0)?;
            let ct: String = row.get(1)?;
            let ts: i64 = row.get(2)?;
            Ok(format!(r#"{{"role":"{}","content":"{}","ts":{}}}"#, r, ct.replace('"', r#"\""#).replace('\n', r#"\n"#), ts))
        }).ok().map(|m| m.filter_map(|r| r.ok()).collect::<Vec<String>>()).unwrap_or_default();
        format!("[{}]", rows.join(","))
    } else { "[]".to_string() };
    let ptr = alloc(); lock!(strs()).insert(ptr, json); ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_load_messages_since(db: i64, session_ptr: i64, ts: i64) -> i64 {
    let session = lock!(strs()).get(&session_ptr).cloned().unwrap_or_default();
    let json = if let Some(conn) = lock!(db_conns()).get(&db).cloned() {
        let c = lock!(conn);
        let mut stmt = match c.prepare("SELECT role, content, ts FROM messages WHERE session=?1 AND ts>=?2 ORDER BY id ASC") { Ok(s) => s, Err(_) => { return alloc_empty(); } };
        let rows: Vec<String> = stmt.query_map(rusqlite::params![session, ts], |row| {
            let r: String = row.get(0)?;
            let ct: String = row.get(1)?;
            let t: i64 = row.get(2)?;
            Ok(format!(r#"{{"role":"{}","content":"{}","ts":{}}}"#, r, ct.replace('"', r#"\""#).replace('\n', r#"\n"#), t))
        }).ok().map(|m| m.filter_map(|r| r.ok()).collect::<Vec<String>>()).unwrap_or_default();
        format!("[{}]", rows.join(","))
    } else { "[]".to_string() };
    let ptr = alloc(); lock!(strs()).insert(ptr, json); ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_close_memory(db: i64) {
    lock!(db_conns()).remove(&db);
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_last_error() -> i64 {
    let msg = LAST_ERROR.get().and_then(|e| Some(lock!(e).clone())).unwrap_or_default();
    let ptr = alloc();
    lock!(strs()).insert(ptr, msg);
    ptr
}

// ── Time ──

// ── Actor ──

static ACTOR_MAILBOXES: OnceLock<Mutex<HashMap<i64, Arc<Mutex<Vec<String>>>>>> = OnceLock::new();
static ACTOR_NEXT_PID: Mutex<i64> = Mutex::new(1);
static ACTOR_ERRORS: OnceLock<Mutex<HashMap<i64, String>>> = OnceLock::new();

fn actor_mailboxes() -> &'static Mutex<HashMap<i64, Arc<Mutex<Vec<String>>>>> {
    ACTOR_MAILBOXES.get_or_init(|| Mutex::new(HashMap::new()))
}
fn actor_errors() -> &'static Mutex<HashMap<i64, String>> {
    ACTOR_ERRORS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[no_mangle]
pub extern "C" fn aial_rt_actor_spawn() -> i64 {
    let pid = { let mut n = lock!(ACTOR_NEXT_PID); let p = *n; *n += 1; p };
    lock!(actor_mailboxes()).insert(pid, Arc::new(Mutex::new(Vec::new())));
    pid
}

#[no_mangle]
pub extern "C" fn aial_rt_actor_spawn_handler(fn_ptr: i64, init_ptr: i64) -> i64 {
    let fn_name = lock!(strs()).get(&fn_ptr).cloned().unwrap_or_default();
    let init_msg = lock!(strs()).get(&init_ptr).cloned().unwrap_or_default();
    let pid = { let mut n = lock!(ACTOR_NEXT_PID); let p = *n; *n += 1; p };
    lock!(actor_mailboxes()).insert(pid, Arc::new(Mutex::new(Vec::new())));

    // Spawn thread — looks up AIAL function via dlsym
    std::thread::spawn(move || {
        // Push init message
        {
            let mboxes = lock!(actor_mailboxes());
            if let Some(mbox) = mboxes.get(&pid) {
                lock!(mbox).push(init_msg);
            }
        }
        // Try to call the handler function via dlsym
        type HandlerFn = extern "C" fn(i64);
        let c_name = std::ffi::CString::new(fn_name.as_str()).unwrap_or_default();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            unsafe {
                let handle = libc::dlopen(std::ptr::null(), libc::RTLD_LAZY);
                if handle.is_null() { return Err("dlopen failed".to_string()); }
                let ptr = libc::dlsym(handle, c_name.as_ptr());
                if ptr.is_null() {
                    libc::dlclose(handle);
                    return Err(format!("handler not found: {}", fn_name));
                }
                let handler: HandlerFn = std::mem::transmute(ptr);
                handler(pid);
                libc::dlclose(handle);
                Ok(())
            }
        }));
        match result {
            Err(_panic) => {
                lock!(actor_errors()).insert(pid, "actor panicked".to_string());
            }
            Ok(Err(msg)) => {
                lock!(actor_errors()).insert(pid, msg);
            }
            _ => {}
        }
    });

    pid
}

#[no_mangle]
pub extern "C" fn aial_rt_actor_send(pid: i64, msg_ptr: i64) {
    let msg = lock!(strs()).get(&msg_ptr).cloned().unwrap_or_default();
    if let Some(mbox) = lock!(actor_mailboxes()).get(&pid) {
        lock!(mbox).push(msg);
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_actor_receive(pid: i64) -> i64 {
    let mbox = {
        let mboxes = lock!(actor_mailboxes());
        mboxes.get(&pid).cloned()
    };
    let ptr = alloc();
    if let Some(mbox) = mbox {
        loop {
            if let Some(msg) = lock!(mbox).pop() {
                lock!(strs()).insert(ptr, msg);
                return ptr;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    lock!(strs()).insert(ptr, String::new());
    ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_actor_try_receive(pid: i64) -> i64 {
    let mbox = {
        let mboxes = lock!(actor_mailboxes());
        mboxes.get(&pid).cloned()
    };
    let ptr = alloc();
    if let Some(mbox) = mbox {
        if let Some(msg) = lock!(mbox).pop() {
            lock!(strs()).insert(ptr, msg);
            return ptr;
        }
    }
    lock!(strs()).insert(ptr, String::new());
    ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_actor_recv_timeout(pid: i64, timeout_ms: i64) -> i64 {
    let mbox = {
        let mboxes = lock!(actor_mailboxes());
        mboxes.get(&pid).cloned()
    };
    let ptr = alloc();
    if let Some(mbox) = mbox {
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms as u64);
        loop {
            if let Some(msg) = lock!(mbox).pop() {
                lock!(strs()).insert(ptr, msg);
                return ptr;
            }
            if std::time::Instant::now() >= deadline { break; }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
    lock!(strs()).insert(ptr, String::new());
    ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_actor_error(pid: i64) -> i64 {
    let err = lock!(actor_errors()).get(&pid).cloned().unwrap_or_default();
    let ptr = alloc();
    lock!(strs()).insert(ptr, err);
    ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_time_sleep(ms: i64) {
    std::thread::sleep(std::time::Duration::from_millis(ms as u64));
}

// ── FFI (stub — requires libffi) ──

#[no_mangle]
pub extern "C" fn aial_rt_ffi_load(_path: i64) -> i64 { -1 }
#[no_mangle]
pub extern "C" fn aial_rt_ffi_call(_handle: i64, _fn_name: i64, _a1: i64, _a2: i64, _a3: i64, _a4: i64, _a5: i64, _a6: i64) -> i64 { -1 }
#[no_mangle]
pub extern "C" fn aial_rt_ffi_close(_handle: i64) {}

// ─── Map (hash table) ───
static MAPS: OnceLock<Mutex<HashMap<i64, HashMap<String, String>>>> = OnceLock::new();
static NEXT_MAP: Mutex<i64> = Mutex::new(1);

fn maps() -> &'static Mutex<HashMap<i64, HashMap<String, String>>> {
    MAPS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[no_mangle]
#[no_mangle]
pub extern "C" fn aial_rt_map_new() -> i64 {
    let mut next = lock!(NEXT_MAP);
    let id = *next;
    *next += 1;
    drop(next);
    lock!(maps()).insert(id, HashMap::new());
    id
}

#[no_mangle]
pub extern "C" fn aial_rt_map_set(handle: i64, key_idx: i64, value_idx: i64) {
    let st = lock!(strs());
    let k = st.get(&key_idx).cloned().unwrap_or_default();
    let v = st.get(&value_idx).cloned().unwrap_or_default();
    drop(st);
    lock!(maps()).get_mut(&handle).map(|m| m.insert(k, v));
}

#[no_mangle]
pub extern "C" fn aial_rt_map_get(handle: i64, key_idx: i64) -> i64 {
    let k = {
        let st = lock!(strs());
        st.get(&key_idx).cloned().unwrap_or_default()
    };
    let maps = lock!(maps());
    if let Some(m) = maps.get(&handle) {
        if let Some(v) = m.get(&k) {
            let addr = alloc();
            lock!(strs()).insert(addr, v.clone());
            return addr;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn aial_rt_map_has(handle: i64, key_idx: i64) -> i64 {
    let k = {
        let st = lock!(strs());
        st.get(&key_idx).cloned().unwrap_or_default()
    };
    match lock!(maps()).get(&handle) {
        Some(m) if m.contains_key(&k) => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_map_remove(handle: i64, key_idx: i64) {
    let k = {
        let st = lock!(strs());
        st.get(&key_idx).cloned().unwrap_or_default()
    };
    lock!(maps()).get_mut(&handle).map(|m| m.remove(&k));
}

// ─── Token estimation ───
#[no_mangle]
pub extern "C" fn aial_rt_token_estimate(text_idx: i64) -> i64 {
    let s = lock!(strs()).get(&text_idx).cloned().unwrap_or_default();
    let bytes = s.len();
    let cjk = s.chars().filter(|&c| c >= '\u{4E00}' && c <= '\u{9FFF}').count();
    let ascii = bytes.saturating_sub(cjk * 3);
    (ascii as i64 / 4 + cjk as i64 * 2 / 3).max(1)
}

// ─── Heap (priority queue) ───
static HEAPS: OnceLock<Mutex<HashMap<i64, Vec<(String, i64)>>>> = OnceLock::new();
static NEXT_HEAP: Mutex<i64> = Mutex::new(1);

fn heaps() -> &'static Mutex<HashMap<i64, Vec<(String, i64)>>> {
    HEAPS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[no_mangle]
pub extern "C" fn aial_rt_heap_new() -> i64 {
    let mut next = lock!(NEXT_HEAP);
    let id = *next; *next += 1; drop(next);
    lock!(heaps()).insert(id, Vec::new());
    id
}

#[no_mangle]
pub extern "C" fn aial_rt_heap_push(handle: i64, value_idx: i64, priority: i64) {
    let v = lock!(strs()).get(&value_idx).cloned().unwrap_or_default();
    lock!(heaps()).get_mut(&handle).map(|h| h.push((v, priority)));
}

#[no_mangle]
pub extern "C" fn aial_rt_heap_pop(handle: i64) -> i64 {
    let mut heaps = lock!(heaps());
    if let Some(h) = heaps.get_mut(&handle) {
        if h.is_empty() { return 0; }
        // Find max priority
        let mut best = 0usize;
        for (i, (_, p)) in h.iter().enumerate() { if *p > h[best].1 { best = i; } }
        let (val, _) = h.swap_remove(best);
        let addr = alloc();
        lock!(strs()).insert(addr, val);
        return addr;
    }
    0
}

#[no_mangle]
pub extern "C" fn aial_rt_heap_peek(handle: i64) -> i64 {
    let heaps = lock!(heaps());
    if let Some(h) = heaps.get(&handle) {
        if let Some(best) = h.iter().max_by_key(|(_, p)| p) {
            let addr = alloc();
            lock!(strs()).insert(addr, best.0.clone());
            return addr;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn aial_rt_heap_len(handle: i64) -> i64 {
    lock!(heaps()).get(&handle).map(|h| h.len() as i64).unwrap_or(0)
}

// ─── Array ───
static ARRAYS: OnceLock<Mutex<HashMap<i64, Vec<String>>>> = OnceLock::new();
static NEXT_ARR: Mutex<i64> = Mutex::new(1);

fn arrays() -> &'static Mutex<HashMap<i64, Vec<String>>> {
    ARRAYS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[no_mangle]
pub extern "C" fn aial_rt_array_new() -> i64 {
    let mut next = lock!(NEXT_ARR);
    let id = *next; *next += 1; drop(next);
    lock!(arrays()).insert(id, Vec::new());
    id
}

#[no_mangle]
pub extern "C" fn aial_rt_array_push(handle: i64, value_idx: i64) {
    let v = lock!(strs()).get(&value_idx).cloned().unwrap_or_default();
    lock!(arrays()).get_mut(&handle).map(|a| a.push(v));
}

#[no_mangle]
pub extern "C" fn aial_rt_array_sort(handle: i64) {
    lock!(arrays()).get_mut(&handle).map(|a| a.sort());
}

#[no_mangle]
pub extern "C" fn aial_rt_array_get(handle: i64, index: i64) -> i64 {
    let arrs = lock!(arrays());
    if let Some(a) = arrs.get(&handle) {
        if let Some(v) = a.get(index as usize) {
            let addr = alloc();
            lock!(strs()).insert(addr, v.clone());
            return addr;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn aial_rt_array_len(handle: i64) -> i64 {
    lock!(arrays()).get(&handle).map(|a| a.len() as i64).unwrap_or(0)
}

// ─── Key management (shared with aial CLI via ~/.aial/keys.json) ───

fn keys_path() -> std::path::PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    std::path::PathBuf::from(home).join(".aial").join("keys.json")
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct KeyStore { keys: std::collections::HashMap<String, KeyEntry> }

#[derive(Serialize, Deserialize)]
struct KeyEntry { key: String, created_at: String }

fn load_key_store(path: &std::path::PathBuf) -> KeyStore {
    if !path.exists() { return KeyStore { keys: std::collections::HashMap::new() }; }
    let content = std::fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or(KeyStore { keys: std::collections::HashMap::new() })
}

fn save_key_store(path: &std::path::PathBuf, store: &KeyStore) -> bool {
    let content = match serde_json::to_string_pretty(store) { Ok(c) => c, Err(_) => return false };
    if let Some(parent) = path.parent() { let _ = std::fs::create_dir_all(parent); }
    std::fs::write(path, &content).is_ok()
}

#[no_mangle]
pub extern "C" fn aial_rt_key_set(provider_ptr: i64, key_ptr: i64) -> i64 {
    let provider = { let st = lock!(strs()); st.get(&provider_ptr).cloned().unwrap_or_default() };
    let key = { let st = lock!(strs()); st.get(&key_ptr).cloned().unwrap_or_default() };
    if provider.is_empty() || key.is_empty() { return 0; }
    let path = keys_path();
    let mut store = load_key_store(&path);
    store.keys.insert(provider.clone(), KeyEntry { key, created_at: String::new() });
    if save_key_store(&path, &store) {
        #[cfg(unix)] { use std::os::unix::fs::PermissionsExt; if let Ok(m) = std::fs::metadata(&path) { let mut p = m.permissions(); p.set_mode(0o600); let _ = std::fs::set_permissions(&path, p); } }
        1
    } else { 0 }
}

#[no_mangle]
pub extern "C" fn aial_rt_key_exists(provider_ptr: i64) -> i64 {
    let provider = { let st = lock!(strs()); st.get(&provider_ptr).cloned().unwrap_or_default() };
    let env_var = format!("AIAL_KEY_{}", provider.to_uppercase());
    if std::env::var(&env_var).is_ok() { return 1; }
    // Also check provider-specific env vars used by AI call functions
    if provider == "deepseek" && std::env::var("DEEPSEEK_API_KEY").is_ok() { return 1; }
    let path = keys_path();
    let store = load_key_store(&path);
    if store.keys.contains_key(&provider) { return 1; }
    0
}

#[no_mangle]
pub extern "C" fn aial_rt_key_delete(provider_ptr: i64) -> i64 {
    let provider = { let st = lock!(strs()); st.get(&provider_ptr).cloned().unwrap_or_default() };
    let path = keys_path();
    let mut store = load_key_store(&path);
    store.keys.remove(&provider);
    if save_key_store(&path, &store) { 1 } else { 0 }
}

#[no_mangle] pub extern "C" fn aial_rt_tool_dispatch(_n: i64, _a: i64) -> i64 { 0 }
#[no_mangle] pub extern "C" fn aial_rt_cap_check(_c: i64) -> i64 { 1 }

#[cfg(test)]
mod tests {
    use super::key_name;

    #[test]
    fn test_enter() { assert_eq!(key_name(b"\r"), "ENTER"); assert_eq!(key_name(b"\n"), "ENTER"); }
    #[test]
    fn test_backspace() { assert_eq!(key_name(b"\x7f"), "BACKSPACE"); assert_eq!(key_name(b"\x08"), "BACKSPACE"); }
    #[test]
    fn test_tab() { assert_eq!(key_name(b"\t"), "TAB"); }
    #[test]
    fn test_esc() { assert_eq!(key_name(b"\x1b"), "ESC"); }
    #[test]
    fn test_arrows() {
        assert_eq!(key_name(b"\x1b[A"), "UP");
        assert_eq!(key_name(b"\x1b[B"), "DOWN");
        assert_eq!(key_name(b"\x1b[C"), "RIGHT");
        assert_eq!(key_name(b"\x1b[D"), "LEFT");
    }
    #[test]
    fn test_ctrl_keys() {
        assert_eq!(key_name(b"\x11"), "CTRL_Q");
        assert_eq!(key_name(b"\x0c"), "CTRL_L");
        assert_eq!(key_name(b"\x04"), "CTRL_D");
    }
    #[test]
    fn test_raw() { assert_eq!(key_name(b"A"), "RAW"); assert_eq!(key_name(b"\xe4\xb8\xad"), "RAW"); }
    #[test]
    fn test_function_keys() {
        assert_eq!(key_name(b"\x1bOP"), "F1");
        assert_eq!(key_name(b"\x1bOQ"), "F2");
        assert_eq!(key_name(b"\x1b[5~"), "PAGEUP");
        assert_eq!(key_name(b"\x1b[6~"), "PAGEDOWN");
    }
    #[test]
    fn test_home_end() {
        assert_eq!(key_name(b"\x1b[H"), "HOME");
        assert_eq!(key_name(b"\x1b[F"), "END");
    }
}
