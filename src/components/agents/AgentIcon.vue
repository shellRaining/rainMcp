<script setup lang="ts">
import { computed, defineAsyncComponent } from 'vue';
import type { AgentType } from '@/types/mcp';

const props = defineProps<{
  agentType: AgentType;
  size?: number;
}>();

const size = computed(() => props.size ?? 24);

// Map agent types to icon file names
const iconMap: Record<AgentType, string> = {
  ClaudeCode: 'claude',
  ClaudeDesktop: 'claude',
  Cursor: 'cursor',
  Windsurf: 'windsurf',
  Cline: 'cline',
  RooCode: 'roo-code',
  Trae: 'trae',
  GeminiCli: 'gemini',
  Kiro: 'kiro',
  OpenAiCodex: 'openai',
};

// Dynamic import SVG as Vue component
const iconComponents: Record<string, ReturnType<typeof defineAsyncComponent>> = {
  claude: defineAsyncComponent(() => import('@/assets/icons/agents/claude.svg')),
  cursor: defineAsyncComponent(() => import('@/assets/icons/agents/cursor.svg')),
  windsurf: defineAsyncComponent(() => import('@/assets/icons/agents/windsurf.svg')),
  cline: defineAsyncComponent(() => import('@/assets/icons/agents/cline.svg')),
  'roo-code': defineAsyncComponent(() => import('@/assets/icons/agents/roo-code.svg')),
  trae: defineAsyncComponent(() => import('@/assets/icons/agents/trae.svg')),
  gemini: defineAsyncComponent(() => import('@/assets/icons/agents/gemini.svg')),
  kiro: defineAsyncComponent(() => import('@/assets/icons/agents/kiro.svg')),
  openai: defineAsyncComponent(() => import('@/assets/icons/agents/openai.svg')),
};

const IconComponent = computed(() => {
  const iconName = iconMap[props.agentType];
  return iconComponents[iconName];
});
</script>

<template>
  <component
    :is="IconComponent"
    :style="{ width: `${size}px`, height: `${size}px` }"
    class="shrink-0"
  />
</template>
