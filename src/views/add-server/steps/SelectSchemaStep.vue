<script setup lang="ts">
import { Package, ChevronRight, ExternalLink } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Badge } from '@/components/ui/badge';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import type { ServerSchema } from '@/types/mcp';
import { openUrl } from '@tauri-apps/plugin-opener';

defineProps<{
  filteredSchemas: ServerSchema[];
  hasMoreResults: boolean;
  isSearching: boolean;
}>();

const emit = defineEmits<{
  select: [schema: ServerSchema];
  loadMore: [];
}>();

async function openProjectUrl(url: string) {
  await openUrl(url);
}
</script>

<template>
  <div class="h-full min-h-0 flex flex-col">
    <ScrollArea class="flex-1 min-h-0">
      <div class="p-4 space-y-2">
        <button
          v-for="schema in filteredSchemas"
          :key="schema.name"
          class="w-full p-4 rounded-lg border hover:bg-accent/50 transition-colors text-left group"
          @click="emit('select', schema)"
        >
          <div class="flex items-center gap-4">
            <!-- 左侧 Icon -->
            <div class="shrink-0">
              <div class="w-12 h-12 rounded-lg bg-muted flex items-center justify-center">
                <Package class="h-6 w-6 text-muted-foreground" />
              </div>
            </div>

            <!-- 中间内容区 -->
            <div class="flex-1 min-w-0">
              <!-- 上层：名称 + badges -->
              <div class="flex flex-wrap items-center gap-2 mb-1">
                <h3 class="font-medium truncate">{{ schema.title || schema.name }}</h3>
                <Badge variant="secondary" class="text-xs shrink-0">v{{ schema.version }}</Badge>
                <Tooltip v-if="schema.repository?.url">
                  <TooltipTrigger as-child>
                    <button
                      class="text-muted-foreground hover:text-foreground shrink-0"
                      @click.stop="openProjectUrl(schema.repository.url)"
                    >
                      <ExternalLink class="h-4 w-4" />
                    </button>
                  </TooltipTrigger>
                  <TooltipContent class="text-[10px]">
                    <p>{{ schema.repository.url }}</p>
                  </TooltipContent>
                </Tooltip>
              </div>
              <!-- 下层：描述 -->
              <p class="text-sm text-muted-foreground line-clamp-2">
                {{ schema.description }}
              </p>
            </div>

            <!-- 右侧按钮 -->
            <div class="shrink-0 pl-2">
              <ChevronRight
                class="h-5 w-5 text-muted-foreground opacity-0 group-hover:opacity-100 transition-opacity"
              />
            </div>
          </div>
        </button>

        <!-- Load More -->
        <Button v-if="hasMoreResults" variant="outline" class="w-full" @click="emit('loadMore')">
          Load More
        </Button>

        <!-- Empty State -->
        <div
          v-if="filteredSchemas.length === 0 && !isSearching"
          class="py-12 text-center text-muted-foreground"
        >
          <p>No servers found</p>
          <p class="text-sm mt-1">Try a different search term</p>
        </div>
      </div>
    </ScrollArea>
  </div>
</template>
