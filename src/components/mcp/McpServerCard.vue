<script setup lang="ts">
import { ref, computed, onUnmounted } from 'vue';
import { Terminal, Globe, Copy, Check, ChevronDown, Clock, Trash2 } from 'lucide-vue-next';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { Card } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import type { AgentServerEntry } from '@/types/mcp';
import * as tauriApi from '@/api/tauri';
import { logger } from '@/utils/logger';
import { useAgentsStore } from '@/stores/agents';

const props = defineProps<{
  server: AgentServerEntry & { name: string };
  agentName: string;
}>();

const agentsStore = useAgentsStore();
const isExpanded = ref(false);
const isCopied = ref(false);
const isDeleting = ref(false);
const isDeleteArmed = ref(false);
let deleteArmTimeout: ReturnType<typeof setTimeout> | null = null;

const isLocal = computed(() => props.server.type === 'local');

// 格式化 timeout 显示
const formattedTimeout = computed(() => {
  if (!props.server.timeout) return null;
  const seconds = props.server.timeout / 1000;
  return seconds >= 1 ? `${seconds}s` : `${props.server.timeout}ms`;
});

// 格式化 arguments 为单行字符串
const formattedArgs = computed(() => {
  if (props.server.type !== 'local') return null;
  const args = props.server.args;
  if (!args || args.length === 0) return null;
  return args.join(' ');
});

function toggleExpand() {
  disarmDelete();
  isExpanded.value = !isExpanded.value;
}

async function handleCopy(event: Event) {
  event.stopPropagation();
  disarmDelete();
  try {
    const rawConfig = await tauriApi.getServerRawConfig(props.agentName, props.server.name);
    await writeText(rawConfig);
    isCopied.value = true;
    setTimeout(() => {
      isCopied.value = false;
    }, 2000);
  } catch (error) {
    logger.error('Failed to copy config:', error);
  }
}

function disarmDelete() {
  isDeleteArmed.value = false;
  if (deleteArmTimeout) {
    clearTimeout(deleteArmTimeout);
    deleteArmTimeout = null;
  }
}

function handleDeleteClick(event: Event) {
  event.stopPropagation();
  if (isDeleting.value) return;

  if (!isDeleteArmed.value) {
    isDeleteArmed.value = true;
    deleteArmTimeout = setTimeout(() => {
      disarmDelete();
    }, 2500);
    return;
  }

  disarmDelete();
  void handleDelete();
}

async function handleDelete() {
  isDeleting.value = true;
  try {
    const agent = agentsStore.agents.find((a) => a.name === props.agentName);
    const servers = agent?.mcp_config?.servers;
    if (!servers || !(props.server.name in servers)) return;

    const nextServers = { ...servers };
    delete nextServers[props.server.name];

    await agentsStore.updateMcpConfig(props.agentName, { servers: nextServers });
    await agentsStore.fetchAgents();
  } catch (error) {
    logger.error('Failed to delete server from agent:', error);
  } finally {
    isDeleting.value = false;
  }
}

onUnmounted(() => {
  disarmDelete();
});
</script>

<template>
  <Card class="overflow-hidden">
    <!-- Header - 可点击展开/收起 -->
    <div
      class="flex items-center justify-between p-4 cursor-pointer hover:bg-accent/30 transition-colors"
      @click="toggleExpand"
    >
      <div class="flex items-center gap-3 min-w-0 flex-1">
        <!-- 图标 -->
        <div class="shrink-0 w-8 h-8 rounded-md bg-muted flex items-center justify-center">
          <Terminal v-if="isLocal" class="h-4 w-4 text-muted-foreground" />
          <Globe v-else class="h-4 w-4 text-muted-foreground" />
        </div>
        <!-- 名称和类型 -->
        <div class="min-w-0">
          <h3 class="font-medium text-sm truncate">{{ server.name }}</h3>
          <p class="text-xs text-muted-foreground">
            {{ isLocal ? 'Local' : 'Remote' }}
          </p>
        </div>
      </div>
      <!-- 操作按钮 -->
      <div class="flex items-center gap-1 shrink-0">
        <button
          class="p-1.5 rounded-md transition-colors"
          :class="
            isDeleteArmed
              ? 'bg-destructive text-destructive-foreground hover:bg-destructive/90'
              : 'hover:bg-muted'
          "
          :title="isDeleteArmed ? 'Click again to delete' : 'Delete from agent'"
          @click="handleDeleteClick"
        >
          <Trash2 class="h-4 w-4" :class="isDeleteArmed ? '' : 'text-muted-foreground'" />
        </button>
        <button
          class="p-1.5 rounded-md hover:bg-muted transition-colors"
          title="Copy config"
          @click="handleCopy"
        >
          <Check v-if="isCopied" class="h-4 w-4 text-green-500" />
          <Copy v-else class="h-4 w-4 text-muted-foreground" />
        </button>
        <button
          class="p-1.5 rounded-md hover:bg-muted transition-colors"
          :title="isExpanded ? 'Collapse' : 'Expand'"
        >
          <ChevronDown
            class="h-4 w-4 text-muted-foreground chevron-icon"
            :class="{ expanded: isExpanded }"
          />
        </button>
      </div>
    </div>

    <!-- 展开内容 - 使用 CSS Grid 实现高度动画 -->
    <div class="grid-collapse" :class="{ expanded: isExpanded }">
      <div class="overflow-hidden">
        <Separator style="background-color: hsl(var(--border))" />
        <div class="p-4 space-y-4">
          <!-- Local Config -->
          <template v-if="isLocal && server.type === 'local'">
            <div class="space-y-1.5">
              <p class="text-xs font-medium text-muted-foreground">Command</p>
              <code
                class="block text-xs bg-muted px-3 py-2 rounded-md font-mono break-all selectable"
              >
                {{ server.command }}
              </code>
            </div>
            <div v-if="formattedArgs" class="space-y-1.5">
              <p class="text-xs font-medium text-muted-foreground">Arguments</p>
              <code
                class="block text-xs bg-muted px-3 py-2 rounded-md font-mono break-all selectable"
              >
                {{ formattedArgs }}
              </code>
            </div>
            <div v-if="server.env && Object.keys(server.env).length > 0" class="space-y-1.5">
              <p class="text-xs font-medium text-muted-foreground">Environment</p>
              <div class="bg-muted rounded-md px-3 py-2 space-y-1">
                <div
                  v-for="(value, key) in server.env"
                  :key="key"
                  class="text-xs font-mono selectable flex"
                >
                  <span class="text-muted-foreground shrink-0">{{ key }}</span>
                  <span class="text-muted-foreground mx-1">=</span>
                  <span class="break-all">{{ value }}</span>
                </div>
              </div>
            </div>
          </template>

          <!-- Remote Config -->
          <template v-else-if="server.type === 'remote'">
            <div class="space-y-1.5">
              <p class="text-xs font-medium text-muted-foreground">URL</p>
              <code
                class="block text-xs bg-muted px-3 py-2 rounded-md font-mono break-all selectable"
              >
                {{ server.url }}
              </code>
            </div>
            <div
              v-if="server.headers && Object.keys(server.headers).length > 0"
              class="space-y-1.5"
            >
              <p class="text-xs font-medium text-muted-foreground">Headers</p>
              <div class="bg-muted rounded-md px-3 py-2 space-y-1">
                <div
                  v-for="(value, key) in server.headers"
                  :key="key"
                  class="text-xs font-mono selectable flex"
                >
                  <span class="text-muted-foreground shrink-0">{{ key }}</span>
                  <span class="text-muted-foreground mx-1">:</span>
                  <span class="break-all">{{ value }}</span>
                </div>
              </div>
            </div>
          </template>

          <!-- Timeout (显示在展开内容底部) -->
          <div
            v-if="formattedTimeout"
            class="flex items-center gap-1.5 text-xs text-muted-foreground"
          >
            <Clock class="h-3 w-3" />
            <span>Timeout: {{ formattedTimeout }}</span>
          </div>
        </div>
      </div>
    </div>
  </Card>
</template>

<style scoped>
/* CSS Grid 方案实现高度从 0 到 auto 的动画 */
.grid-collapse {
  display: grid;
  grid-template-rows: 0fr;
  transition: grid-template-rows 250ms ease-out;
}

.grid-collapse.expanded {
  grid-template-rows: 1fr;
}

.grid-collapse > div {
  overflow: hidden;
}

/* 展开按钮旋转动画 */
.chevron-icon {
  transition: transform 250ms ease-out;
}

.chevron-icon.expanded {
  transform: rotate(-180deg);
}
</style>
