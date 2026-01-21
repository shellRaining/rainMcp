<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import MarkdownRender from 'markstream-vue';
import { Button } from '@/components/ui/button';
import { Card } from '@/components/ui/card';

const showBadge = ref(false);
const showModal = ref(false);
const updateVersion = ref('');
const updateNotes = ref('');
const isDownloading = ref(false);
const downloadProgress = ref(0);
const errorMessage = ref('');
let pendingUpdate: any = null;

onMounted(async () => {
  await checkForUpdates();
});

async function checkForUpdates() {
  try {
    const result = await check();
    if (result) {
      pendingUpdate = result;
      updateVersion.value = result.version;
      updateNotes.value = result.body || '';
      showBadge.value = true;
    }
  } catch (error) {
    console.error('检查更新失败:', error);
  }
}

async function installUpdate() {
  errorMessage.value = '';

  if (!pendingUpdate) {
    await checkForUpdates();
    if (!pendingUpdate) return;
  }

  isDownloading.value = true;
  downloadProgress.value = 0;

  try {
    let downloaded = 0;
    let contentLength = 0;

    await pendingUpdate.downloadAndInstall((event: any) => {
      switch (event.event) {
        case 'Started':
          contentLength = event?.data?.contentLength || 0;
          break;
        case 'Progress':
          downloaded += event?.data?.chunkLength || 0;
          if (contentLength > 0) {
            downloadProgress.value = Math.round((downloaded / contentLength) * 100);
          }
          break;
        case 'Finished':
          downloadProgress.value = 100;
          break;
      }
    });

    try {
      await relaunch();
    } catch (error) {
      console.warn('重启应用失败（更新可能已安装）:', error);
      errorMessage.value = '更新已安装，请手动重启应用以生效。';
      isDownloading.value = false;
    }
  } catch (error) {
    console.error('更新安装失败:', error);
    isDownloading.value = false;
    errorMessage.value = `更新失败：${String((error as any)?.message || error)}`;
  }
}

function dismissNotification() {
  showModal.value = false;
}

function openModal() {
  showModal.value = true;
}
</script>

<template>
  <Transition name="badge-fade">
    <button
      v-if="showBadge && !showModal"
      class="fixed top-4 right-4 z-50 px-3 py-1 -m-1 bg-primary text-primary-foreground text-xs font-medium rounded-full shadow-lg hover:bg-primary/90 transition-colors leading-normal"
      @click="openModal"
    >
      发现新版本 {{ updateVersion }}
    </button>
  </Transition>

  <Transition name="modal-fade">
    <div
      v-if="showModal"
      class="fixed inset-0 z-50 flex items-start justify-center pt-20 bg-black/50"
      @click.self="dismissNotification"
    >
      <Card class="w-[600px] p-6 shadow-xl">
        <div class="flex items-start gap-4">
          <div class="flex-1">
            <div class="flex items-center gap-2 mb-3">
              <span class="text-base font-semibold">发现新版本 {{ updateVersion }}</span>
            </div>
            <div
              v-if="updateNotes"
              class="update-notes text-sm text-muted-foreground max-h-96 overflow-y-auto mb-4"
            >
              <MarkdownRender :content="updateNotes" />
            </div>

            <div v-if="isDownloading" class="mt-4">
              <div class="flex items-center gap-2 text-sm text-muted-foreground mb-2">
                <span>下载中...</span>
                <span>{{ downloadProgress }}%</span>
              </div>
              <div class="w-full h-2 bg-secondary rounded-full overflow-hidden">
                <div
                  class="h-full bg-primary transition-all duration-300"
                  :style="{ width: `${downloadProgress}%` }"
                />
              </div>
            </div>

            <p
              v-if="errorMessage"
              class="mt-3 text-sm text-destructive whitespace-pre-wrap break-words"
            >
              {{ errorMessage }}
            </p>
          </div>
        </div>

        <div class="flex items-center justify-end gap-2 mt-4">
          <Button v-if="!isDownloading" variant="ghost" @click="dismissNotification"> 稍后 </Button>
          <Button :disabled="isDownloading" @click="installUpdate">
            {{ isDownloading ? '更新中...' : '立即更新' }}
          </Button>
        </div>
      </Card>
    </div>
  </Transition>
</template>

<style scoped>
.badge-fade-enter-active,
.badge-fade-leave-active {
  transition: all 0.3s ease;
}

.badge-fade-enter-from,
.badge-fade-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}

.modal-fade-enter-active .card,
.modal-fade-leave-active .card {
  transition: transform 0.3s ease;
}

.modal-fade-enter-from .card {
  transform: scale(0.95);
}

.modal-fade-leave-to .card {
  transform: scale(0.95);
}

.update-notes :deep(.markstream-vue) {
  font-size: inherit;
  color: inherit;
}

.update-notes :deep(.heading-node),
.update-notes :deep(.heading-1),
.update-notes :deep(.heading-2),
.update-notes :deep(.heading-3),
.update-notes :deep(.heading-4),
.update-notes :deep(.heading-5),
.update-notes :deep(.heading-6) {
  font-size: 0.875rem !important;
  line-height: 1.4 !important;
  margin: 0.5rem 0 !important;
  font-weight: 600 !important;
}

.update-notes :deep(.heading-1) {
  font-size: 1rem !important;
}

.update-notes :deep(.paragraph-node) {
  margin: 0.5rem 0 !important;
}

.update-notes :deep(.blockquote) {
  margin: 0 !important;
  border-left: 0 !important;
  padding-left: 0 !important;
  font-style: normal !important;
}

.update-notes :deep(.list-node) {
  margin: 0.5rem 0 !important;
  padding-left: 1.25em !important;
}

.update-notes :deep(.list-item-node) {
  margin: 0.25rem 0 !important;
}

.update-notes :deep(hr) {
  border-color: hsl(var(--border)) !important;
  opacity: 0.5 !important;
  margin: 0.75rem 0 !important;
}

.update-notes :deep(details) {
  overflow: hidden;
}

.update-notes :deep(details summary) {
  cursor: pointer;
  user-select: none;
}

.update-notes :deep(details[open] > *:not(summary)) {
  animation: slideDown 0.3s ease;
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.update-notes :deep(.code-block-container),
.update-notes :deep(.code-editor-container) {
  max-height: 8rem;
  overflow: auto;
}
</style>
