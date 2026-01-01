import { createRouter, createWebHistory } from 'vue-router';

const routes = [
  {
    path: '/',
    name: 'main',
    component: () => import('@/App.vue'),
  },
  {
    path: '/add-server',
    name: 'add-server',
    component: () => import('@/views/add-server/AddServerWindow.vue'),
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
