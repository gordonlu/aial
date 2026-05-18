use super::*;

use std::collections::VecDeque;

static ACTOR_MAILBOXES: OnceLock<Mutex<HashMap<i64, Arc<Mutex<VecDeque<String>>>>>> = OnceLock::new();
static ACTOR_NEXT_PID: Mutex<i64> = Mutex::new(1);
static ACTOR_ERRORS: OnceLock<Mutex<HashMap<i64, String>>> = OnceLock::new();

fn actor_mailboxes() -> &'static Mutex<HashMap<i64, Arc<Mutex<VecDeque<String>>>>> {
    ACTOR_MAILBOXES.get_or_init(|| Mutex::new(HashMap::new()))
}
fn actor_errors() -> &'static Mutex<HashMap<i64, String>> {
    ACTOR_ERRORS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[no_mangle]
pub extern "C" fn aial_rt_actor_spawn() -> i64 {
    let pid = { let mut n = lock!(ACTOR_NEXT_PID); let p = *n; *n += 1; p };
    lock!(actor_mailboxes()).insert(pid, Arc::new(Mutex::new(VecDeque::new())));
    pid
}

#[no_mangle]
pub extern "C" fn aial_rt_actor_spawn_handler(fn_ptr: i64, init_ptr: i64) -> i64 {
    let fn_name = lock!(strs()).get(&fn_ptr).cloned().unwrap_or_default();
    let init_msg = lock!(strs()).get(&init_ptr).cloned().unwrap_or_default();
    let pid = { let mut n = lock!(ACTOR_NEXT_PID); let p = *n; *n += 1; p };
    lock!(actor_mailboxes()).insert(pid, Arc::new(Mutex::new(VecDeque::new())));

    // Spawn thread — looks up AIAL function via dlsym
    std::thread::spawn(move || {
        // Push init message
        {
            let mboxes = lock!(actor_mailboxes());
            if let Some(mbox) = mboxes.get(&pid) {
                lock!(mbox).push_back(init_msg);
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
        lock!(mbox).push_back(msg);
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
            if let Some(msg) = lock!(mbox).pop_front() {
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
        if let Some(msg) = lock!(mbox).pop_front() {
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
            if let Some(msg) = lock!(mbox).pop_front() {
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
