<script setup lang="ts">
import { Cog, Palette, Info, ChevronRight } from 'lucide-vue-next';
import { ScrollArea } from '@/components/ui/scroll-area';
import { useAppStore } from '@/stores/app';

const appStore = useAppStore();

const settingsItems = [
  {
    id: 'agent-management',
    label: 'Agent Management',
    description: 'Enable or disable AI coding agents',
    icon: Cog,
  },
  {
    id: 'theme',
    label: 'Theme',
    description: 'Customize appearance and color scheme',
    icon: Palette,
  },
  {
    id: 'about',
    label: 'About',
    description: 'Version info and links',
    icon: Info,
  },
];

function handleItemClick(id: string) {
  appStore.clickDetailItem('settings', id);
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Header -->
    <header class="shrink-0 h-13 px-6 flex items-center border-b" data-tauri-drag-region>
      <h1 class="text-lg font-semibold tracking-tight">Settings</h1>
    </header>

    <!-- Content -->
    <ScrollArea class="flex-1">
      <div class="p-6">
        <div class="space-y-2">
          <div
            v-for="item in settingsItems"
            :key="item.id"
            class="flex items-center gap-4 p-4 rounded-lg border bg-card hover:bg-accent/50 cursor-pointer transition-colors"
            @click="handleItemClick(item.id)"
          >
            <div class="p-2 rounded-md bg-accent">
              <component :is="item.icon" class="h-5 w-5 text-muted-foreground" />
            </div>
            <div class="flex-1">
              <p class="font-medium">{{ item.label }}</p>
              <p class="text-sm text-muted-foreground">{{ item.description }}</p>
            </div>
            <ChevronRight class="h-5 w-5 text-muted-foreground" />
          </div>
        </div>
      </div>
    </ScrollArea>
  </div>
</template>
