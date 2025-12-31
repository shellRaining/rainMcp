import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { SupportedAgent, McpConfig } from '@/types/mcp';
import {
  getSupportedAgents,
  updateEnabledAgents,
  updateAgentMcpConfig,
  openConfigFile,
} from '@/api/tauri';

export const useAgentsStore = defineStore('agents', () => {
  const agents = ref<SupportedAgent[]>([]);
  const selectedAgentName = ref<string | null>(null);
  const isLoading = ref(false);
  const error = ref<string | null>(null);

  const selectedAgent = computed(() => {
    if (!selectedAgentName.value) return null;
    return agents.value.find((a) => a.name === selectedAgentName.value) ?? null;
  });

  const enabledAgents = computed(() => agents.value.filter((a) => a.enabled));

  const configuredAgents = computed(() => agents.value.filter((a) => a.is_configured));

  async function fetchAgents() {
    isLoading.value = true;
    error.value = null;
    try {
      agents.value = await getSupportedAgents();
      // 只从 enabled agents 中选择，没有则不选中
      if (!selectedAgentName.value) {
        const firstEnabled = agents.value.find((a) => a.enabled);
        selectedAgentName.value = firstEnabled?.name ?? null;
      }
      // 如果当前选中的 agent 被禁用了，清除选择
      if (selectedAgentName.value) {
        const selected = agents.value.find((a) => a.name === selectedAgentName.value);
        if (selected && !selected.enabled) {
          selectedAgentName.value = null;
        }
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
    } finally {
      isLoading.value = false;
    }
  }

  function selectAgent(name: string) {
    selectedAgentName.value = name;
  }

  async function toggleAgentEnabled(name: string) {
    const agent = agents.value.find((a) => a.name === name);
    if (!agent) return;

    const newEnabled = !agent.enabled;
    const newEnabledList = agents.value
      .filter((a) => (a.name === name ? newEnabled : a.enabled))
      .map((a) => a.name);

    try {
      await updateEnabledAgents(newEnabledList);
      agent.enabled = newEnabled;
      // 如果禁用了当前选中的 agent，清除选择
      if (!newEnabled && selectedAgentName.value === name) {
        selectedAgentName.value = null;
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
    }
  }

  async function updateMcpConfig(agentName: string, config: McpConfig) {
    try {
      await updateAgentMcpConfig(agentName, config);
      const agent = agents.value.find((a) => a.name === agentName);
      if (agent) {
        agent.mcp_config = config;
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      throw e;
    }
  }

  async function openAgentConfigFile(agentName: string) {
    try {
      await openConfigFile(agentName);
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
    }
  }

  return {
    agents,
    selectedAgentName,
    selectedAgent,
    enabledAgents,
    configuredAgents,
    isLoading,
    error,
    fetchAgents,
    selectAgent,
    toggleAgentEnabled,
    updateMcpConfig,
    openAgentConfigFile,
  };
});
