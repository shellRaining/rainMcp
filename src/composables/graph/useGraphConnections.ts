import { ref } from 'vue';
import { useAgentsStore } from '@/stores/agents';
import { useServersStore } from '@/stores/servers';
import type { GraphNode, GraphEdge } from './useGraphData';

export function useGraphConnections() {
  const agentsStore = useAgentsStore();
  const serversStore = useServersStore();

  const newEdgeAnimation = ref<{
    sourceId: string;
    targetId: string;
    sourcePos: { x: number; y: number };
  } | null>(null);

  async function createConnection(
    sourceNode: GraphNode,
    targetNode: GraphNode,
    sourceCurrentPos?: { x: number; y: number }
  ) {
    const serverId = sourceNode.name;
    const agentName = targetNode.name;

    const agent = agentsStore.agents.find((a) => a.name === agentName);
    const server = serversStore.userServers.find((s) => s.id === serverId);

    if (!server) return;
    if (agent?.mcp_config?.servers && server.name in agent.mcp_config.servers) return;

    newEdgeAnimation.value = {
      sourceId: sourceNode.id,
      targetId: targetNode.id,
      sourcePos: sourceCurrentPos || { x: sourceNode.x, y: sourceNode.y },
    };

    await serversStore.deployToAgent(agentName, serverId);
    await agentsStore.fetchAgents();
  }

  async function deleteConnection(edge: GraphEdge) {
    const agentName = edge.target.replace('agent-', '');
    const agent = agentsStore.agents.find((a) => a.name === agentName);
    if (!agent?.mcp_config?.servers) return;

    const { [edge.serverName]: _, ...remaining } = agent.mcp_config.servers;
    await agentsStore.updateMcpConfig(agentName, { servers: remaining });
    await agentsStore.fetchAgents();
  }

  async function deleteAllConnectionsForAgent(agentName: string) {
    const agent = agentsStore.agents.find((a) => a.name === agentName);
    if (!agent?.mcp_config?.servers) return;

    await agentsStore.updateMcpConfig(agentName, { servers: {} });
    await agentsStore.fetchAgents();
  }

  return {
    newEdgeAnimation,
    createConnection,
    deleteConnection,
    deleteAllConnectionsForAgent,
  };
}
