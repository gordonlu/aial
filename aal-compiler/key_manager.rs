// key_manager.rs —— AAL API Key 安全管理
// 密钥不进入源代码，不在运行时暴露给应用层
// 存储：~/.aal/keys.json（0600 权限）
// 回退：AAL_KEY_<PROVIDER> 环境变量（CI/容器）

use std::collections::HashMap;
use std::path::PathBuf;

const KEYS_DIR: &str = ".aal";
const KEYS_FILE: &str = "keys.json";

#[derive(serde::Serialize, serde::Deserialize)]
struct KeyStore {
    keys: HashMap<String, KeyEntry>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct KeyEntry {
    key: String,
    created_at: String,
}

fn keys_path() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(KEYS_DIR).join(KEYS_FILE)
}

/// 添加密钥
pub fn set_key(provider: &str, key: &str) -> Result<(), String> {
    let path = keys_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create key directory: {}", e))?;
    }
    let mut store = load_store(&path);
    store.keys.insert(provider.to_string(), KeyEntry {
        key: key.to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    });
    save_store(&path, &store)?;
    set_restrictive_permissions(&path);
    Ok(())
}

/// 获取密钥（优先环境变量，回退密钥文件）
pub fn get_key(provider: &str) -> Result<String, String> {
    // 1. 尝试环境变量（CI/容器模式）
    let env_var = format!("AAL_KEY_{}", provider.to_uppercase());
    if let Ok(key) = std::env::var(&env_var) {
        return Ok(key);
    }
    // 2. 尝试密钥文件
    let path = keys_path();
    let store = load_store(&path);
    store.keys.get(provider)
        .map(|e| e.key.clone())
        .ok_or_else(|| format!(
            "no API key found for provider `{}`.\n  to add:    cargo run -- key add --provider {} --key YOUR_KEY\n  or set:    export AAL_KEY_{}=sk-xxx\n  or mock:   AAL_MOCK=1",
            provider, provider, provider.to_uppercase()
        ))
}

/// 列出所有已注册 provider（掩码显示）
pub fn list_keys() -> Result<Vec<(String, String)>, String> {
    let path = keys_path();
    let store = load_store(&path);
    let mut result: Vec<(String, String)> = store.keys.iter().map(|(p, e)| {
        let masked = if e.key.len() > 8 {
            format!("{}…{}", &e.key[..4], &e.key[e.key.len()-4..])
        } else {
            "****".to_string()
        };
        (p.clone(), masked)
    }).collect();
    // 也列出环境变量中的 key
    for (var, val) in std::env::vars() {
        if let Some(provider) = var.strip_prefix("AAL_KEY_") {
            let masked = if val.len() > 8 {
                format!("{}…{} (env)", &val[..4], &val[val.len()-4..])
            } else {
                "**** (env)".to_string()
            };
            result.push((provider.to_lowercase(), masked));
        }
    }
    result.sort_by(|a, b| a.0.cmp(&b.0));
    result.dedup_by(|a, b| a.0 == b.0);
    Ok(result)
}

/// Return the first provider that has a stored key, or "openai" as fallback.
pub fn first_provider() -> String {
    let path = keys_path();
    let store = load_store(&path);
    if let Some(key) = store.keys.keys().next() {
        key.clone()
    } else {
        "openai".to_string()
    }
}

/// 删除密钥
pub fn remove_key(provider: &str) -> Result<(), String> {
    let path = keys_path();
    let mut store = load_store(&path);
    store.keys.remove(provider);
    save_store(&path, &store)
}

// ──── 内部实现 ────

fn load_store(path: &PathBuf) -> KeyStore {
    if !path.exists() {
        return KeyStore { keys: HashMap::new() };
    }
    let content = std::fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or(KeyStore { keys: HashMap::new() })
}

fn save_store(path: &PathBuf, store: &KeyStore) -> Result<(), String> {
    let content = serde_json::to_string_pretty(store)
        .map_err(|e| format!("failed to serialize key store: {}", e))?;
    std::fs::write(path, &content)
        .map_err(|e| format!("failed to write key file: {}", e))
}

#[cfg(unix)]
fn set_restrictive_permissions(path: &PathBuf) {
    use std::os::unix::fs::PermissionsExt;
    if let Ok(meta) = std::fs::metadata(path) {
        let mut perms = meta.permissions();
        perms.set_mode(0o600);
        let _ = std::fs::set_permissions(path, perms);
    }
}

#[cfg(not(unix))]
fn set_restrictive_permissions(_path: &PathBuf) {}
