<script setup lang="ts">
import { ref } from 'vue';
import { Sparkles, ChevronDown } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import type { GeneratedSchema } from '@/api/tauri';

defineProps<{
  schema: GeneratedSchema;
  disabled?: boolean;
}>();

const emit = defineEmits<{
  confirm: [];
}>();

const isOpen = ref(false);

function toggle() {
  isOpen.value = !isOpen.value;
}
</script>

<template>
  <div class="mt-3 bg-background rounded border select-text overflow-hidden">
    <button
      type="button"
      class="flex items-center gap-2 w-full p-3 text-left hover:bg-muted/50 transition-colors"
      @click="toggle"
    >
      <Sparkles class="h-4 w-4 text-primary flex-shrink-0" />
      <span class="font-medium text-sm">Generated Schema</span>
      <span class="text-xs text-muted-foreground truncate">{{ schema.schema.name }}</span>
      <div class="flex-1" />
      <ChevronDown
        class="h-4 w-4 text-muted-foreground chevron-icon"
        :class="{ expanded: isOpen }"
      />
    </button>

    <div class="grid-collapse" :class="{ expanded: isOpen }">
      <div class="overflow-hidden">
        <div class="px-3 pb-3 space-y-2">
          <div class="text-xs text-muted-foreground">
            {{ schema.explanation }}
          </div>

          <div class="flex items-center gap-2">
            <span class="text-xs text-muted-foreground">Name:</span>
            <span class="text-xs font-medium">{{ schema.schema.name }}</span>
          </div>

          <div v-if="schema.schema.title" class="flex items-center gap-2">
            <span class="text-xs text-muted-foreground">Title:</span>
            <span class="text-xs font-medium">{{ schema.schema.title }}</span>
          </div>

          <pre class="text-xs bg-muted p-2 rounded overflow-x-auto max-h-48 select-text">{{
            JSON.stringify(schema.schema, null, 2)
          }}</pre>
        </div>
      </div>
    </div>

    <div class="px-3 pb-3">
      <Button class="w-full" size="sm" :disabled="disabled" @click="emit('confirm')">
        Use This Schema
      </Button>
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
