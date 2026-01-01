<script setup lang="ts">
import { ExternalLink } from 'lucide-vue-next';
import { openUrl } from '@tauri-apps/plugin-opener';
import { logger } from '@/utils/logger';

async function openGitHub() {
  try {
    await openUrl('https://github.com/shellraining/rainMcp');
  } catch (error) {
    logger.error('Failed to open URL:', error);
  }
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Header -->
    <header class="shrink-0 h-13 px-6 flex items-center border-b" data-tauri-drag-region>
      <h1 class="text-lg font-semibold tracking-tight">About</h1>
    </header>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto p-6">
      <div class="max-w-md space-y-6">
        <div class="space-y-2">
          <h2 class="text-2xl font-bold">Rain MCP</h2>
          <p class="text-sm text-muted-foreground">MCP Configuration Manager</p>
        </div>

        <div class="space-y-1 text-sm">
          <p>
            <span class="text-muted-foreground">Version:</span>
            <span class="ml-2 font-mono">0.1.0</span>
          </p>
        </div>

        <div class="pt-4 border-t space-y-2">
          <p class="text-sm text-muted-foreground">
            A desktop application for managing MCP (Model Context Protocol) configurations across
            different AI coding agents.
          </p>
        </div>

        <div class="flex gap-2">
          <span
            class="inline-flex items-center gap-1 text-sm text-primary hover:underline cursor-pointer"
            @click="openGitHub"
          >
            <ExternalLink class="h-3.5 w-3.5" />
            GitHub
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
