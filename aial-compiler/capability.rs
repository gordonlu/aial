// capability.rs — AAL capability declaration system (aial.toml)
// Rooted in the Legalist principle: capabilities must be explicitly
// declared; the compiler and runtime enforce them together.

use std::path::Path;

/// Top-level project configuration
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct Config {
    pub capabilities: Option<Capabilities>,
    pub lints: Option<LintConfig>,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct LintConfig {
    pub unused_match_variable: Option<String>,
    pub silent_error_discard: Option<String>,
}

pub fn lint_level(config: &Config, name: &str) -> String {
    let defaults = match name {
        "unused_match_variable" => "warn",
        "silent_error_discard" => "warn",
        _ => "off",
    };
    if let Some(lints) = &config.lints {
        match name {
            "unused_match_variable" => lints.unused_match_variable.clone().unwrap_or_else(|| defaults.to_string()),
            "silent_error_discard" => lints.silent_error_discard.clone().unwrap_or_else(|| defaults.to_string()),
            _ => defaults.to_string(),
        }
    } else {
        defaults.to_string()
    }
}

/// Declared capabilities
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct Capabilities {
    pub allow_network: Option<Vec<NetworkAccess>>,
    pub allow_filesystem: Option<Vec<FilesystemAccess>>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct FilesystemAccess {
    pub path: String,
    pub access: String,  // "read" | "write" | "append"
}

/// A single network access permission
#[derive(Debug, Clone, serde::Deserialize)]
pub struct NetworkAccess {
    pub provider: String,
    pub models: Option<Vec<String>>,
}

/// Load aial.toml from the current directory
pub fn load_config() -> Result<Config, String> {
    let path = Path::new("aial.toml");
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read aial.toml: {}", e))?;
    toml::from_str(&content)
        .map_err(|e| format!("failed to parse aial.toml: {}", e))
}

/// Check whether a given provider+model combination has been declared in capabilities
pub fn check_provider_allowed(config: &Config, provider: &str, model: &str) -> Result<(), String> {
    let caps = config.capabilities.as_ref()
        .ok_or_else(|| format!(
            "capability error: [capabilities] section not found, cannot use provider `{}`",
            provider
        ))?;
    let networks = caps.allow_network.as_ref()
        .ok_or_else(|| format!(
            "capability error: allow_network not declared, cannot use provider `{}`",
            provider
        ))?;
    for access in networks {
        if access.provider == provider {
            if let Some(models) = &access.models {
                if models.iter().any(|m| m == model) {
                    return Ok(());
                }
                return Err(format!(
                    "capability error: model `{}` not authorized for provider `{}` (allowed: {})",
                    model, provider, models.join(", ")
                ));
            }
            return Ok(()); // no model restriction = all allowed
        }
    }
    Err(format!(
        "capability error: provider `{}` not declared in [capabilities].allow_network",
        provider
    ))
}

/// Check if a filesystem path is allowed for the given access level.
pub fn check_filesystem_allowed(config: &Config, path: &str, access: &str) -> Result<(), String> {
    let caps = config.capabilities.as_ref()
        .ok_or_else(|| "filesystem access denied: no [capabilities] declared".to_string())?;
    let allowed = caps.allow_filesystem.as_ref()
        .ok_or_else(|| "filesystem access denied: allow_filesystem not declared".to_string())?;
    let canonical = std::fs::canonicalize(path).unwrap_or_else(|_| Path::new(path).to_path_buf());
    let path_str = canonical.to_string_lossy();
    for entry in allowed {
        if path_str.starts_with(&entry.path) || path == entry.path {
            if entry.access == access || entry.access == "write" && (access == "read" || access == "append") {
                return Ok(());
            }
        }
    }
    Err(format!("filesystem access denied: `{}` not allowed to {} {}", path, access, allowed.iter().map(|a| format!("{}({})", a.path, a.access)).collect::<Vec<_>>().join(", ")))
}

/// Resolve a numeric model code to (provider, model_name).
/// Override via `AIAL_MODEL_<CODE>` environment variable in `provider:model` format.
/// Example: `AIAL_MODEL_0=deepseek:deepseek-v4-flash`
fn default_model(code: i64) -> (&'static str, &'static str) {
    match code {
        0 => ("deepseek", "deepseek-v4-flash"),
        1 => ("deepseek", "deepseek-v4-pro"),
        2 => ("openai", "gpt-4o"),
        3 => ("openai", "gpt-4o-mini"),
        4 => ("anthropic", "claude-sonnet-4-6"),
        _ => ("deepseek", "deepseek-v4-flash"),
    }
}

pub fn resolve_model(model_code: i64) -> (String, String) {
    let env_key = format!("AIAL_MODEL_{}", model_code);
    if let Ok(val) = std::env::var(&env_key) {
        if let Some((provider, model)) = val.split_once(':') {
            return (provider.to_string(), model.to_string());
        }
    }
    let (provider, model) = default_model(model_code);
    (provider.to_string(), model.to_string())
}
