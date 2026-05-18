use super::*;

#[no_mangle]
pub extern "C" fn aial_rt_process_run(cmd_idx: i64) -> i64 {
    let cmd = lock!(strs()).get(&cmd_idx).cloned().unwrap_or_default();
    let output = std::process::Command::new("sh")
        .arg("-c").arg(&cmd)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_else(|e| format!("[error: {}]", e));
    let ptr = alloc(); lock!(strs()).insert(ptr, output); ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_process_run_with_status(cmd_idx: i64) -> i64 {
    let cmd = lock!(strs()).get(&cmd_idx).cloned().unwrap_or_default();
    let full = std::process::Command::new("sh")
        .arg("-c").arg(&cmd)
        .output();
    let (stdout_str, exit_code) = match full {
        Ok(o) => (String::from_utf8_lossy(&o.stdout).to_string(), o.status.code().unwrap_or(-1) as i64),
        Err(e) => (format!("[error: {}]", e), -1),
    };
    let base = alloc();
    let stdout_addr = alloc();
    lock!(strs()).insert(stdout_addr, stdout_str);
    lock!(heap()).insert(base, stdout_addr);
    lock!(heap()).insert(base + 1, exit_code);
    base
}
