import { ref } from 'vue';
import type { GraphEdge } from './useGraphData';

export function useGraphFocus() {
  const focusedNodeId = ref<string | null>(null);

  function isConnectedToFocused(nodeId: string, edges: GraphEdge[]): boolean {
    if (!focusedNodeId.value) return true;
    if (nodeId === focusedNodeId.value) return true;

    return edges.some(
      (e) =>
        (e.source === focusedNodeId.value && e.target === nodeId) ||
        (e.target === focusedNodeId.value && e.source === nodeId)
    );
  }

  function isEdgeConnectedToFocused(edge: GraphEdge): boolean {
    if (!focusedNodeId.value) return false;
    return edge.source === focusedNodeId.value || edge.target === focusedNodeId.value;
  }

  function toggleFocus(nodeId: string) {
    focusedNodeId.value = focusedNodeId.value === nodeId ? null : nodeId;
  }

  function clearFocus() {
    focusedNodeId.value = null;
  }

  return {
    focusedNodeId,
    isConnectedToFocused,
    isEdgeConnectedToFocused,
    toggleFocus,
    clearFocus,
  };
}
