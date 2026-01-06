import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { UserServer, SchemaStore, ServerSchema } from '@/types/mcp';
import * as api from '@/api/tauri';
import { logger } from '@/utils/logger';

export const useServersStore = defineStore('servers', () => {
  // User servers state
  const userServers = ref<UserServer[]>([]);
  const selectedServerId = ref<string | null>(null);
  const isLoadingServers = ref(false);

  // Schema store state
  const schemaStore = ref<SchemaStore | null>(null);
  const isLoadingSchema = ref(false);
  const isRefreshingSchema = ref(false);
  const refreshProgress = ref<string | null>(null);

  // Computed
  const selectedServer = computed(() => {
    if (!selectedServerId.value) return null;
    return userServers.value.find((s) => s.id === selectedServerId.value) ?? null;
  });

  const serverSchemas = computed(() => schemaStore.value?.servers ?? []);

  const schemaLastUpdated = computed(() => schemaStore.value?.updated_at ?? null);

  // Get server type for display
  function getServerType(server: UserServer): 'remote' | 'custom' | 'registry' {
    if (server.config.type === 'remote') return 'remote';
    if (server.origin?.originType === 'registry') return 'registry';
    return 'custom';
  }

  // Get schema for a user server
  function getServerSchema(server: UserServer): ServerSchema | null {
    if (!server.origin?.schemaName || !schemaStore.value) return null;
    return schemaStore.value.servers.find((s) => s.name === server.origin?.schemaName) ?? null;
  }

  // Actions
  async function fetchUserServers() {
    isLoadingServers.value = true;
    try {
      userServers.value = await api.getUserServers();
    } catch (error) {
      logger.error('Failed to fetch user servers:', error);
    } finally {
      isLoadingServers.value = false;
    }
  }

  async function fetchSchemaStore() {
    isLoadingSchema.value = true;
    try {
      schemaStore.value = await api.getSchemaStore();
    } catch (error) {
      logger.error('Failed to fetch schema store:', error);
    } finally {
      isLoadingSchema.value = false;
    }
  }

  async function refreshSchemaStore() {
    isRefreshingSchema.value = true;
    refreshProgress.value = 'Connecting...';
    let unlisten: UnlistenFn | null = null;

    try {
      unlisten = await listen<{ page: number; total: number }>(
        'refresh-registry-progress',
        (event) => {
          const { page, total } = event.payload;
          refreshProgress.value = `Fetching page ${page} (${total} servers found)`;
        }
      );
      schemaStore.value = await api.refreshSchemaStore();
    } catch (error) {
      logger.error('Failed to refresh schema store:', error);
      throw error;
    } finally {
      if (unlisten) {
        unlisten();
      }
      isRefreshingSchema.value = false;
      refreshProgress.value = null;
    }
  }

  async function addServer(server: Omit<UserServer, 'id' | 'createdAt'>) {
    const newServer: UserServer = {
      ...server,
      id: crypto.randomUUID(),
    };
    const created = await api.addUserServer(newServer);
    userServers.value.push(created);
    return created;
  }

  async function updateServer(server: UserServer) {
    const updated = await api.updateUserServer(server);
    const index = userServers.value.findIndex((s) => s.id === server.id);
    if (index !== -1) {
      userServers.value[index] = updated;
    }
    return updated;
  }

  async function deleteServer(serverId: string) {
    await api.deleteUserServer(serverId);
    userServers.value = userServers.value.filter((s) => s.id !== serverId);
    if (selectedServerId.value === serverId) {
      selectedServerId.value = null;
    }
  }

  async function deployToAgent(agentName: string, serverId: string, serverName?: string) {
    await api.addServerToAgent(agentName, serverId, serverName);
  }

  function selectServer(serverId: string | null) {
    selectedServerId.value = serverId;
  }

  // Initialize
  async function init() {
    await Promise.all([fetchUserServers(), fetchSchemaStore()]);
  }

  return {
    // State
    userServers,
    selectedServerId,
    selectedServer,
    isLoadingServers,
    schemaStore,
    serverSchemas,
    schemaLastUpdated,
    isLoadingSchema,
    isRefreshingSchema,
    refreshProgress,
    // Actions
    fetchUserServers,
    fetchSchemaStore,
    refreshSchemaStore,
    addServer,
    updateServer,
    deleteServer,
    deployToAgent,
    selectServer,
    init,
    // Helpers
    getServerType,
    getServerSchema,
  };
});
