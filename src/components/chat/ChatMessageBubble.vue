<script setup lang="ts">
import MarkdownRender from 'markstream-vue';
import ChatToolCall from './ChatToolCall.vue';
import type { GeneratedSchema } from '@/api/tauri';
import type { ToolCallData } from './ChatToolCall.vue';

export type ContentItem =
  | { type: 'text'; content: string }
  | ({ type: 'toolCall' } & ToolCallData);

export interface MessageData {
  id: string;
  role: 'user' | 'assistant';
  items: ContentItem[];
  generatedSchema?: GeneratedSchema;
}

defineProps<{
  message: MessageData;
  isSubmitting?: boolean;
}>();

const emit = defineEmits<{
  schemaConfirm: [schema: GeneratedSchema];
}>();

function handleToolUse(item: ContentItem) {
  if (item.type === 'toolCall' && item.name === 'generate_server_schema' && item.resultPreview) {
    try {
      // resultPreview may be double-escaped (JSON string containing JSON)
      let parsed = JSON.parse(item.resultPreview);
      // If still a string, parse again
      if (typeof parsed === 'string') {
        parsed = JSON.parse(parsed);
      }
      emit('schemaConfirm', {
        schema: parsed,
        confidence: 0.8,
        explanation: 'Generated from AI response',
      });
    } catch {
      // Ignore parse errors
    }
  }
}
</script>

<template>
  <div
    :class="[
      'py-2',
      message.role === 'user' ? 'pl-12' : '',
    ]"
  >
    <!-- User message with bubble -->
    <div v-if="message.role === 'user'" class="flex justify-end">
      <div
        class="px-3 py-2 rounded-lg bg-primary text-primary-foreground text-sm select-text max-w-[85%]"
      >
        <template v-for="(item, idx) in message.items" :key="idx">
          <span v-if="item.type === 'text'">{{ item.content }}</span>
        </template>
      </div>
    </div>

    <!-- Assistant message without bubble -->
    <div v-else class="space-y-2 select-text">
      <template v-for="(item, idx) in message.items" :key="idx">
        <div v-if="item.type === 'text'" class="text-sm markdown-content">
          <MarkdownRender :content="item.content" />
        </div>

        <ChatToolCall
          v-else-if="item.type === 'toolCall'"
          :tool-call="item"
          :show-use-button="item.name === 'generate_server_schema'"
          :use-button-disabled="isSubmitting"
          @use="handleToolUse(item)"
        />
      </template>
    </div>
  </div>
</template>
