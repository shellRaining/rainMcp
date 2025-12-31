<script setup lang="ts">
import { Check, X } from 'lucide-vue-next';
import { Switch } from '@/components/ui/switch';
import { Badge } from '@/components/ui/badge';
import { ScrollArea } from '@/components/ui/scroll-area';
import { useAgentsStore } from '@/stores/agents';
import { AGENT_DISPLAY_NAMES } from '@/types/mcp';

const agentsStore = useAgentsStore();

function handleToggle(agentName: string) {
  agentsStore.toggleAgentEnabled(agentName);
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Header -->
    <header class="shrink-0 h-13 px-6 flex items-center border-b" data-tauri-drag-region>
      <h1 class="text-lg font-semibold tracking-tight">Agent Management</h1>
    </header>

    <!-- Content -->
    <ScrollArea class="flex-1">
      <div class="p-6 space-y-4">
        <p class="text-sm text-muted-foreground">
          Enable or disable agents to manage which ones appear in your sidebar.
        </p>

        <div class="space-y-2">
          <div
            v-for="agent in agentsStore.agents"
            :key="agent.name"
            class="flex items-center justify-between p-4 rounded-lg border bg-card"
          >
            <div class="flex items-center gap-3">
              <div>
                <div class="flex items-center gap-2">
                  <span class="font-medium">
                    {{ AGENT_DISPLAY_NAMES[agent.agent_type] || agent.name }}
                  </span>
                  <Badge v-if="agent.is_configured" variant="secondary" class="text-xs px-1.5 py-0">
                    <Check class="h-3 w-3" />
                  </Badge>
                  <Badge v-else variant="outline" class="text-xs px-1.5 py-0 text-muted-foreground">
                    <X class="h-3 w-3" />
                  </Badge>
                </div>
                <p class="text-xs text-muted-foreground mt-0.5 font-mono selectable">
                  {{ agent.config_path }}
                </p>
              </div>
            </div>
            <Switch
              :model-value="agent.enabled"
              @update:model-value="() => handleToggle(agent.name)"
            />
          </div>
        </div>
      </div>
    </ScrollArea>
  </div>
</template>
