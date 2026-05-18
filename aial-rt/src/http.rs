use super::*;

use crate::json::json_value_to_string_runtime;

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
