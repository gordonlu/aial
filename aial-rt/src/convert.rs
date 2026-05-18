use super::*;

#[no_mangle]
pub extern "C" fn aial_rt_int_to_string(n: i64) -> i64 {
    let ptr = alloc(); lock!(strs()).insert(ptr, n.to_string()); ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_string_to_int(s_idx: i64) -> i64 {
    let s = lock!(strs()).get(&s_idx).cloned().unwrap_or_default();
    s.trim().parse::<i64>().unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn aial_rt_args() -> i64 {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ptr = alloc(); lock!(strs()).insert(ptr, args.join("\n")); ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_html_escape(text_ptr: i64) -> i64 {
    let text = lock!(strs()).get(&text_ptr).cloned().unwrap_or_default();
    let escaped = text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;");
    let ptr = alloc(); lock!(strs()).insert(ptr, escaped); ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_token_estimate(text_idx: i64) -> i64 {
    let s = lock!(strs()).get(&text_idx).cloned().unwrap_or_default();
    let bytes = s.len();
    let cjk = s.chars().filter(|&c| c >= '\u{4E00}' && c <= '\u{9FFF}').count();
    let ascii = bytes.saturating_sub(cjk * 3);
    (ascii as i64 / 4 + cjk as i64 * 2 / 3).max(1)
}
