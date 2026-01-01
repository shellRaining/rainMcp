<script setup lang="ts">
import { useRoute } from 'vue-router';
import { TooltipProvider } from '@/components/ui/tooltip';
import MainLayout from '@/components/layout/MainLayout.vue';

const route = useRoute();
</script>

<template>
  <TooltipProvider>
    <!-- 独立窗口路由直接显示 -->
    <router-view v-if="route.name === 'add-server'" />

    <!-- 主应用显示 MainLayout -->
    <MainLayout v-else />
  </TooltipProvider>
</template>
