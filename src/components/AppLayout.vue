<template>
  <a-layout class="app-layout">
    <a-layout-sider
      v-model:collapsed="collapsed"
      :width="220"
      :collapsed-width="72"
      :trigger="null"
      collapsible
      class="app-sider"
    >
      <div class="sider-logo">
        <div class="logo-icon">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M12 2L2 7L12 12L22 7L12 2Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M2 17L12 22L22 17" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M2 12L12 17L22 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
        <span v-if="!collapsed" class="logo-text">DevAI Skill Hub</span>
      </div>

      <nav class="sider-menu">
        <template v-for="item in menuItems" :key="item.key">
          <a-tooltip v-if="collapsed" :title="item.label" placement="right">
            <div
              class="sider-menu-item"
              :class="{ 'sider-menu-item--active': selectedKey === item.key }"
            >
              <router-link :to="item.path" class="sider-menu-link">
                <component :is="item.icon" class="sider-menu-icon" />
              </router-link>
            </div>
          </a-tooltip>
          <div
            v-else
            class="sider-menu-item"
            :class="{ 'sider-menu-item--active': selectedKey === item.key }"
          >
            <router-link :to="item.path" class="sider-menu-link">
              <component :is="item.icon" class="sider-menu-icon" />
              <span class="sider-menu-label">{{ item.label }}</span>
            </router-link>
          </div>
        </template>
      </nav>

      <div class="sider-footer">
        <button class="sider-collapse-btn" @click="collapsed = !collapsed">
          <MenuFoldOutlined v-if="!collapsed" class="sider-menu-icon" />
          <MenuUnfoldOutlined v-else class="sider-menu-icon" />
          <span v-if="!collapsed" class="sider-collapse-text">收起</span>
        </button>
      </div>
    </a-layout-sider>

    <a-layout>
      <a-layout-header class="app-header">
        <div class="header-inner">
          <div class="header-title">{{ currentTitle }}</div>
          <div class="header-actions">
            <a-button type="text" @click="refreshData" :loading="loading">
              <template #icon><ReloadOutlined /></template>
            </a-button>
          </div>
        </div>
      </a-layout-header>

      <a-layout-content class="app-content">
        <router-view />
      </a-layout-content>
    </a-layout>
  </a-layout>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRoute } from 'vue-router'
import { useSkillStore } from '@/stores/skill'
import {
  DashboardOutlined,
  SettingOutlined,
  ToolOutlined,
  MenuUnfoldOutlined,
  MenuFoldOutlined,
  ReloadOutlined
} from '@ant-design/icons-vue'

const route = useRoute()
const store = useSkillStore()

const collapsed = ref(false)
const loading = computed(() => store.loading)

const menuItems = [
  { key: 'dashboard', path: '/dashboard', icon: DashboardOutlined, label: '监控面板' },
  { key: 'skills', path: '/skills', icon: SettingOutlined, label: 'Skill 配置' },
  { key: 'settings', path: '/settings', icon: ToolOutlined, label: '设置' }
]

const routeTitles: Record<string, string> = {
  Dashboard: '监控面板',
  Skills: 'Skill 配置',
  Settings: '设置'
}

const currentTitle = computed(() => {
  return routeTitles[route.name as string] || 'DevAI Skill Hub'
})

const selectedKey = computed(() => {
  const matched = menuItems.find(item => route.path.startsWith(item.path))
  return matched ? matched.key : 'dashboard'
})

async function refreshData() {
  await store.fetchAll()
}
</script>

<style scoped>
.app-layout {
  min-height: 100vh;
}

/* ========== 侧边栏核心样式 ========== */
.app-sider {
  background: #1e293b !important;
  display: flex;
  flex-direction: column;
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.15);
}

.app-sider :deep(.ant-layout-sider-children) {
  display: flex;
  flex-direction: column;
  flex: 1;
}

/* ========== Logo 区域 ========== */
.sider-logo {
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  padding: 0 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}

.logo-icon {
  color: #60a5fa;
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.logo-text {
  color: #f1f5f9;
  font-size: 15px;
  font-weight: 600;
  white-space: nowrap;
  letter-spacing: -0.01em;
}

/* ========== 菜单区域 ========== */
.sider-menu {
  flex: 1;
  padding: 12px 8px;
  overflow-y: auto;
}

.sider-menu::-webkit-scrollbar {
  width: 4px;
}

.sider-menu::-webkit-scrollbar-track {
  background: transparent;
}

.sider-menu::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.15);
  border-radius: 2px;
}

.sider-menu-item {
  margin: 2px 0;
  border-radius: 8px;
  transition: all 0.2s ease;
}

.sider-menu-item:hover {
  background: rgba(255, 255, 255, 0.06);
}

/* 选中态 - 柔和浅色高亮 */
.sider-menu-item--active {
  background: rgba(96, 165, 250, 0.12) !important;
}

/* ========== 菜单链接 ========== */
.sider-menu-link {
  display: flex;
  align-items: center;
  padding: 10px 16px;
  color: #94a3b8;
  text-decoration: none;
  border-radius: 8px;
  font-weight: 450;
  font-size: 15px;
  transition: all 0.2s ease;
  white-space: nowrap;
}

.sider-menu-link:hover {
  color: #e2e8f0;
}

/* 选中态 */
.sider-menu-item--active .sider-menu-link {
  color: #60a5fa !important;
  font-weight: 500;
}

.sider-menu-item--active:hover {
  background: rgba(96, 165, 250, 0.18) !important;
}

/* 收起状态图标居中 */
.app-sider :deep(.ant-layout-sider-collapsed) .sider-menu-link {
  justify-content: center;
  padding: 10px 0;
}

/* ========== 图标样式 ========== */
.sider-menu-icon {
  font-size: 18px;
  flex-shrink: 0;
  min-width: 18px;
}

.sider-menu-label {
  margin-left: 12px;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* ========== 底部收起按钮 ========== */
.sider-footer {
  padding: 12px 8px;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  flex-shrink: 0;
}

.sider-collapse-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  width: 100%;
  padding: 10px 12px;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: #94a3b8;
  font-size: 14px;
  font-weight: 450;
  cursor: pointer;
  transition: all 0.2s ease;
}

.sider-collapse-btn:hover {
  color: #e2e8f0;
  background: rgba(255, 255, 255, 0.06);
}

.sider-collapse-text {
  white-space: nowrap;
}

/* ========== 顶部 Header ========== */
.app-header {
  background: #fff;
  padding: 0 24px;
  display: flex;
  align-items: center;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
}

.header-inner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.header-title {
  font-size: 16px;
  font-weight: 500;
  color: #1e293b;
}

.header-actions {
  display: flex;
  gap: 8px;
}

/* ========== 内容区域 ========== */
.app-content {
  margin: 16px;
  min-height: calc(100vh - 64px - 32px);
}
</style>
