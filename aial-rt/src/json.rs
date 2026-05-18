use super::*;

// JsonValue heap layout: [type, aux, size/f64_bits, data_ptr, _]
// types: 0=null, 1=bool, 2=number, 3=string, 4=array, 5=object, -1=error

fn json_write(s: &mut HashMap<i64, String>, h: &mut HashMap<i64, i64>, ptr: i64, v: &serde_json::Value) {
    match v {
        serde_json::Value::Null => { h.insert(ptr, 0); }
        serde_json::Value::Bool(b) => { h.insert(ptr, 1); h.insert(ptr + 1, *b as i64); }
        serde_json::Value::Number(n) => {
            h.insert(ptr, 2);
            if let Some(f) = n.as_f64() { h.insert(ptr + 2, f.to_bits() as i64); }
        }
        serde_json::Value::String(t) => {
            h.insert(ptr, 3);
            let s_ptr = alloc(); s.insert(s_ptr, t.clone()); h.insert(ptr + 1, s_ptr);
        }
        serde_json::Value::Array(arr) => {
            h.insert(ptr, 4);
            let arr_ptr = alloc_block(arr.len());
            h.insert(ptr + 3, arr_ptr);
            h.insert(ptr + 2, arr.len() as i64);
            for (i, item) in arr.iter().enumerate() {
                let item_ptr = alloc_block(5);
                json_write(s, h, item_ptr, item);
                h.insert(arr_ptr + i as i64, item_ptr);
            }
        }
        serde_json::Value::Object(obj) => {
            h.insert(ptr, 5);
            let n = obj.len();
            let obj_ptr = alloc_block(n * 2);
            h.insert(ptr + 3, obj_ptr);
            h.insert(ptr + 2, n as i64);
            for (i, (k, val)) in obj.iter().enumerate() {
                let k_ptr = alloc(); s.insert(k_ptr, k.clone());
                h.insert(obj_ptr + (i * 2) as i64, k_ptr);
                let v_ptr = alloc_block(5);
                json_write(s, h, v_ptr, val);
                h.insert(obj_ptr + (i * 2 + 1) as i64, v_ptr);
            }
        }
    }
}

fn json_lookup(ptr: i64, key: &str) -> Option<i64> {
    let h = lock!(heap());
    let s = lock!(strs());
    let tag = h.get(&ptr).copied().unwrap_or(0);
    if tag != 5 { return None; }
    let n = h.get(&(ptr + 2)).copied().unwrap_or(0) as usize;
    let obj_ptr = h.get(&(ptr + 3)).copied().unwrap_or(0);
    for i in 0..n {
        let k_ptr = h.get(&(obj_ptr + (i * 2) as i64)).copied().unwrap_or(0);
        if s.get(&k_ptr).map(|k| k == key).unwrap_or(false) {
            return h.get(&(obj_ptr + (i * 2 + 1) as i64)).copied();
        }
    }
    None
}

pub(crate) fn json_value_to_string_runtime(val_ptr: i64) -> String {
    // Collect child pointers under lock, then release before recursive calls
    let (tag, aux, f64_val, children): (i64, i64, u64, Vec<i64>) = {
        let h = lock!(heap());
        let s = lock!(strs());
        let tag = h.get(&val_ptr).copied().unwrap_or(0);
        match tag {
            3 => {
                let sp = h.get(&(val_ptr + 1)).copied().unwrap_or(0);
                return format!("\"{}\"", s.get(&sp).cloned().unwrap_or_default());
            }
            4 => {
                let n = h.get(&(val_ptr + 2)).copied().unwrap_or(0) as usize;
                let arr = h.get(&(val_ptr + 3)).copied().unwrap_or(0);
                let mut children = Vec::new();
                for i in 0..n {
                    if let Some(ip) = h.get(&(arr + i as i64)).copied() { children.push(ip); }
                }
                (tag, 0, 0, children)
            }
            5 => {
                let n = h.get(&(val_ptr + 2)).copied().unwrap_or(0) as usize;
                let obj = h.get(&(val_ptr + 3)).copied().unwrap_or(0);
                let mut children = Vec::new();
                for i in 0..n {
                    if let Some(kp) = h.get(&(obj + (i * 2) as i64)).copied() { children.push(kp); }
                    if let Some(vp) = h.get(&(obj + (i * 2 + 1) as i64)).copied() { children.push(vp); }
                }
                (tag, n as i64, 0, children)
            }
            _ => (tag, h.get(&(val_ptr + 1)).copied().unwrap_or(0), h.get(&(val_ptr + 2)).copied().unwrap_or(0) as u64, vec![]),
        }
    };
    // Format without holding locks
    match tag {
        0 => "null".to_string(),
        1 => (aux != 0).to_string(),
        2 => format!("{}", f64::from_bits(f64_val)),
        4 => {
            let items: Vec<String> = children.iter().map(|&ip| json_value_to_string_runtime(ip)).collect();
            format!("[{}]", items.join(","))
        }
        5 => {
            let n = aux as usize;
            let mut pairs = Vec::new();
            for i in 0..n {
                let kp = children[i * 2];
                let vp = children[i * 2 + 1];
                let key = lock!(strs()).get(&kp).cloned().unwrap_or_default();
                pairs.push(format!("\"{}\":{}", key, json_value_to_string_runtime(vp)));
            }
            format!("{{{}}}", pairs.join(","))
        }
        _ => "null".to_string(),
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_json_parse(text_ptr: i64) -> i64 {
    let text = lock!(strs()).get(&text_ptr).cloned().unwrap_or_default();
    let val_ptr = alloc_block(5);
    let mut h = lock!(heap());
    let mut s = lock!(strs());
    match serde_json::from_str::<serde_json::Value>(&text) {
        Ok(v) => { json_write(&mut s, &mut h, val_ptr, &v); val_ptr }
        Err(e) => {
            h.insert(val_ptr, -1); // error tag
            let err_ptr = alloc();
            s.insert(err_ptr, e.to_string());
            h.insert(val_ptr + 1, err_ptr);
            val_ptr
        }
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_json_get(val_ptr: i64, key_ptr: i64) -> i64 {
    let key = lock!(strs()).get(&key_ptr).cloned().unwrap_or_default();
    match json_lookup(val_ptr, &key) {
        Some(r) => r,
        None => {
            let null_ptr = alloc_block(5);
            lock!(heap()).insert(null_ptr, 0);
            null_ptr
        }
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_json_get_or(val_ptr: i64, key_ptr: i64, default_ptr: i64) -> i64 {
    let key = lock!(strs()).get(&key_ptr).cloned().unwrap_or_default();
    json_lookup(val_ptr, &key).unwrap_or(default_ptr)
}

#[no_mangle]
pub extern "C" fn aial_rt_json_type(val_ptr: i64) -> i64 {
    lock!(heap()).get(&val_ptr).copied().unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn aial_rt_json_stringify(val_ptr: i64) -> i64 {
    let s = json_value_to_string_runtime(val_ptr);
    let ptr = alloc(); lock!(strs()).insert(ptr, s); ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_json_value_to_string(val_ptr: i64) -> i64 {
    let h = lock!(heap());
    let s = lock!(strs());
    let tag = h.get(&val_ptr).copied().unwrap_or(0);
    let text = match tag {
        3 => s.get(&h.get(&(val_ptr+1)).copied().unwrap_or(0)).cloned().unwrap_or_default(),
        2 => format!("{}", f64::from_bits(h.get(&(val_ptr+2)).copied().unwrap_or(0) as u64)),
        1 => (h.get(&(val_ptr+1)).copied().unwrap_or(0) != 0).to_string(),
        0 => "null".to_string(),
        _ => json_value_to_string_runtime(val_ptr),
    };
    drop(h); drop(s);
    let ptr = alloc(); lock!(strs()).insert(ptr, text); ptr
}

#[no_mangle]
pub extern "C" fn aial_rt_json_to_int(val_ptr: i64) -> i64 {
    let h = lock!(heap());
    let tag = h.get(&val_ptr).copied().unwrap_or(0);
    match tag {
        2 => f64::from_bits(h.get(&(val_ptr+2)).copied().unwrap_or(0) as u64) as i64,
        1 => h.get(&(val_ptr+1)).copied().unwrap_or(0),
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_json_to_float(val_ptr: i64) -> f64 {
    let h = lock!(heap());
    let tag = h.get(&val_ptr).copied().unwrap_or(0);
    match tag {
        2 => f64::from_bits(h.get(&(val_ptr+2)).copied().unwrap_or(0) as u64),
        1 => h.get(&(val_ptr+1)).copied().unwrap_or(0) as f64,
        _ => 0.0,
    }
}

#[no_mangle]
pub extern "C" fn aial_rt_json_array_len(val_ptr: i64) -> i64 {
    let h = lock!(heap());
    if h.get(&val_ptr).copied().unwrap_or(0) == 4 { h.get(&(val_ptr+2)).copied().unwrap_or(0) } else { 0 }
}

#[no_mangle]
pub extern "C" fn aial_rt_json_array_get(val_ptr: i64, idx: i64) -> i64 {
    let h = lock!(heap());
    let arr_ptr = h.get(&(val_ptr + 3)).copied().unwrap_or(0);
    match h.get(&(arr_ptr + idx)).copied() {
        Some(v) => v,
        None => {
            drop(h);
            let null_ptr = alloc_block(5); lock!(heap()).insert(null_ptr, 0); null_ptr
        }
    }
}
