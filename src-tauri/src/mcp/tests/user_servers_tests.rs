use std::collections::HashMap;

use rstest::rstest;
use tempfile::TempDir;

use crate::config::{load_app_config, save_app_config, AppConfig};
use crate::mcp::user_server::{OriginType, ServerOrigin, UserServer};
use crate::mcp::{AgentServerEntry, BaseServerEntry, LocalServerEntry, RemoteServerEntry};

use super::fixtures::test_env;
use super::EnvGuard;

fn create_test_user_server(id: &str, name: &str) -> UserServer {
    let mut env = HashMap::new();
    env.insert("API_KEY".to_string(), "test-key-123".to_string());

    UserServer {
        id: id.to_string(),
        name: name.to_string(),
        config: AgentServerEntry::Local(LocalServerEntry {
            base: BaseServerEntry { timeout: None },
            command: "npx".to_string(),
            args: Some(vec!["-y".to_string(), "@test/mcp-server".to_string()]),
            env: Some(env),
        }),
        origin: Some(ServerOrigin {
            origin_type: OriginType::Registry,
            schema_name: Some("io.github.test/test-server".to_string()),
            package_id: Some("npm:@test/mcp-server".to_string()),
        }),
        created_at: Some("2025-01-01T00:00:00Z".to_string()),
    }
}

#[rstest]
fn get_user_servers_returns_empty_initially(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let app_config = load_app_config();
    assert!(app_config.user_servers.is_empty());
}

#[rstest]
fn add_user_server_saves_to_config(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let server = create_test_user_server("test-id", "Test Server");

    let mut app_config = load_app_config();
    app_config.user_servers.push(server);
    save_app_config(&app_config).unwrap();

    let loaded = load_app_config();
    assert_eq!(loaded.user_servers.len(), 1);
    assert_eq!(loaded.user_servers[0].id, "test-id");
    assert_eq!(loaded.user_servers[0].name, "Test Server");
}

#[rstest]
fn user_server_preserves_config(test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>)) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let server = create_test_user_server("config-test", "Config Test Server");

    let mut app_config = load_app_config();
    app_config.user_servers.push(server);
    save_app_config(&app_config).unwrap();

    let loaded = load_app_config();
    let server = &loaded.user_servers[0];
    match &server.config {
        AgentServerEntry::Local(local) => {
            assert_eq!(local.command, "npx");
            assert!(local.env.as_ref().unwrap().contains_key("API_KEY"));
        }
        _ => panic!("Expected Local config"),
    }
}

#[rstest]
fn update_user_server_modifies_existing(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let server = create_test_user_server("update-test", "Original Name");

    let mut app_config = load_app_config();
    app_config.user_servers.push(server);
    save_app_config(&app_config).unwrap();

    // Update the server
    let mut app_config = load_app_config();
    let server = &mut app_config.user_servers[0];
    server.name = "Updated Name".to_string();
    save_app_config(&app_config).unwrap();

    let loaded = load_app_config();
    assert_eq!(loaded.user_servers[0].name, "Updated Name");
}

#[rstest]
fn delete_user_server_removes_from_config(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let server1 = create_test_user_server("server-1", "Server 1");
    let server2 = create_test_user_server("server-2", "Server 2");

    let mut app_config = load_app_config();
    app_config.user_servers.push(server1);
    app_config.user_servers.push(server2);
    save_app_config(&app_config).unwrap();

    // Delete server-1
    let mut app_config = load_app_config();
    app_config.user_servers.retain(|s| s.id != "server-1");
    save_app_config(&app_config).unwrap();

    let loaded = load_app_config();
    assert_eq!(loaded.user_servers.len(), 1);
    assert_eq!(loaded.user_servers[0].id, "server-2");
}

#[rstest]
fn user_server_with_custom_command(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let server = UserServer {
        id: "custom-cmd".to_string(),
        name: "Custom Server".to_string(),
        config: AgentServerEntry::Local(LocalServerEntry {
            base: BaseServerEntry { timeout: None },
            command: "python".to_string(),
            args: Some(vec!["-m".to_string(), "mcp_server".to_string()]),
            env: None,
        }),
        origin: Some(ServerOrigin {
            origin_type: OriginType::Custom,
            schema_name: None,
            package_id: None,
        }),
        created_at: None,
    };

    let mut app_config = load_app_config();
    app_config.user_servers.push(server);
    save_app_config(&app_config).unwrap();

    let loaded = load_app_config();
    let server = &loaded.user_servers[0];
    match &server.config {
        AgentServerEntry::Local(local) => {
            assert_eq!(local.command, "python");
            assert_eq!(local.args, Some(vec!["-m".to_string(), "mcp_server".to_string()]));
        }
        _ => panic!("Expected Local config"),
    }
}

#[rstest]
fn user_server_with_remote_url(test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>)) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let server = UserServer {
        id: "remote-server".to_string(),
        name: "Remote Server".to_string(),
        config: AgentServerEntry::Remote(RemoteServerEntry {
            base: BaseServerEntry { timeout: None },
            url: "https://mcp.example.com/sse".to_string(),
            headers: None,
        }),
        origin: Some(ServerOrigin {
            origin_type: OriginType::Custom,
            schema_name: None,
            package_id: None,
        }),
        created_at: None,
    };

    let mut app_config = load_app_config();
    app_config.user_servers.push(server);
    save_app_config(&app_config).unwrap();

    let loaded = load_app_config();
    let server = &loaded.user_servers[0];
    match &server.config {
        AgentServerEntry::Remote(remote) => {
            assert_eq!(remote.url, "https://mcp.example.com/sse");
        }
        _ => panic!("Expected Remote config"),
    }
}

#[rstest]
fn multiple_user_servers_preserved(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let servers = vec![
        create_test_user_server("server-a", "Server A"),
        create_test_user_server("server-b", "Server B"),
        create_test_user_server("server-c", "Server C"),
    ];

    let mut app_config = load_app_config();
    app_config.user_servers = servers;
    save_app_config(&app_config).unwrap();

    let loaded = load_app_config();
    assert_eq!(loaded.user_servers.len(), 3);
    assert_eq!(loaded.user_servers[0].id, "server-a");
    assert_eq!(loaded.user_servers[1].id, "server-b");
    assert_eq!(loaded.user_servers[2].id, "server-c");
}

#[rstest]
fn user_servers_coexist_with_clients(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let server = create_test_user_server("coexist-test", "Coexist Server");

    let mut app_config = AppConfig::default();
    app_config.clients.insert(
        "ClaudeCode".to_string(),
        crate::config::ClientConfigItem { enabled: true, custom_config_path: None },
    );
    app_config.user_servers.push(server);
    save_app_config(&app_config).unwrap();

    let loaded = load_app_config();
    assert_eq!(loaded.clients.len(), 1);
    assert!(loaded.clients.get("ClaudeCode").unwrap().enabled);
    assert_eq!(loaded.user_servers.len(), 1);
    assert_eq!(loaded.user_servers[0].id, "coexist-test");
}
