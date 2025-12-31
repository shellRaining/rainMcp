import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export type PrimaryMenu = 'overview' | 'agents' | 'servers' | 'settings';

export const useAppStore = defineStore('app', () => {
  // 当前选中的一级菜单（决定右侧显示哪个总览/详情）
  const currentPrimaryMenu = ref<PrimaryMenu>('overview');
  // 当前展开的一级菜单（互斥，只能有一个）
  const expandedMenu = ref<PrimaryMenu | null>(null);
  // 选中的二级项 ID（agent name / server name / settings submenu）
  const selectedDetailId = ref<string | null>(null);

  // 是否在查看详情（有选中的二级项）
  const isViewingDetail = computed(() => selectedDetailId.value !== null);

  // 点击一级菜单
  function clickPrimaryMenu(menu: PrimaryMenu) {
    currentPrimaryMenu.value = menu;
    selectedDetailId.value = null; // 清除二级选择，显示总览

    if (menu === 'overview') {
      expandedMenu.value = null;
    } else {
      // 互斥展开：点击已展开的则收起，否则展开新的
      expandedMenu.value = expandedMenu.value === menu ? null : menu;
    }
  }

  // 点击二级菜单项
  function clickDetailItem(menu: PrimaryMenu, detailId: string) {
    currentPrimaryMenu.value = menu;
    selectedDetailId.value = detailId;
    expandedMenu.value = menu;
  }

  // 检查某个菜单是否展开
  function isMenuExpanded(menu: PrimaryMenu): boolean {
    return expandedMenu.value === menu;
  }

  return {
    currentPrimaryMenu,
    expandedMenu,
    selectedDetailId,
    isViewingDetail,
    clickPrimaryMenu,
    clickDetailItem,
    isMenuExpanded,
  };
});
