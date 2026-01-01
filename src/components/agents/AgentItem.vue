<script setup lang="ts">
import { Badge } from '@/components/ui/badge';
import AgentIcon from './AgentIcon.vue';
import type { SupportedAgent } from '@/types/mcp';
import { AGENT_DISPLAY_NAMES } from '@/types/mcp';

defineProps<{
  agent: SupportedAgent;
  serverCount: number;
}>();

const emit = defineEmits<{
  select: [];
}>();

function handleClick() {
  emit('select');
}
</script>

<template>
  <div
    class="px-4 py-3 rounded-lg border bg-card hover:bg-accent/50 cursor-pointer transition-colors agent-card"
    @click="handleClick"
  >
    <div class="flex items-center gap-3">
      <AgentIcon :agent-type="agent.agent_type" :size="24" class="text-foreground shrink-0" />

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

      <div class="flex items-center gap-1 text-muted-foreground shrink-0">
        <span class="text-lg font-semibold tabular-nums">{{ serverCount }}</span>
        <span class="text-xs server-label">servers</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.agent-card {
  container-type: inline-size;
}

@container (max-width: 280px) {
  .badge-configured {
    display: none;
  }
}

@container (max-width: 200px) {
  .server-label {
    display: none;
  }
}
</style>
