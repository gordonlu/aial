use super::*;

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
    let bytes = s.as_bytes();
    let bstart = (start.max(0) as usize).min(bytes.len());
    let blen = (len.max(0) as usize).min(bytes.len() - bstart);
    let slice = String::from_utf8_lossy(&bytes[bstart..bstart + blen]).to_string();
    let addr = alloc();
    lock!(strs()).insert(addr, slice);
    addr
}

#[no_mangle]
pub extern "C" fn aial_rt_strchr(ptr: i64, idx: i64) -> i64 {
    lock!(strs()).get(&ptr)
        .and_then(|s| {
            let bytes = s.as_bytes();
            let pos = (idx.max(0) as usize).min(bytes.len());
            if pos >= bytes.len() { return None; }
            s[pos..].chars().next().map(|c| c as i64)
        })
        .unwrap_or(-1)
}

#[no_mangle]
pub extern "C" fn aial_rt_str_prev_char(s_ptr: i64, byte_pos: i64) -> i64 {
    let s = lock!(strs()).get(&s_ptr).cloned().unwrap_or_default();
    let bytes = s.as_bytes();
    let mut pos = byte_pos.min(bytes.len() as i64).max(0) as usize;
    if pos == 0 { return 0; }
    pos -= 1;
    while pos > 0 && (bytes[pos] & 0xC0) == 0x80 { pos -= 1; }
    pos as i64
}

#[no_mangle]
pub extern "C" fn aial_rt_str_next_char(s_ptr: i64, byte_pos: i64) -> i64 {
    let s = lock!(strs()).get(&s_ptr).cloned().unwrap_or_default();
    let bytes = s.as_bytes();
    let pos = byte_pos.min(bytes.len() as i64).max(0) as usize;
    if pos >= bytes.len() { return bytes.len() as i64; }
    let b = bytes[pos];
    let len = if b < 0x80 { 1 } else if (b & 0xE0) == 0xC0 { 2 } else if (b & 0xF0) == 0xE0 { 3 } else { 4 };
    ((pos + len).min(bytes.len())) as i64
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
pub extern "C" fn aial_rt_str_find(haystack_idx: i64, needle_idx: i64) -> i64 {
    let haystack = lock!(strs()).get(&haystack_idx).cloned().unwrap_or_default();
    let needle = lock!(strs()).get(&needle_idx).cloned().unwrap_or_default();
    haystack.find(&needle).map(|i| i as i64).unwrap_or(-1)
}
