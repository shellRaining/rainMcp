<script setup lang="ts">
import { Globe, Terminal, Package, ClipboardPaste, Sparkles } from 'lucide-vue-next';
import { ScrollArea } from '@/components/ui/scroll-area';
import type { ServerType } from '../types';

function handleSelect(type: ServerType) {
  emit('select', type);
}

const emit = defineEmits<{
  select: [type: ServerType];
}>();
</script>

<template>
  <ScrollArea class="h-full">
    <div class="py-6 px-4">
      <div class="max-w-2xl mx-auto space-y-4">
        <div class="text-center mb-6">
          <h2 class="text-xl font-semibold mb-2">Choose how to add a server</h2>
          <p class="text-sm text-muted-foreground">
            Select from the registry, create a custom command, or connect to a remote server
          </p>
        </div>

        <div class="grid grid-cols-1 gap-4">
          <!-- Registry -->
          <button
            class="p-6 rounded-lg border-2 hover:border-primary hover:bg-accent/50 transition-all text-left group"
            @click="handleSelect('registry')"
          >
            <div class="flex items-start gap-4">
              <div class="p-3 rounded-lg bg-primary/10 group-hover:bg-primary/20 transition-colors">
                <Package class="h-6 w-6 text-primary" />
              </div>
              <div class="flex-1">
                <h3 class="font-semibold text-lg mb-1">From Registry</h3>
                <p class="text-sm text-muted-foreground">
                  Browse and install servers from the official MCP registry
                </p>
              </div>
            </div>
          </button>

          <!-- Custom Command -->
          <button
            class="p-6 rounded-lg border-2 hover:border-primary hover:bg-accent/50 transition-all text-left group"
            @click="handleSelect('custom')"
          >
            <div class="flex items-start gap-4">
              <div class="p-3 rounded-lg bg-primary/10 group-hover:bg-primary/20 transition-colors">
                <Terminal class="h-6 w-6 text-primary" />
              </div>
              <div class="flex-1">
                <h3 class="font-semibold text-lg mb-1">Custom Command</h3>
                <p class="text-sm text-muted-foreground">
                  Configure a local server with custom command and arguments
                </p>
              </div>
            </div>
          </button>

          <!-- Remote URL -->
          <button
            class="p-6 rounded-lg border-2 hover:border-primary hover:bg-accent/50 transition-all text-left group"
            @click="handleSelect('remote')"
          >
            <div class="flex items-start gap-4">
              <div class="p-3 rounded-lg bg-primary/10 group-hover:bg-primary/20 transition-colors">
                <Globe class="h-6 w-6 text-primary" />
              </div>
              <div class="flex-1">
                <h3 class="font-semibold text-lg mb-1">Remote URL</h3>
                <p class="text-sm text-muted-foreground">
                  Connect to a remote MCP server via SSE or HTTP
                </p>
              </div>
            </div>
          </button>

          <!-- Import from Clipboard -->
          <button
            class="p-6 rounded-lg border-2 hover:border-primary hover:bg-accent/50 transition-all text-left group"
            @click="handleSelect('clipboard')"
          >
            <div class="flex items-start gap-4">
              <div class="p-3 rounded-lg bg-primary/10 group-hover:bg-primary/20 transition-colors">
                <ClipboardPaste class="h-6 w-6 text-primary" />
              </div>
              <div class="flex-1">
                <h3 class="font-semibold text-lg mb-1">Import from Clipboard</h3>
                <p class="text-sm text-muted-foreground">
                  Paste and import server configuration from clipboard
                </p>
              </div>
            </div>
          </button>

          <!-- AI Assistant -->
          <button
            class="p-6 rounded-lg border-2 hover:border-primary hover:bg-accent/50 transition-all text-left group"
            @click="handleSelect('ai')"
          >
            <div class="flex items-start gap-4">
              <div class="p-3 rounded-lg bg-primary/10 group-hover:bg-primary/20 transition-colors">
                <Sparkles class="h-6 w-6 text-primary" />
              </div>
              <div class="flex-1">
                <h3 class="font-semibold text-lg mb-1">AI Assistant</h3>
                <p class="text-sm text-muted-foreground">
                  Describe what you need and let AI generate the configuration
                </p>
              </div>
            </div>
          </button>
        </div>
      </div>
    </div>
  </ScrollArea>
</template>
