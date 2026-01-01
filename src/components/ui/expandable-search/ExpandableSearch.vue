<script setup lang="ts">
import { ref, nextTick, type ComponentPublicInstance } from 'vue';
import { onClickOutside } from '@vueuse/core';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Search, Loader2 } from 'lucide-vue-next';

const props = defineProps<{
  modelValue: string;
  placeholder?: string;
  isLoading?: boolean;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
}>();

const isExpanded = ref(false);
const searchInputRef = ref<ComponentPublicInstance | null>(null);
const containerRef = ref<HTMLElement | null>(null);

function toggleSearch() {
  if (isExpanded.value && !props.modelValue) {
    isExpanded.value = false;
  } else {
    isExpanded.value = true;
    nextTick(() => {
      // Access the underlying input element from Shadcn/Vue component
      const inputEl = searchInputRef.value?.$el?.nextElementSibling as HTMLInputElement || searchInputRef.value?.$el as HTMLInputElement;
      if (inputEl && typeof inputEl.focus === 'function') {
        inputEl.focus();
      } else {
         // Fallback if structure is different
         const input = searchInputRef.value?.$el?.querySelector('input');
         input?.focus();
      }
    });
  }
}

// Close search when clicking outside
onClickOutside(containerRef, () => {
  if (isExpanded.value && !props.modelValue) {
    isExpanded.value = false;
  }
});
</script>

<template>
  <div ref="containerRef" class="flex items-center justify-end">
    <div
      class="relative flex items-center transition-all duration-300 ease-in-out overflow-hidden"
      :class="isExpanded ? 'w-64' : 'w-8'"
    >
      <!-- Search Icon Button (visible when collapsed) -->
      <Button
        v-show="!isExpanded"
        variant="ghost"
        size="icon"
        class="h-8 w-8 absolute right-0 top-0 z-10"
        @click="toggleSearch"
      >
        <Search class="h-4 w-4 text-muted-foreground" />
      </Button>

      <!-- Search Input (visible when expanded) -->
      <div
        class="w-full relative transition-opacity duration-200"
        :class="isExpanded ? 'opacity-100' : 'opacity-0 pointer-events-none'"
      >
        <Search
          class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground pointer-events-none"
        />
        <Input
          ref="searchInputRef"
          :model-value="modelValue"
          @update:model-value="(v) => emit('update:modelValue', v as string)"
          :placeholder="placeholder || 'Search...'"
          class="pl-9 pr-9 w-full h-8 text-sm"
        />
        <Loader2
          v-if="isLoading"
          class="absolute right-3 top-1/2 -translate-y-1/2 h-4 w-4 animate-spin text-muted-foreground pointer-events-none"
        />
      </div>
    </div>
  </div>
</template>
