<script setup lang="ts">
import { Bot, Settings } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import { ScrollArea } from '@/components/ui/scroll-area';
import { useAgentsStore } from '@/stores/agents';
import { useAppStore } from '@/stores/app';
import AgentList from '@/components/agents/AgentList.vue';

const agentsStore = useAgentsStore();
const appStore = useAppStore();

function handleAgentClick(agentName: string) {
  agentsStore.selectAgent(agentName);
  appStore.clickDetailItem('agents', agentName);
}

function goToSettings() {
  appStore.clickDetailItem('settings', 'agent-management');
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

        <!-- Agent Cards -->
        <AgentList v-else :agents="agentsStore.enabledAgents" @select="handleAgentClick" />
      </div>
    </ScrollArea>
  </div>
</template>
