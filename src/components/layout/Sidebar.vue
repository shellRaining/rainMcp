<script setup lang="ts">
import { onMounted } from 'vue';
import {
  LayoutDashboard,
  Bot,
  Server,
  Settings,
  ChevronRight,
  Cog,
  Palette,
  Info,
} from 'lucide-vue-next';
import { cn } from '@/lib/utils';
import { Separator } from '@/components/ui/separator';
import { useAgentsStore } from '@/stores/agents';
import { useServersStore } from '@/stores/servers';
import { useAppStore, type PrimaryMenu } from '@/stores/app';
import { AGENT_DISPLAY_NAMES } from '@/types/mcp';

const agentsStore = useAgentsStore();
const serversStore = useServersStore();
const appStore = useAppStore();

onMounted(() => {
  agentsStore.fetchAgents();
  serversStore.fetchUserServers();
});

const primaryMenuItems = [
  { id: 'overview' as PrimaryMenu, label: 'Overview', icon: LayoutDashboard, hasSubmenu: false },
  { id: 'agents' as PrimaryMenu, label: 'Agents', icon: Bot, hasSubmenu: true },
  { id: 'servers' as PrimaryMenu, label: 'Servers', icon: Server, hasSubmenu: true },
  { id: 'settings' as PrimaryMenu, label: 'Settings', icon: Settings, hasSubmenu: true },
];

const settingsSubmenuItems = [
  { id: 'agent-management', label: 'Agent Management', icon: Cog },
  { id: 'theme', label: 'Theme', icon: Palette },
  { id: 'about', label: 'About', icon: Info },
];

function handlePrimaryClick(item: (typeof primaryMenuItems)[0]) {
  appStore.clickPrimaryMenu(item.id);
}

function handleAgentClick(agentName: string) {
  agentsStore.selectAgent(agentName);
  appStore.clickDetailItem('agents', agentName);
}

function handleSettingsSubmenuClick(submenuId: string) {
  appStore.clickDetailItem('settings', submenuId);
}

function handleServerClick(serverId: string) {
  serversStore.selectServer(serverId);
  appStore.clickDetailItem('servers', serverId);
}
</script>

<template>
  <div class="h-full flex flex-col bg-card border-r w-56">
    <!-- Header: 拖拽区域 + 红绿灯空间 -->
    <div class="h-13 shrink-0 flex items-end h-8" data-tauri-drag-region />

    <Separator style="background-color: hsl(var(--border))" />

    <!-- 导航菜单 -->
    <nav class="flex-1 overflow-y-auto p-2 space-y-1">
      <template v-for="item in primaryMenuItems" :key="item.id">
        <!-- 一级菜单项 -->
        <button
          :class="
            cn(
              'w-full flex items-center gap-2 px-3 py-2 rounded-md text-sm',
              'transition-colors duration-[var(--duration-fast)]',
              'hover:bg-accent',
              appStore.currentPrimaryMenu === item.id && !appStore.isViewingDetail && 'bg-accent',
              appStore.isMenuExpanded(item.id) && appStore.isViewingDetail && 'bg-accent/50'
            )
          "
          @click="handlePrimaryClick(item)"
        >
          <component :is="item.icon" class="h-4 w-4 text-muted-foreground" />
          <span class="flex-1 text-left">{{ item.label }}</span>
          <ChevronRight
            v-if="item.hasSubmenu"
            :class="
              cn(
                'h-4 w-4 text-muted-foreground',
                'transition-transform duration-[var(--duration-normal)] ease-[var(--ease-out)]',
                appStore.isMenuExpanded(item.id) && 'rotate-90'
              )
            "
          />
        </button>

        <!-- 二级菜单：Agents -->
        <Transition name="collapse">
          <div v-if="item.id === 'agents' && appStore.isMenuExpanded('agents')" class="grid">
            <div class="overflow-hidden pl-4 space-y-0.5">
              <div
                v-if="agentsStore.enabledAgents.length === 0"
                class="px-3 py-2 text-xs text-muted-foreground"
              >
                No enabled agents
              </div>
              <TransitionGroup name="list">
                <button
                  v-for="agent in agentsStore.enabledAgents"
                  :key="agent.name"
                  :class="
                    cn(
                      'w-full flex items-center gap-2 px-3 py-1.5 rounded-md text-sm',
                      'transition-colors duration-[var(--duration-fast)]',
                      'hover:bg-accent',
                      appStore.selectedDetailId === agent.name &&
                        appStore.currentPrimaryMenu === 'agents' &&
                        'bg-accent text-accent-foreground'
                    )
                  "
                  @click="handleAgentClick(agent.name)"
                >
                  <span class="truncate">{{
                    AGENT_DISPLAY_NAMES[agent.agent_type] || agent.name
                  }}</span>
                </button>
              </TransitionGroup>
            </div>
          </div>
        </Transition>

        <!-- 二级菜单：Servers -->
        <Transition name="collapse">
          <div v-if="item.id === 'servers' && appStore.isMenuExpanded('servers')" class="grid">
            <div class="overflow-hidden pl-4 space-y-0.5">
              <div
                v-if="serversStore.userServers.length === 0"
                class="px-3 py-2 text-xs text-muted-foreground"
              >
                No servers configured
              </div>
              <TransitionGroup name="list">
                <button
                  v-for="server in serversStore.userServers"
                  :key="server.id"
                  :class="
                    cn(
                      'w-full flex items-center gap-2 px-3 py-1.5 rounded-md text-sm',
                      'transition-colors duration-[var(--duration-fast)]',
                      'hover:bg-accent',
                      appStore.selectedDetailId === server.id &&
                        appStore.currentPrimaryMenu === 'servers' &&
                        'bg-accent text-accent-foreground'
                    )
                  "
                  @click="handleServerClick(server.id)"
                >
                  <span class="truncate">{{ server.name }}</span>
                </button>
              </TransitionGroup>
            </div>
          </div>
        </Transition>

        <!-- 二级菜单：Settings -->
        <Transition name="collapse">
          <div v-if="item.id === 'settings' && appStore.isMenuExpanded('settings')" class="grid">
            <div class="overflow-hidden pl-4 space-y-0.5">
              <TransitionGroup name="list">
                <button
                  v-for="submenu in settingsSubmenuItems"
                  :key="submenu.id"
                  :class="
                    cn(
                      'w-full flex items-center gap-2 px-3 py-1.5 rounded-md text-sm',
                      'transition-colors duration-[var(--duration-fast)]',
                      'hover:bg-accent',
                      appStore.selectedDetailId === submenu.id &&
                        appStore.currentPrimaryMenu === 'settings' &&
                        'bg-accent text-accent-foreground'
                    )
                  "
                  @click="handleSettingsSubmenuClick(submenu.id)"
                >
                  <component :is="submenu.icon" class="h-3.5 w-3.5 text-muted-foreground" />
                  <span>{{ submenu.label }}</span>
                </button>
              </TransitionGroup>
            </div>
          </div>
        </Transition>
      </template>
    </nav>

    <Separator style="background-color: hsl(var(--border))" />

    <!-- Footer：暂时置空 -->
    <div class="h-10 shrink-0" />
  </div>
</template>
