use super::*;

#[no_mangle]
pub extern "C" fn aial_rt_time_now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn aial_rt_time_now() -> i64 {
    let now = unsafe {
        let mut t: libc::time_t = 0;
        libc::time(&mut t);
        let tm = libc::localtime(&t);
        if tm.is_null() { "unknown".to_string() }
        else {
            format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
                (*tm).tm_year as i32 + 1900,
                (*tm).tm_mon as i32 + 1,
                (*tm).tm_mday,
                (*tm).tm_hour,
                (*tm).tm_min,
                (*tm).tm_sec)
        }
    };
    let ptr = alloc(); lock!(strs()).insert(ptr, now); ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_time_sleep(ms: i64) {
    std::thread::sleep(std::time::Duration::from_millis(ms as u64));
}
