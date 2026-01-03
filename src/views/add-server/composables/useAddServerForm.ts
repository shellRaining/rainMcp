import { ref, computed, watch } from 'vue';
import { useDebounceFn } from '@vueuse/core';
import Fuse from 'fuse.js';
import type { Step, ServerType } from '../types';
import type {
  ServerSchema,
  Package as PackageType,
  RemoteTransport,
  AgentServerEntry,
} from '@/types/mcp';
import { useServersStore } from '@/stores/servers';
import { logger } from '@/utils/logger';
import type { ParsedServer } from './useClipboardParser';
import type { GeneratedSchema } from '@/api/tauri';

const PAGE_SIZE = 50;

const fuseOptions = {
  keys: [
    { name: 'name', weight: 0.4 },
    { name: 'title', weight: 0.4 },
    { name: 'description', weight: 0.2 },
  ],
  threshold: 0.4,
  includeScore: true,
  shouldSort: true,
  minMatchCharLength: 2,
};

export function useAddServerForm() {
  const serversStore = useServersStore();

  // Step management
  const currentStep = ref<Step>('select-type');
  const stepHistory = ref<Step[]>([]); // History stack for back navigation
  const selectedType = ref<ServerType | null>(null);
  const isGoingForward = ref(true);

  // Registry selection
  const searchQuery = ref('');
  const debouncedQuery = ref('');
  const isSearching = ref(false);
  const displayLimit = ref(PAGE_SIZE);
  const selectedSchema = ref<ServerSchema | null>(null);
  const selectedPackageIndex = ref(0);

  // Form data
  const serverName = ref('');
  const customCommand = ref('');
  const customArgs = ref('');
  const customEnv = ref('');
  const remoteUrl = ref('');
  const remoteHeaders = ref('');
  const envValues = ref<Record<string, string>>({});

  // Loading state
  const isSubmitting = ref(false);

  // Fuse.js instance
  const fuse = computed(() => {
    return new Fuse(serversStore.serverSchemas, fuseOptions);
  });

  // Debounced search
  const updateDebouncedQuery = useDebounceFn((query: string) => {
    debouncedQuery.value = query;
    displayLimit.value = PAGE_SIZE;
    isSearching.value = false;
  }, 300);

  watch(searchQuery, (query) => {
    isSearching.value = true;
    updateDebouncedQuery(query);
  });

  // Filtered schemas
  const allFilteredSchemas = computed(() => {
    if (!debouncedQuery.value || debouncedQuery.value.length < 2) {
      return serversStore.serverSchemas;
    }
    const results = fuse.value.search(debouncedQuery.value);
    return results.map((r) => r.item);
  });

  const filteredSchemas = computed(() => {
    return allFilteredSchemas.value.slice(0, displayLimit.value);
  });

  const hasMoreResults = computed(() => {
    return allFilteredSchemas.value.length > displayLimit.value;
  });

  function loadMore() {
    displayLimit.value += PAGE_SIZE;
  }

  // Selected package/remote
  const selectedPackage = computed(() => {
    if (!selectedSchema.value?.packages || selectedSchema.value.packages.length === 0) {
      return null;
    }
    return selectedSchema.value.packages[selectedPackageIndex.value] || null;
  });

  const selectedRemote = computed(() => {
    if (!selectedSchema.value?.remotes || selectedSchema.value.remotes.length === 0) {
      return null;
    }
    const packagesCount = selectedSchema.value.packages?.length || 0;
    const remoteIndex = selectedPackageIndex.value - packagesCount;
    return selectedSchema.value.remotes[remoteIndex] || null;
  });

  // All env variables (not just required)
  const allEnvVars = computed(() => {
    const pkg = selectedPackage.value;
    if (!pkg?.environmentVariables) return [];
    return pkg.environmentVariables;
  });

  // Required env variables
  const requiredEnvVars = computed(() => {
    return allEnvVars.value.filter((v) => v.isRequired);
  });

  // Helper: extract short name from schema name (e.g., "io.github.user/weather" -> "weather")
  function extractServerName(schema: ServerSchema): string {
    if (schema.title) {
      // Use title directly, convert to kebab-case
      return schema.title
        .toLowerCase()
        .replace(/[^a-z0-9]+/g, '-')
        .replace(/^-|-$/g, '');
    }
    // Extract last segment from name
    const name = schema.name || '';
    if (!name) return 'mcp-server';
    const lastSlash = name.lastIndexOf('/');
    return lastSlash >= 0 ? name.slice(lastSlash + 1) : name;
  }

  // Helper: initialize env values with defaults
  function initEnvDefaults(pkg: PackageType | null) {
    if (!pkg?.environmentVariables) return;
    const defaults: Record<string, string> = {};
    for (const envVar of pkg.environmentVariables) {
      if (envVar.default) {
        defaults[envVar.name] = envVar.default;
      }
    }
    envValues.value = defaults;
  }

  // Helper: navigate to a new step and record history
  function navigateTo(step: Step) {
    stepHistory.value.push(currentStep.value);
    currentStep.value = step;
    isGoingForward.value = true;
  }

  // Navigation functions
  function selectType(type: ServerType) {
    selectedType.value = type;

    if (type === 'registry') {
      navigateTo('select-schema');
    } else if (type === 'custom') {
      navigateTo('custom-form');
    } else if (type === 'remote') {
      navigateTo('remote-form');
    } else if (type === 'clipboard') {
      navigateTo('clipboard-import');
    } else if (type === 'ai') {
      navigateTo('ai-chat');
    }
  }

  function selectSchema(schema: ServerSchema) {
    selectedSchema.value = schema;
    selectedPackageIndex.value = 0;

    // Auto-fill server name
    serverName.value = extractServerName(schema);

    const packagesCount = schema.packages?.length || 0;
    const remotesCount = schema.remotes?.length || 0;
    const totalOptions = packagesCount + remotesCount;

    if (totalOptions > 1) {
      navigateTo('select-package');
    } else {
      // Initialize env defaults for first package
      if (schema.packages && schema.packages.length > 0) {
        initEnvDefaults(schema.packages[0]);
      }
      navigateTo('configure');
    }
  }

  function selectPackage(index: number) {
    selectedPackageIndex.value = index;

    // Initialize env defaults for selected package
    const schema = selectedSchema.value;
    if (schema?.packages && index < schema.packages.length) {
      initEnvDefaults(schema.packages[index]);
    }

    navigateTo('configure');
  }

  function goBack() {
    isGoingForward.value = false;

    // Use history stack if available
    if (stepHistory.value.length > 0) {
      currentStep.value = stepHistory.value.pop()!;
      return;
    }

    // Fallback to select-type if no history
    currentStep.value = 'select-type';
    selectedType.value = null;
  }

  // Helper functions
  function parseEnvString(envStr: string): Record<string, string> {
    const result: Record<string, string> = {};
    for (const line of envStr.split('\n')) {
      const trimmed = line.trim();
      if (!trimmed) continue;
      const idx = trimmed.indexOf('=');
      if (idx > 0) {
        result[trimmed.slice(0, idx).trim()] = trimmed.slice(idx + 1).trim();
      }
    }
    return result;
  }

  function parseHeadersString(headersStr: string): Record<string, string> {
    const result: Record<string, string> = {};
    for (const line of headersStr.split('\n')) {
      const trimmed = line.trim();
      if (!trimmed) continue;
      const idx = trimmed.indexOf(':');
      if (idx > 0) {
        result[trimmed.slice(0, idx).trim()] = trimmed.slice(idx + 1).trim();
      }
    }
    return result;
  }

  function convertPackageToConfig(
    pkg: PackageType,
    userEnvValues: Record<string, string>
  ): AgentServerEntry {
    let command: string;
    let args: string[];

    const runtimeHint = pkg.runtimeHint;
    const registryType = pkg.registryType;

    if (runtimeHint === 'npx' || (!runtimeHint && registryType === 'npm')) {
      command = 'npx';
      args = ['-y'];
    } else if (runtimeHint === 'uvx' || (!runtimeHint && registryType === 'pypi')) {
      command = 'uvx';
      args = [];
    } else if (runtimeHint === 'docker' || (!runtimeHint && registryType === 'oci')) {
      command = 'docker';
      args = ['run', '-i'];
    } else if (runtimeHint === 'dnx') {
      command = 'dnx';
      args = [];
    } else {
      command = 'npx';
      args = ['-y'];
    }

    const identifier = pkg.version ? `${pkg.identifier}@${pkg.version}` : pkg.identifier;
    args.push(identifier);

    if (pkg.packageArguments) {
      for (const arg of pkg.packageArguments) {
        if (arg.type === 'named' && arg.name) {
          args.push(arg.name);
        }
        if (arg.value) {
          args.push(arg.value);
        }
      }
    }

    if (pkg.transport.type === 'sse' || pkg.transport.type === 'streamable-http') {
      const transportWithUrl = pkg.transport as { url: string };
      return {
        type: 'remote',
        url: transportWithUrl.url,
        headers: Object.keys(userEnvValues).length > 0 ? userEnvValues : null,
      };
    }

    return {
      type: 'local',
      command,
      args: args.length > 0 ? args : null,
      env: Object.keys(userEnvValues).length > 0 ? userEnvValues : null,
    };
  }

  function convertRemoteToConfig(
    remote: RemoteTransport,
    userEnvValues: Record<string, string>
  ): AgentServerEntry {
    return {
      type: 'remote',
      url: remote.url,
      headers: Object.keys(userEnvValues).length > 0 ? userEnvValues : null,
    };
  }

  // Submit functions
  async function submitRegistryServer() {
    if (!selectedSchema.value || !serverName.value) return false;

    isSubmitting.value = true;
    try {
      let config: AgentServerEntry;

      if (selectedPackage.value) {
        config = convertPackageToConfig(selectedPackage.value, envValues.value);
      } else if (selectedRemote.value) {
        config = convertRemoteToConfig(selectedRemote.value, envValues.value);
      } else {
        throw new Error('No package or remote selected');
      }

      await serversStore.addServer({
        name: serverName.value,
        config,
        origin: {
          originType: 'registry',
          schemaName: selectedSchema.value.name,
        },
      });

      return true;
    } catch (error) {
      logger.error('Failed to add server:', error);
      return false;
    } finally {
      isSubmitting.value = false;
    }
  }

  async function submitCustomServer() {
    if (!serverName.value || !customCommand.value) return false;

    isSubmitting.value = true;
    try {
      const args = customArgs.value.split(/\s+/).filter(Boolean);
      const env = customEnv.value ? parseEnvString(customEnv.value) : {};

      await serversStore.addServer({
        name: serverName.value,
        config: {
          type: 'local',
          command: customCommand.value,
          args: args.length > 0 ? args : null,
          env: Object.keys(env).length > 0 ? env : null,
        },
        origin: {
          originType: 'custom',
        },
      });

      return true;
    } catch (error) {
      logger.error('Failed to add custom server:', error);
      return false;
    } finally {
      isSubmitting.value = false;
    }
  }

  async function submitRemoteServer() {
    if (!serverName.value || !remoteUrl.value) return false;

    isSubmitting.value = true;
    try {
      const headers = remoteHeaders.value ? parseHeadersString(remoteHeaders.value) : {};

      await serversStore.addServer({
        name: serverName.value,
        config: {
          type: 'remote',
          url: remoteUrl.value,
          headers: Object.keys(headers).length > 0 ? headers : null,
        },
        origin: {
          originType: 'custom',
        },
      });

      return true;
    } catch (error) {
      logger.error('Failed to add remote server:', error);
      return false;
    } finally {
      isSubmitting.value = false;
    }
  }

  async function submitClipboardServers(servers: ParsedServer[]) {
    if (servers.length === 0) return false;

    isSubmitting.value = true;
    try {
      for (const server of servers) {
        await serversStore.addServer({
          name: server.name,
          config: server.config,
          origin: {
            originType: 'custom',
          },
        });
      }

      return true;
    } catch (error) {
      logger.error('Failed to add servers from clipboard:', error);
      return false;
    } finally {
      isSubmitting.value = false;
    }
  }

  async function submitAiSchema(generatedSchema: GeneratedSchema) {
    if (!generatedSchema.schema) return false;

    // Use the generated schema and go through the normal schema selection flow
    const schema = generatedSchema.schema;

    // Set the selected schema
    selectedSchema.value = schema;
    selectedPackageIndex.value = 0;

    // Auto-fill server name
    serverName.value = extractServerName(schema);

    const packagesCount = schema.packages?.length || 0;
    const remotesCount = schema.remotes?.length || 0;
    const totalOptions = packagesCount + remotesCount;

    if (totalOptions > 1) {
      // Multiple options: go to package selection step
      navigateTo('select-package');
    } else if (totalOptions === 1) {
      // Single option: go directly to configure step
      if (schema.packages && schema.packages.length > 0) {
        initEnvDefaults(schema.packages[0]);
      }
      navigateTo('configure');
    } else {
      // No packages or remotes - this shouldn't happen with valid schema
      logger.error('Generated schema has no packages or remotes');
      return false;
    }

    return true;
  }

  // Reset form
  function resetForm() {
    currentStep.value = 'select-type';
    stepHistory.value = [];
    selectedType.value = null;
    searchQuery.value = '';
    debouncedQuery.value = '';
    displayLimit.value = PAGE_SIZE;
    selectedSchema.value = null;
    selectedPackageIndex.value = 0;
    serverName.value = '';
    customCommand.value = '';
    customArgs.value = '';
    customEnv.value = '';
    remoteUrl.value = '';
    remoteHeaders.value = '';
    envValues.value = {};
    isSubmitting.value = false;
  }

  return {
    // State
    currentStep,
    selectedType,
    isGoingForward,
    searchQuery,
    debouncedQuery,
    isSearching,
    selectedSchema,
    selectedPackageIndex,
    serverName,
    customCommand,
    customArgs,
    customEnv,
    remoteUrl,
    remoteHeaders,
    envValues,
    isSubmitting,

    // Computed
    filteredSchemas,
    hasMoreResults,
    selectedPackage,
    selectedRemote,
    allEnvVars,
    requiredEnvVars,

    // Methods
    selectType,
    selectSchema,
    selectPackage,
    goBack,
    loadMore,
    submitRegistryServer,
    submitCustomServer,
    submitRemoteServer,
    submitClipboardServers,
    submitAiSchema,
    resetForm,
    parseEnvString,
    parseHeadersString,
  };
}
