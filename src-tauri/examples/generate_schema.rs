//! 生成 JSON Schema 文件，用于前端类型生成
//!
//! 运行: cargo run --example generate_schema

use rain_mcp_lib::mcp::{AgentType, McpConfig, McpServerConfig, SupportedAgent};
use schemars::{schema_for, JsonSchema};
use serde::Serialize;
use std::fs;
use std::path::Path;

/// 包含所有需要导出的类型
#[derive(JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
struct AllTypes {
    agent_type: AgentType,
    mcp_server_config: McpServerConfig,
    mcp_config: McpConfig,
    supported_agent: SupportedAgent,
}

fn main() {
    let schema_dir = Path::new("../src/types/schemas");
    fs::create_dir_all(schema_dir).expect("Failed to create schema directory");

    // 生成包含所有类型的 schema
    let schema = schema_for!(AllTypes);
    let json = serde_json::to_string_pretty(&schema).expect("Failed to serialize schema");
    let path = schema_dir.join("all.json");
    fs::write(&path, &json).expect("Failed to write schema file");
    println!("Generated: {}", path.display());

    println!("\nSchema generated successfully!");
    println!("Run 'bun run generate:types' to generate TypeScript types.");
}
