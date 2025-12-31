use std::fs;

use rstest::rstest;
use tempfile::TempDir;

use crate::config::{AppConfig, ClientConfigItem};
use crate::mcp::{get_app_config_command, update_app_config_command};

use super::fixtures::test_env;
use super::EnvGuard;

#[rstest]
fn get_app_config_command_returns_default_when_no_file(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let config = get_app_config_command();
    assert!(config.clients.is_empty());
}

#[rstest]
fn get_app_config_command_reads_existing_config(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    use crate::config::get_app_config_path;
    let config_path = get_app_config_path().unwrap();
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(
        &config_path,
        r#"{
  "clients": {
    "ClaudeCode": {
      "enabled": true
    },
    "Cursor": {
      "enabled": false,
      "custom_config_path": "/custom/path"
    }
  }
}
"#,
    )
    .unwrap();

    let config = get_app_config_command();
    assert_eq!(config.clients.len(), 2);

    let claude = config.clients.get("ClaudeCode").unwrap();
    assert!(claude.enabled);
    assert!(claude.custom_config_path.is_none());

    let cursor = config.clients.get("Cursor").unwrap();
    assert!(!cursor.enabled);
    assert_eq!(cursor.custom_config_path.as_deref(), Some("/custom/path"));
}

#[rstest]
fn get_app_config_command_handles_invalid_json(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    use crate::config::get_app_config_path;
    let config_path = get_app_config_path().unwrap();
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(&config_path, "invalid json {").unwrap();

    let config = get_app_config_command();
    assert!(config.clients.is_empty());
}

#[rstest]
fn update_app_config_command_creates_new_config(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let mut config = AppConfig::default();
    config.clients.insert(
        "ClaudeCode".to_string(),
        ClientConfigItem { enabled: true, custom_config_path: None },
    );

    update_app_config_command(config).unwrap();

    let saved_config = get_app_config_command();
    assert_eq!(saved_config.clients.len(), 1);
    assert!(saved_config.clients.get("ClaudeCode").unwrap().enabled);
}

#[rstest]
fn update_app_config_command_overwrites_existing_config(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    use crate::config::get_app_config_path;
    let config_path = get_app_config_path().unwrap();
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(
        &config_path,
        r#"{
  "clients": {
    "OldAgent": {
      "enabled": true
    }
  }
}
"#,
    )
    .unwrap();

    let mut config = AppConfig::default();
    config.clients.insert(
        "ClaudeCode".to_string(),
        ClientConfigItem { enabled: true, custom_config_path: Some("/new/path".to_string()) },
    );

    update_app_config_command(config).unwrap();

    let saved_config = get_app_config_command();
    assert_eq!(saved_config.clients.len(), 1);
    assert!(!saved_config.clients.contains_key("OldAgent"));
    assert!(saved_config.clients.contains_key("ClaudeCode"));
    assert_eq!(
        saved_config.clients.get("ClaudeCode").unwrap().custom_config_path.as_deref(),
        Some("/new/path")
    );
}

#[rstest]
fn update_app_config_command_creates_parent_directories(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    use crate::config::get_app_config_path;
    let config_path = get_app_config_path().unwrap();
    assert!(!config_path.exists());
    assert!(!config_path.parent().unwrap().exists());

    let config = AppConfig::default();
    update_app_config_command(config).unwrap();

    assert!(config_path.exists());
}

#[rstest]
fn update_app_config_command_preserves_custom_config_path(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let mut config = AppConfig::default();
    config.clients.insert(
        "ClaudeCode".to_string(),
        ClientConfigItem {
            enabled: true,
            custom_config_path: Some("/custom/claude.json".to_string()),
        },
    );
    config.clients.insert(
        "Cursor".to_string(),
        ClientConfigItem { enabled: false, custom_config_path: None },
    );

    update_app_config_command(config).unwrap();

    let saved_config = get_app_config_command();
    assert_eq!(saved_config.clients.len(), 2);
    assert_eq!(
        saved_config.clients.get("ClaudeCode").unwrap().custom_config_path.as_deref(),
        Some("/custom/claude.json")
    );
    assert!(saved_config.clients.get("Cursor").unwrap().custom_config_path.is_none());
}

#[rstest]
fn update_app_config_command_serializes_correctly(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let mut config = AppConfig::default();
    config.clients.insert(
        "Agent1".to_string(),
        ClientConfigItem { enabled: true, custom_config_path: Some("/path".to_string()) },
    );
    config.clients.insert(
        "Agent2".to_string(),
        ClientConfigItem { enabled: false, custom_config_path: None },
    );

    update_app_config_command(config).unwrap();

    use crate::config::get_app_config_path;
    let config_path = get_app_config_path().unwrap();
    let content = fs::read_to_string(&config_path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    let clients = parsed.get("clients").and_then(|v| v.as_object()).unwrap();
    assert_eq!(clients.len(), 2);

    let agent1 = clients.get("Agent1").and_then(|v| v.as_object()).unwrap();
    assert_eq!(agent1.get("enabled").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(agent1.get("custom_config_path").and_then(|v| v.as_str()), Some("/path"));

    let agent2 = clients.get("Agent2").and_then(|v| v.as_object()).unwrap();
    assert_eq!(agent2.get("enabled").and_then(|v| v.as_bool()), Some(false));
    assert!(agent2.get("custom_config_path").is_none());
}
