<script setup lang="ts">
import { computed } from 'vue';
import Sidebar from './Sidebar.vue';
import OverviewView from '@/views/overview/OverviewView.vue';
import AgentsOverviewView from '@/views/agents/AgentsOverviewView.vue';
import AgentDetail from '@/components/mcp/AgentDetail.vue';
import ServersOverviewView from '@/views/servers/ServersOverviewView.vue';
import ServerDetailView from '@/views/servers/ServerDetailView.vue';
import SettingsOverviewView from '@/views/settings/SettingsOverviewView.vue';
import AgentManagementView from '@/views/settings/AgentManagementView.vue';
import ThemeView from '@/views/settings/ThemeView.vue';
import AboutView from '@/views/settings/AboutView.vue';
import { useAppStore } from '@/stores/app';

const appStore = useAppStore();

// 根据当前状态决定显示哪个组件
const currentComponent = computed(() => {
  const { currentPrimaryMenu, selectedDetailId, isViewingDetail } = appStore;

  if (currentPrimaryMenu === 'overview') {
    return OverviewView;
  }

  if (currentPrimaryMenu === 'agents') {
    return isViewingDetail ? AgentDetail : AgentsOverviewView;
  }

  if (currentPrimaryMenu === 'servers') {
    return isViewingDetail ? ServerDetailView : ServersOverviewView;
  }

  if (currentPrimaryMenu === 'settings') {
    if (!isViewingDetail) return SettingsOverviewView;
    switch (selectedDetailId) {
      case 'agent-management':
        return AgentManagementView;
      case 'theme':
        return ThemeView;
      case 'about':
        return AboutView;
      default:
        return SettingsOverviewView;
    }
  }

  return OverviewView;
});

// 用于 Transition 的 key
const viewKey = computed(() => {
  const { currentPrimaryMenu, selectedDetailId, isViewingDetail } = appStore;
  if (isViewingDetail) {
    return `${currentPrimaryMenu}-detail-${selectedDetailId}`;
  }
  return `${currentPrimaryMenu}-overview`;
});
</script>

<template>
  <div class="h-screen w-screen flex bg-background overflow-hidden">
    <Sidebar />
    <main class="flex-1 overflow-hidden relative">
      <Transition name="fade" mode="out-in">
        <component :is="currentComponent" :key="viewKey" />
      </Transition>
    </main>
  </div>
</template>
