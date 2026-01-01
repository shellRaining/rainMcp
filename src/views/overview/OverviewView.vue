<script setup lang="ts">
import { computed } from 'vue';
import { Bot, Server, Activity } from 'lucide-vue-next';
import { useAgentsStore } from '@/stores/agents';

const agentsStore = useAgentsStore();

const stats = computed(() => {
  const enabledCount = agentsStore.enabledAgents.length;
  const totalCount = agentsStore.agents.length;
  let serverCount = 0;
  for (const agent of agentsStore.enabledAgents) {
    if (agent.mcp_config?.servers) {
      serverCount += Object.keys(agent.mcp_config.servers).length;
    }
  }
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
    <div class="flex-1 overflow-y-auto p-6">
      <div class="max-w-2xl space-y-6">
        <!-- Stats Cards -->
        <div class="grid grid-cols-3 gap-4">
          <div class="p-4 rounded-lg bg-accent/50 border">
            <div class="flex items-center gap-2 text-muted-foreground mb-2">
              <Bot class="h-4 w-4" />
              <span class="text-xs font-medium uppercase tracking-wider">Agents</span>
            </div>
            <p class="text-2xl font-semibold">
              {{ stats.enabledCount }}
              <span class="text-sm font-normal text-muted-foreground"
                >/ {{ stats.totalCount }}</span
              >
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

        <!-- Placeholder for future AntV integration -->
        <div class="p-8 rounded-lg border border-dashed text-center text-muted-foreground">
          <p class="text-sm">AntV visualization area</p>
          <p class="text-xs mt-1">Coming soon...</p>
        </div>
      </div>
    </div>
  </div>
</template>
