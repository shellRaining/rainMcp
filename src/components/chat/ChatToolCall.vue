<script setup lang="ts">
import { ref, computed } from 'vue';
import { Loader2, Check, ChevronDown, Globe, FileCode } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';

export interface ToolCallData {
  name: string;
  status: 'running' | 'success';
  argsPreview?: string;
  resultPreview?: string;
}

const props = defineProps<{
  toolCall: ToolCallData;
  showUseButton?: boolean;
  useButtonDisabled?: boolean;
}>();

const emit = defineEmits<{
  use: [];
}>();

const isOpen = ref(false);

function extractUrl(args?: string): string | null {
  if (!args) return null;
  try {
    const parsed = JSON.parse(args);
    return parsed.url || null;
  } catch {
    const match = args.match(/"url"\s*:\s*"([^"]+)"/);
    return match ? match[1] : null;
  }
}

function formatJson(str?: string): string {
  if (!str) return '';
  try {
    const parsed = JSON.parse(str);
    return JSON.stringify(parsed, null, 2);
  } catch {
    return str;
  }
}

const url = computed(() => extractUrl(props.toolCall.argsPreview));
const hasDetails = computed(() => !!(props.toolCall.argsPreview || props.toolCall.resultPreview));
const formattedArgs = computed(() => formatJson(props.toolCall.argsPreview));
const formattedResult = computed(() => formatJson(props.toolCall.resultPreview));
const isGenerateSchema = computed(() => props.toolCall.name === 'generate_server_schema');

function toggle() {
  if (hasDetails.value) {
    isOpen.value = !isOpen.value;
  }
}
</script>

<template>
  <div class="rounded border bg-background/60 overflow-hidden">
    <div class="flex items-center">
      <!-- Left: Collapsible trigger -->
      <button
        type="button"
        class="flex items-center gap-2 flex-1 p-2 text-xs hover:bg-muted/50 transition-colors text-left min-w-0"
        @click="toggle"
      >
        <Loader2
          v-if="toolCall.status === 'running'"
          class="h-3 w-3 animate-spin text-muted-foreground flex-shrink-0"
        />
        <Check v-else class="h-3 w-3 text-green-500 flex-shrink-0" />

        <Globe
          v-if="toolCall.name === 'fetch_webpage'"
          class="h-3 w-3 text-blue-500 flex-shrink-0"
        />
        <FileCode v-else-if="isGenerateSchema" class="h-3 w-3 text-purple-500 flex-shrink-0" />

        <span class="font-mono font-medium truncate">{{ toolCall.name }}</span>

        <span v-if="url" class="text-muted-foreground truncate">
          {{ url }}
        </span>

        <span v-if="toolCall.status === 'running'" class="text-muted-foreground flex-shrink-0">
          {{ isGenerateSchema ? 'Generating...' : 'Loading...' }}
        </span>

        <div class="flex-1" />

        <ChevronDown
          v-if="hasDetails"
          class="h-3 w-3 text-muted-foreground chevron-icon flex-shrink-0"
          :class="{ expanded: isOpen }"
        />
      </button>

      <!-- Right: Use button -->
      <div
        v-if="showUseButton && isGenerateSchema && toolCall.status === 'success'"
        class="border-l px-2"
      >
        <Button
          size="sm"
          variant="default"
          class="h-6 px-2 text-xs"
          :disabled="useButtonDisabled"
          @click="emit('use')"
        >
          Use
        </Button>
      </div>
    </div>

    <div v-if="hasDetails" class="grid-collapse" :class="{ expanded: isOpen }">
      <div class="overflow-hidden">
        <div class="px-2 pb-2 space-y-2">
          <div v-if="toolCall.argsPreview" class="space-y-1">
            <div class="text-[10px] uppercase tracking-wide text-muted-foreground">Arguments</div>
            <pre
              class="text-xs bg-muted p-2 rounded overflow-x-auto max-h-32 select-text whitespace-pre-wrap break-all"
              >{{ formattedArgs }}</pre
            >
          </div>
          <div v-if="toolCall.resultPreview" class="space-y-1">
            <div class="text-[10px] uppercase tracking-wide text-muted-foreground">Result</div>
            <pre
              class="text-xs bg-muted p-2 rounded overflow-x-auto max-h-48 select-text whitespace-pre-wrap break-all"
              >{{ formattedResult }}</pre
            >
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.grid-collapse {
  display: grid;
  grid-template-rows: 0fr;
  transition: grid-template-rows 200ms ease-out;
}

.grid-collapse.expanded {
  grid-template-rows: 1fr;
}

.grid-collapse > div {
  overflow: hidden;
}

.chevron-icon {
  transition: transform 200ms ease-out;
}

.chevron-icon.expanded {
  transform: rotate(-180deg);
}
</style>
