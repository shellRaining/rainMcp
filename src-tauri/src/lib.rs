pub mod agent;
pub mod config;
pub mod mcp;
pub mod window;

use std::env;
use tauri_plugin_log::{Target, TargetKind};

/// 获取日志级别
/// 优先级：环境变量 RUST_LOG > 环境变量 LOG_LEVEL > debug_assertions > Info
fn get_log_level() -> log::LevelFilter {
    // 首先检查 RUST_LOG 环境变量（Rust log crate 的标准环境变量）
    if let Ok(rust_log) = env::var("RUST_LOG") {
        if let Ok(level) = rust_log.parse() {
            return level;
        }
    }

    // 检查自定义的 LOG_LEVEL 环境变量
    if let Ok(log_level) = env::var("LOG_LEVEL") {
        let level_str = log_level.to_uppercase();
        match level_str.as_str() {
            "TRACE" => return log::LevelFilter::Trace,
            "DEBUG" => return log::LevelFilter::Debug,
            "INFO" => return log::LevelFilter::Info,
            "WARN" => return log::LevelFilter::Warn,
            "ERROR" => return log::LevelFilter::Error,
            "OFF" => return log::LevelFilter::Off,
            _ => {}
        }
    }

    // 如果没有环境变量，检查是否是 debug 模式（开发模式）
    if cfg!(debug_assertions) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let log_level = get_log_level();

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                ])
                .level(log_level)
                .build(),
        )
        .plugin(tauri_plugin_decorum::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            mcp::get_agent_mcp_config_command,
            mcp::get_server_raw_config_command,
            mcp::update_agent_mcp_config_command,
            mcp::get_supported_agents_command,
            mcp::get_enabled_agents_command,
            mcp::update_enabled_agents_command,
            mcp::get_app_config_command,
            mcp::update_app_config_command,
            mcp::open_config_file_command,
            mcp::refresh_schema_store_command,
            mcp::get_schema_store_command,
            mcp::get_user_servers_command,
            mcp::add_user_server_command,
            mcp::update_user_server_command,
            mcp::delete_user_server_command,
            mcp::add_server_to_agent_command,
            window::set_traffic_lights_inset_command,
            agent::agent_chat_command,
            agent::agent_chat_stream_command,
            agent::agent_reset_command,
            agent::get_openrouter_api_key_command,
            agent::set_openrouter_api_key_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
