import { openConfigFile } from '@/api/tauri';
import { useAgentsStore } from '@/stores/agents';
import { useAppStore } from '@/stores/app';
import { useGraphConnections } from './useGraphConnections';

export function useAgentContextMenu() {
  const agentsStore = useAgentsStore();
  const appStore = useAppStore();
  const { deleteAllConnectionsForAgent } = useGraphConnections();

  async function toggleAgentEnabled(agentName: string) {
    await agentsStore.toggleAgentEnabled(agentName);
  }

  function viewAgentDetail(agentName: string) {
    appStore.clickDetailItem('agents', agentName);
  }

  async function deleteAllConnections(agentName: string) {
    await deleteAllConnectionsForAgent(agentName);
  }

  async function openAgentConfig(agentName: string) {
    await openConfigFile(agentName);
  }

  return {
    toggleAgentEnabled,
    viewAgentDetail,
    deleteAllConnections,
    openAgentConfig,
  };
}
