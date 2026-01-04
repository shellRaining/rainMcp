use std::fs;

use rstest::rstest;
use tempfile::TempDir;

use crate::mcp::{get_enabled_agents_command, update_enabled_agents_command, AgentType};

use super::fixtures::test_env;
use super::EnvGuard;

#[rstest]
fn get_enabled_agents_command_returns_empty_when_no_config(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let agents = get_enabled_agents_command().unwrap();
    assert!(agents.is_empty());
}

#[rstest]
fn get_enabled_agents_command_returns_enabled_agents(
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
      "enabled": false
    },
    "Windsurf": {
      "enabled": true
    }
  }
}
"#,
    )
    .unwrap();

    let agents = get_enabled_agents_command().unwrap();
    assert_eq!(agents.len(), 2);
    assert!(agents.contains(&AgentType::ClaudeCode));
    assert!(agents.contains(&AgentType::Windsurf));
    assert!(!agents.contains(&AgentType::Cursor));
}

#[rstest]
fn get_enabled_agents_command_ignores_unknown_agents(
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
    "UnknownAgent": {
      "enabled": true
    }
  }
}
"#,
    )
    .unwrap();

    let agents = get_enabled_agents_command().unwrap();
    assert_eq!(agents.len(), 1);
    assert!(agents.contains(&AgentType::ClaudeCode));
}

#[rstest]
fn update_enabled_agents_command_creates_config(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    update_enabled_agents_command(vec!["ClaudeCode".to_string(), "Cursor".to_string()]).unwrap();

    let agents = get_enabled_agents_command().unwrap();
    assert_eq!(agents.len(), 2);
    assert!(agents.contains(&AgentType::ClaudeCode));
    assert!(agents.contains(&AgentType::Cursor));
}

#[rstest]
fn update_enabled_agents_command_disables_previously_enabled(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    update_enabled_agents_command(vec![
        "ClaudeCode".to_string(),
        "Cursor".to_string(),
        "Windsurf".to_string(),
    ])
    .unwrap();

    let agents = get_enabled_agents_command().unwrap();
    assert_eq!(agents.len(), 3);

    update_enabled_agents_command(vec!["ClaudeCode".to_string()]).unwrap();

    let agents = get_enabled_agents_command().unwrap();
    assert_eq!(agents.len(), 1);
    assert!(agents.contains(&AgentType::ClaudeCode));
    assert!(!agents.contains(&AgentType::Cursor));
    assert!(!agents.contains(&AgentType::Windsurf));
}

#[rstest]
fn update_enabled_agents_command_preserves_all_agent_configs(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    update_enabled_agents_command(vec!["ClaudeCode".to_string()]).unwrap();

    use crate::config::get_app_config_path;
    let config_path = get_app_config_path().unwrap();
    let content = fs::read_to_string(&config_path).unwrap();
    let config: serde_json::Value = serde_json::from_str(&content).unwrap();
    let clients = config.get("clients").and_then(|v| v.as_object()).unwrap();

    assert_eq!(clients.len(), 14);

    assert_eq!(
        clients.get("ClaudeCode").and_then(|v| v.get("enabled")).and_then(|v| v.as_bool()),
        Some(true)
    );
    assert_eq!(
        clients.get("Cursor").and_then(|v| v.get("enabled")).and_then(|v| v.as_bool()),
        Some(false)
    );
}

#[rstest]
fn update_enabled_agents_command_rejects_unknown_agent(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let result = update_enabled_agents_command(vec!["unknown-agent".to_string()]);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unknown agent"));
}

#[rstest]
fn update_enabled_agents_command_accepts_various_name_formats(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    update_enabled_agents_command(vec![
        "claude-code".to_string(),
        "cursor".to_string(),
        "openai_codex".to_string(),
        "vscode_copilot".to_string(),
        "copilot-cli".to_string(),
    ])
    .unwrap();

    let agents = get_enabled_agents_command().unwrap();
    assert_eq!(agents.len(), 5);
    assert!(agents.contains(&AgentType::ClaudeCode));
    assert!(agents.contains(&AgentType::Cursor));
    assert!(agents.contains(&AgentType::OpenAiCodex));
    assert!(agents.contains(&AgentType::VsCodeCopilot));
    assert!(agents.contains(&AgentType::CopilotCli));
}
