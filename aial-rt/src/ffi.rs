use super::*;

#[no_mangle]
pub extern "C" fn aial_rt_ffi_load(path_idx: i64) -> i64 {
    let path = lock!(strs()).get(&path_idx).cloned().unwrap_or_default();
    let c_path = std::ffi::CString::new(path).unwrap_or_default();
    let handle = unsafe { libc::dlopen(c_path.as_ptr(), libc::RTLD_LAZY) };
    if handle.is_null() { return 0; }
    let id = alloc();
    lock!(heap()).insert(id, handle as i64);
    id
}

#[no_mangle]
pub extern "C" fn aial_rt_ffi_call(handle_id: i64, fn_name_idx: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i64 {
    let fn_name = lock!(strs()).get(&fn_name_idx).cloned().unwrap_or_default();
    let c_name = std::ffi::CString::new(fn_name).unwrap_or_default();
    let handle_ptr = lock!(heap()).get(&handle_id).copied().unwrap_or(0);
    let ptr = if handle_ptr != 0 {
        unsafe { libc::dlsym(handle_ptr as *mut libc::c_void, c_name.as_ptr()) }
    } else {
        unsafe { libc::dlsym(std::ptr::null_mut(), c_name.as_ptr()) }
    };
    if ptr.is_null() { return -1; }
    type Fn6 = unsafe extern "C" fn(i64, i64, i64, i64, i64, i64) -> i64;
    let func: Fn6 = unsafe { std::mem::transmute(ptr) };
    unsafe { func(a1, a2, a3, a4, a5, a6) }
}

#[no_mangle]
pub extern "C" fn aial_rt_ffi_close(handle_id: i64) {
    let handle = lock!(heap()).get(&handle_id).copied().unwrap_or(0) as *mut libc::c_void;
    if !handle.is_null() { unsafe { libc::dlclose(handle); } }
}
