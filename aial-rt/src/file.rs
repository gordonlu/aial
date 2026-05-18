use super::*;

#[no_mangle]
pub extern "C" fn aial_rt_file_read(path_ptr: i64) -> i64 {
    let path = lock!(strs()).get(&path_ptr).cloned().unwrap_or_default();
    let content = std::fs::read_to_string(&path).unwrap_or_else(|e| format!("[read error: {}]", e));
    let addr = alloc();
    lock!(strs()).insert(addr, content);
    addr
}

#[no_mangle]
pub extern "C" fn aial_rt_file_write(path_ptr: i64, content_ptr: i64) {
    let path = lock!(strs()).get(&path_ptr).cloned().unwrap_or_default();
    let content = lock!(strs()).get(&content_ptr).cloned().unwrap_or_default();
    let _ = std::fs::write(&path, &content);
}

#[no_mangle]
pub extern "C" fn aial_rt_file_append(path_ptr: i64, content_ptr: i64) {
    let path = lock!(strs()).get(&path_ptr).cloned().unwrap_or_default();
    let content = lock!(strs()).get(&content_ptr).cloned().unwrap_or_default();
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().append(true).create(true).open(&path) {
        let _ = f.write_all(content.as_bytes());
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_file_patch(path_ptr: i64, old_ptr: i64, new_ptr: i64) {
    let path = lock!(strs()).get(&path_ptr).cloned().unwrap_or_default();
    let old = lock!(strs()).get(&old_ptr).cloned().unwrap_or_default();
    let new = lock!(strs()).get(&new_ptr).cloned().unwrap_or_default();
    let content = std::fs::read_to_string(&path).unwrap_or_default();
    let _ = std::fs::write(&path, content.replace(&old, &new));
}

#[no_mangle]
pub extern "C" fn aial_rt_file_list_dir(path_idx: i64) -> i64 {
    let path = lock!(strs()).get(&path_idx).cloned().unwrap_or_default();
    let entries: Vec<String> = std::fs::read_dir(&path)
        .map(|dir| dir.filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().to_string())).collect())
        .unwrap_or_default();
    let ptr = alloc(); lock!(strs()).insert(ptr, entries.join("\n")); ptr
}
