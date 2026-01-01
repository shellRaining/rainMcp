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
  remoteUrl: string;
  remoteHeaders: string;
  isSubmitting: boolean;
}>();

const emit = defineEmits<{
  'update:serverName': [value: string];
  'update:remoteUrl': [value: string];
  'update:remoteHeaders': [value: string];
  'submit': [];
}>();

const canSubmit = computed(() => {
  return props.serverName.trim().length > 0 && props.remoteUrl.trim().length > 0;
});

function updateServerName(value: string | number) {
  emit('update:serverName', String(value));
}

function updateRemoteUrl(value: string | number) {
  emit('update:remoteUrl', String(value));
}

function updateRemoteHeaders(value: string | number) {
  emit('update:remoteHeaders', String(value));
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
            placeholder="my-remote-server"
            required
          />
        </div>

        <!-- Server URL -->
        <div class="space-y-2">
          <Label for="url">Server URL *</Label>
          <Input
            id="url"
            :model-value="remoteUrl"
            @update:model-value="updateRemoteUrl"
            placeholder="https://api.example.com/mcp"
            type="url"
            required
          />
          <p class="text-xs text-muted-foreground">
            The URL of the remote MCP server (SSE or HTTP endpoint)
          </p>
        </div>

        <!-- Headers -->
        <div class="space-y-2">
          <Label for="headers">Headers (Optional)</Label>
          <Textarea
            id="headers"
            :model-value="remoteHeaders"
            @update:model-value="updateRemoteHeaders"
            placeholder="Authorization: Bearer your-token&#10;X-API-Key: your-api-key"
            rows="4"
            class="font-mono text-sm"
          />
          <p class="text-xs text-muted-foreground">One header per line in KEY: VALUE format</p>
        </div>

        <!-- Example -->
        <div class="p-3 rounded-lg bg-muted text-sm space-y-1">
          <p class="font-medium">Example:</p>
          <p class="font-mono text-xs">URL: https://mcp-server.example.com</p>
          <p class="font-mono text-xs">Header: Authorization: Bearer abc123</p>
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
