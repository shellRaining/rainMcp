//! Agent chat logic

use std::sync::Mutex;

use futures::StreamExt;
use log::{debug, warn};
use once_cell::sync::Lazy;
use rig::agent::MultiTurnStreamItem;
use rig::client::{CompletionClient, ProviderClient};
use rig::completion::message::{AssistantContent, ToolChoice, ToolResultContent, UserContent};
use rig::completion::request::PromptError;
use rig::completion::{CompletionError, Message, Prompt};
use rig::message::Text;
use rig::providers::openrouter;
use rig::streaming::{StreamedAssistantContent, StreamedUserContent, StreamingPrompt};
use tauri::{AppHandle, Emitter};

use super::tools::{FetchWebpage, GenerateServerSchema};
use super::types::{
    ChatMessage, ChatRole, GeneratedSchema, StreamEvent, ToolCallInfo, ToolCallStatus,
};
use crate::mcp::server_schema::ServerSchema;

/// Global chat history (simplified single session)
static CHAT_HISTORY: Lazy<Mutex<Vec<Message>>> = Lazy::new(|| Mutex::new(Vec::new()));

const SYSTEM_PREAMBLE: &str = r#"
You are an MCP (Model Context Protocol) ServerSchema generator. Your job is to create complete configuration schemas conforming to the official MCP Registry format.

MANDATORY WORKFLOW - You MUST follow ALL these steps:

Step 1: Call fetch_webpage(url) to get documentation
Step 2: EXPLAIN briefly what you found (package name, installation method)
Step 3: Call generate_server_schema with the complete schema
Step 4: Say ONLY "Schema generated. Click 'Use' to add it." - nothing more!

CRITICAL RULES:
- After generate_server_schema succeeds, your response MUST be extremely short (under 20 words)
- DO NOT repeat or explain the schema - the UI shows it automatically
- DO NOT list features, configuration options, or usage instructions after schema generation
- AVOID redundant fetch_webpage calls: Do NOT fetch both a GitHub repo page AND its raw README - they contain the same information. One fetch is enough.

SCHEMA RULES:
For npm: registryType='npm', runtimeHint='npx', identifier='@org/package'
For pypi: registryType='pypi', runtimeHint='uvx', identifier='package-name'
For hosted: type='sse', url='https://...'
Naming: 'io.github.owner.repo' format

START WORKING NOW. Don't ask - just do all steps automatically.
"#;

/// Send a message to the agent and get a response
pub async fn send_message(api_key: &str, user_message: &str) -> Result<ChatMessage, String> {
    // Set the API key in environment for OpenRouter client
    std::env::set_var("OPENROUTER_API_KEY", api_key);

    // Initialize OpenRouter client
    let client = openrouter::Client::from_env();

    // Create agent with tools
    let agent = client
        .agent("anthropic/claude-sonnet-4.5")
        .preamble(SYSTEM_PREAMBLE)
        .tool(FetchWebpage)
        .tool(GenerateServerSchema)
        .build();

    // Get chat history
    let base_history = CHAT_HISTORY.lock().map_err(|e| e.to_string())?.clone();
    debug!(
        "[agent] request model=claude-sonnet-4.5 message_len={} history_len={}",
        user_message.len(),
        base_history.len()
    );

    // Call agent (allow tool calls)
    let (mut response, mut history) = {
        let mut history = base_history.clone();
        match agent.prompt(user_message).with_history(&mut history).multi_turn(6).await {
            Ok(response) => {
                debug!(
                    "[agent] response received chars={} history_len={}",
                    response.len(),
                    history.len()
                );
                (response, history)
            }
            Err(err) => {
                if is_empty_response_error(&err) {
                    warn!("[agent] empty response from provider, retrying without tools");
                    let fallback_agent = client
                        .agent("anthropic/claude-sonnet-4.5")
                        .preamble(SYSTEM_PREAMBLE)
                        .build();
                    let fallback_prompt = format!(
                        "{}\n\nRespond directly without calling tools. If you need more information, ask one concise question.",
                        user_message
                    );
                    let mut history = base_history.clone();
                    match fallback_agent.prompt(fallback_prompt).with_history(&mut history).await {
                        Ok(response) => {
                            debug!(
                                "[agent] fallback response received chars={} history_len={}",
                                response.len(),
                                history.len()
                            );
                            (response, history)
                        }
                        Err(_) => {
                            warn!("[agent] fallback response empty or failed");
                            let message = ChatMessage {
                                id: uuid::Uuid::new_v4().to_string(),
                                role: ChatRole::Assistant,
                                content: "The model returned an empty response. Please try again."
                                    .to_string(),
                                timestamp: chrono::Utc::now().to_rfc3339(),
                                tool_calls: None,
                                generated_schema: None,
                            };
                            return Ok(message);
                        }
                    }
                } else if let PromptError::MaxDepthError { max_depth, chat_history, .. } = err {
                    let history = chat_history.to_vec();
                    let response = extract_last_assistant_text(&history).unwrap_or_else(|| {
                        format!(
                            "The model reached the tool-call limit ({}). Attempting to recover the configuration.",
                            max_depth
                        )
                    });
                    warn!("[agent] max depth reached: {}", max_depth);
                    (response, history)
                } else {
                    warn!("[agent] error: {}", err);
                    return Err(format!("Agent error: {}", err));
                }
            }
        }
    };

    let mut generated_schema = extract_generated_schema_from_history(&history)
        .or_else(|| parse_generated_schema(&response));

    if generated_schema.is_none() {
        warn!("[agent] no generated schema found, forcing generate_server_schema");
        let forced_agent = client
            .agent("anthropic/claude-sonnet-4.5")
            .preamble(SYSTEM_PREAMBLE)
            .tool(FetchWebpage)
            .tool(GenerateServerSchema)
            .tool_choice(ToolChoice::Specific {
                function_names: vec!["generate_server_schema".to_string()],
            })
            .build();
        let forced_prompt = "Based on the conversation so far, call generate_server_schema now.";
        let mut forced_history = history.clone();
        match forced_agent
            .prompt(forced_prompt)
            .with_history(&mut forced_history)
            .multi_turn(2)
            .await
        {
            Ok(forced_response) => {
                debug!(
                    "[agent] forced response received chars={} history_len={}",
                    forced_response.len(),
                    forced_history.len()
                );
                history = forced_history;
                if response.trim().is_empty() {
                    response = forced_response;
                }
                generated_schema = extract_generated_schema_from_history(&history)
                    .or_else(|| parse_generated_schema(&response));
            }
            Err(err) => {
                warn!("[agent] forced generate_server_schema failed: {}", err);
            }
        }
    }

    if generated_schema.is_some() {
        debug!("[agent] generated schema extracted");
    } else {
        warn!("[agent] generated schema missing");
    }

    let tool_calls = extract_tool_calls_from_history(&history);

    // Update history
    {
        let mut history_guard = CHAT_HISTORY.lock().map_err(|e| e.to_string())?;
        *history_guard = history;
    }

    // Build response message
    let message = ChatMessage {
        id: uuid::Uuid::new_v4().to_string(),
        role: ChatRole::Assistant,
        content: response,
        timestamp: chrono::Utc::now().to_rfc3339(),
        tool_calls: if tool_calls.is_empty() { None } else { Some(tool_calls) },
        generated_schema,
    };

    Ok(message)
}

/// Event name for streaming chat events
pub const STREAM_EVENT_NAME: &str = "agent-stream";

/// Send a message to the agent with streaming response
pub async fn send_message_stream(
    app: AppHandle,
    api_key: &str,
    user_message: &str,
) -> Result<(), String> {
    // Set the API key in environment for OpenRouter client
    std::env::set_var("OPENROUTER_API_KEY", api_key);

    // Initialize OpenRouter client
    let client = openrouter::Client::from_env();

    // Create agent with tools
    let agent = client
        .agent("anthropic/claude-sonnet-4.5")
        .preamble(SYSTEM_PREAMBLE)
        .tool(FetchWebpage)
        .tool(GenerateServerSchema)
        .build();

    // Get chat history
    let base_history = CHAT_HISTORY.lock().map_err(|e| e.to_string())?.clone();
    debug!(
        "[agent:stream] request model=claude-sonnet-4.5 message_len={} history_len={}",
        user_message.len(),
        base_history.len()
    );

    let message_id = uuid::Uuid::new_v4().to_string();

    // Use streaming with multi-turn for tool calling
    let mut stream =
        agent.stream_prompt(user_message).with_history(base_history.clone()).multi_turn(6).await;

    let mut full_text = String::new();
    let mut tool_calls: Vec<ToolCallInfo> = Vec::new();
    let mut generated_schema: Option<GeneratedSchema> = None;
    let mut final_history = base_history;

    // Add user message to history
    final_history.push(Message::user(user_message));

    // Process stream chunks
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(item) => match item {
                MultiTurnStreamItem::StreamAssistantItem(content) => match content {
                    StreamedAssistantContent::Text(Text { text }) => {
                        full_text.push_str(&text);
                        let event = StreamEvent::Text { text };
                        if let Err(e) = app.emit(STREAM_EVENT_NAME, &event) {
                            warn!("[agent:stream] emit error: {}", e);
                        }
                    }
                    StreamedAssistantContent::ToolCall(tool_call) => {
                        let name = tool_call.function.name.clone();
                        debug!("[agent:stream] tool call: {}", name);

                        // Extract args preview
                        let args_str = tool_call.function.arguments.to_string();
                        let args_preview = if args_str == "null" || args_str == "{}" {
                            None
                        } else if args_str.len() > 200 {
                            Some(format!("{}...", &args_str[..200]))
                        } else {
                            Some(args_str)
                        };

                        // Emit tool call start
                        let start_event =
                            StreamEvent::ToolCallStart { name: name.clone(), args_preview };
                        if let Err(e) = app.emit(STREAM_EVENT_NAME, &start_event) {
                            warn!("[agent:stream] emit error: {}", e);
                        }

                        tool_calls.push(ToolCallInfo {
                            name,
                            status: ToolCallStatus::Running,
                            result_preview: None,
                        });
                    }
                    _ => {}
                },
                MultiTurnStreamItem::StreamUserItem(StreamedUserContent::ToolResult(
                    tool_result,
                )) => {
                    debug!("[agent:stream] tool result received");

                    // Extract text from tool result
                    let result_text: String = tool_result
                        .content
                        .clone()
                        .into_iter()
                        .filter_map(|c| {
                            if let ToolResultContent::Text(text) = c {
                                Some(text.text().to_string())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("");

                    // Try to extract schema from tool result
                    debug!(
                        "[agent:stream] tool result text len={}, preview={}",
                        result_text.len(),
                        if result_text.len() > 100 { &result_text[..100] } else { &result_text }
                    );
                    if let Some(schema) = parse_generated_schema_from_text(&result_text) {
                        debug!("[agent:stream] schema parsed successfully: {}", schema.schema.name);
                        if generated_schema.is_none() {
                            generated_schema = Some(schema.clone());
                            let schema_event = StreamEvent::Schema { schema };
                            debug!("[agent:stream] emitting schema event");
                            if let Err(e) = app.emit(STREAM_EVENT_NAME, &schema_event) {
                                warn!("[agent:stream] emit error: {}", e);
                            }
                        }
                    } else {
                        debug!("[agent:stream] schema parse failed for tool result");
                    }

                    // Get the name of the last tool call
                    let last_call_name = tool_calls.last().map(|c| c.name.as_str()).unwrap_or("");

                    // Update the last tool call with result
                    // For generate_server_schema, don't truncate so frontend can parse it
                    let result_preview = if last_call_name == "generate_server_schema" {
                        if !result_text.is_empty() {
                            Some(result_text.clone())
                        } else {
                            None
                        }
                    } else if result_text.len() > 200 {
                        Some(format!("{}...", &result_text[..200]))
                    } else if !result_text.is_empty() {
                        Some(result_text.clone())
                    } else {
                        None
                    };

                    if let Some(last_call) = tool_calls.last_mut() {
                        last_call.status = ToolCallStatus::Success;
                        last_call.result_preview = result_preview.clone();

                        // Emit tool call end
                        let end_event = StreamEvent::ToolCallEnd {
                            name: last_call.name.clone(),
                            result_preview,
                        };
                        if let Err(e) = app.emit(STREAM_EVENT_NAME, &end_event) {
                            warn!("[agent:stream] emit error: {}", e);
                        }
                    }
                }
                MultiTurnStreamItem::FinalResponse(final_response) => {
                    debug!(
                        "[agent:stream] final response: {} chars",
                        final_response.response().len()
                    );
                    // Use the final response text if we didn't capture any streaming text
                    if full_text.is_empty() {
                        full_text = final_response.response().to_string();
                    }
                }
                _ => {}
            },
            Err(e) => {
                warn!("[agent:stream] stream error: {}", e);
                let error_event = StreamEvent::Error { error: e.to_string() };
                if let Err(emit_err) = app.emit(STREAM_EVENT_NAME, &error_event) {
                    warn!("[agent:stream] emit error: {}", emit_err);
                }
            }
        }
    }

    debug!(
        "[agent:stream] stream completed, text_len={} tool_calls={}",
        full_text.len(),
        tool_calls.len()
    );

    // Add assistant message to history
    if !full_text.is_empty() {
        final_history.push(Message::assistant(&full_text));
    }

    // Update global history
    {
        let mut history_guard = CHAT_HISTORY.lock().map_err(|e| e.to_string())?;
        *history_guard = final_history;
    }

    // Emit done event
    let done_event = StreamEvent::Done { message_id: message_id.clone() };
    if let Err(e) = app.emit(STREAM_EVENT_NAME, &done_event) {
        warn!("[agent:stream] emit error: {}", e);
    }

    Ok(())
}

fn is_empty_response_error(error: &PromptError) -> bool {
    match error {
        PromptError::CompletionError(CompletionError::ResponseError(message)) => {
            message.contains("Response contained no message or tool call")
        }
        _ => false,
    }
}

/// Reset chat history
pub fn reset_chat() {
    if let Ok(mut history) = CHAT_HISTORY.lock() {
        history.clear();
    }
}

/// Try to parse generated schema from agent response
fn parse_generated_schema(response: &str) -> Option<GeneratedSchema> {
    parse_generated_schema_from_text(response)
}

fn parse_generated_schema_from_text(text: &str) -> Option<GeneratedSchema> {
    // First, try to parse if the text is a JSON string (escaped JSON)
    // This handles cases where rig returns tool result as a JSON string value
    let unescaped_text = if text.starts_with('"') && text.ends_with('"') {
        // Try to unescape JSON string
        match serde_json::from_str::<String>(text) {
            Ok(s) => s,
            Err(_) => text.to_string(),
        }
    } else {
        text.to_string()
    };

    // Try to parse as ServerSchema directly
    match serde_json::from_str::<ServerSchema>(&unescaped_text) {
        Ok(schema) => {
            return Some(GeneratedSchema {
                schema,
                confidence: 0.8,
                explanation: "Generated from AI response".to_string(),
            });
        }
        Err(e) => {
            debug!("[agent:schema] direct parse failed: {}", e);
        }
    }

    // Try to find JSON-like content
    let start = unescaped_text.find('{')?;
    let end = unescaped_text.rfind('}')?;

    if start >= end {
        return None;
    }

    let json_str = &unescaped_text[start..=end];

    match serde_json::from_str::<ServerSchema>(json_str) {
        Ok(schema) => Some(GeneratedSchema {
            schema,
            confidence: 0.8,
            explanation: "Generated from AI response".to_string(),
        }),
        Err(e) => {
            debug!("[agent:schema] extracted json parse failed: {}", e);
            None
        }
    }
}

fn extract_generated_schema_from_history(history: &[Message]) -> Option<GeneratedSchema> {
    for message in history.iter().rev() {
        if let Message::User { content } = message {
            for user_content in content.clone().into_iter() {
                if let UserContent::ToolResult(tool_result) = user_content {
                    for result_content in tool_result.content.clone().into_iter() {
                        if let ToolResultContent::Text(text) = result_content {
                            if let Some(schema) = parse_generated_schema_from_text(text.text()) {
                                return Some(schema);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn extract_tool_calls_from_history(history: &[Message]) -> Vec<ToolCallInfo> {
    let mut calls: Vec<(String, ToolCallInfo)> = Vec::new();

    for message in history {
        match message {
            Message::Assistant { content, .. } => {
                for item in content.clone().into_iter() {
                    if let AssistantContent::ToolCall(call) = item {
                        let info = ToolCallInfo {
                            name: call.function.name.clone(),
                            status: ToolCallStatus::Running,
                            result_preview: None,
                        };
                        calls.push((call.id.clone(), info));
                    }
                }
            }
            Message::User { content } => {
                for item in content.clone().into_iter() {
                    if let UserContent::ToolResult(result) = item {
                        let preview = summarize_tool_result(&result);
                        if let Some(index) = calls.iter().position(|(id, _)| id == &result.id) {
                            calls[index].1.status = ToolCallStatus::Success;
                            calls[index].1.result_preview = preview;
                        } else if let Some(last) = calls
                            .iter_mut()
                            .rev()
                            .find(|(_, info)| matches!(info.status, ToolCallStatus::Running))
                        {
                            last.1.status = ToolCallStatus::Success;
                            last.1.result_preview = preview;
                        }
                    }
                }
            }
        }
    }

    for (_, info) in calls.iter_mut() {
        if matches!(info.status, ToolCallStatus::Running) {
            info.status = ToolCallStatus::Pending;
        }
    }

    calls.into_iter().map(|(_, info)| info).collect()
}

fn summarize_tool_result(result: &rig::completion::message::ToolResult) -> Option<String> {
    const PREVIEW_LIMIT: usize = 200;
    for content in result.content.clone().into_iter() {
        match content {
            ToolResultContent::Text(text) => {
                let trimmed = text.text().trim().to_string();
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed.len() > PREVIEW_LIMIT {
                    return Some(format!("{}...", &trimmed[..PREVIEW_LIMIT]));
                }
                return Some(trimmed);
            }
            ToolResultContent::Image(_) => return Some("[image result]".to_string()),
        }
    }
    None
}

fn extract_last_assistant_text(history: &[Message]) -> Option<String> {
    for message in history.iter().rev() {
        if let Message::Assistant { content, .. } = message {
            let mut parts = Vec::new();
            for item in content.clone().into_iter() {
                match item {
                    AssistantContent::Text(text) => parts.push(text.text().to_string()),
                    AssistantContent::Reasoning(reasoning) => {
                        for line in reasoning.reasoning {
                            if !line.trim().is_empty() {
                                parts.push(line);
                            }
                        }
                    }
                    _ => {}
                }
            }
            if !parts.is_empty() {
                return Some(parts.join("\n"));
            }
        }
    }
    None
}
