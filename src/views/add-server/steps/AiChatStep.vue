<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, computed, watch } from 'vue';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Loader2, AlertCircle } from 'lucide-vue-next';
import ChatMessageBubble, { type ContentItem, type MessageData } from '@/components/chat/ChatMessageBubble.vue';
import ChatInput from '@/components/chat/ChatInput.vue';
import * as api from '@/api/tauri';
import type { GeneratedSchema, StreamEvent } from '@/api/tauri';

defineProps<{
  isSubmitting: boolean;
}>();

const emit = defineEmits<{
  submit: [schema: GeneratedSchema];
}>();

const messages = ref<MessageData[]>([]);
const inputValue = ref('');
const isLoading = ref(false);
const error = ref<string | null>(null);
const hasApiKey = ref(false);
const messagesEndRef = ref<HTMLDivElement | null>(null);

// Message queue for sending while loading
const messageQueue = ref<string[]>([]);

// Streaming state
const streamingMessageId = ref<string | null>(null);
const streamingItems = ref<ContentItem[]>([]);
const streamingSchema = ref<GeneratedSchema | null>(null);

let unlistenStream: UnlistenFn | null = null;

function filterJsonFromContent(content: string, hasSchema: boolean): string {
  if (!hasSchema) return content;

  let filtered = content.replace(/```json[\s\S]*?```/g, '');
  filtered = filtered.replace(/\{[\s\S]*?"name"[\s\S]*?"description"[\s\S]*?\}/g, '');
  filtered = filtered.replace(/\n{3,}/g, '\n\n').trim();

  return filtered;
}

const displayMessages = computed(() => {
  const result = [...messages.value];

  if (streamingMessageId.value) {
    result.push({
      id: streamingMessageId.value,
      role: 'assistant',
      items: [...streamingItems.value],
      generatedSchema: streamingSchema.value || undefined,
    });
  }

  return result;
});

// Process queue when loading finishes
watch(isLoading, async (loading) => {
  if (!loading && messageQueue.value.length > 0) {
    // Combine all queued messages
    const combinedMessage = messageQueue.value.join('\n\n');
    messageQueue.value = [];
    await sendMessageInternal(combinedMessage);
  }
});

async function setupStreamListener() {
  unlistenStream = await listen<StreamEvent>('agent-stream', (event) => {
    const data = event.payload;

    switch (data.type) {
      case 'text':
        {
          const lastItem = streamingItems.value[streamingItems.value.length - 1];
          if (lastItem?.type === 'text') {
            lastItem.content += data.text;
          } else {
            streamingItems.value.push({ type: 'text', content: data.text });
          }
        }
        scrollToBottom();
        break;

      case 'toolCallStart':
        streamingItems.value.push({
          type: 'toolCall',
          name: data.name,
          status: 'running',
          argsPreview: data.argsPreview,
        });
        scrollToBottom();
        break;

      case 'toolCallEnd':
        {
          const toolCall = [...streamingItems.value]
            .reverse()
            .find((item) => item.type === 'toolCall' && item.name === data.name && item.status === 'running');
          if (toolCall && toolCall.type === 'toolCall') {
            toolCall.status = 'success';
            toolCall.resultPreview = data.resultPreview;
          }
        }
        scrollToBottom();
        break;

      case 'schema':
        streamingSchema.value = data.schema;
        scrollToBottom();
        break;

      case 'done':
        if (streamingMessageId.value) {
          const hasSchema = !!streamingSchema.value;
          const filteredItems = streamingItems.value.map((item) => {
            if (item.type === 'text' && hasSchema) {
              return { ...item, content: filterJsonFromContent(item.content, true) };
            }
            return item;
          });

          messages.value.push({
            id: streamingMessageId.value,
            role: 'assistant',
            items: filteredItems,
            generatedSchema: streamingSchema.value || undefined,
          });

          streamingMessageId.value = null;
          streamingItems.value = [];
          streamingSchema.value = null;
        }
        isLoading.value = false;
        scrollToBottom();
        break;

      case 'error':
        error.value = data.error;
        isLoading.value = false;
        streamingMessageId.value = null;
        streamingItems.value = [];
        streamingSchema.value = null;
        break;
    }
  });
}

onMounted(async () => {
  try {
    const key = await api.getOpenRouterApiKey();
    hasApiKey.value = !!key;
  } catch {
    hasApiKey.value = false;
  }

  try {
    await api.agentReset();
  } catch {
    // Ignore
  }

  await setupStreamListener();

  messages.value.push({
    id: 'welcome',
    role: 'assistant',
    items: [
      {
        type: 'text',
        content:
          'Hello! I can help you configure an MCP server. You can:\n\n- Paste a URL to documentation or GitHub repo\n- Describe what server you want to set up\n- Ask me questions about MCP configuration\n\nWhat would you like to configure?',
      },
    ],
  });
});

onUnmounted(() => {
  if (unlistenStream) {
    unlistenStream();
  }
});

async function sendMessageInternal(userMessage: string) {
  error.value = null;

  messages.value.push({
    id: `user-${Date.now()}`,
    role: 'user',
    items: [{ type: 'text', content: userMessage }],
  });

  await scrollToBottom();
  isLoading.value = true;

  streamingMessageId.value = `assistant-${Date.now()}`;
  streamingItems.value = [];
  streamingSchema.value = null;

  try {
    await api.agentChatStream(userMessage);
  } catch (e) {
    error.value = String(e);
    isLoading.value = false;
    streamingMessageId.value = null;
  }
}

async function sendMessage() {
  if (!inputValue.value.trim()) return;

  const userMessage = inputValue.value.trim();
  inputValue.value = '';

  if (isLoading.value) {
    // Queue message if currently loading
    messageQueue.value.push(userMessage);
    return;
  }

  await sendMessageInternal(userMessage);
}

function handleSchemaConfirm(schema: GeneratedSchema) {
  emit('submit', schema);
}

async function scrollToBottom() {
  await nextTick();
  messagesEndRef.value?.scrollIntoView({ behavior: 'smooth' });
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- API Key not configured -->
    <div v-if="!hasApiKey" class="flex-1 flex items-center justify-center p-6">
      <div class="text-center max-w-md">
        <AlertCircle class="h-12 w-12 text-muted-foreground mx-auto mb-4" />
        <h3 class="font-semibold mb-2">OpenRouter API Key Required</h3>
        <p class="text-sm text-muted-foreground mb-4">
          To use AI Assistant, please add your OpenRouter API key to the configuration file:
        </p>
        <code class="text-xs bg-muted p-2 rounded block mb-4">
          ~/.config/rain-mcp/settings.json
        </code>
        <p class="text-xs text-muted-foreground">Add: "openrouter_api_key": "your-api-key-here"</p>
      </div>
    </div>

    <!-- Chat interface -->
    <template v-else>
      <ScrollArea class="flex-1">
        <div class="p-4 space-y-4">
          <ChatMessageBubble
            v-for="msg in displayMessages"
            :key="msg.id"
            :message="msg"
            :is-submitting="isSubmitting"
            @schema-confirm="handleSchemaConfirm"
          />

          <!-- Loading indicator -->
          <div
            v-if="isLoading && streamingItems.length === 0"
            class="flex items-center gap-2 text-muted-foreground p-3"
          >
            <Loader2 class="h-4 w-4 animate-spin" />
            <span class="text-sm">Thinking...</span>
          </div>

          <div ref="messagesEndRef" />
        </div>
      </ScrollArea>

      <ChatInput
        v-model="inputValue"
        :is-loading="isLoading"
        :queued-count="messageQueue.length"
        :error="error"
        @submit="sendMessage"
      />
    </template>
  </div>
</template>
