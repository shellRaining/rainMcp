<script setup lang="ts">
import { Server } from 'lucide-vue-next';
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Header -->
    <header class="shrink-0 h-13 px-6 flex items-center border-b" data-tauri-drag-region>
      <h1 class="text-lg font-semibold tracking-tight">Servers</h1>
    </header>

    <!-- Placeholder -->
    <div class="flex-1 flex items-center justify-center text-muted-foreground">
      <div class="text-center space-y-2">
        <Server class="h-12 w-12 mx-auto opacity-50" />
        <p>Coming soon...</p>
      </div>
    </div>
  </div>
</template>
