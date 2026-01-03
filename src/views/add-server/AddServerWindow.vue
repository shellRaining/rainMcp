<script setup lang="ts">
import { computed, watch, onMounted, onBeforeMount, onBeforeUnmount } from 'vue';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { emit as emitEvent } from '@tauri-apps/api/event';
import { Button } from '@/components/ui/button';
import { ChevronLeft } from 'lucide-vue-next';
import { ExpandableSearch } from '@/components/ui/expandable-search';
import { useAddServerForm } from './composables/useAddServerForm';
import { setTrafficLightsInset } from '@/api/tauri';
import { useServersStore } from '@/stores/servers';
import { logger } from '@/utils/logger';
import SelectTypeStep from './steps/SelectTypeStep.vue';
import SelectSchemaStep from './steps/SelectSchemaStep.vue';
import SelectPackageStep from './steps/SelectPackageStep.vue';
import ConfigureStep from './steps/ConfigureStep.vue';
import CustomFormStep from './steps/CustomFormStep.vue';
import RemoteFormStep from './steps/RemoteFormStep.vue';
import ClipboardImportStep from './steps/ClipboardImportStep.vue';
import AiChatStep from './steps/AiChatStep.vue';
import type { ParsedServer } from './composables/useClipboardParser';
import type { GeneratedSchema } from '@/api/tauri';

const currentWindow = getCurrentWebviewWindow();
const serversStore = useServersStore();
const form = useAddServerForm();

// 设置 macOS 红绿灯按钮的位置
onBeforeMount(async () => {
  try {
    await setTrafficLightsInset('add-server', 12, 24);
  } catch (error) {
    logger.debug('[AddServerWindow] Failed to adjust traffic lights:', error);
  }
})

// Initialize store data
onMounted(async () => {
  logger.debug('[AddServerWindow] Initializing servers store');
  // 如果 schema store 还没有数据，初始化它
  if (!serversStore.schemaStore || serversStore.serverSchemas.length === 0) {
    await serversStore.init();
  }

  // 添加鼠标侧键导航监听器
  window.addEventListener('mousedown', handleMouseNavigation);
});

// 清理事件监听器
onBeforeUnmount(() => {
  window.removeEventListener('mousedown', handleMouseNavigation);
});

// 处理鼠标侧键导航
function handleMouseNavigation(event: MouseEvent) {
  // button === 3: 鼠标后退键
  // button === 4: 鼠标前进键
  if (event.button === 3) {
    // 后退键：如果不在第一步且未提交中，执行后退
    if (form.currentStep.value !== 'select-type' && !form.isSubmitting.value) {
      event.preventDefault();
      form.goBack();
    }
  }
  // 暂不实现前进功能，因为步骤是单向流程
}

// Transition name based on direction
const transitionName = computed(() => {
  return form.isGoingForward.value ? 'slide-right' : 'slide-left';
});

// Handle submit
async function handleSubmit() {
  let success = false;

  if (form.currentStep.value === 'configure') {
    success = await form.submitRegistryServer();
  } else if (form.currentStep.value === 'custom-form') {
    success = await form.submitCustomServer();
  } else if (form.currentStep.value === 'remote-form') {
    success = await form.submitRemoteServer();
  }

  if (success) {
    // Emit event to main window
    await emitEvent('server-added', {});
    // Close window
    await currentWindow.close();
  }
}

// Handle clipboard import submit
async function handleClipboardSubmit(servers: ParsedServer[]) {
  const success = await form.submitClipboardServers(servers);

  if (success) {
    await emitEvent('server-added', {});
    await currentWindow.close();
  }
}

// Handle AI schema submit - navigates to schema selection flow
async function handleAiSubmit(schema: GeneratedSchema) {
  // This will set the selected schema and navigate to select-package or configure step
  await form.submitAiSchema(schema);
  // Don't close window - user will continue with package selection and configuration
}

// Reset form when window closes
watch(() => currentWindow, () => {
  form.resetForm();
}, { immediate: true });
</script>

<template>
  <div class="h-screen flex flex-col bg-background">
    <!-- Header with drag region -->
    <header class="shrink-0 h-[48px] border-b flex items-center relative" data-tauri-drag-region>
      <!-- Traffic lights spacer (approx 80px to clear buttons comfortably) -->
      <div class="w-[80px] h-full" data-tauri-drag-region />

      <!-- Back button -->
      <Button
        v-if="form.currentStep.value !== 'select-type'"
        variant="ghost"
        size="icon"
        class="h-8 w-8 -ml-2"
        @click="form.goBack"
        :disabled="form.isSubmitting.value"
      >
        <ChevronLeft class="h-4 w-4" />
      </Button>

      <!-- Title centered -->
      <h1
        class="absolute left-1/2 -translate-x-1/2 text-sm font-semibold pointer-events-none select-none"
      >
        Add MCP Server
      </h1>

      <!-- Right Content: Expandable Search or Spacer -->
      <div class="ml-auto mr-4 flex items-center justify-end">
        <div
          v-if="form.currentStep.value === 'select-schema'"
          class="flex items-center justify-end"
        >
          <ExpandableSearch
            :model-value="form.searchQuery.value"
            @update:model-value="(v) => (form.searchQuery.value = v)"
            :is-loading="form.isSearching.value"
            placeholder="Search servers..."
          />
        </div>
      </div>
    </header>

    <!-- Content with transition -->
    <main class="flex-1 min-h-0 overflow-hidden relative">
      <Transition :name="transitionName" mode="out-in">
        <!-- Select Type Step -->
        <SelectTypeStep
          v-if="form.currentStep.value === 'select-type'"
          key="select-type"
          class="absolute inset-0"
          @select="form.selectType"
        />

        <!-- Select Schema Step -->
        <SelectSchemaStep
          v-else-if="form.currentStep.value === 'select-schema'"
          key="select-schema"
          class="absolute inset-0"
          :filtered-schemas="form.filteredSchemas.value"
          :has-more-results="form.hasMoreResults.value"
          :is-searching="form.isSearching.value"
          @select="form.selectSchema"
          @load-more="form.loadMore"
        />

        <!-- Select Package Step -->
        <SelectPackageStep
          v-else-if="form.currentStep.value === 'select-package'"
          key="select-package"
          class="absolute inset-0"
          :schema="form.selectedSchema.value!"
          @select="form.selectPackage"
        />

        <!-- Configure Step -->
        <ConfigureStep
          v-else-if="form.currentStep.value === 'configure'"
          key="configure"
          class="absolute inset-0"
          :server-name="form.serverName.value"
          :env-values="form.envValues.value"
          :selected-schema="form.selectedSchema.value"
          :selected-package="form.selectedPackage.value"
          :selected-remote="form.selectedRemote.value"
          :env-vars="form.allEnvVars?.value ?? []"
          :is-submitting="form.isSubmitting.value"
          @update:server-name="(v) => (form.serverName.value = v)"
          @update:env-values="(v) => (form.envValues.value = v)"
          @submit="handleSubmit"
        />

        <!-- Custom Form Step -->
        <CustomFormStep
          v-else-if="form.currentStep.value === 'custom-form'"
          key="custom-form"
          class="absolute inset-0"
          :server-name="form.serverName.value"
          :custom-command="form.customCommand.value"
          :custom-args="form.customArgs.value"
          :custom-env="form.customEnv.value"
          :is-submitting="form.isSubmitting.value"
          @update:server-name="(v) => (form.serverName.value = v)"
          @update:custom-command="(v) => (form.customCommand.value = v)"
          @update:custom-args="(v) => (form.customArgs.value = v)"
          @update:custom-env="(v) => (form.customEnv.value = v)"
          @submit="handleSubmit"
        />

        <!-- Remote Form Step -->
        <RemoteFormStep
          v-else-if="form.currentStep.value === 'remote-form'"
          key="remote-form"
          class="absolute inset-0"
          :server-name="form.serverName.value"
          :remote-url="form.remoteUrl.value"
          :remote-headers="form.remoteHeaders.value"
          :is-submitting="form.isSubmitting.value"
          @update:server-name="(v) => (form.serverName.value = v)"
          @update:remote-url="(v) => (form.remoteUrl.value = v)"
          @update:remote-headers="(v) => (form.remoteHeaders.value = v)"
          @submit="handleSubmit"
        />

        <!-- Clipboard Import Step -->
        <ClipboardImportStep
          v-else-if="form.currentStep.value === 'clipboard-import'"
          key="clipboard-import"
          class="absolute inset-0"
          :is-submitting="form.isSubmitting.value"
          @submit="handleClipboardSubmit"
        />

        <!-- AI Chat Step -->
        <AiChatStep
          v-else-if="form.currentStep.value === 'ai-chat'"
          key="ai-chat"
          class="absolute inset-0"
          :is-submitting="form.isSubmitting.value"
          @submit="handleAiSubmit"
        />
      </Transition>
    </main>
  </div>
</template>

<style scoped>
/* Slide right (forward) */
.slide-right-enter-active,
.slide-right-leave-active {
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

.slide-right-enter-from {
  opacity: 0;
  transform: translateX(30px);
}

.slide-right-leave-to {
  opacity: 0;
  transform: translateX(-30px);
}

/* Slide left (backward) */
.slide-left-enter-active,
.slide-left-leave-active {
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

.slide-left-enter-from {
  opacity: 0;
  transform: translateX(-30px);
}

.slide-left-leave-to {
  opacity: 0;
  transform: translateX(30px);
}
</style>
