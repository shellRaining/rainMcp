import { invoke } from '@tauri-apps/api/core';
import type {
  AgentType,
  AppConfig,
  AgentServers,
  SchemaStore,
  SupportedAgent,
  UserServer,
} from '@/types/mcp';

// ===== Agent APIs =====

export async function getSupportedAgents(): Promise<SupportedAgent[]> {
  return invoke('get_supported_agents_command');
}

export async function getEnabledAgents(): Promise<AgentType[]> {
  return invoke('get_enabled_agents_command');
}

export async function updateEnabledAgents(enabledAgents: string[]): Promise<void> {
  return invoke('update_enabled_agents_command', { enabledAgents });
}

export async function getAgentMcpConfig(agentName: string): Promise<AgentServers> {
  return invoke('get_agent_mcp_config_command', { agentName });
}

export async function updateAgentMcpConfig(agentName: string, config: AgentServers): Promise<void> {
  return invoke('update_agent_mcp_config_command', { agentName, config });
}

export async function openConfigFile(agentName: string): Promise<void> {
  return invoke('open_config_file_command', { agentName });
}

export async function getServerRawConfig(agentName: string, serverName: string): Promise<string> {
  return invoke('get_server_raw_config_command', { agentName, serverName });
}

// ===== App Config APIs =====

export async function getAppConfig(): Promise<AppConfig> {
  return invoke('get_app_config_command');
}

export async function updateAppConfig(config: AppConfig): Promise<void> {
  return invoke('update_app_config_command', { config });
}

// ===== Schema Store APIs =====

export async function getSchemaStore(): Promise<SchemaStore> {
  return invoke('get_schema_store_command');
}

export async function refreshSchemaStore(): Promise<SchemaStore> {
  return invoke('refresh_schema_store_command');
}

// ===== User Server APIs =====

export async function getUserServers(): Promise<UserServer[]> {
  return invoke('get_user_servers_command');
}

export async function addUserServer(server: UserServer): Promise<UserServer> {
  return invoke('add_user_server_command', { server });
}

export async function updateUserServer(server: UserServer): Promise<UserServer> {
  return invoke('update_user_server_command', { server });
}

export async function deleteUserServer(serverId: string): Promise<void> {
  return invoke('delete_user_server_command', { serverId });
}

export async function addServerToAgent(
  agentName: string,
  serverId: string,
  serverName?: string
): Promise<void> {
  return invoke('add_server_to_agent_command', { agentName, serverId, serverName });
}

// ===== Window APIs =====

export async function setTrafficLightsInset(
  windowLabel: string,
  x: number,
  y: number
): Promise<void> {
  return invoke('set_traffic_lights_inset_command', { windowLabel, x, y });
}

// ===== AI Agent APIs =====

export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: string;
  toolCalls?: ToolCallInfo[];
  generatedSchema?: GeneratedSchema;
}

export interface ToolCallInfo {
  name: string;
  status: 'pending' | 'running' | 'success' | 'failed';
  resultPreview?: string;
}

export interface GeneratedSchema {
  schema: import('@/types/mcp').ServerSchema;
  confidence: number;
  explanation: string;
}

// Stream event types
export type StreamEvent =
  | { type: 'text'; text: string }
  | { type: 'toolCallStart'; name: string; argsPreview?: string }
  | { type: 'toolCallEnd'; name: string; resultPreview?: string }
  | { type: 'schema'; schema: GeneratedSchema }
  | { type: 'done'; messageId: string }
  | { type: 'error'; error: string };

export async function agentChat(message: string): Promise<ChatMessage> {
  return invoke('agent_chat_command', { message });
}

export async function agentChatStream(message: string): Promise<void> {
  return invoke('agent_chat_stream_command', { message });
}

export async function agentReset(): Promise<void> {
  return invoke('agent_reset_command');
}

export async function getOpenRouterApiKey(): Promise<string | null> {
  return invoke('get_openrouter_api_key_command');
}

export async function setOpenRouterApiKey(apiKey: string): Promise<void> {
  return invoke('set_openrouter_api_key_command', { apiKey });
}
