<script setup lang="ts">
import { Package, Globe, ChevronRight } from 'lucide-vue-next';
import type { ServerSchema, Package as PackageType, RemoteTransport } from '@/types/mcp';

defineProps<{
  schema: ServerSchema;
}>();

const emit = defineEmits<{
  select: [index: number];
}>();

function getPackageLabel(pkg: PackageType) {
  if (pkg.registryType === 'npm') return 'npm';
  if (pkg.registryType === 'pypi') return 'PyPI';
  if (pkg.registryType === 'oci') return 'Docker';
  return pkg.registryType || 'Package';
}

const remoteLabels: Record<RemoteTransport['type'], string> = {
  sse: 'SSE',
  'streamable-http': 'Streamable HTTP',
};

function getRemoteLabel(remote: RemoteTransport) {
  return remoteLabels[remote.type];
}

function selectOption(index: number) {
  emit('select', index);
}
</script>

<template>
  <div class="flex-1 py-4 px-4">
    <div class="max-w-2xl mx-auto">
      <div class="mb-4">
        <h2 class="text-lg font-semibold mb-1">Choose installation method</h2>
        <p class="text-sm text-muted-foreground">
          This server provides multiple installation options
        </p>
      </div>

      <div class="space-y-2">
        <!-- Packages -->
        <button
          v-for="(pkg, index) in schema.packages"
          :key="`pkg-${index}`"
          class="w-full flex items-center gap-3 p-3 rounded-lg border hover:bg-accent/50 transition-colors text-left"
          @click="selectOption(index)"
        >
          <div class="w-10 h-10 rounded-lg bg-muted flex items-center justify-center shrink-0">
            <Package class="h-5 w-5 text-muted-foreground" />
          </div>
          <div class="min-w-0 flex-1">
            <h3 class="font-medium">{{ getPackageLabel(pkg) }}</h3>
            <p class="text-sm text-muted-foreground font-mono truncate">{{ pkg.identifier }}</p>
          </div>
          <ChevronRight class="h-5 w-5 text-muted-foreground shrink-0" />
        </button>

        <!-- Remotes -->
        <button
          v-for="(remote, index) in schema.remotes"
          :key="`remote-${index}`"
          class="w-full flex items-center gap-3 p-3 rounded-lg border hover:bg-accent/50 transition-colors text-left"
          @click="selectOption((schema.packages?.length || 0) + index)"
        >
          <div class="w-10 h-10 rounded-lg bg-muted flex items-center justify-center shrink-0">
            <Globe class="h-5 w-5 text-muted-foreground" />
          </div>
          <div class="min-w-0 flex-1">
            <h3 class="font-medium">{{ getRemoteLabel(remote) }}</h3>
            <p class="text-sm text-muted-foreground font-mono truncate">{{ remote.url }}</p>
          </div>
          <ChevronRight class="h-5 w-5 text-muted-foreground shrink-0" />
        </button>
      </div>
    </div>
  </div>
</template>
