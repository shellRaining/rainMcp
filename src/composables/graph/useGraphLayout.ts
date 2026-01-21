import type { GraphNode } from './useGraphData';

export function useGraphLayout() {
  function calculateLayout(nodes: GraphNode[], width: number, height: number) {
    const padding = 80;
    const serverX = padding + 60;
    const agentX = width - padding - 60;

    const serverNodes = nodes.filter((n) => n.nodeType === 'server');
    const agentNodes = nodes.filter((n) => n.nodeType === 'agent');

    serverNodes.forEach((node, i) => {
      const spacing = Math.max((height - padding * 2) / Math.max(serverNodes.length, 1), 60);
      node.x = serverX;
      node.y = padding + spacing / 2 + spacing * i;
    });

    agentNodes.forEach((node, i) => {
      const spacing = Math.max((height - padding * 2) / Math.max(agentNodes.length, 1), 50);
      node.x = agentX;
      node.y = padding + spacing / 2 + spacing * i;
    });

    return { serverNodes, agentNodes };
  }

  return { calculateLayout };
}
