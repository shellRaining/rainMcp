<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch, computed } from 'vue';
import * as d3 from 'd3';
import { useGraphData } from '@/composables/graph/useGraphData';
import { useGraphLayout } from '@/composables/graph/useGraphLayout';
import { useGraphFocus } from '@/composables/graph/useGraphFocus';
import { useGraphConnections } from '@/composables/graph/useGraphConnections';
import { useAgentContextMenu } from '@/composables/graph/useAgentContextMenu';
import { useAgentsStore } from '@/stores/agents';
import { useServersStore } from '@/stores/servers';
import { useThemeStore } from '@/stores/theme';
import type { GraphNode, GraphEdge } from '@/composables/graph/useGraphData';

const agentsStore = useAgentsStore();
const serversStore = useServersStore();
const themeStore = useThemeStore();

const { graphData } = useGraphData();
const { calculateLayout } = useGraphLayout();
const { focusedNodeId, isConnectedToFocused, isEdgeConnectedToFocused, toggleFocus, clearFocus } =
  useGraphFocus();
const { newEdgeAnimation, createConnection, deleteConnection } = useGraphConnections();
const { toggleAgentEnabled, viewAgentDetail, deleteAllConnections, openAgentConfig } =
  useAgentContextMenu();

const containerRef = ref<HTMLDivElement>();
const animatingNodeId = ref<string | null>(null);
const animatingNodeStartPos = ref<{ x: number; y: number } | null>(null);

let svg: d3.Selection<SVGSVGElement, unknown, null, undefined> | null = null;

// Context menu state
const contextMenuOpen = ref(false);
const contextMenuAgent = ref<GraphNode | null>(null);
const contextMenuPosition = ref({ x: 0, y: 0 });

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
const colors = computed(() => {
  const isDark = themeStore.theme === 'dark' || 
    (themeStore.theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);
  
  return {
    server: '#3b82f6',
    serverText: '#ffffff',
    agent: '#8b5cf6',
    agentText: '#ffffff',
    edge: isDark ? '#6b7280' : '#9ca3af',
    edgeHover: '#ef4444',
    dragLine: '#22c55e',
    highlight: '#fbbf24',
    border: isDark ? '#374151' : '#d1d5db',
  };
});

function truncateText(text: string, maxLen: number): string {
  return text.length > maxLen ? text.slice(0, maxLen - 1) + '…' : text;
}

function renderGraph() {
  if (!containerRef.value) return;

  const container = containerRef.value;
  const rect = container.getBoundingClientRect();
  const width = rect.width || 600;
  const height = rect.height || 400;

  d3.select(container).selectAll('svg').remove();

  const { nodes, edges } = graphData.value;
  if (nodes.length === 0) return;

  const { serverNodes, agentNodes } = calculateLayout(nodes, width, height);

  // Create SVG
  svg = d3
    .select(container)
    .append('svg')
    .attr('width', width)
    .attr('height', height)
    .style('display', 'block')
    .on('click', () => clearFocus());

  const g = svg.append('g').attr('class', 'graph-content');

  // Draw edges
  const edgeLines = g
    .selectAll<SVGLineElement, GraphEdge>('line.edge')
    .data(edges)
    .join('line')
    .attr('class', 'edge')
    .attr('x1', (d) => {
      // For new edge, start from the dragged position
      if (
        newEdgeAnimation.value &&
        d.source === newEdgeAnimation.value.sourceId &&
        d.target === newEdgeAnimation.value.targetId
      ) {
        return newEdgeAnimation.value.sourcePos.x;
      }
      return nodes.find((n) => n.id === d.source)?.x ?? 0;
    })
    .attr('y1', (d) => {
      if (
        newEdgeAnimation.value &&
        d.source === newEdgeAnimation.value.sourceId &&
        d.target === newEdgeAnimation.value.targetId
      ) {
        return newEdgeAnimation.value.sourcePos.y;
      }
      return nodes.find((n) => n.id === d.source)?.y ?? 0;
    })
    .attr('x2', (d) => {
      // For new edge, start x2 from the dragged position too
      if (
        newEdgeAnimation.value &&
        d.source === newEdgeAnimation.value.sourceId &&
        d.target === newEdgeAnimation.value.targetId
      ) {
        return newEdgeAnimation.value.sourcePos.x;
      }
      return nodes.find((n) => n.id === d.target)?.x ?? 0;
    })
    .attr('y2', (d) => {
      if (
        newEdgeAnimation.value &&
        d.source === newEdgeAnimation.value.sourceId &&
        d.target === newEdgeAnimation.value.targetId
      ) {
        return newEdgeAnimation.value.sourcePos.y;
      }
      return nodes.find((n) => n.id === d.target)?.y ?? 0;
    })
    .attr('stroke', (d) => (isEdgeConnectedToFocused(d) ? colors.value.highlight : colors.value.edge))
    .attr('stroke-width', (d) => (isEdgeConnectedToFocused(d) ? 3 : 2))
    .attr('stroke-opacity', (d) =>
      focusedNodeId.value ? (isEdgeConnectedToFocused(d) ? 0.9 : 0.15) : 0.7
    )
    .style('cursor', 'pointer')
    .on('mouseenter', function () {
      if (dragState.value.active) return;
      d3.select(this)
        .transition()
        .duration(200)
        .attr('stroke', colors.value.edgeHover)
        .attr('stroke-opacity', 1);
    })
    .on('mouseleave', function (_, d) {
      d3.select(this)
        .transition()
        .duration(200)
        .attr('stroke', isEdgeConnectedToFocused(d) ? colors.value.highlight : colors.value.edge)
        .attr('stroke-width', isEdgeConnectedToFocused(d) ? 3 : 2)
        .attr(
          'stroke-opacity',
          focusedNodeId.value ? (isEdgeConnectedToFocused(d) ? 0.9 : 0.15) : 0.7
        );
    })
    .on('click', async (event, d) => {
      event.stopPropagation();
      await deleteConnection(d);
    });

  // Animate newly created edge: expand from source point to full line
  if (newEdgeAnimation.value) {
    const animData = newEdgeAnimation.value;
    const sourceNode = nodes.find((n) => n.id === animData.sourceId);
    const targetNode = nodes.find((n) => n.id === animData.targetId);
    if (sourceNode && targetNode) {
      const newEdge = edgeLines.filter(
        (d) => d.source === animData.sourceId && d.target === animData.targetId
      );
      // 先设置高亮颜色
      newEdge.attr('stroke', colors.value.dragLine).attr('stroke-opacity', 1);
      // 展开动画
      newEdge
        .transition()
        .duration(300)
        .ease(d3.easeQuadOut)
        .attr('x1', sourceNode.x)
        .attr('y1', sourceNode.y)
        .attr('x2', targetNode.x)
        .attr('y2', targetNode.y)
        .transition()
        .duration(500)
        .attr('stroke', colors.value.edge)
        .attr('stroke-opacity', 0.7)
        .attr('stroke-width', 2)
        .on('end', () => {
          newEdgeAnimation.value = null;
        });
    }
  }

  // Drag line (initially hidden)
  const dragLine = g
    .append('line')
    .attr('class', 'drag-line')
    .attr('stroke', colors.value.dragLine)
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
      toggleFocus(d.id);
    });

  // Server circle
  serverNodeGroups
    .append('circle')
    .attr('r', 40)
    .attr('fill', colors.value.server)
    .attr('stroke', (d) => (d.id === focusedNodeId.value ? colors.value.highlight : colors.value.border))
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
    .attr('fill', colors.value.serverText)
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
      .on('drag', function (event, d) {
        dragState.value.currentX = event.x;
        dragState.value.currentY = event.y;

        // 计算拖拽距离
        const dragDistX = event.x - d.x;
        const dragDistY = event.y - d.y;
        const dragDist = Math.sqrt(dragDistX * dragDistX + dragDistY * dragDistY);

        // 弹簧效果：只有当拖拽距离超过阈值时，小球才开始移动
        const threshold = 30;
        const damping = 0.3;
        const maxRadius = 50; // 小球最大移动半径

        let newX = d.x;
        let newY = d.y;

        if (dragDist > threshold) {
          const pullDist = dragDist - threshold;
          const moveRatio = (pullDist * damping) / dragDist;
          const offsetX = dragDistX * moveRatio;
          const offsetY = dragDistY * moveRatio;

          // 使用渐近函数实现"逼近"效果，而不是硬性限制
          const offsetDist = Math.sqrt(offsetX * offsetX + offsetY * offsetY);
          const asymptoteK = 30; // 控制渐近速度
          const moveDistance = (offsetDist * maxRadius) / (offsetDist + asymptoteK);
          const scale = moveDistance / offsetDist;
          newX = d.x + offsetX * scale;
          newY = d.y + offsetY * scale;
        }

        // 保存当前位置，用于回弹动画
        animatingNodeStartPos.value = { x: newX, y: newY };

        // 更新节点位置
        d3.select(this).attr('transform', `translate(${newX},${newY})`);

        // 更新拖拽线
        dragLine.attr('x1', newX).attr('y1', newY).attr('x2', event.x).attr('y2', event.y);

        // 更新与该节点相关的所有连接线
        g.selectAll<SVGLineElement, GraphEdge>('line.edge')
          .filter((e) => e.source === d.id)
          .attr('x1', newX)
          .attr('y1', newY);

        // Highlight agent nodes near cursor
        agentNodeGroups.select('rect').attr('stroke', (d) => {
          const dist = Math.sqrt(Math.pow(d.x - event.x, 2) + Math.pow(d.y - event.y, 2));
          return dist < 50
            ? colors.value.dragLine
            : d.id === focusedNodeId.value
              ? colors.value.highlight
              : colors.value.border;
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

        // 立即创建连接，让新连线出现
        if (targetAgent && sourceNode) {
          // 标记正在动画的节点
          animatingNodeId.value = sourceNode.id;

          // Pass the current dragged position for edge animation
          const currentPos = animatingNodeStartPos.value || { x: sourceNode.x, y: sourceNode.y };
          await createConnection(sourceNode, targetAgent, currentPos);

          // 动画结束后清除标记
          setTimeout(() => {
            animatingNodeId.value = null;
            animatingNodeStartPos.value = null;
          }, 300);
        } else {
          // 没有连接目标，直接执行回弹动画
          d3.select(this)
            .transition()
            .duration(300)
            .ease(d3.easeBackOut)
            .attr('transform', `translate(${sourceNode.x},${sourceNode.y})`);

          g.selectAll<SVGLineElement, GraphEdge>('line.edge')
            .filter((e) => e.source === sourceNode.id)
            .transition()
            .duration(300)
            .ease(d3.easeBackOut)
            .attr('x1', sourceNode.x)
            .attr('y1', sourceNode.y);
        }

        // Reset highlights
        agentNodeGroups
          .select('rect')
          .attr('stroke', (d) => (d.id === focusedNodeId.value ? colors.value.highlight : colors.value.border));

        dragState.value = { active: false, sourceNode: null, currentX: 0, currentY: 0 };
      }) as unknown as (
      selection: d3.Selection<SVGGElement, GraphNode, SVGGElement, unknown>
    ) => void
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
      toggleFocus(d.id);
    })
    .on('contextmenu', (event, d) => {
      event.preventDefault();
      event.stopPropagation();
      contextMenuAgent.value = d;
      contextMenuPosition.value = { x: event.pageX, y: event.pageY };
      contextMenuOpen.value = true;
    });

  // Agent rectangle
  agentNodeGroups
    .append('rect')
    .attr('x', -50)
    .attr('y', -18)
    .attr('width', 100)
    .attr('height', 36)
    .attr('rx', 8)
    .attr('fill', colors.value.agent)
    .attr('stroke', (d) => (d.id === focusedNodeId.value ? colors.value.highlight : colors.value.border))
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
    .attr('fill', colors.value.agentText)
    .attr('pointer-events', 'none');

  // 如果有节点正在动画，重新应用回弹动画
  if (animatingNodeId.value && animatingNodeStartPos.value) {
    const animatingNode = nodes.find((n) => n.id === animatingNodeId.value);
    if (animatingNode) {
      // 先将节点设置到拖动后的位置
      serverNodeGroups
        .filter((d) => d.id === animatingNodeId.value)
        .attr(
          'transform',
          `translate(${animatingNodeStartPos.value.x},${animatingNodeStartPos.value.y})`
        )
        .transition()
        .duration(300)
        .ease(d3.easeBackOut)
        .attr('transform', `translate(${animatingNode.x},${animatingNode.y})`);

      // 对相关连接线应用回弹动画（排除新边，新边有自己的动画）
      g.selectAll<SVGLineElement, GraphEdge>('line.edge')
        .filter((e) => {
          if (e.source !== animatingNodeId.value) return false;
          // Skip the new edge as it has its own animation
          if (
            newEdgeAnimation.value &&
            e.source === newEdgeAnimation.value.sourceId &&
            e.target === newEdgeAnimation.value.targetId
          ) {
            return false;
          }
          return true;
        })
        .attr('x1', animatingNodeStartPos.value.x)
        .attr('y1', animatingNodeStartPos.value.y)
        .transition()
        .duration(300)
        .ease(d3.easeBackOut)
        .attr('x1', animatingNode.x)
        .attr('y1', animatingNode.y);
    }
  }
}

// Handle resize with requestAnimationFrame
let resizeRAF: number | null = null;
function handleResize() {
  if (resizeRAF) cancelAnimationFrame(resizeRAF);
  resizeRAF = requestAnimationFrame(() => {
    renderGraph();
    resizeRAF = null;
  });
}

function handleClickOutside() {
  if (contextMenuOpen.value) {
    contextMenuOpen.value = false;
  }
}

onMounted(() => {
  setTimeout(() => {
    renderGraph();
  }, 50);
  window.addEventListener('resize', handleResize);
  window.addEventListener('click', handleClickOutside);
});

onUnmounted(() => {
  window.removeEventListener('resize', handleResize);
  window.removeEventListener('click', handleClickOutside);
  if (resizeRAF) {
    cancelAnimationFrame(resizeRAF);
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

// Watch for theme changes
watch(
  () => themeStore.theme,
  () => {
    renderGraph();
  }
);

// Watch for focus changes
watch(focusedNodeId, () => {
  if (!svg) return;

  const { edges } = graphData.value;

  svg
    .selectAll<SVGLineElement, GraphEdge>('line.edge')
    .transition()
    .duration(300)
    .attr('stroke', (d) => (isEdgeConnectedToFocused(d) ? colors.value.highlight : colors.value.edge))
    .attr('stroke-width', (d) => (isEdgeConnectedToFocused(d) ? 3 : 2))
    .attr('stroke-opacity', (d) =>
      focusedNodeId.value ? (isEdgeConnectedToFocused(d) ? 0.9 : 0.15) : 0.7
    );

  svg
    .selectAll<SVGCircleElement, GraphNode>('.server-node circle')
    .transition()
    .duration(300)
    .attr('stroke', (d) => (d.id === focusedNodeId.value ? colors.value.highlight : colors.value.border))
    .attr('stroke-width', (d) => (d.id === focusedNodeId.value ? 3 : 2))
    .attr('opacity', (d) => (isConnectedToFocused(d.id, edges) ? 1 : 0.25));

  svg
    .selectAll<SVGRectElement, GraphNode>('.agent-node rect')
    .transition()
    .duration(300)
    .attr('stroke', (d) => (d.id === focusedNodeId.value ? colors.value.highlight : colors.value.border))
    .attr('stroke-width', (d) => (d.id === focusedNodeId.value ? 3 : 2))
    .attr('opacity', (d) => (isConnectedToFocused(d.id, edges) ? 1 : 0.25));
});
</script>

<template>
  <div ref="containerRef" class="w-full h-full overflow-hidden relative">
    <div
      v-if="graphData.nodes.length === 0"
      class="h-full flex flex-col items-center justify-center text-muted-foreground"
    >
      <p class="text-sm">No servers or agents to display</p>
      <p class="text-xs mt-1">Enable some agents and add servers to see connections</p>
    </div>

    <div
      v-if="contextMenuOpen && contextMenuAgent"
      :style="{
        position: 'fixed',
        left: `${contextMenuPosition.x}px`,
        top: `${contextMenuPosition.y}px`,
        zIndex: 50,
      }"
      class="min-w-32 overflow-hidden rounded-md border border-gray-700/50 p-1 text-popover-foreground shadow-lg backdrop-blur-md bg-popover/80"
      @click.stop
    >
      <div
        class="relative flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground"
        @click="
          viewAgentDetail(contextMenuAgent.name);
          contextMenuOpen = false;
        "
      >
        查看详情
      </div>
      <div
        class="relative flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground"
        @click="
          toggleAgentEnabled(contextMenuAgent.name);
          contextMenuOpen = false;
        "
      >
        {{
          agentsStore.agents.find((a) => a.name === contextMenuAgent?.name)?.enabled
            ? '禁用'
            : '启用'
        }}
      </div>
      <div class="-mx-1 my-1 h-px bg-border" />
      <div
        class="relative flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground"
        @click="
          deleteAllConnections(contextMenuAgent.name);
          contextMenuOpen = false;
        "
      >
        删除所有连接
      </div>
      <div
        class="relative flex cursor-pointer select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground"
        @click="
          openAgentConfig(contextMenuAgent.name);
          contextMenuOpen = false;
        "
      >
        打开配置文件
      </div>
    </div>
  </div>
</template>

<style scoped>
:deep(.server-node:hover circle),
:deep(.agent-node:hover rect) {
  filter: brightness(1.1);
}

:deep(.drag-line) {
  pointer-events: none;
}
</style>
