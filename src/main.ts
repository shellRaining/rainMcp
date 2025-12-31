import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { attachConsole } from '@tauri-apps/plugin-log';
import App from './App.vue';
import './styles/globals.css';
import 'virtual:uno.css';

attachConsole();

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);

// 初始化主题（必须在 pinia 注册后、mount 前）
import { useThemeStore } from './stores/theme';
useThemeStore();

app.mount('#app');
