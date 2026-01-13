use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::mcp::user_server::UserServer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfigItem {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_config_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub clients: HashMap<String, ClientConfigItem>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_servers: Vec<UserServer>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub openrouter_api_key: Option<String>,
}

pub fn get_app_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("rain-mcp").join("settings.json"))
}

pub fn get_app_config_path_display_string() -> Option<String> {
    let path = get_app_config_path()?;

    #[cfg(windows)]
    {
        return Some(path.to_string_lossy().to_string());
    }

    #[cfg(not(windows))]
    {
        if let Some(home_dir) = dirs::home_dir() {
            if path.starts_with(&home_dir) {
                if let Ok(rel) = path.strip_prefix(&home_dir) {
                    let rel = rel.to_string_lossy();
                    let rel = rel.as_ref().trim_start_matches('/');
                    return Some(format!("~/{}", rel));
                }
            }
        }

        Some(path.to_string_lossy().to_string())
    }
}

pub fn load_app_config() -> AppConfig {
    let path = match get_app_config_path() {
        Some(p) => p,
        None => return AppConfig::default(),
    };

    if !path.exists() {
        return AppConfig::default();
    }

    match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => AppConfig::default(),
    }
}

pub fn save_app_config(config: &AppConfig) -> Result<(), String> {
    let path = get_app_config_path().ok_or("Could not determine config path")?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let content = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}
