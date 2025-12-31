<script setup lang="ts">
import AgentItem from './AgentItem.vue';
import { useAgentsStore } from '@/stores/agents';

const agentsStore = useAgentsStore();
</script>

<template>
  <div class="space-y-1">
    <div v-if="agentsStore.isLoading" class="p-4 text-center text-sm text-muted-foreground">
      Loading...
    </div>
    <div v-else-if="agentsStore.error" class="p-4 text-center text-sm text-destructive">
      {{ agentsStore.error }}
    </div>
    <template v-else>
      <AgentItem
        v-for="agent in agentsStore.agents"
        :key="agent.name"
        :agent="agent"
        :is-selected="agentsStore.selectedAgentName === agent.name"
        @select="agentsStore.selectAgent(agent.name)"
        @toggle-enabled="agentsStore.toggleAgentEnabled(agent.name)"
      />
    </template>
  </div>
</template>
