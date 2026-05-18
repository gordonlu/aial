use super::*;

use std::collections::VecDeque;
use std::sync::Condvar;

struct Mailbox {
    messages: VecDeque<String>,
    condvar: Condvar,
}

impl Mailbox {
    fn new() -> Self { Mailbox { messages: VecDeque::new(), condvar: Condvar::new() } }
}

static ACTOR_MAILBOXES: OnceLock<Mutex<HashMap<i64, Arc<Mutex<Mailbox>>>>> = OnceLock::new();
static ACTOR_NEXT_PID: Mutex<i64> = Mutex::new(1);
static ACTOR_ERRORS: OnceLock<Mutex<HashMap<i64, String>>> = OnceLock::new();

fn actor_mailboxes() -> &'static Mutex<HashMap<i64, Arc<Mutex<Mailbox>>>> {
    ACTOR_MAILBOXES.get_or_init(|| Mutex::new(HashMap::new()))
}
fn actor_errors() -> &'static Mutex<HashMap<i64, String>> {
    ACTOR_ERRORS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[no_mangle]
pub extern "C" fn aial_rt_actor_spawn() -> i64 {
    let pid = { let mut n = lock!(ACTOR_NEXT_PID); let p = *n; *n += 1; p };
    lock!(actor_mailboxes()).insert(pid, Arc::new(Mutex::new(Mailbox::new())));
    pid
}

#[no_mangle]
pub extern "C" fn aial_rt_actor_spawn_handler(fn_ptr: i64, init_ptr: i64) -> i64 {
    let fn_name = lock!(strs()).get(&fn_ptr).cloned().unwrap_or_default();
    let init_msg = lock!(strs()).get(&init_ptr).cloned().unwrap_or_default();
    let pid = { let mut n = lock!(ACTOR_NEXT_PID); let p = *n; *n += 1; p };
    let mbox = Arc::new(Mutex::new(Mailbox::new()));
    lock!(actor_mailboxes()).insert(pid, mbox.clone());

    // Spawn thread — looks up AIAL function via dlsym
    std::thread::spawn(move || {
        // Push init message and notify
        {
            let mut mb = lock!(mbox);
            mb.messages.push_back(init_msg);
            mb.condvar.notify_one();
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
                // SAFETY: dlsym returns a pointer to an extern "C" fn(i64) defined via #[no_mangle].
                // The handler signature is checked at compile time by the AIAL compiler's #[tool] attribute.
                let handler: HandlerFn = unsafe { std::mem::transmute(ptr) };
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
        let mut mb = lock!(mbox);
        mb.messages.push_back(msg);
        mb.condvar.notify_one();
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
        let mut guard = lock!(mbox);
        loop {
            if let Some(msg) = guard.messages.pop_front() {
                lock!(strs()).insert(ptr, msg);
                return ptr;
            }
            let g = guard;
            let cv_ptr: *const Condvar = &g.condvar;
            guard = unsafe { (*cv_ptr).wait(g).unwrap() };
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
        let mut mb = lock!(mbox);
        if let Some(msg) = mb.messages.pop_front() {
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
        let mut mb = lock!(mbox);
        loop {
            if let Some(msg) = mb.messages.pop_front() {
                lock!(strs()).insert(ptr, msg);
                return ptr;
            }
            let now = std::time::Instant::now();
            if now >= deadline { break; }
            let remaining = ((deadline - now).as_millis() as u64).min(100);
            let g = mb;
            let cv_ptr: *const Condvar = &g.condvar;
            let (new_g, timed_out) = unsafe { (*cv_ptr).wait_timeout(g, std::time::Duration::from_millis(remaining)).unwrap() };
            mb = new_g;
            if timed_out.timed_out() && std::time::Instant::now() >= deadline { break; }
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
