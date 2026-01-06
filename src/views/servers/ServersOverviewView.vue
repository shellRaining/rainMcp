<script setup lang="ts">
import { onMounted } from 'vue';
import { Server, Plus, RefreshCw, Globe, Terminal, Package } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { ScrollArea } from '@/components/ui/scroll-area';
import { useServersStore } from '@/stores/servers';
import { useAppStore } from '@/stores/app';
import { useAddServerWindow } from '@/composables/useAddServerWindow';
import { logger } from '@/utils/logger';

const serversStore = useServersStore();
const appStore = useAppStore();
const { openAddServerWindow } = useAddServerWindow();

onMounted(async () => {
  await serversStore.init();
});

function handleServerClick(serverId: string) {
  serversStore.selectServer(serverId);
  appStore.clickDetailItem('servers', serverId);
}

function getServerTypeIcon(server: (typeof serversStore.userServers)[0]) {
  const type = serversStore.getServerType(server);
  switch (type) {
    case 'remote':
      return Globe;
    case 'custom':
      return Terminal;
    default:
      return Package;
  }
}

function getServerTypeLabel(server: (typeof serversStore.userServers)[0]) {
  const type = serversStore.getServerType(server);
  switch (type) {
    case 'remote':
      return 'Remote';
    case 'custom':
      return 'Custom';
    default:
      return 'Registry';
  }
}

async function handleRefreshSchema() {
  try {
    await serversStore.refreshSchemaStore();
  } catch (error) {
    logger.error('Failed to refresh schema:', error);
  }
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Header -->
    <header
      class="shrink-0 h-13 px-6 flex items-center justify-between border-b"
      data-tauri-drag-region
    >
      <h1 class="text-lg font-semibold tracking-tight">Servers</h1>
      <div class="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          :disabled="serversStore.isRefreshingSchema"
          @click="handleRefreshSchema"
        >
          <RefreshCw
            class="h-4 w-4 mr-2"
            :class="{ 'animate-spin': serversStore.isRefreshingSchema }"
          />
          {{ serversStore.refreshProgress ?? 'Refresh Registry' }}
        </Button>
        <Button size="sm" @click="openAddServerWindow">
          <Plus class="h-4 w-4 mr-2" />
          Add Server
        </Button>
      </div>
    </header>

    <!-- Content -->
    <ScrollArea class="flex-1">
      <div class="p-6">
        <!-- Loading State -->
        <div v-if="serversStore.isLoadingServers" class="py-12 text-center text-muted-foreground">
          <RefreshCw class="h-8 w-8 mx-auto animate-spin opacity-50" />
          <p class="mt-3">Loading servers...</p>
        </div>

        <!-- Empty State -->
        <div
          v-else-if="serversStore.userServers.length === 0"
          class="py-12 text-center text-muted-foreground"
        >
          <Server class="h-12 w-12 mx-auto opacity-50" />
          <p class="mt-3 text-lg">No servers configured</p>
          <p class="text-sm mt-1">Add a server from the registry or create a custom one</p>
          <Button class="mt-4" @click="openAddServerWindow">
            <Plus class="h-4 w-4 mr-2" />
            Add Server
          </Button>
        </div>

        <!-- Server Cards -->
        <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
          <div
            v-for="server in serversStore.userServers"
            :key="server.id"
            class="server-card px-4 py-3 rounded-lg border bg-card hover:bg-accent/50 cursor-pointer transition-colors"
            @click="handleServerClick(server.id)"
          >
            <div class="flex items-center gap-3">
              <!-- Icon -->
              <div class="shrink-0 w-10 h-10 rounded-md bg-muted flex items-center justify-center">
                <component :is="getServerTypeIcon(server)" class="h-5 w-5 text-muted-foreground" />
              </div>
              <!-- Info -->
              <div class="min-w-0 flex-1">
                <h3 class="font-medium truncate">{{ server.name }}</h3>
                <div class="flex items-center gap-2 mt-0.5">
                  <Badge variant="secondary" class="text-xs">
                    {{ getServerTypeLabel(server) }}
                  </Badge>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Schema Store Info -->
        <div
          v-if="serversStore.schemaLastUpdated"
          class="mt-6 pt-4 border-t text-xs text-muted-foreground"
        >
          Registry last updated:
          {{ new Date(serversStore.schemaLastUpdated).toLocaleString() }}
          ({{ serversStore.serverSchemas.length }} servers available)
        </div>
      </div>
    </ScrollArea>
  </div>
</template>

<style scoped>
.server-card {
  container-type: inline-size;
}
</style>
