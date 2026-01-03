<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch, computed } from 'vue';
import * as d3 from 'd3';
import { useAgentsStore } from '@/stores/agents';
import { useServersStore } from '@/stores/servers';

interface GraphNode {
  id: string;
  nodeType: 'server' | 'agent';
  name: string;
  displayName: string;
  x: number;
  y: number;
}

interface GraphEdge {
  source: string;
  target: string;
  serverName: string;
  serverId: string;
}

const agentsStore = useAgentsStore();
const serversStore = useServersStore();

const containerRef = ref<HTMLDivElement>();
const focusedNodeId = ref<string | null>(null);

let svg: d3.Selection<SVGSVGElement, unknown, null, undefined> | null = null;

// Drag state for creating connections
const dragState = ref<{
  active: boolean;
  sourceNode: GraphNode | null;
  currentX: number;
  currentY: number;
}>({
  active: false,
  sourceNode: null,
  currentX: 0,
  currentY: 0,
});

// Color palette
const colors = {
  server: '#3b82f6', // blue-500
  serverText: '#ffffff',
  agent: '#8b5cf6', // violet-500
  agentText: '#ffffff',
  edge: '#6b7280', // gray-500
  edgeHover: '#ef4444', // red-500
  dragLine: '#22c55e', // green-500
  highlight: '#fbbf24', // amber-400
  border: '#374151', // gray-700
};

// Build graph data from stores
const graphData = computed(() => {
  const nodes: GraphNode[] = [];
  const edges: GraphEdge[] = [];

  // Build nodes
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

  // Build edges from agent mcp_config
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

// Check if a node is connected to the focused node
function isConnectedToFocused(nodeId: string, edges: GraphEdge[]): boolean {
  if (!focusedNodeId.value) return true;
  if (nodeId === focusedNodeId.value) return true;

  return edges.some(
    (e) =>
      (e.source === focusedNodeId.value && e.target === nodeId) ||
      (e.target === focusedNodeId.value && e.source === nodeId)
  );
}

// Check if an edge is connected to the focused node
function isEdgeConnectedToFocused(edge: GraphEdge): boolean {
  if (!focusedNodeId.value) return true;
  return edge.source === focusedNodeId.value || edge.target === focusedNodeId.value;
}

function renderGraph() {
  if (!containerRef.value) return;

  const container = containerRef.value;
  const rect = container.getBoundingClientRect();
  const width = rect.width || 600;
  const height = rect.height || 400;

  // Clear existing
  d3.select(container).selectAll('svg').remove();

  const { nodes, edges } = graphData.value;
  if (nodes.length === 0) return;

  // Calculate positions
  const padding = 80;
  const serverX = padding + 60;
  const agentX = width - padding - 60;

  const serverNodes = nodes.filter((n) => n.nodeType === 'server');
  const agentNodes = nodes.filter((n) => n.nodeType === 'agent');

  // Position nodes
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

  // Create SVG
  svg = d3
    .select(container)
    .append('svg')
    .attr('width', width)
    .attr('height', height)
    .style('display', 'block');

  // Click on canvas to clear focus
  svg.on('click', () => {
    focusedNodeId.value = null;
  });

  const g = svg.append('g').attr('class', 'graph-content');

  // Draw edges
  g.selectAll<SVGLineElement, GraphEdge>('line.edge')
    .data(edges)
    .join('line')
    .attr('class', 'edge')
    .attr('x1', (d) => nodes.find((n) => n.id === d.source)?.x ?? 0)
    .attr('y1', (d) => nodes.find((n) => n.id === d.source)?.y ?? 0)
    .attr('x2', (d) => nodes.find((n) => n.id === d.target)?.x ?? 0)
    .attr('y2', (d) => nodes.find((n) => n.id === d.target)?.y ?? 0)
    .attr('stroke', colors.edge)
    .attr('stroke-width', 2)
    .attr('stroke-opacity', (d) => (isEdgeConnectedToFocused(d) ? 0.7 : 0.15))
    .style('cursor', 'pointer')
    .on('mouseenter', function () {
      d3.select(this).attr('stroke', colors.edgeHover).attr('stroke-opacity', 1);
    })
    .on('mouseleave', function (_, d) {
      d3.select(this)
        .attr('stroke', colors.edge)
        .attr('stroke-opacity', isEdgeConnectedToFocused(d) ? 0.7 : 0.15);
    })
    .on('click', async (event, d) => {
      event.stopPropagation();
      await handleEdgeDelete(d);
    });

  // Drag line (initially hidden)
  const dragLine = g
    .append('line')
    .attr('class', 'drag-line')
    .attr('stroke', colors.dragLine)
    .attr('stroke-width', 3)
    .attr('stroke-dasharray', '8,4')
    .attr('opacity', 0);

  // Draw server nodes (circles with text inside)
  const serverNodeGroups = g
    .selectAll<SVGGElement, GraphNode>('g.server-node')
    .data(serverNodes)
    .join('g')
    .attr('class', 'server-node')
    .attr('transform', (d) => `translate(${d.x},${d.y})`)
    .style('cursor', 'grab')
    .on('click', (event, d) => {
      event.stopPropagation();
      handleNodeClick(d.id);
    });

  // Server circle
  serverNodeGroups
    .append('circle')
    .attr('r', 40)
    .attr('fill', colors.server)
    .attr('stroke', (d) => (d.id === focusedNodeId.value ? colors.highlight : colors.border))
    .attr('stroke-width', (d) => (d.id === focusedNodeId.value ? 3 : 2))
    .attr('opacity', (d) => (isConnectedToFocused(d.id, edges) ? 1 : 0.25));

  // Server text (inside)
  serverNodeGroups
    .append('text')
    .text((d) => truncateText(d.displayName, 8))
    .attr('text-anchor', 'middle')
    .attr('dy', '0.35em')
    .attr('font-size', '11px')
    .attr('font-weight', '500')
    .attr('fill', colors.serverText)
    .attr('pointer-events', 'none');

  // Add drag behavior for servers (to create connections)
  serverNodeGroups.call(
    d3
      .drag<SVGGElement, GraphNode>()
      .on('start', function (_event, d) {
        dragState.value = {
          active: true,
          sourceNode: d,
          currentX: d.x,
          currentY: d.y,
        };
        dragLine.attr('x1', d.x).attr('y1', d.y).attr('x2', d.x).attr('y2', d.y).attr('opacity', 1);
        d3.select(this).style('cursor', 'grabbing');
      })
      .on('drag', function (event) {
        dragState.value.currentX = event.x;
        dragState.value.currentY = event.y;
        dragLine.attr('x2', event.x).attr('y2', event.y);

        // Highlight agent nodes near cursor
        agentNodeGroups.select('rect').attr('stroke', (d) => {
          const dist = Math.sqrt(Math.pow(d.x - event.x, 2) + Math.pow(d.y - event.y, 2));
          return dist < 50 ? colors.dragLine : d.id === focusedNodeId.value ? colors.highlight : colors.border;
        });
      })
      .on('end', async function (event, sourceNode) {
        dragLine.attr('opacity', 0);
        d3.select(this).style('cursor', 'grab');

        // Find target agent
        const targetAgent = agentNodes.find((n) => {
          const dist = Math.sqrt(Math.pow(n.x - event.x, 2) + Math.pow(n.y - event.y, 2));
          return dist < 50;
        });

        if (targetAgent && sourceNode) {
          await handleCreateConnection(sourceNode, targetAgent);
        }

        // Reset highlights
        agentNodeGroups.select('rect').attr('stroke', (d) =>
          d.id === focusedNodeId.value ? colors.highlight : colors.border
        );

        dragState.value = { active: false, sourceNode: null, currentX: 0, currentY: 0 };
      }) as unknown as (selection: d3.Selection<SVGGElement, GraphNode, SVGGElement, unknown>) => void
  );

  // Draw agent nodes (rectangles with text inside)
  const agentNodeGroups = g
    .selectAll<SVGGElement, GraphNode>('g.agent-node')
    .data(agentNodes)
    .join('g')
    .attr('class', 'agent-node')
    .attr('transform', (d) => `translate(${d.x},${d.y})`)
    .style('cursor', 'pointer')
    .on('click', (event, d) => {
      event.stopPropagation();
      handleNodeClick(d.id);
    });

  // Agent rectangle
  agentNodeGroups
    .append('rect')
    .attr('x', -50)
    .attr('y', -18)
    .attr('width', 100)
    .attr('height', 36)
    .attr('rx', 8)
    .attr('fill', colors.agent)
    .attr('stroke', (d) => (d.id === focusedNodeId.value ? colors.highlight : colors.border))
    .attr('stroke-width', (d) => (d.id === focusedNodeId.value ? 3 : 2))
    .attr('opacity', (d) => (isConnectedToFocused(d.id, edges) ? 1 : 0.25));

  // Agent text (inside)
  agentNodeGroups
    .append('text')
    .text((d) => truncateText(d.displayName, 10))
    .attr('text-anchor', 'middle')
    .attr('dy', '0.35em')
    .attr('font-size', '12px')
    .attr('font-weight', '500')
    .attr('fill', colors.agentText)
    .attr('pointer-events', 'none');
}

function truncateText(text: string, maxLen: number): string {
  return text.length > maxLen ? text.slice(0, maxLen - 1) + '…' : text;
}

function handleNodeClick(nodeId: string) {
  if (focusedNodeId.value === nodeId) {
    focusedNodeId.value = null;
  } else {
    focusedNodeId.value = nodeId;
  }
}

async function handleCreateConnection(sourceNode: GraphNode, targetNode: GraphNode) {
  const serverId = sourceNode.name;
  const agentName = targetNode.name;

  // Check if connection already exists
  const agent = agentsStore.agents.find((a) => a.name === agentName);
  const server = serversStore.userServers.find((s) => s.id === serverId);

  if (!server) return;

  // Check if already connected
  if (agent?.mcp_config?.servers && server.name in agent.mcp_config.servers) {
    return; // Already connected
  }

  // Deploy server to agent
  await serversStore.deployToAgent(agentName, serverId);
  await agentsStore.fetchAgents();
}

async function handleEdgeDelete(edge: GraphEdge) {
  const agentName = edge.target.replace('agent-', '');

  // Find the agent and remove the server from its config
  const agent = agentsStore.agents.find((a) => a.name === agentName);
  if (!agent?.mcp_config?.servers) return;

  const { [edge.serverName]: _, ...remaining } = agent.mcp_config.servers;
  await agentsStore.updateMcpConfig(agentName, { servers: remaining });
  await agentsStore.fetchAgents();
}

// Handle resize with debounce
let resizeTimeout: ReturnType<typeof setTimeout> | null = null;
function handleResize() {
  if (resizeTimeout) clearTimeout(resizeTimeout);
  resizeTimeout = setTimeout(() => {
    renderGraph();
  }, 100);
}

onMounted(() => {
  setTimeout(() => {
    renderGraph();
  }, 50);
  window.addEventListener('resize', handleResize);
});

onUnmounted(() => {
  window.removeEventListener('resize', handleResize);
  if (resizeTimeout) {
    clearTimeout(resizeTimeout);
  }
});

// Watch for data changes
watch(
  () => [agentsStore.enabledAgents, serversStore.userServers],
  () => {
    renderGraph();
  },
  { deep: true }
);

// Watch for focus changes
watch(focusedNodeId, () => {
  renderGraph();
});
</script>

<template>
  <div ref="containerRef" class="w-full h-full overflow-hidden">
    <!-- Empty state -->
    <div
      v-if="graphData.nodes.length === 0"
      class="h-full flex flex-col items-center justify-center text-muted-foreground"
    >
      <p class="text-sm">No servers or agents to display</p>
      <p class="text-xs mt-1">Enable some agents and add servers to see connections</p>
    </div>
  </div>
</template>

<style scoped>
:deep(.edge) {
  transition:
    stroke 0.15s ease,
    stroke-opacity 0.15s ease;
}

:deep(.server-node:hover circle),
:deep(.agent-node:hover rect) {
  filter: brightness(1.1);
}

:deep(.drag-line) {
  pointer-events: none;
}
</style>
