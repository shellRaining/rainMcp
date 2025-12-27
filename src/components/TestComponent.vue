<script setup lang="ts">
import { useAppStore } from '../stores/app';
import { logger } from '../utils/logger';

const appStore = useAppStore();

const handleClick = () => {
  appStore.setCurrentView('test');
  logger.info('View changed to: test');
};
</script>

<template>
  <div class="p-4 bg-blue-500 text-white rounded-lg">
    <h1 class="text-2xl font-bold mb-4">Configuration Test</h1>
    <p class="mb-2">Current View: {{ appStore.currentView }}</p>
    <button
      class="px-4 py-2 bg-white text-blue-500 rounded hover:bg-gray-100 transition"
      @click="handleClick"
    >
      Test Store & Logger
    </button>
  </div>
</template>

<style scoped></style>
