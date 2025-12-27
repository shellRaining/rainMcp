import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useAppStore = defineStore('app', () => {
  const currentView = ref('home');

  function setCurrentView(view: string) {
    currentView.value = view;
  }

  return {
    currentView,
    setCurrentView,
  };
});
