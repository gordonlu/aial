use super::*;

#[no_mangle]
pub extern "C" fn aial_rt_ai_call(
    model: i64, ctx_id: i64, prompt_idx: i64,
    temperature: f64, max_tokens: i64, _format: i64,
) -> i64 {
    let prompt = lock!(strs()).get(&prompt_idx).cloned().unwrap_or_default();

    if std::env::var("AIAL_MOCK").is_ok() {
        let text = format!("[mock AI response to: {}]", prompt);
        let text_ptr = alloc(); lock!(strs()).insert(text_ptr, text.clone());
        let resp_ptr = alloc_block(4);
        let mut h = lock!(heap());
        h.insert(resp_ptr, 0); h.insert(resp_ptr + 1, text_ptr); h.insert(resp_ptr + 2, 0); h.insert(resp_ptr + 3, 0);
        return resp_ptr;
    }

    let api_key = std::env::var("AIAL_KEY_DEEPSEEK").ok()
        .or_else(|| std::env::var("DEEPSEEK_API_KEY").ok())
        .unwrap_or_default();

    // Build messages from context
    let mut messages: Vec<serde_json::Value> = Vec::new();
    if let Some(ctx) = lock!(ctxs()).get(&ctx_id) {
        if !ctx.system_prompt.is_empty() {
            messages.push(serde_json::json!({"role": "system", "content": ctx.system_prompt}));
        }
        for (role, content) in &ctx.messages {
            messages.push(serde_json::json!({"role": role, "content": content}));
        }
    }
    messages.push(serde_json::json!({"role": "user", "content": prompt}));

    let model_name = if model == 0 {
        std::env::var("AIAL_MODEL_0").unwrap_or_else(|_| "deepseek-v4-flash".to_string())
    } else { format!("model_{}", model) };

    let client = reqwest::blocking::Client::new();
    let body = serde_json::json!({
        "model": model_name,
        "messages": messages,
        "max_tokens": max_tokens,
        "temperature": temperature,
        "stream": false,
        "thinking": {"type": "enabled"}
    });
    let api_url = std::env::var("AIAL_API_URL")
        .unwrap_or_else(|_| "https://api.deepseek.com/chat/completions".to_string());

    let text = match client.post(&api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
    {
        Ok(r) => match r.json::<serde_json::Value>() {
            Ok(v) => v["choices"][0]["message"]["content"].as_str().unwrap_or("[no response]").to_string(),
            Err(e) => format!("[error: parse failed: {}]", e),
        },
        Err(e) => format!("[error: {}]", e),
    };

    if let Some(ctx) = lock!(ctxs()).get_mut(&ctx_id) {
        ctx.tokens_used += max_tokens / 2;
    }

    let text_ptr = alloc(); lock!(strs()).insert(text_ptr, text);
    let resp_ptr = alloc_block(4);
    let mut h = lock!(heap());
    h.insert(resp_ptr, 0); h.insert(resp_ptr + 1, text_ptr); h.insert(resp_ptr + 2, 0); h.insert(resp_ptr + 3, 0);
    resp_ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_ai_call_many(json_ptr: i64) -> i64 {
    let json_str = lock!(strs()).get(&json_ptr).cloned().unwrap_or_default();
    let groups: Vec<serde_json::Value> = serde_json::from_str(&json_str).unwrap_or_default();
    let results = Arc::new(Mutex::new(vec![String::new(); groups.len()]));
    let mut handles = Vec::new();

    for (i, group) in groups.into_iter().enumerate() {
        let results = results.clone();
        let api_key = std::env::var("AIAL_KEY_DEEPSEEK").ok()
            .or_else(|| std::env::var("DEEPSEEK_API_KEY").ok());
        let handle = std::thread::spawn(move || {
            let model_code = group["model"].as_i64().unwrap_or(0);
            let prompt = group["prompt"].as_str().unwrap_or("");
            let max_tokens = group["max_tokens"].as_i64().unwrap_or(256);
            let model_name = if model_code == 0 {
                std::env::var("AIAL_MODEL_0").unwrap_or_else(|_| "deepseek-v4-flash".to_string())
            } else { format!("model_{}", model_code) };

            let api_key = match &api_key { Some(k) => k.clone(), None => { lock!(results)[i] = "[error: no API key]".to_string(); return; } };

            let client = reqwest::blocking::Client::new();
            let body = serde_json::json!({
                "model": model_name, "messages": [{"role": "user", "content": prompt}],
                "max_tokens": max_tokens, "temperature": 1.0
            });
            let api_url = if model_code == 0 {
                std::env::var("AIAL_API_URL").unwrap_or_else(|_| "https://api.deepseek.com/chat/completions".to_string())
            } else { "https://api.openai.com/v1/chat/completions".to_string() };

            match client.post(&api_url)
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json").json(&body).send()
            {
                Ok(resp) => {
                    let text = resp.text().unwrap_or_default();
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                        let content = v["choices"][0]["message"]["content"].as_str().unwrap_or(&text);
                        lock!(results)[i] = content.to_string();
                    }
                }
                Err(e) => { lock!(results)[i] = format!("[error: {}]", e); }
            }
        });
        handles.push(handle);
    }
    for h in handles { h.join().ok(); }

    let base = alloc();
    let count = Arc::into_inner(results).unwrap().into_inner().unwrap();
    lock!(heap()).insert(base, count.len() as i64);
    for (i, s) in count.iter().enumerate() {
        let ptr = alloc(); lock!(strs()).insert(ptr, s.clone());
        lock!(heap()).insert(base + 1 + i as i64, ptr);
    }
    base
}

#[no_mangle]
pub extern "C" fn aial_rt_ai_call_race(json_ptr: i64) -> i64 {
    let json_str = lock!(strs()).get(&json_ptr).cloned().unwrap_or_default();
    let groups: Vec<serde_json::Value> = serde_json::from_str(&json_str).unwrap_or_default();
    let winner = Arc::new(Mutex::new(None));
    let mut handles = Vec::new();

    for (_i, group) in groups.into_iter().enumerate() {
        let winner = winner.clone();
        let api_key = std::env::var("AIAL_KEY_DEEPSEEK").ok()
            .or_else(|| std::env::var("DEEPSEEK_API_KEY").ok());
        let handle = std::thread::spawn(move || {
            if lock!(winner).is_some() { return; }

            let model_code = group["model"].as_i64().unwrap_or(0);
            let prompt = group["prompt"].as_str().unwrap_or("");
            let max_tokens = group["max_tokens"].as_i64().unwrap_or(256);
            let model_name = if model_code == 0 {
                std::env::var("AIAL_MODEL_0").unwrap_or_else(|_| "deepseek-v4-flash".to_string())
            } else { format!("model_{}", model_code) };

            let api_key = match &api_key { Some(k) => k.clone(), None => { *lock!(winner) = Some("[error: no API key]".to_string()); return; } };

            let client = reqwest::blocking::Client::new();
            let body = serde_json::json!({
                "model": model_name, "messages": [{"role": "user", "content": prompt}],
                "max_tokens": max_tokens, "temperature": 1.0
            });
            let api_url = if model_code == 0 {
                std::env::var("AIAL_API_URL").unwrap_or_else(|_| "https://api.deepseek.com/chat/completions".to_string())
            } else { "https://api.openai.com/v1/chat/completions".to_string() };

            match client.post(&api_url)
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json").json(&body).send()
            {
                Ok(resp) => {
                    let text = resp.text().unwrap_or_default();
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                        let content = v["choices"][0]["message"]["content"].as_str().unwrap_or(&text);
                        let mut w = lock!(winner);
                        if w.is_none() { *w = Some(content.to_string()); }
                    }
                }
                Err(e) => { let mut w = lock!(winner); if w.is_none() { *w = Some(format!("[error: {}]", e)); } }
            }
        });
        handles.push(handle);
    }
    for h in handles { h.join().ok(); }

    let result = Arc::into_inner(winner).unwrap().into_inner().unwrap().unwrap_or_else(|| "[error: all racers failed]".to_string());
    let ptr = alloc(); lock!(strs()).insert(ptr, result); ptr
}

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
    let model_name = if model == 0 {
        std::env::var("AIAL_MODEL_0").unwrap_or_else(|_| "deepseek-v4-flash".to_string())
    } else { format!("model_{}", model) };
    let client = reqwest::blocking::Client::new();
    let body = serde_json::json!({
        "model": model_name,
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": max_tokens,
        "stream": false
    });
    let api_url = std::env::var("AIAL_API_URL")
        .unwrap_or_else(|_| "https://api.deepseek.com/chat/completions".to_string());
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

// ── Context ──

#[no_mangle]
pub extern "C" fn aial_rt_ctx_new(prompt_ptr: i64, budget: i64, _strategy: i64, _ws: i64) -> i64 {
    let system_prompt = lock!(strs()).get(&prompt_ptr).cloned().unwrap_or_default();
    let mut n = lock!(NEXT_CTX);
    let id = *n; *n += 1;
    lock!(ctxs()).insert(id, ContextState { system_prompt, token_budget: budget, tokens_used: 0, messages: Vec::new() });
    id
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_current() -> i64 { 0 }  // returns 0 = no active context

#[no_mangle]
pub extern "C" fn aial_rt_ctx_budget(id: i64) -> i64 {
    lock!(ctxs()).get(&id).map_or(0, |s| s.token_budget - s.tokens_used)
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_forget(ctx_id: i64, msg_id: i64) {
    if let Some(ctx) = lock!(ctxs()).get_mut(&ctx_id) {
        let idx = msg_id as usize;
        if idx < ctx.messages.len() { ctx.messages.remove(idx); }
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_ctx_reflect(ctx_id: i64) -> i64 {
    let summary = if let Some(ctx) = lock!(ctxs()).get(&ctx_id) {
        format!("context #{}: {} messages, {} / {} tokens used",
            ctx_id, ctx.messages.len(), ctx.tokens_used, ctx.token_budget)
    } else { "context not found".to_string() };
    let ptr = alloc(); lock!(strs()).insert(ptr, summary); ptr
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

// ── Extractors ──

#[no_mangle]
pub extern "C" fn aial_rt_extract_ai_text(resp: i64) -> i64 {
    lock!(heap()).get(&(resp + 1)).copied().unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn aial_rt_extract_ai_variant(resp: i64) -> i64 {
    lock!(heap()).get(&resp).copied().unwrap_or(-1)
}

#[no_mangle]
pub extern "C" fn aial_rt_extract_ai_reasoning(resp: i64) -> i64 {
    // reasoning stored at resp + 4
    lock!(heap()).get(&(resp + 4)).copied().unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn aial_rt_extract_ai_usage(resp: i64) -> i64 {
    lock!(heap()).get(&(resp + 3)).copied().unwrap_or(0)
}

// ── Tool dispatch ──

#[no_mangle] pub extern "C" fn aial_rt_tool_dispatch(_n: i64, _a: i64) -> i64 { 0 }

// ── AI Streaming ──

#[no_mangle]
pub extern "C" fn aial_rt_ai_stream_start(
    model: i64, ctx_id: i64, prompt_idx: i64,
    temperature: f64, max_tokens: i64, _format: i64,
    tools_json_idx: i64,
) -> i64 {
    let api_key = std::env::var("AIAL_KEY_DEEPSEEK").ok()
        .or_else(|| std::env::var("AIAL_KEY_OPENAI").ok())
        .or_else(|| std::env::var("DEEPSEEK_API_KEY").ok());
    let prompt = lock!(strs()).get(&prompt_idx).cloned().unwrap_or_default();
    let tools_json = if tools_json_idx != 0 {
        lock!(strs()).get(&tools_json_idx).cloned().unwrap_or_default()
    } else { String::new() };
    let handle = alloc();
    let tokens: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let tokens_clone = tokens.clone();
    let ended = Arc::new(AtomicBool::new(false));
    let ended_clone = ended.clone();   // for mock thread / early return
    let ended_clone3 = ended.clone();  // for real API thread
    lock!(stream_tokens()).insert(handle, (tokens, 0, ended));

    let api_key = match api_key {
        Some(k) => k,
        None => {
            lock!(tokens_clone).push("[error: no API key set] ".to_string());
            ended_clone.store(true, Ordering::SeqCst);
            return handle;
        }
    };

    // Mock mode: return fake streaming tokens without API call
    if std::env::var("AIAL_MOCK").is_ok() {
        std::thread::spawn(move || {
            let mock_text = format!("[mock AI response to: {}]", prompt);
            for word in mock_text.split_whitespace() {
                lock!(tokens_clone).push(format!("{} ", word));
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            ended_clone.store(true, Ordering::SeqCst);
        });
        return handle;
    }

    std::thread::spawn(move || {
        // Build messages array from context state
        let mut messages: Vec<serde_json::Value> = Vec::new();
        if let Some(ctx) = lock!(ctxs()).get(&ctx_id) {
            if !ctx.system_prompt.is_empty() {
                messages.push(serde_json::json!({"role": "system", "content": ctx.system_prompt}));
            }
            for (role, content) in &ctx.messages {
                messages.push(serde_json::json!({"role": role, "content": content}));
            }
        }
        messages.push(serde_json::json!({"role": "user", "content": prompt}));

        let model_name = if model == 0 {
            std::env::var("AIAL_MODEL_0").unwrap_or_else(|_| "deepseek-v4-flash".to_string())
        } else { format!("model_{}", model) };

        let client = reqwest::blocking::Client::new();
        let mut body_map = serde_json::json!({
            "model": model_name,
            "messages": messages,
            "max_tokens": max_tokens,
            "temperature": temperature,
            "stream": true,
            "thinking": {"type": "enabled"}
        });
        if !tools_json.is_empty() {
            if let Ok(tools) = serde_json::from_str::<serde_json::Value>(&tools_json) {
                body_map["tools"] = tools;
            }
        }
        let api_url = if model == 0 {
            std::env::var("AIAL_API_URL").unwrap_or_else(|_| "https://api.deepseek.com/chat/completions".to_string())
        } else {
            std::env::var("AIAL_API_URL").unwrap_or_else(|_| "https://api.openai.com/v1/chat/completions".to_string())
        };
        let mut resp = match client.post(&api_url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body_map)
            .send() {
            Ok(r) => r,
            Err(e) => { lock!(tokens_clone).push(format!("[error: {}] ", e)); ended_clone3.store(true, Ordering::SeqCst); return; }
        };

        let status = resp.status().as_u16();
        if status >= 400 {
            use std::io::Read;
            let mut err_body = String::new();
            let _ = resp.read_to_string(&mut err_body);
            if err_body.is_empty() { err_body = format!("{:?}", resp.headers()); }
            if err_body.len() > 300 { err_body.truncate(300); }
            let msg = match status {
                401 => format!("[error: HTTP 401 — Invalid API key]"),
                402 => format!("[error: HTTP 402 — Insufficient balance]"),
                422 => format!("[error: HTTP 422 — Invalid parameters]"),
                429 => format!("[error: HTTP 429 — Rate limit exceeded]"),
                500 => format!("[error: HTTP 500 — Server error, retry later]"),
                503 => format!("[error: HTTP 503 — Server busy, retry later]"),
                _ => format!("[error: HTTP {status} — {err_body}]"),
            };
            lock!(tokens_clone).push(msg);
            ended_clone3.store(true, Ordering::SeqCst);
            return;
        }

        // SSE streaming: read line by line, parse data: chunks
        use std::io::{BufRead, BufReader, Read};
        let mut reader = BufReader::new(resp);
        let mut line = String::new();
        let mut tool_call_frags: std::collections::HashMap<i64, serde_json::Value> = std::collections::HashMap::new();
        let mut had_reasoning = false;
        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() { continue; }
                    if trimmed == "data: [DONE]" { break; }
                    if let Some(data) = trimmed.strip_prefix("data: ") {
                        if let Ok(v) = serde_json::from_str::<serde_json::Value>(data) {
                            let delta = &v["choices"][0]["delta"];
                            // Accumulate tool call fragments
                            if let Some(tool_calls) = delta["tool_calls"].as_array() {
                                for tc in tool_calls {
                                    let idx = tc["index"].as_i64().unwrap_or(0);
                                    let mut frag = tool_call_frags.remove(&idx).unwrap_or(serde_json::json!({}));
                                    if let Some(id) = tc["id"].as_str() { frag["id"] = serde_json::json!(id); }
                                    if let Some(tp) = tc["type"].as_str() { frag["type"] = serde_json::json!(tp); }
                                    if let Some(fn_name) = tc["function"]["name"].as_str() {
                                        frag["function"] = serde_json::json!({"name": fn_name, "arguments": ""});
                                    }
                                    if let Some(args) = tc["function"]["arguments"].as_str() {
                                        let existing = frag["function"]["arguments"].as_str().unwrap_or("");
                                        frag["function"]["arguments"] = serde_json::json!(format!("{}{}", existing, args));
                                    }
                                    tool_call_frags.insert(idx, frag);
                                }
                            }
                            // Push reasoning content in dim gray
                            if let Some(reasoning) = delta["reasoning_content"].as_str() {
                                if !reasoning.is_empty() {
                                    lock!(tokens_clone).push(format!("\x1b[90m{}\x1b[0m", reasoning));
                                    had_reasoning = true;
                                }
                            }
                            // Push text content (newline before first content after reasoning)
                            if let Some(content) = delta["content"].as_str() {
                                if !content.is_empty() {
                                    if had_reasoning {
                                        lock!(tokens_clone).push("\n".to_string());
                                        had_reasoning = false;
                                    }
                                    lock!(tokens_clone).push(content.to_string());
                                }
                            }
                            // On finish_reason, flush completed tool calls
                            if delta["finish_reason"].as_str().is_some() && !tool_call_frags.is_empty() {
                                let mut indices: Vec<i64> = tool_call_frags.keys().copied().collect();
                                indices.sort();
                                for idx in indices {
                                    if let Some(tc) = tool_call_frags.remove(&idx) {
                                        lock!(tokens_clone).push(format!("[TOOL_CALL:{}]", tc));
                                    }
                                }
                            }
                        }
                    }
                }
                Err(_) => break,
            }
        }
        ended_clone3.store(true, Ordering::SeqCst);
    });

    handle
}

#[no_mangle]
pub extern "C" fn aial_rt_ai_stream_read(handle: i64) -> i64 {
    loop {
        let (token, stream_ended) = {
            let mut map = lock!(stream_tokens());
            if let Some((tokens, pos, ended_flag)) = map.get_mut(&handle) {
                let ended = ended_flag.load(Ordering::SeqCst);
                if let Ok(guard) = tokens.lock() {
                    if (*pos as usize) < guard.len() {
                        let t = guard[*pos as usize].clone();
                        *pos += 1;
                        (Some(t), ended)
                    } else {
                        (None, ended)
                    }
                } else { (None, true) }
            } else { (None, true) }
        };
        match token {
            Some(t) => {
                let ptr = alloc();
                lock!(strs()).insert(ptr, t);
                return ptr;
            }
            None if stream_ended => {
                lock!(stream_tokens()).remove(&handle);
                let ptr = alloc();
                lock!(strs()).insert(ptr, String::new());
                return ptr;
            }
            None => {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_ai_stream_close(handle: i64) {
    lock!(stream_tokens()).remove(&handle);
}
