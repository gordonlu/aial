use super::*;

static GLOBALS: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
pub fn globals() -> &'static Mutex<HashMap<String, String>> {
    GLOBALS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[no_mangle]
pub extern "C" fn aial_rt_global_set(key_idx: i64, val_idx: i64) {
    let key = lock!(strs()).get(&key_idx).cloned().unwrap_or_default();
    let val = lock!(strs()).get(&val_idx).cloned().unwrap_or_default();
    lock!(globals()).insert(key, val);
}

#[no_mangle]
pub extern "C" fn aial_rt_global_get(key_idx: i64) -> i64 {
    let key = lock!(strs()).get(&key_idx).cloned().unwrap_or_default();
    let val = lock!(globals()).get(&key).cloned().unwrap_or_default();
    let ptr = alloc(); lock!(strs()).insert(ptr, val); ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_global_has(key_idx: i64) -> i64 {
    let key = lock!(strs()).get(&key_idx).cloned().unwrap_or_default();
    if lock!(globals()).contains_key(&key) { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn aial_rt_global_delete(key_idx: i64) {
    let key = lock!(strs()).get(&key_idx).cloned().unwrap_or_default();
    lock!(globals()).remove(&key);
}
