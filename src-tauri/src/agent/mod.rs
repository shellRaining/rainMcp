//! AI Agent module for MCP configuration assistance

pub mod chat;
pub mod tools;
pub mod types;

use crate::config::{load_app_config, save_app_config};
use tauri::AppHandle;
use types::ChatMessage;

/// Send a chat message to the AI agent
#[tauri::command]
pub async fn agent_chat_command(message: String) -> Result<ChatMessage, String> {
    let config = load_app_config();
    let api_key = config
        .openrouter_api_key
        .ok_or("OpenRouter API key not configured. Please add 'openrouter_api_key' to ~/.config/rain-mcp/settings.json")?;

    chat::send_message(&api_key, &message).await
}

/// Send a chat message to the AI agent with streaming response
#[tauri::command]
pub async fn agent_chat_stream_command(app: AppHandle, message: String) -> Result<(), String> {
    let config = load_app_config();
    let api_key = config
        .openrouter_api_key
        .ok_or("OpenRouter API key not configured. Please add 'openrouter_api_key' to ~/.config/rain-mcp/settings.json")?;

    chat::send_message_stream(app, &api_key, &message).await
}

/// Reset the chat session
#[tauri::command]
pub fn agent_reset_command() -> Result<(), String> {
    chat::reset_chat();
    Ok(())
}

/// Check if OpenRouter API key is configured
#[tauri::command]
pub fn get_openrouter_api_key_command() -> Result<Option<String>, String> {
    let config = load_app_config();
    // Return whether configured (not the actual key)
    Ok(config.openrouter_api_key.map(|_| "configured".to_string()))
}

/// Set the OpenRouter API key
#[tauri::command]
pub fn set_openrouter_api_key_command(api_key: String) -> Result<(), String> {
    let mut config = load_app_config();
    config.openrouter_api_key = if api_key.is_empty() { None } else { Some(api_key) };
    save_app_config(&config)?;
    Ok(())
}
