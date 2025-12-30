pub mod config;
pub mod mcp;

use tauri_plugin_log::{Target, TargetKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                ])
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            mcp::get_agent_mcp_config_command,
            mcp::update_agent_mcp_config_command,
            mcp::get_supported_agents_command,
            mcp::get_enabled_agents_command,
            mcp::update_enabled_agents_command,
            mcp::get_app_config_command,
            mcp::update_app_config_command,
            mcp::open_config_file_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
