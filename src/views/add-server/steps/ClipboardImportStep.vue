<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { AlertCircle, Globe, Terminal, RefreshCw } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { useClipboardParser, type ParsedServer, type ParseResult } from '../composables/useClipboardParser';

const props = defineProps<{
  isSubmitting: boolean;
}>();

const emit = defineEmits<{
  submit: [servers: ParsedServer[]];
}>();

const { parseFromClipboard } = useClipboardParser();

const parseResult = ref<ParseResult | null>(null);
const isLoading = ref(false);
const editableServers = ref<ParsedServer[]>([]);

onMounted(async () => {
  await doParseClipboard();
});

async function doParseClipboard() {
  isLoading.value = true;
  parseResult.value = null;

  try {
    parseResult.value = await parseFromClipboard();
    if (parseResult.value.ok) {
      editableServers.value = parseResult.value.servers.map((s) => ({ ...s }));
    }
  } catch (error) {
    parseResult.value = {
      ok: false,
      message: 'Failed to read clipboard',
      hint: String(error),
    };
  } finally {
    isLoading.value = false;
  }
}

function updateServerName(index: number, name: string) {
  editableServers.value[index].name = name;
}

function handleSubmit() {
  if (editableServers.value.length > 0) {
    emit('submit', editableServers.value);
  }
}

function canSubmit(): boolean {
  return (
    !props.isSubmitting &&
    editableServers.value.length > 0 &&
    editableServers.value.every((s) => s.name.trim().length > 0)
  );
}
</script>

<template>
  <div class="flex-1 flex flex-col p-6 overflow-y-auto">
    <div class="max-w-2xl mx-auto w-full space-y-6">
      <!-- Loading State -->
      <div v-if="isLoading" class="flex flex-col items-center justify-center py-12 space-y-4">
        <RefreshCw class="h-8 w-8 text-muted-foreground animate-spin" />
        <p class="text-sm text-muted-foreground">Reading clipboard...</p>
      </div>

      <!-- Error State -->
      <div
        v-else-if="parseResult && !parseResult.ok"
        class="flex flex-col items-center justify-center py-12 space-y-4"
      >
        <div class="p-4 rounded-full bg-destructive/10">
          <AlertCircle class="h-8 w-8 text-destructive" />
        </div>
        <div class="text-center space-y-2">
          <p class="font-medium">{{ parseResult.message }}</p>
          <p v-if="parseResult.hint" class="text-sm text-muted-foreground">
            {{ parseResult.hint }}
          </p>
        </div>
        <Button variant="outline" @click="doParseClipboard">
          <RefreshCw class="h-4 w-4 mr-2" />
          Retry
        </Button>
      </div>

      <!-- Success State -->
      <template v-else-if="parseResult?.ok && editableServers.length > 0">
        <div class="text-center mb-4">
          <h2 class="text-lg font-semibold">
            Detected {{ editableServers.length }} server{{ editableServers.length > 1 ? 's' : '' }}
          </h2>
          <p class="text-sm text-muted-foreground">Review and edit the configuration before importing</p>
        </div>

        <!-- Server Cards -->
        <div class="space-y-4">
          <div
            v-for="(server, index) in editableServers"
            :key="index"
            class="p-4 border rounded-lg space-y-4"
          >
            <!-- Server Name Input -->
            <div class="space-y-2">
              <Label :for="`server-name-${index}`">Name</Label>
              <Input
                :id="`server-name-${index}`"
                :model-value="server.name"
                @update:model-value="(v) => updateServerName(index, v as string)"
                placeholder="Server name"
              />
            </div>

            <!-- Server Type Badge and Details -->
            <div class="space-y-2">
              <div class="flex items-center gap-2">
                <div
                  class="inline-flex items-center gap-1.5 px-2 py-1 rounded-md text-xs font-medium"
                  :class="
                    server.config.type === 'remote'
                      ? 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400'
                      : 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400'
                  "
                >
                  <Globe v-if="server.config.type === 'remote'" class="h-3 w-3" />
                  <Terminal v-else class="h-3 w-3" />
                  {{ server.config.type === 'remote' ? 'Remote' : 'Local' }}
                </div>
              </div>

              <!-- Config Details -->
              <div class="text-sm space-y-1 text-muted-foreground font-mono bg-muted/50 p-3 rounded-md">
                <template v-if="server.config.type === 'remote'">
                  <p class="break-all">
                    <span class="text-foreground">URL:</span> {{ server.config.url }}
                  </p>
                  <p v-if="server.config.headers && Object.keys(server.config.headers).length > 0">
                    <span class="text-foreground">Headers:</span>
                    {{ Object.keys(server.config.headers).length }} defined
                  </p>
                </template>
                <template v-else>
                  <p>
                    <span class="text-foreground">Command:</span> {{ server.config.command }}
                  </p>
                  <p v-if="server.config.args && server.config.args.length > 0" class="break-all">
                    <span class="text-foreground">Args:</span> {{ server.config.args.join(' ') }}
                  </p>
                  <p v-if="server.config.env && Object.keys(server.config.env).length > 0">
                    <span class="text-foreground">Env:</span>
                    {{ Object.keys(server.config.env).length }} variables
                  </p>
                </template>
              </div>

              <!-- Description if available -->
              <p v-if="server.description" class="text-sm text-muted-foreground">
                {{ server.description }}
              </p>
            </div>
          </div>
        </div>

        <!-- Submit Button -->
        <div class="flex justify-end pt-4">
          <Button @click="handleSubmit" :disabled="!canSubmit()">
            {{ isSubmitting ? 'Importing...' : `Import ${editableServers.length} Server${editableServers.length > 1 ? 's' : ''}` }}
          </Button>
        </div>
      </template>
    </div>
  </div>
</template>
