//! Agent chat types

use serde::{Deserialize, Serialize};

use crate::mcp::server_schema::ServerSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub id: String,
    pub role: ChatRole,
    pub content: String,
    pub timestamp: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallInfo>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_schema: Option<GeneratedSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCallInfo {
    pub name: String,
    pub status: ToolCallStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_preview: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolCallStatus {
    Pending,
    Running,
    Success,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedSchema {
    pub schema: ServerSchema,
    pub confidence: f32,
    pub explanation: String,
}

/// Streaming event payload sent to frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
#[allow(clippy::large_enum_variant)]
pub enum StreamEvent {
    /// Text chunk from the model
    #[serde(rename_all = "camelCase")]
    Text { text: String },

    /// Tool call started
    #[serde(rename_all = "camelCase")]
    ToolCallStart {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        args_preview: Option<String>,
    },

    /// Tool call completed
    #[serde(rename_all = "camelCase")]
    ToolCallEnd { name: String, result_preview: Option<String> },

    /// Generated schema extracted
    #[serde(rename_all = "camelCase")]
    Schema { schema: GeneratedSchema },

    /// Stream completed
    #[serde(rename_all = "camelCase")]
    Done { message_id: String },

    /// Error occurred
    #[serde(rename_all = "camelCase")]
    Error { error: String },
}
