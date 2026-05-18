use super::*;

use serde::{Serialize, Deserialize};

fn keys_path() -> std::path::PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    std::path::PathBuf::from(home).join(".aial").join("keys.json")
}

#[derive(Serialize, Deserialize)]
struct KeyStore { keys: std::collections::HashMap<String, KeyEntry> }

#[derive(Serialize, Deserialize)]
struct KeyEntry { key: String, created_at: String }

fn load_key_store(path: &std::path::PathBuf) -> KeyStore {
    if !path.exists() { return KeyStore { keys: std::collections::HashMap::new() }; }
    let content = std::fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or(KeyStore { keys: std::collections::HashMap::new() })
}

fn save_key_store(path: &std::path::PathBuf, store: &KeyStore) -> bool {
    let content = match serde_json::to_string_pretty(store) { Ok(c) => c, Err(_) => return false };
    if let Some(parent) = path.parent() { let _ = std::fs::create_dir_all(parent); }
    std::fs::write(path, &content).is_ok()
}

#[no_mangle]
pub extern "C" fn aial_rt_key_set(provider_ptr: i64, key_ptr: i64) -> i64 {
    let provider = { let st = lock!(strs()); st.get(&provider_ptr).cloned().unwrap_or_default() };
    let key = { let st = lock!(strs()); st.get(&key_ptr).cloned().unwrap_or_default() };
    if provider.is_empty() || key.is_empty() { return 0; }
    let path = keys_path();
    let mut store = load_key_store(&path);
    store.keys.insert(provider.clone(), KeyEntry { key, created_at: String::new() });
    if save_key_store(&path, &store) {
        #[cfg(unix)] { use std::os::unix::fs::PermissionsExt; if let Ok(m) = std::fs::metadata(&path) { let mut p = m.permissions(); p.set_mode(0o600); let _ = std::fs::set_permissions(&path, p); } }
        1
    } else { 0 }
}

#[no_mangle]
pub extern "C" fn aial_rt_key_exists(provider_ptr: i64) -> i64 {
    let provider = { let st = lock!(strs()); st.get(&provider_ptr).cloned().unwrap_or_default() };
    let env_var = format!("AIAL_KEY_{}", provider.to_uppercase());
    if std::env::var(&env_var).is_ok() { return 1; }
    if provider == "deepseek" && std::env::var("DEEPSEEK_API_KEY").is_ok() { return 1; }
    if std::env::var("OPENAI_API_KEY").is_ok() { return 1; }
    let path = keys_path();
    let store = load_key_store(&path);
    if store.keys.contains_key(&provider) { return 1; }
    0
}

#[no_mangle]
pub extern "C" fn aial_rt_key_delete(provider_ptr: i64) -> i64 {
    let provider = { let st = lock!(strs()); st.get(&provider_ptr).cloned().unwrap_or_default() };
    let path = keys_path();
    let mut store = load_key_store(&path);
    store.keys.remove(&provider);
    if save_key_store(&path, &store) { 1 } else { 0 }
}
