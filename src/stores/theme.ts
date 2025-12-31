import { defineStore } from 'pinia';
import { usePreferredDark, useStorage } from '@vueuse/core';
import { computed, watch } from 'vue';

export type Theme = 'light' | 'dark' | 'system';

export const useThemeStore = defineStore('theme', () => {
  const prefersDark = usePreferredDark();

  // 用户选择的主题（持久化存储）
  const theme = useStorage<Theme>('rain-mcp-theme', 'system');

  // 实际应用的主题（考虑系统偏好）
  const isDark = computed(() => {
    if (theme.value === 'system') {
      return prefersDark.value;
    }
    return theme.value === 'dark';
  });

  // 监听实际主题变化，更新 DOM class
  watch(
    isDark,
    (dark) => {
      if (dark) {
        document.documentElement.classList.remove('light');
        document.documentElement.classList.add('dark');
      } else {
        document.documentElement.classList.remove('dark');
        document.documentElement.classList.add('light');
      }
    },
    { immediate: true }
  );

  function toggleTheme() {
    if (isDark.value) {
      theme.value = 'light';
    } else {
      theme.value = 'dark';
    }
  }

  function setTheme(newTheme: Theme) {
    theme.value = newTheme;
  }

  return {
    theme,
    isDark,
    toggleTheme,
    setTheme,
  };
});
