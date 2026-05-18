use super::*;

// ─── Map (hash table) ───
static MAPS: OnceLock<Mutex<HashMap<i64, HashMap<String, String>>>> = OnceLock::new();
static NEXT_MAP: Mutex<i64> = Mutex::new(1);

fn maps() -> &'static Mutex<HashMap<i64, HashMap<String, String>>> {
    MAPS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[no_mangle]
pub extern "C" fn aial_rt_map_new() -> i64 {
    let mut next = lock!(NEXT_MAP);
    let id = *next;
    *next += 1;
    drop(next);
    lock!(maps()).insert(id, HashMap::new());
    id
}

#[no_mangle]
pub extern "C" fn aial_rt_map_set(handle: i64, key_idx: i64, value_idx: i64) {
    let st = lock!(strs());
    let k = st.get(&key_idx).cloned().unwrap_or_default();
    let v = st.get(&value_idx).cloned().unwrap_or_default();
    drop(st);
    lock!(maps()).get_mut(&handle).map(|m| m.insert(k, v));
}

#[no_mangle]
pub extern "C" fn aial_rt_map_get(handle: i64, key_idx: i64) -> i64 {
    let k = {
        let st = lock!(strs());
        st.get(&key_idx).cloned().unwrap_or_default()
    };
    let maps = lock!(maps());
    if let Some(m) = maps.get(&handle) {
        if let Some(v) = m.get(&k) {
            let addr = alloc();
            lock!(strs()).insert(addr, v.clone());
            return addr;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn aial_rt_map_has(handle: i64, key_idx: i64) -> i64 {
    let k = {
        let st = lock!(strs());
        st.get(&key_idx).cloned().unwrap_or_default()
    };
    match lock!(maps()).get(&handle) {
        Some(m) if m.contains_key(&k) => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_map_remove(handle: i64, key_idx: i64) {
    let k = {
        let st = lock!(strs());
        st.get(&key_idx).cloned().unwrap_or_default()
    };
    lock!(maps()).get_mut(&handle).map(|m| m.remove(&k));
}

// ─── Heap (priority queue) ───
static HEAPS: OnceLock<Mutex<HashMap<i64, Vec<(String, i64)>>>> = OnceLock::new();
static NEXT_HEAP: Mutex<i64> = Mutex::new(1);

fn heaps() -> &'static Mutex<HashMap<i64, Vec<(String, i64)>>> {
    HEAPS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[no_mangle]
pub extern "C" fn aial_rt_heap_new() -> i64 {
    let mut next = lock!(NEXT_HEAP);
    let id = *next; *next += 1; drop(next);
    lock!(heaps()).insert(id, Vec::new());
    id
}

#[no_mangle]
pub extern "C" fn aial_rt_heap_push(handle: i64, value_idx: i64, priority: i64) {
    let v = lock!(strs()).get(&value_idx).cloned().unwrap_or_default();
    lock!(heaps()).get_mut(&handle).map(|h| h.push((v, priority)));
}

#[no_mangle]
pub extern "C" fn aial_rt_heap_pop(handle: i64) -> i64 {
    let mut heaps = lock!(heaps());
    if let Some(h) = heaps.get_mut(&handle) {
        if h.is_empty() { return 0; }
        let mut best = 0usize;
        for (i, (_, p)) in h.iter().enumerate() { if *p > h[best].1 { best = i; } }
        let (val, _) = h.swap_remove(best);
        let addr = alloc();
        lock!(strs()).insert(addr, val);
        return addr;
    }
    0
}

#[no_mangle]
pub extern "C" fn aial_rt_heap_peek(handle: i64) -> i64 {
    let heaps = lock!(heaps());
    if let Some(h) = heaps.get(&handle) {
        if let Some(best) = h.iter().max_by_key(|(_, p)| p) {
            let addr = alloc();
            lock!(strs()).insert(addr, best.0.clone());
            return addr;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn aial_rt_heap_len(handle: i64) -> i64 {
    lock!(heaps()).get(&handle).map(|h| h.len() as i64).unwrap_or(0)
}

// ─── Array ───
static ARRAYS: OnceLock<Mutex<HashMap<i64, Vec<String>>>> = OnceLock::new();
static NEXT_ARR: Mutex<i64> = Mutex::new(1);

fn arrays() -> &'static Mutex<HashMap<i64, Vec<String>>> {
    ARRAYS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[no_mangle]
pub extern "C" fn aial_rt_array_new() -> i64 {
    let mut next = lock!(NEXT_ARR);
    let id = *next; *next += 1; drop(next);
    lock!(arrays()).insert(id, Vec::new());
    id
}

#[no_mangle]
pub extern "C" fn aial_rt_array_push(handle: i64, value_idx: i64) {
    let v = lock!(strs()).get(&value_idx).cloned().unwrap_or_default();
    lock!(arrays()).get_mut(&handle).map(|a| a.push(v));
}

#[no_mangle]
pub extern "C" fn aial_rt_array_sort(handle: i64) {
    lock!(arrays()).get_mut(&handle).map(|a| a.sort());
}

#[no_mangle]
pub extern "C" fn aial_rt_array_get(handle: i64, index: i64) -> i64 {
    let arrs = lock!(arrays());
    if let Some(a) = arrs.get(&handle) {
        if let Some(v) = a.get(index as usize) {
            let addr = alloc();
            lock!(strs()).insert(addr, v.clone());
            return addr;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn aial_rt_array_len(handle: i64) -> i64 {
    lock!(arrays()).get(&handle).map(|a| a.len() as i64).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn aial_rt_array_join(handle: i64, sep_ptr: i64) -> i64 {
    let sep = lock!(strs()).get(&sep_ptr).cloned().unwrap_or_default();
    let result = lock!(arrays()).get(&handle).map(|a| a.join(&sep)).unwrap_or_default();
    let ptr = alloc();
    lock!(strs()).insert(ptr, result);
    ptr
}
