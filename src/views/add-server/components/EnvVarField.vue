<script setup lang="ts">
import { Label } from '@/components/ui/label';
import { Input } from '@/components/ui/input';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Info } from 'lucide-vue-next';
import type { EnvironmentVariable } from '@/types/mcp';

const props = withDefaults(
  defineProps<{
    envVar: EnvironmentVariable;
    modelValue: string;
    required?: boolean;
  }>(),
  {
    required: false,
  }
);

const emit = defineEmits<{
  'update:modelValue': [value: string];
}>();

function getInputType(): string {
  if (props.envVar.isSecret) return 'password';
  if (props.envVar.format === 'number') return 'number';
  return 'text';
}

function getPlaceholder(): string {
  if (props.envVar.placeholder) return props.envVar.placeholder;
  if (props.envVar.description) return props.envVar.description;
  return `Enter ${props.envVar.name}`;
}

function updateValue(value: unknown) {
  emit('update:modelValue', String(value ?? ''));
}
</script>

<template>
  <div class="space-y-2">
    <div class="flex items-center gap-1">
      <Label :for="`env-${envVar.name}`">
        {{ envVar.name }}
        <span v-if="required" class="text-destructive">*</span>
      </Label>
      <Tooltip v-if="envVar.description">
        <TooltipTrigger as-child>
          <Info class="h-3 w-3 text-muted-foreground" />
        </TooltipTrigger>
        <TooltipContent class="max-w-xs text-xs">
          <p>{{ envVar.description }}</p>
        </TooltipContent>
      </Tooltip>
    </div>

    <!-- Select for choices -->
    <Select
      v-if="envVar.choices && envVar.choices.length > 0"
      :model-value="modelValue"
      @update:model-value="updateValue"
    >
      <SelectTrigger>
        <SelectValue :placeholder="getPlaceholder()" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem v-for="choice in envVar.choices" :key="choice" :value="choice">
          {{ choice }}
        </SelectItem>
      </SelectContent>
    </Select>

    <!-- Input for other types -->
    <Input
      v-else
      :id="`env-${envVar.name}`"
      :model-value="modelValue"
      @update:model-value="updateValue"
      :placeholder="getPlaceholder()"
      :type="getInputType()"
      :required="required"
    />
  </div>
</template>
