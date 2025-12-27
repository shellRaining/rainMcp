import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { attachConsole } from '@tauri-apps/plugin-log';
import App from './App.vue';
import './styles/reset.css';
import 'virtual:uno.css';

attachConsole();

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);
app.mount('#app');
