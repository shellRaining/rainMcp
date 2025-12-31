import { invoke } from '@tauri-apps/api/core';
import type { AgentType, AppConfig, McpConfig, SupportedAgent } from '@/types/mcp';

export async function getSupportedAgents(): Promise<SupportedAgent[]> {
  return invoke('get_supported_agents_command');
}

export async function getEnabledAgents(): Promise<AgentType[]> {
  return invoke('get_enabled_agents_command');
}

export async function updateEnabledAgents(enabledAgents: string[]): Promise<void> {
  return invoke('update_enabled_agents_command', { enabledAgents });
}

export async function getAgentMcpConfig(agentName: string): Promise<McpConfig> {
  return invoke('get_agent_mcp_config_command', { agentName });
}

export async function updateAgentMcpConfig(agentName: string, config: McpConfig): Promise<void> {
  return invoke('update_agent_mcp_config_command', { agentName, config });
}

export async function getAppConfig(): Promise<AppConfig> {
  return invoke('get_app_config_command');
}

export async function updateAppConfig(config: AppConfig): Promise<void> {
  return invoke('update_app_config_command', { config });
}

export async function openConfigFile(agentName: string): Promise<void> {
  return invoke('open_config_file_command', { agentName });
}

export async function getServerRawConfig(agentName: string, serverName: string): Promise<string> {
  return invoke('get_server_raw_config_command', { agentName, serverName });
}
