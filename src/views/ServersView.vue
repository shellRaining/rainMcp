<script setup lang="ts">
import { computed } from 'vue';
import { Server } from 'lucide-vue-next';
import { ScrollArea } from '@/components/ui/scroll-area';
import { useAgentsStore } from '@/stores/agents';
import { useAppStore } from '@/stores/app';
import { AGENT_DISPLAY_NAMES } from '@/types/mcp';
import McpServerCard from '@/components/mcp/McpServerCard.vue';

const agentsStore = useAgentsStore();
const appStore = useAppStore();

// 找到选中的 server 及其所属 agent
const selectedServerInfo = computed(() => {
  if (!appStore.selectedDetailId) return null;

  for (const agent of agentsStore.enabledAgents) {
    if (agent.mcp_config?.servers) {
      const serverConfig = agent.mcp_config.servers[appStore.selectedDetailId];
      if (serverConfig) {
        return {
          name: appStore.selectedDetailId,
          config: serverConfig,
          agent,
        };
      }
    }
  }
  return null;
});
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Header -->
    <header class="shrink-0 h-13 px-6 flex items-center border-b" data-tauri-drag-region>
      <h1 class="text-lg font-semibold tracking-tight">Servers</h1>
    </header>

    <!-- Empty State -->
    <div
      v-if="!selectedServerInfo"
      class="flex-1 flex items-center justify-center text-muted-foreground"
    >
      <div class="text-center space-y-2">
        <Server class="h-12 w-12 mx-auto opacity-50" />
        <p>Select a server from the sidebar</p>
      </div>
    </div>

    <!-- Server Detail -->
    <ScrollArea v-else class="flex-1">
      <div class="p-6 space-y-4">
        <div class="flex items-center gap-2 text-sm text-muted-foreground">
          <span>From:</span>
          <span class="font-medium text-foreground">
            {{ AGENT_DISPLAY_NAMES[selectedServerInfo.agent.agent_type] }}
          </span>
        </div>

        <McpServerCard :server="{ name: selectedServerInfo.name, ...selectedServerInfo.config }" />
      </div>
    </ScrollArea>
  </div>
</template>
