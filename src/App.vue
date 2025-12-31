<script setup lang="ts">
import { TooltipProvider } from '@/components/ui/tooltip';
import MainLayout from '@/components/layout/MainLayout.vue';
</script>

<template>
  <TooltipProvider>
    <MainLayout />
  </TooltipProvider>
</template>
