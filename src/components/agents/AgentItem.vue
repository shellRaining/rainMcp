<script setup lang="ts">
import { Settings, Check } from 'lucide-vue-next';
import { Switch } from '@/components/ui/switch';
import { Badge } from '@/components/ui/badge';
import { cn } from '@/lib/utils';
import type { SupportedAgent } from '@/types/mcp';
import { AGENT_DISPLAY_NAMES } from '@/types/mcp';

const props = defineProps<{
  agent: SupportedAgent;
  isSelected: boolean;
}>();

const emit = defineEmits<{
  select: [];
  toggleEnabled: [];
}>();

function handleClick() {
  emit('select');
}

function handleSwitchClick(e: Event) {
  e.stopPropagation();
  emit('toggleEnabled');
}
</script>

<template>
  <div
    :class="
      cn(
        'group flex items-center gap-3 px-3 py-2 rounded-md cursor-pointer transition-colors',
        'hover:bg-accent',
        isSelected && 'bg-accent'
      )
    "
    @click="handleClick"
  >
    <div class="flex-1 min-w-0">
      <div class="flex items-center gap-2">
        <span class="text-sm font-medium truncate">
          {{ AGENT_DISPLAY_NAMES[agent.agent_type] || agent.name }}
        </span>
        <Badge v-if="agent.is_configured" variant="secondary" class="text-xs px-1.5 py-0">
          <Check class="h-3 w-3" />
        </Badge>
      </div>
      <div class="flex items-center gap-1 mt-0.5">
        <Settings class="h-3 w-3 text-muted-foreground" />
        <span class="text-xs text-muted-foreground truncate">
          {{
            agent.mcp_config
              ? `${Object.keys(agent.mcp_config.servers || {}).length} servers`
              : 'No config'
          }}
        </span>
      </div>
    </div>
    <Switch :checked="agent.enabled" @click="handleSwitchClick" />
  </div>
</template>
