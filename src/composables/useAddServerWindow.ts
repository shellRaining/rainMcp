import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { listen } from '@tauri-apps/api/event';
import { useServersStore } from '@/stores/servers';
import { logger } from '@/utils/logger';

let addServerWindow: WebviewWindow | null = null;
const addServerWindowLabel = 'add-server';

export function useAddServerWindow() {
  async function focusExistingWindow(window: WebviewWindow) {
    addServerWindow = window;
    try {
      await window.show();
    } catch (error) {
      logger.debug('[useAddServerWindow] Failed to show existing window:', error);
    }
    try {
      await window.setFocus();
    } catch (error) {
      logger.debug('[useAddServerWindow] Failed to focus existing window:', error);
    }
    return window;
  }

  async function openAddServerWindow() {
    logger.debug('[useAddServerWindow] openAddServerWindow called');

    // 如果窗口已存在，聚焦它
    if (addServerWindow) {
      logger.debug('[useAddServerWindow] Window already exists, focusing');
      try {
        return await focusExistingWindow(addServerWindow);
      } catch (error) {
        logger.error('[useAddServerWindow] Error focusing window:', error);
        // 窗口可能已关闭，重置状态
        addServerWindow = null;
      }
    }

    const existingWindow = await WebviewWindow.getByLabel(addServerWindowLabel);
    if (existingWindow) {
      logger.debug('[useAddServerWindow] Window exists by label, focusing');
      return await focusExistingWindow(existingWindow);
    }

    try {
      logger.debug('[useAddServerWindow] Creating new window');

      // 创建窗口，样式与主窗口一致
      addServerWindow = new WebviewWindow(addServerWindowLabel, {
        url: '/add-server',
        title: 'Add MCP Server',
        width: 800,
        height: 600,
        minWidth: 800,
        minHeight: 600,
        center: true,
        resizable: true,
        minimizable: true,
        maximizable: false,
        decorations: true,
        titleBarStyle: 'overlay', // 与主窗口一致
        hiddenTitle: true, // 与主窗口一致
        visible: true,
        focus: true,
      });

      logger.debug('[useAddServerWindow] Window created:', addServerWindow);

      // 等待窗口加载完成
      await addServerWindow.once('tauri://created', () => {
        logger.debug('[useAddServerWindow] Window tauri://created event');
      });

      await addServerWindow.once('tauri://error', (e) => {
        logger.error('[useAddServerWindow] Window error event:', e);
      });

      // 监听窗口关闭
      const unlistenClose = await addServerWindow.onCloseRequested(async () => {
        logger.debug('[useAddServerWindow] Window close requested');
        addServerWindow = null;
        unlistenClose();
      });

      // 监听服务器添加成功事件
      const unlistenAdded = await listen('server-added', async () => {
        logger.info('[useAddServerWindow] Server added event received');
        // 刷新主窗口的服务器列表
        const serversStore = useServersStore();
        await serversStore.fetchUserServers();
        unlistenAdded();
      });

      logger.debug('[useAddServerWindow] Window setup complete');
      return addServerWindow;
    } catch (error) {
      logger.error('[useAddServerWindow] Error creating window:', error);
      const fallbackWindow = await WebviewWindow.getByLabel(addServerWindowLabel);
      if (fallbackWindow) {
        logger.debug('[useAddServerWindow] Fallback to existing window after create error');
        return await focusExistingWindow(fallbackWindow);
      }
      addServerWindow = null;
      throw error;
    }
  }

  return {
    openAddServerWindow,
  };
}
