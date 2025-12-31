// This file is auto-generated from Rust types via JSON Schema.
// Do not edit manually. Run "bun run generate" to regenerate.

export type AgentType =
  | 'ClaudeCode'
  | 'Cursor'
  | 'Windsurf'
  | 'Cline'
  | 'ClaudeDesktop'
  | 'RooCode'
  | 'Trae'
  | 'GeminiCli'
  | 'Kiro'
  | 'OpenAiCodex';
export type McpServerConfig =
  | {
      args?: string[] | null;
      command: string;
      env?: {
        [k: string]: string;
      } | null;
      timeout?: number | null;
      type: 'local';
    }
  | {
      headers?: {
        [k: string]: string;
      } | null;
      timeout?: number | null;
      type: 'remote';
      url: string;
    };

/**
 * 包含所有需要导出的类型
 */
export interface McpConfig {
  servers: {
    [k: string]: McpServerConfig;
  };
}
export interface SupportedAgent {
  agent_type: AgentType;
  config_path: string;
  enabled: boolean;
  is_configured: boolean;
  mcp_config?: McpConfig | null;
  name: string;
}
