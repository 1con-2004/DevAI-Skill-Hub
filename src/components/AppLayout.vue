<template>
  <a-layout class="app-layout">
    <a-layout-sider
      v-model:collapsed="collapsed"
      :width="200"
      :collapsed-width="64"
      :trigger="null"
      collapsible
      class="app-sider"
    >
      <div class="sider-logo">
        <span v-if="!collapsed">DevAI Skill Hub</span>
        <span v-else>DSH</span>
      </div>

      <nav class="sider-menu">
        <template v-for="item in menuItems" :key="item.key">
          <a-tooltip v-if="collapsed" :title="item.label" placement="right">
            <router-link
              :to="item.path"
              class="sider-menu-item"
              :class="{ 'sider-menu-item--active': selectedKey === item.key }"
            >
              <component :is="item.icon" class="sider-menu-icon" />
            </router-link>
          </a-tooltip>
          <router-link
            v-else
            :to="item.path"
            class="sider-menu-item"
            :class="{ 'sider-menu-item--active': selectedKey === item.key }"
          >
            <component :is="item.icon" class="sider-menu-icon" />
            <span class="sider-menu-label">{{ item.label }}</span>
          </router-link>
        </template>
      </nav>

      <div class="sider-footer">
        <button class="sider-collapse-btn" @click="collapsed = !collapsed">
          <MenuFoldOutlined v-if="!collapsed" />
          <MenuUnfoldOutlined v-else />
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
              <template #icon><ReloadOutlined :spin="loading" /></template>
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
import { ref, computed, watch } from 'vue'
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

.app-sider {
  background: #001529 !important;
  display: flex;
  flex-direction: column;
}

.app-sider :deep(.ant-layout-sider-children) {
  display: flex;
  flex-direction: column;
  flex: 1;
}

.sider-logo {
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 16px;
  font-weight: bold;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.sider-menu {
  flex: 1;
  padding: 8px 0;
  overflow-y: auto;
}

.sider-menu-item {
  display: flex;
  align-items: center;
  padding: 12px 24px;
  color: rgba(255, 255, 255, 0.65);
  text-decoration: none;
  transition: all 0.3s;
  margin: 2px 8px;
  border-radius: 4px;
}

.sider-menu-item:hover {
  color: #fff;
  background: rgba(255, 255, 255, 0.08);
}

.sider-menu-item--active {
  color: #fff !important;
  background: #1890ff !important;
}

.sider-menu-item--active:hover {
  background: #40a9ff !important;
}

.sider-menu-icon {
  font-size: 18px;
  flex-shrink: 0;
}

.sider-menu-label {
  margin-left: 12px;
  white-space: nowrap;
}

.app-sider :deep(.ant-layout-sider-collapsed) .sider-menu-item {
  justify-content: center;
  padding: 12px 0;
}

.sider-footer {
  padding: 8px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  flex-shrink: 0;
}

.sider-collapse-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: rgba(255, 255, 255, 0.65);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.3s;
}

.sider-collapse-btn:hover {
  color: #fff;
  background: rgba(255, 255, 255, 0.08);
}

.sider-collapse-text {
  margin-left: 8px;
}

.app-header {
  background: #fff;
  padding: 0 24px;
  display: flex;
  align-items: center;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.1);
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
}

.header-actions {
  display: flex;
  gap: 8px;
}

.app-content {
  margin: 16px;
  min-height: calc(100vh - 64px - 32px);
}
</style>
