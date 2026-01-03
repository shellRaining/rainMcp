<script setup lang="ts">
import { Send, Clock } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';

const model = defineModel<string>({ required: true });

defineProps<{
  isLoading?: boolean;
  queuedCount?: number;
  error?: string | null;
}>();

const emit = defineEmits<{
  submit: [];
}>();

function handleKeyDown(event: KeyboardEvent) {
  if (event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault();
    emit('submit');
  }
}
</script>

<template>
  <div class="border-t p-4">
    <div v-if="error" class="mb-2 text-sm text-destructive">
      {{ error }}
    </div>
    <div
      v-if="queuedCount && queuedCount > 0"
      class="mb-2 flex items-center gap-1.5 text-xs text-muted-foreground"
    >
      <Clock class="h-3 w-3" />
      <span>{{ queuedCount }} message{{ queuedCount > 1 ? 's' : '' }} queued</span>
    </div>
    <form class="flex gap-2" @submit.prevent="emit('submit')">
      <Input
        v-model="model"
        placeholder="Describe what you want to configure..."
        class="flex-1"
        @keydown="handleKeyDown"
      />
      <Button type="submit" :disabled="!model.trim()">
        <Send class="h-4 w-4" />
      </Button>
    </form>
  </div>
</template>
