<script setup lang="ts">
import { computed } from 'vue';
import { Label } from '@/components/ui/label';
import { Input } from '@/components/ui/input';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import { Loader2, Package, Globe, Info } from 'lucide-vue-next';
import EnvVarField from '../components/EnvVarField.vue';
import type {
  ServerSchema,
  Package as PackageType,
  RemoteTransport,
  EnvironmentVariable,
} from '@/types/mcp';

const props = withDefaults(
  defineProps<{
    serverName: string;
    envValues: Record<string, string>;
    selectedSchema: ServerSchema | null;
    selectedPackage: PackageType | null;
    selectedRemote: RemoteTransport | null;
    envVars?: EnvironmentVariable[];
    isSubmitting: boolean;
  }>(),
  {
    envVars: () => [],
  }
);

const emit = defineEmits<{
  'update:serverName': [value: string];
  'update:envValues': [value: Record<string, string>];
  submit: [];
}>();

const canSubmit = computed(() => {
  if (!props.serverName.trim()) return false;
  // Check required env vars are filled
  for (const envVar of props.envVars) {
    if (envVar.isRequired && !props.envValues[envVar.name]?.trim()) {
      return false;
    }
  }
  return true;
});

// Separate required and optional env vars
const requiredEnvVars = computed(() => props.envVars.filter((v) => v.isRequired));
const optionalEnvVars = computed(() => props.envVars.filter((v) => !v.isRequired));

function updateEnvValue(key: string, value: string) {
  const newEnvValues = { ...props.envValues, [key]: value };
  emit('update:envValues', newEnvValues);
}

function updateServerName(value: string | number) {
  emit('update:serverName', String(value));
}
</script>

<template>
  <div class="h-full flex flex-col">
    <ScrollArea class="flex-1">
      <div class="p-4 space-y-6 max-w-2xl mx-auto">
        <!-- Schema Info Header -->
        <div v-if="selectedSchema" class="space-y-2">
          <div class="flex items-center gap-2">
            <h2 class="text-lg font-medium">{{ selectedSchema.title || selectedSchema.name }}</h2>
            <Badge variant="secondary" class="text-xs">v{{ selectedSchema.version }}</Badge>
          </div>
          <p class="text-sm text-muted-foreground">{{ selectedSchema.description }}</p>

          <!-- Package/Remote Info -->
          <div class="flex items-center gap-2 text-xs text-muted-foreground">
            <template v-if="selectedPackage">
              <Package class="h-3 w-3" />
              <span>{{ selectedPackage.registryType }}:{{ selectedPackage.identifier }}</span>
            </template>
            <template v-else-if="selectedRemote">
              <Globe class="h-3 w-3" />
              <span>{{ selectedRemote.url }}</span>
            </template>
          </div>
        </div>

        <div class="border-t pt-4 space-y-4">
          <!-- Server Name -->
          <div class="space-y-2">
            <div class="flex items-center gap-1">
              <Label for="server-name">
                Server Name
                <span class="text-destructive">*</span>
              </Label>
              <Tooltip>
                <TooltipTrigger as-child>
                  <Info class="h-3 w-3 text-muted-foreground" />
                </TooltipTrigger>
                <TooltipContent class="max-w-xs text-xs">
                  <p>This name will be used to identify the server in your configuration</p>
                </TooltipContent>
              </Tooltip>
            </div>
            <Input
              id="server-name"
              :model-value="serverName"
              @update:model-value="updateServerName"
              placeholder="my-server"
              required
            />
          </div>

          <!-- Required Environment Variables -->
          <div v-if="requiredEnvVars.length > 0" class="space-y-4">
            <EnvVarField
              v-for="envVar in requiredEnvVars"
              :key="envVar.name"
              :env-var="envVar"
              :model-value="envValues[envVar.name] || ''"
              @update:model-value="updateEnvValue(envVar.name, $event)"
              required
            />
          </div>

          <!-- Optional Environment Variables -->
          <div v-if="optionalEnvVars.length > 0" class="space-y-4">
            <EnvVarField
              v-for="envVar in optionalEnvVars"
              :key="envVar.name"
              :env-var="envVar"
              :model-value="envValues[envVar.name] || ''"
              @update:model-value="updateEnvValue(envVar.name, $event)"
            />
          </div>
        </div>
      </div>
    </ScrollArea>

    <!-- Footer -->
    <footer class="shrink-0 p-4 border-t flex items-center justify-end">
      <Button @click="emit('submit')" :disabled="!canSubmit || isSubmitting">
        <Loader2 v-if="isSubmitting" class="h-4 w-4 mr-2 animate-spin" />
        {{ isSubmitting ? 'Adding...' : 'Add Server' }}
      </Button>
    </footer>
  </div>
</template>
