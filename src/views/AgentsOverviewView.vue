<script setup lang="ts">
import { Bot, Settings } from 'lucide-vue-next';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { ScrollArea } from '@/components/ui/scroll-area';
import { useAgentsStore } from '@/stores/agents';
import { useAppStore } from '@/stores/app';
import { AGENT_DISPLAY_NAMES } from '@/types/mcp';
import AgentIcon from '@/components/agents/AgentIcon.vue';

const agentsStore = useAgentsStore();
const appStore = useAppStore();

function handleAgentClick(agentName: string) {
  agentsStore.selectAgent(agentName);
  appStore.clickDetailItem('agents', agentName);
}

function goToSettings() {
  appStore.clickDetailItem('settings', 'agent-management');
}

function getServerCount(agent: (typeof agentsStore.enabledAgents)[0]): number {
  return agent.mcp_config?.servers ? Object.keys(agent.mcp_config.servers).length : 0;
}
</script>

<template>
  <div class="h-full flex flex-col agents-overview">
    <!-- Header -->
    <header
      class="shrink-0 h-13 px-6 flex items-center justify-between border-b"
      data-tauri-drag-region
    >
      <h1 class="text-lg font-semibold tracking-tight">Agents</h1>
      <Button variant="outline" size="sm" @click="goToSettings">
        <Settings class="h-4 w-4 mr-2" />
        Manage
      </Button>
    </header>

    <!-- Content -->
    <ScrollArea class="flex-1">
      <div class="p-6">
        <!-- Empty State -->
        <div
          v-if="agentsStore.enabledAgents.length === 0"
          class="py-12 text-center text-muted-foreground"
        >
          <Bot class="h-12 w-12 mx-auto mb-4 opacity-50" />
          <p class="mb-2">No agents enabled</p>
          <Button variant="link" size="sm" @click="goToSettings">
            Go to Settings to enable agents
          </Button>
        </div>

        <!-- Agent Cards - Simplified -->
        <div v-else class="flex flex-col gap-3">
          <div
            v-for="agent in agentsStore.enabledAgents"
            :key="agent.name"
            class="px-4 py-3 rounded-lg border bg-card hover:bg-accent/50 cursor-pointer transition-colors agent-card"
            @click="handleAgentClick(agent.name)"
          >
            <div class="flex items-center gap-3">
              <!-- Icon -->
              <AgentIcon
                :agent-type="agent.agent_type"
                :size="24"
                class="text-foreground shrink-0"
              />

              <!-- Name + Badge (can shrink and truncate) -->
              <div class="flex items-center gap-2 min-w-0 flex-1">
                <span class="font-medium truncate">
                  {{ AGENT_DISPLAY_NAMES[agent.agent_type] || agent.name }}
                </span>
                <Badge
                  v-if="agent.is_configured"
                  variant="secondary"
                  class="text-xs shrink-0 badge-configured"
                >
                  Configured
                </Badge>
              </div>

              <!-- Server Count -->
              <div class="flex items-center gap-1 text-muted-foreground shrink-0">
                <span class="text-lg font-semibold tabular-nums">{{ getServerCount(agent) }}</span>
                <span class="text-xs server-label">servers</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </ScrollArea>
  </div>
</template>

<style scoped>
/* 每个卡片作为独立的容器 */
.agent-card {
  container-type: inline-size;
}

/* 当卡片宽度小于 280px 时隐藏 badge */
@container (max-width: 280px) {
  .badge-configured {
    display: none;
  }
}

/* 当卡片宽度小于 200px 时隐藏 servers 文字 */
@container (max-width: 200px) {
  .server-label {
    display: none;
  }
}
</style>
