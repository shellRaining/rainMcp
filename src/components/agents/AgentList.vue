<script setup lang="ts">
import AgentItem from './AgentItem.vue';
import type { SupportedAgent } from '@/types/mcp';

defineProps<{
  agents: SupportedAgent[];
}>();

const emit = defineEmits<{
  select: [agentName: string];
}>();

function getServerCount(agent: SupportedAgent): number {
  return agent.mcp_config?.servers ? Object.keys(agent.mcp_config.servers).length : 0;
}

function handleSelect(agentName: string) {
  emit('select', agentName);
}
</script>

<template>
  <div class="flex flex-col gap-3">
    <AgentItem
      v-for="agent in agents"
      :key="agent.name"
      :agent="agent"
      :server-count="getServerCount(agent)"
      @select="handleSelect(agent.name)"
    />
  </div>
</template>
