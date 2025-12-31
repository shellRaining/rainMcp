// Re-export generated types from Rust
export type { AgentType, McpConfig, McpServerConfig, SupportedAgent } from './generated';

import type { AgentType } from './generated';

// Frontend-specific constants
export const AGENT_DISPLAY_NAMES: Record<AgentType, string> = {
  ClaudeCode: 'Claude Code',
  Cursor: 'Cursor',
  Windsurf: 'Windsurf',
  Cline: 'Cline',
  ClaudeDesktop: 'Claude Desktop',
  RooCode: 'Roo Code',
  Trae: 'Trae',
  GeminiCli: 'Gemini CLI',
  Kiro: 'Kiro',
  OpenAiCodex: 'OpenAI Codex',
};

// App config types (frontend-only, not in Rust schema)
export interface ClientConfigItem {
  enabled: boolean;
  custom_config_path?: string;
}

export interface AppConfig {
  clients: Record<string, ClientConfigItem>;
}
