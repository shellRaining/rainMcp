<script setup lang="ts">
import { computed, ref } from 'vue';
import { FileText, FolderOpen, Plus, Copy, Check } from 'lucide-vue-next';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Separator } from '@/components/ui/separator';
import McpServerCard from './McpServerCard.vue';
import { useAgentsStore } from '@/stores/agents';
import { AGENT_DISPLAY_NAMES } from '@/types/mcp';

const agentsStore = useAgentsStore();
const isPathCopied = ref(false);

const servers = computed(() => {
  if (!agentsStore.selectedAgent?.mcp_config?.servers) return [];
  return Object.entries(agentsStore.selectedAgent.mcp_config.servers).map(([name, config]) => ({
    name,
    ...config,
  }));
});

function handleOpenConfigFile() {
  if (agentsStore.selectedAgent) {
    agentsStore.openAgentConfigFile(agentsStore.selectedAgent.name);
  }
}

async function handleCopyPath() {
  if (agentsStore.selectedAgent?.config_path) {
    try {
      await writeText(agentsStore.selectedAgent.config_path);
      isPathCopied.value = true;
      setTimeout(() => {
        isPathCopied.value = false;
      }, 2000);
    } catch (error) {
      console.error('Failed to copy path:', error);
    }
  }
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- 空状态 -->
    <div
      v-if="!agentsStore.selectedAgent"
      class="flex-1 flex items-center justify-center text-muted-foreground"
    >
      <!-- 顶部拖拽区域 -->
      <div class="absolute top-0 left-0 right-0 h-13" data-tauri-drag-region />
      <div class="text-center space-y-2">
        <FileText class="h-12 w-12 mx-auto opacity-50" />
        <p>Select an agent to view details</p>
      </div>
    </div>

    <!-- Agent 详情 -->
    <template v-else>
      <!-- Header 作为拖拽区域 -->
      <header class="shrink-0 px-6 py-4 border-b" data-tauri-drag-region>
        <div class="flex items-center justify-between gap-3">
          <div class="flex items-center gap-2 min-w-0 flex-1">
            <h1 class="text-lg font-semibold tracking-tight truncate min-w-0">
              {{ AGENT_DISPLAY_NAMES[agentsStore.selectedAgent.agent_type] }}
            </h1>
            <Badge
              v-if="agentsStore.selectedAgent.enabled"
              variant="default"
              class="text-xs shrink-0"
            >
              Enabled
            </Badge>
            <Badge v-else variant="secondary" class="text-xs shrink-0"> Disabled </Badge>
          </div>
          <Button variant="outline" size="sm" class="shrink-0" @click="handleOpenConfigFile">
            <FolderOpen class="h-4 w-4 sm:mr-2" />
            <span class="hidden sm:inline">Open Config</span>
          </Button>
        </div>
        <!-- 配置路径作为副标题 - 点击复制 -->
        <button
          class="flex items-center gap-1 text-[10px] text-muted-foreground font-mono mt-1 hover:text-foreground transition-colors cursor-pointer max-w-full"
          title="Click to copy path"
          @click="handleCopyPath"
        >
          <span class="truncate min-w-0">{{ agentsStore.selectedAgent.config_path }}</span>
          <Check v-if="isPathCopied" class="h-3 w-3 text-green-500 shrink-0" />
          <Copy v-else class="h-3 w-3 opacity-50 shrink-0" />
        </button>
      </header>

      <!-- MCP Servers -->
      <ScrollArea class="flex-1">
        <div class="p-6 space-y-4">
          <div class="flex items-center justify-between">
            <h2 class="text-base font-medium">MCP Servers</h2>
            <Button variant="outline" size="sm" disabled>
              <Plus class="h-4 w-4 mr-2" />
              Add Server
            </Button>
          </div>

          <Separator style="background-color: hsl(var(--border))" />

          <div v-if="servers.length === 0" class="py-12 text-center text-muted-foreground">
            <p>No MCP servers configured</p>
            <p class="text-sm mt-1">Click "Open Config" to edit the configuration file</p>
          </div>

          <div v-else class="flex flex-col gap-4">
            <McpServerCard
              v-for="server in servers"
              :key="server.name"
              :server="server"
              :agent-name="agentsStore.selectedAgent.name"
            />
          </div>
        </div>
      </ScrollArea>
    </template>
  </div>
</template>
