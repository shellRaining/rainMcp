<script setup lang="ts">
import { computed, ref } from 'vue';
import { Server, Globe, Terminal, Package, Trash2, Send, Copy, Check } from 'lucide-vue-next';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { Button } from '@/components/ui/button';
import { Card } from '@/components/ui/card';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Separator } from '@/components/ui/separator';
import { Label } from '@/components/ui/label';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog';
import { useServersStore } from '@/stores/servers';
import { useAgentsStore } from '@/stores/agents';
import { useAppStore } from '@/stores/app';
import { logger } from '@/utils/logger';

const serversStore = useServersStore();
const agentsStore = useAgentsStore();
const appStore = useAppStore();

const isDeleting = ref(false);
const isDeploying = ref(false);
const isCopied = ref(false);
const isSaving = ref(false);

const server = computed(() => serversStore.selectedServer);

const serverType = computed(() => {
  if (!server.value) return null;
  return serversStore.getServerType(server.value);
});

const serverTypeIcon = computed(() => {
  switch (serverType.value) {
    case 'remote':
      return Globe;
    case 'custom':
      return Terminal;
    default:
      return Package;
  }
});

const serverTypeLabel = computed(() => {
  switch (serverType.value) {
    case 'remote':
      return 'Remote';
    case 'custom':
      return 'Custom Command';
    default:
      return 'Registry';
  }
});

function parseEnvString(envStr: string): Record<string, string> | null {
  const result: Record<string, string> = {};
  for (const line of envStr.split('\n')) {
    const trimmed = line.trim();
    if (!trimmed) continue;
    const idx = trimmed.indexOf('=');
    if (idx > 0) {
      result[trimmed.slice(0, idx).trim()] = trimmed.slice(idx + 1).trim();
    }
  }
  return Object.keys(result).length > 0 ? result : null;
}

function parseHeadersString(headersStr: string): Record<string, string> | null {
  const result: Record<string, string> = {};
  for (const line of headersStr.split('\n')) {
    const trimmed = line.trim();
    if (!trimmed) continue;
    const idx = trimmed.indexOf(':');
    if (idx > 0) {
      result[trimmed.slice(0, idx).trim()] = trimmed.slice(idx + 1).trim();
    }
  }
  return Object.keys(result).length > 0 ? result : null;
}

function envToString(env: Record<string, string> | null | undefined): string {
  if (!env) return '';
  return Object.entries(env)
    .map(([k, v]) => `${k}=${v}`)
    .join('\n');
}

function headersToString(headers: Record<string, string> | null | undefined): string {
  if (!headers) return '';
  return Object.entries(headers)
    .map(([k, v]) => `${k}: ${v}`)
    .join('\n');
}

async function saveServer() {
  if (!server.value) return;
  isSaving.value = true;
  try {
    await serversStore.updateServer(server.value);
  } catch (error) {
    logger.error('Failed to save server:', error);
  } finally {
    isSaving.value = false;
  }
}

function updateServerName(value: string | number) {
  if (!server.value) return;
  server.value.name = String(value);
}

function updateRemoteUrl(value: string | number) {
  if (!server.value || server.value.config.type !== 'remote') return;
  server.value.config.url = String(value);
}

function updateRemoteHeaders(value: string | number) {
  if (!server.value || server.value.config.type !== 'remote') return;
  server.value.config.headers = parseHeadersString(String(value));
}

function updateLocalCommand(value: string | number) {
  if (!server.value || server.value.config.type !== 'local') return;
  server.value.config.command = String(value);
}

function updateLocalArgs(value: string | number) {
  if (!server.value || server.value.config.type !== 'local') return;
  const args = String(value).split(/\s+/).filter(Boolean);
  server.value.config.args = args.length > 0 ? args : null;
}

function updateLocalEnv(value: string | number) {
  if (!server.value || server.value.config.type !== 'local') return;
  server.value.config.env = parseEnvString(String(value));
}

async function handleDelete() {
  if (!server.value) return;
  isDeleting.value = true;
  try {
    await serversStore.deleteServer(server.value.id);
    appStore.clickPrimaryMenu('servers');
  } catch (error) {
    logger.error('Failed to delete server:', error);
  } finally {
    isDeleting.value = false;
  }
}

async function handleDeploy(agentName: string) {
  if (!server.value) return;
  isDeploying.value = true;
  try {
    await serversStore.deployToAgent(agentName, server.value.id);
    // Refresh agents to update their config
    await agentsStore.fetchAgents();
  } catch (error) {
    logger.error('Failed to deploy server:', error);
  } finally {
    isDeploying.value = false;
  }
}

async function handleCopyConfig() {
  if (!server.value) return;
  try {
    await writeText(JSON.stringify(server.value, null, 2));
    isCopied.value = true;
    setTimeout(() => {
      isCopied.value = false;
    }, 2000);
  } catch (error) {
    logger.error('Failed to copy config:', error);
  }
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Empty State -->
    <div v-if="!server" class="flex-1 flex items-center justify-center text-muted-foreground">
      <div class="absolute top-0 left-0 right-0 h-13" data-tauri-drag-region />
      <div class="text-center space-y-2">
        <Server class="h-12 w-12 mx-auto opacity-50" />
        <p>Select a server to view details</p>
      </div>
    </div>

    <!-- Server Detail -->
    <template v-else>
      <!-- Header -->
      <header class="shrink-0 px-6 py-4 border-b" data-tauri-drag-region>
        <div class="flex items-center justify-between gap-3">
          <div class="flex items-center gap-3 min-w-0 flex-1">
            <Tooltip>
              <TooltipTrigger as-child>
                <div
                  class="w-10 h-10 rounded-lg bg-muted flex items-center justify-center shrink-0 cursor-help"
                >
                  <component :is="serverTypeIcon" class="h-5 w-5 text-muted-foreground" />
                </div>
              </TooltipTrigger>
              <TooltipContent class="text-xs">
                {{ serverTypeLabel }}
              </TooltipContent>
            </Tooltip>
            <div class="min-w-0 flex-1">
              <Input
                :model-value="server.name"
                @update:model-value="updateServerName"
                @blur="saveServer"
                class="h-8 text-lg font-semibold tracking-tight px-0 border-0 focus-visible:ring-0 bg-transparent"
              />
            </div>
          </div>
          <div class="flex items-center gap-2 shrink-0">
            <Button variant="outline" size="sm" @click="handleCopyConfig">
              <Check v-if="isCopied" class="h-4 w-4 mr-2 text-green-500" />
              <Copy v-else class="h-4 w-4 mr-2" />
              Copy
            </Button>
            <AlertDialog>
              <AlertDialogTrigger as-child>
                <Button variant="outline" size="sm">
                  <Trash2 class="h-4 w-4 mr-2" />
                  Delete
                </Button>
              </AlertDialogTrigger>
              <AlertDialogContent>
                <AlertDialogHeader>
                  <AlertDialogTitle>Delete Server</AlertDialogTitle>
                  <AlertDialogDescription>
                    Are you sure you want to delete "{{ server.name }}"? This action cannot be
                    undone.
                  </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                  <AlertDialogCancel>Cancel</AlertDialogCancel>
                  <AlertDialogAction :disabled="isDeleting" @click="handleDelete">
                    {{ isDeleting ? 'Deleting...' : 'Delete' }}
                  </AlertDialogAction>
                </AlertDialogFooter>
              </AlertDialogContent>
            </AlertDialog>
          </div>
        </div>
      </header>

      <!-- Content -->
      <ScrollArea class="flex-1">
        <div class="p-6 space-y-6">
          <!-- Server Info -->
          <Card class="p-4">
            <h2 class="text-sm font-medium mb-3">Configuration</h2>
            <div class="space-y-3">
              <!-- Remote URL -->
              <div v-if="server.config.type === 'remote'" class="space-y-1">
                <Label for="remote-url">URL</Label>
                <Input
                  id="remote-url"
                  :model-value="server.config.url"
                  @update:model-value="updateRemoteUrl"
                  @blur="saveServer"
                  class="font-mono text-xs"
                  placeholder="https://api.example.com/mcp"
                />
              </div>

              <!-- Local Command -->
              <div v-if="server.config.type === 'local'" class="space-y-1">
                <Label for="local-command">Command</Label>
                <Input
                  id="local-command"
                  :model-value="server.config.command"
                  @update:model-value="updateLocalCommand"
                  @blur="saveServer"
                  class="font-mono text-xs"
                  placeholder="node"
                />
              </div>

              <!-- Local Args -->
              <div v-if="server.config.type === 'local'" class="space-y-1">
                <Label for="local-args">Arguments</Label>
                <Input
                  id="local-args"
                  :model-value="server.config.args?.join(' ') || ''"
                  @update:model-value="updateLocalArgs"
                  @blur="saveServer"
                  class="font-mono text-xs"
                  placeholder="server.js --port 3000"
                />
              </div>

              <!-- Schema Info (from origin) -->
              <div v-if="server.origin?.schemaName" class="space-y-1">
                <p class="text-xs text-muted-foreground">Schema</p>
                <code
                  class="block text-xs bg-muted px-3 py-2 rounded-md font-mono break-all selectable"
                >
                  {{ server.origin.schemaName }}
                </code>
              </div>

              <!-- Package ID (from origin) -->
              <div v-if="server.origin?.packageId" class="space-y-1">
                <p class="text-xs text-muted-foreground">Package</p>
                <code
                  class="block text-xs bg-muted px-3 py-2 rounded-md font-mono break-all selectable"
                >
                  {{ server.origin.packageId }}
                </code>
              </div>

              <!-- Environment Variables (for local) -->
              <div v-if="server.config.type === 'local'" class="space-y-1">
                <Label for="local-env">Environment Variables</Label>
                <Textarea
                  id="local-env"
                  :model-value="envToString(server.config.env)"
                  @update:model-value="updateLocalEnv"
                  @blur="saveServer"
                  placeholder="API_KEY=your-key&#10;DEBUG=true"
                  rows="4"
                  class="font-mono text-xs"
                />
              </div>

              <!-- Headers (for remote) -->
              <div v-if="server.config.type === 'remote'" class="space-y-1">
                <Label for="remote-headers">Headers</Label>
                <Textarea
                  id="remote-headers"
                  :model-value="headersToString(server.config.headers)"
                  @update:model-value="updateRemoteHeaders"
                  @blur="saveServer"
                  placeholder="Authorization: Bearer your-token&#10;X-API-Key: your-api-key"
                  rows="4"
                  class="font-mono text-xs"
                />
              </div>

              <!-- Created At -->
              <div v-if="server.createdAt" class="space-y-1">
                <p class="text-xs text-muted-foreground">Created</p>
                <p class="text-sm">{{ new Date(server.createdAt).toLocaleString() }}</p>
              </div>
            </div>
          </Card>

          <Separator style="background-color: hsl(var(--border))" />

          <!-- Deploy to Agents -->
          <div>
            <h2 class="text-sm font-medium mb-3">Deploy to Agents</h2>
            <p class="text-sm text-muted-foreground mb-4">
              Add this server configuration to an AI agent.
            </p>
            <div class="grid grid-cols-2 gap-2">
              <Button
                v-for="agent in agentsStore.enabledAgents"
                :key="agent.name"
                variant="outline"
                class="justify-start"
                :disabled="isDeploying"
                @click="handleDeploy(agent.name)"
              >
                <Send class="h-4 w-4 mr-2" />
                {{ agent.name }}
              </Button>
            </div>
            <p v-if="agentsStore.enabledAgents.length === 0" class="text-sm text-muted-foreground">
              No agents enabled. Go to Settings to enable agents.
            </p>
          </div>
        </div>
      </ScrollArea>
    </template>
  </div>
</template>
