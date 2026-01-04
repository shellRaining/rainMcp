// Re-export generated types from Rust
export type {
  AgentType,
  AgentServers,
  AgentServerEntry,
  SupportedAgent,
  // User server types
  UserServer,
  ServerOrigin,
  OriginType,
  // Schema store types
  ServerSchema,
  SchemaStore,
  Package,
  EnvironmentVariable,
  Icon,
  LocalTransport,
  RemoteTransport,
} from './generated';

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
  Comate: 'Comate',
  VsCodeCopilot: 'VS Code Copilot',
  CopilotCli: 'Copilot CLI',
  Alma: 'Alma',
};

// App config types (frontend-only, not in Rust schema)
export interface ClientConfigItem {
  enabled: boolean;
  custom_config_path?: string;
}

export interface AppConfig {
  clients: Record<string, ClientConfigItem>;
}
