import { computed } from 'vue';
import { useAgentsStore } from '@/stores/agents';
import { useServersStore } from '@/stores/servers';

export interface GraphNode {
  id: string;
  nodeType: 'server' | 'agent';
  name: string;
  displayName: string;
  x: number;
  y: number;
}

export interface GraphEdge {
  source: string;
  target: string;
  serverName: string;
  serverId: string;
}

export function useGraphData() {
  const agentsStore = useAgentsStore();
  const serversStore = useServersStore();

  const graphData = computed(() => {
    const nodes: GraphNode[] = [];
    const edges: GraphEdge[] = [];

    const serverNodes = serversStore.userServers.map((s) => ({
      id: `server-${s.id}`,
      nodeType: 'server' as const,
      name: s.id,
      displayName: s.name,
      x: 0,
      y: 0,
    }));

    const agentNodes = agentsStore.enabledAgents.map((a) => ({
      id: `agent-${a.name}`,
      nodeType: 'agent' as const,
      name: a.name,
      displayName: a.name,
      x: 0,
      y: 0,
    }));

    nodes.push(...serverNodes, ...agentNodes);

    for (const agent of agentsStore.enabledAgents) {
      if (agent.mcp_config?.servers) {
        for (const serverName of Object.keys(agent.mcp_config.servers)) {
          const userServer = serversStore.userServers.find((s) => s.name === serverName);
          if (userServer) {
            edges.push({
              source: `server-${userServer.id}`,
              target: `agent-${agent.name}`,
              serverName,
              serverId: userServer.id,
            });
          }
        }
      }
    }

    return { nodes, edges };
  });

  return { graphData };
}
