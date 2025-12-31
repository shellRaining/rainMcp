<script setup lang="ts">
import { Moon, Sun, Monitor } from 'lucide-vue-next';
import { cn } from '@/lib/utils';
import { useThemeStore, type Theme } from '@/stores/theme';

const themeStore = useThemeStore();

const themeOptions: { value: Theme; label: string; icon: typeof Sun }[] = [
  { value: 'light', label: 'Light', icon: Sun },
  { value: 'dark', label: 'Dark', icon: Moon },
  { value: 'system', label: 'System', icon: Monitor },
];

function selectTheme(theme: Theme) {
  themeStore.setTheme(theme);
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Header -->
    <header class="shrink-0 h-13 px-6 flex items-center border-b" data-tauri-drag-region>
      <h1 class="text-lg font-semibold tracking-tight">Theme</h1>
    </header>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto p-6">
      <div class="max-w-md space-y-4">
        <p class="text-sm text-muted-foreground">Choose your preferred color theme.</p>

        <div class="grid grid-cols-3 gap-3">
          <button
            v-for="option in themeOptions"
            :key="option.value"
            :class="
              cn(
                'flex flex-col items-center gap-2 p-4 rounded-lg border transition-colors',
                'hover:bg-accent',
                themeStore.theme === option.value && 'border-primary bg-accent'
              )
            "
            @click="selectTheme(option.value)"
          >
            <component :is="option.icon" class="h-5 w-5" />
            <span class="text-sm">{{ option.label }}</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
