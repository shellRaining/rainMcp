<script setup lang="ts">
import { computed } from 'vue';
import { Label } from '@/components/ui/label';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Button } from '@/components/ui/button';
import { Loader2 } from 'lucide-vue-next';

const props = defineProps<{
  serverName: string;
  customCommand: string;
  customArgs: string;
  customEnv: string;
  isSubmitting: boolean;
}>();

const emit = defineEmits<{
  'update:serverName': [value: string];
  'update:customCommand': [value: string];
  'update:customArgs': [value: string];
  'update:customEnv': [value: string];
  'submit': [];
}>();

const canSubmit = computed(() => {
  return props.serverName.trim().length > 0 && props.customCommand.trim().length > 0;
});

function updateServerName(value: string | number) {
  emit('update:serverName', String(value));
}

function updateCustomCommand(value: string | number) {
  emit('update:customCommand', String(value));
}

function updateCustomArgs(value: string | number) {
  emit('update:customArgs', String(value));
}

function updateCustomEnv(value: string | number) {
  emit('update:customEnv', String(value));
}
</script>

<template>
  <div class="h-full flex flex-col">
    <ScrollArea class="flex-1">
      <div class="p-4 space-y-4 max-w-2xl mx-auto">
        <!-- Server Name -->
        <div class="space-y-2">
          <Label for="server-name">Server Name *</Label>
          <Input
            id="server-name"
            :model-value="serverName"
            @update:model-value="updateServerName"
            placeholder="my-custom-server"
            required
          />
        </div>

        <!-- Command -->
        <div class="space-y-2">
          <Label for="command">Command *</Label>
          <Input
            id="command"
            :model-value="customCommand"
            @update:model-value="updateCustomCommand"
            placeholder="node"
            required
          />
          <p class="text-xs text-muted-foreground">
            The executable command (e.g., node, python, ./my-script)
          </p>
        </div>

        <!-- Arguments -->
        <div class="space-y-2">
          <Label for="args">Arguments</Label>
          <Input
            id="args"
            :model-value="customArgs"
            @update:model-value="updateCustomArgs"
            placeholder="server.js --port 3000"
          />
          <p class="text-xs text-muted-foreground">Command arguments, space-separated</p>
        </div>

        <!-- Environment Variables -->
        <div class="space-y-2">
          <Label for="env">Environment Variables</Label>
          <Textarea
            id="env"
            :model-value="customEnv"
            @update:model-value="updateCustomEnv"
            placeholder="API_KEY=your-key&#10;DEBUG=true"
            rows="4"
            class="font-mono text-sm"
          />
          <p class="text-xs text-muted-foreground">One variable per line in KEY=VALUE format</p>
        </div>

        <!-- Example -->
        <div class="p-3 rounded-lg bg-muted text-sm space-y-1">
          <p class="font-medium">Example:</p>
          <p class="font-mono text-xs">Command: node</p>
          <p class="font-mono text-xs">Arguments: server.js --mode production</p>
          <p class="font-mono text-xs">Env: PORT=3000</p>
        </div>
      </div>
    </ScrollArea>

    <!-- Footer -->
    <footer class="shrink-0 p-4 border-t flex items-center justify-end">
      <Button @click="emit('submit')" :disabled="!canSubmit || isSubmitting">
        <Loader2 v-if="isSubmitting" class="h-4 w-4 mr-2 animate-spin" />
        {{ isSubmitting ? 'Adding...' : 'Add Server' }}
      </Button>
    </footer>
  </div>
</template>
