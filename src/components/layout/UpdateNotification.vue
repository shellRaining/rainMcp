<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'

const showNotification = ref(false)
const updateVersion = ref('')
const updateNotes = ref('')
const isDownloading = ref(false)
const downloadProgress = ref(0)
let pendingUpdate: any = null

// 开发模式：设置为 true 可以模拟下载过程而不真正安装
const DEV_MODE = import.meta.env.DEV

onMounted(async () => {
  await checkForUpdates()
})

async function checkForUpdates() {
  try {
    const result = await check()
    if (result) {
      pendingUpdate = result
      updateVersion.value = result.version
      updateNotes.value = result.body || ''
      showNotification.value = true
    }
  } catch (error) {
    console.error('检查更新失败:', error)
  }
}

async function installUpdate() {
  if (DEV_MODE) {
    // 开发模式：模拟下载进度
    await simulateDownload()
    return
  }

  if (!pendingUpdate) {
    await checkForUpdates()
    if (!pendingUpdate) return
  }
  
  isDownloading.value = true
  downloadProgress.value = 0
  
  try {
    let downloaded = 0
    let contentLength = 0
    
    await pendingUpdate.downloadAndInstall((event: any) => {
      switch (event.event) {
        case 'Started':
          contentLength = event.data.contentLength || 0
          break
        case 'Progress':
          downloaded += event.data.chunkLength
          if (contentLength > 0) {
            downloadProgress.value = Math.round((downloaded / contentLength) * 100)
          }
          break
        case 'Finished':
          downloadProgress.value = 100
          break
      }
    })
    
    await relaunch()
  } catch (error) {
    console.error('更新安装失败:', error)
    isDownloading.value = false
  }
}

async function simulateDownload() {
  isDownloading.value = true
  downloadProgress.value = 0
  
  // 模拟下载进度
  for (let i = 0; i <= 100; i += 10) {
    downloadProgress.value = i
    await new Promise(resolve => setTimeout(resolve, 200))
  }
  
  // 模拟完成后关闭通知
  setTimeout(() => {
    console.log('模拟更新完成')
    isDownloading.value = false
    showNotification.value = false
  }, 500)
}

function dismissNotification() {
  showNotification.value = false
}
</script>

<template>
  <Transition name="slide-down">
    <Card
      v-if="showNotification"
      class="fixed top-4 left-1/2 -translate-x-1/2 z-50 w-[600px] p-4 shadow-lg border-2 border-primary/20"
    >
      <div class="flex items-start gap-4">
        <div class="flex-1">
          <div class="flex items-center gap-2 mb-1">
            <span class="text-sm font-medium">发现新版本 {{ updateVersion }}</span>
          </div>
          <p v-if="updateNotes" class="text-xs text-muted-foreground line-clamp-2">
            {{ updateNotes }}
          </p>
          
          <div v-if="isDownloading" class="mt-3">
            <div class="flex items-center gap-2 text-xs text-muted-foreground mb-1">
              <span>下载中...</span>
              <span>{{ downloadProgress }}%</span>
            </div>
            <div class="w-full h-1.5 bg-secondary rounded-full overflow-hidden">
              <div 
                class="h-full bg-primary transition-all duration-300"
                :style="{ width: `${downloadProgress}%` }"
              />
            </div>
          </div>
        </div>
        
        <div class="flex items-center gap-2">
          <Button
            v-if="!isDownloading"
            variant="ghost"
            size="sm"
            @click="dismissNotification"
          >
            稍后
          </Button>
          <Button
            size="sm"
            :disabled="isDownloading"
            @click="installUpdate"
          >
            {{ isDownloading ? '更新中...' : '立即更新' }}
          </Button>
        </div>
      </div>
    </Card>
  </Transition>
</template>

<style scoped>
.slide-down-enter-active,
.slide-down-leave-active {
  transition: all 0.3s ease;
}

.slide-down-enter-from {
  opacity: 0;
  transform: translate(-50%, -20px);
}

.slide-down-leave-to {
  opacity: 0;
  transform: translate(-50%, -20px);
}
</style>
