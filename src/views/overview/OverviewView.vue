<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { Bot, Server, Activity } from 'lucide-vue-next';
import { useAgentsStore } from '@/stores/agents';
import { useServersStore } from '@/stores/servers';
import ServerAgentGraph from '@/components/overview/ServerAgentGraph.vue';

const agentsStore = useAgentsStore();
const serversStore = useServersStore();

// Initialize servers store if not already loaded
onMounted(async () => {
  if (serversStore.userServers.length === 0) {
    await serversStore.fetchUserServers();
  }
});

const stats = computed(() => {
  const enabledCount = agentsStore.enabledAgents.length;
  const totalCount = agentsStore.agents.length;
  const serverCount = serversStore.userServers.length;
  return { enabledCount, totalCount, serverCount };
});
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Header -->
    <header class="shrink-0 h-13 px-6 flex items-center border-b" data-tauri-drag-region>
      <h1 class="text-lg font-semibold tracking-tight">Overview</h1>
    </header>

    <!-- Content -->
    <div class="flex-1 flex flex-col overflow-hidden p-6">
      <!-- Stats Cards -->
      <div class="shrink-0 grid grid-cols-3 gap-4 mb-6">
        <div class="p-4 rounded-lg bg-accent/50 border">
          <div class="flex items-center gap-2 text-muted-foreground mb-2">
            <Bot class="h-4 w-4" />
            <span class="text-xs font-medium uppercase tracking-wider">Agents</span>
          </div>
          <p class="text-2xl font-semibold">
            {{ stats.enabledCount }}
            <span class="text-sm font-normal text-muted-foreground">/ {{ stats.totalCount }}</span>
          </p>
        </div>

        <div class="p-4 rounded-lg bg-accent/50 border">
          <div class="flex items-center gap-2 text-muted-foreground mb-2">
            <Server class="h-4 w-4" />
            <span class="text-xs font-medium uppercase tracking-wider">Servers</span>
          </div>
          <p class="text-2xl font-semibold">{{ stats.serverCount }}</p>
        </div>

        <div class="p-4 rounded-lg bg-accent/50 border">
          <div class="flex items-center gap-2 text-muted-foreground mb-2">
            <Activity class="h-4 w-4" />
            <span class="text-xs font-medium uppercase tracking-wider">Status</span>
          </div>
          <p class="text-2xl font-semibold text-green-500">Active</p>
        </div>
      </div>

      <!-- Server-Agent Connection Graph - fills remaining space -->
      <div class="flex-1 min-h-0">
        <div class="text-xs text-muted-foreground mb-2">
          Click a node to focus · Click edges to remove bindings
        </div>
        <ServerAgentGraph class="h-full w-full" />
      </div>
    </div>
  </div>
</template>
