<template>
  <a-layout class="app-layout">
    <a-layout-sider
      v-model:collapsed="collapsed"
      :trigger="null"
      collapsible
      class="app-sider"
    >
      <div class="logo">
        <span v-if="!collapsed">DevAI Skill Hub</span>
        <span v-else>DSH</span>
      </div>
      <a-menu
        v-model:selectedKeys="selectedKeys"
        theme="dark"
        mode="inline"
        @click="handleMenuClick"
      >
        <a-menu-item key="dashboard">
          <DashboardOutlined />
          <span>监控面板</span>
        </a-menu-item>
        <a-menu-item key="skills">
          <SettingOutlined />
          <span>Skill 配置</span>
        </a-menu-item>
        <a-menu-item key="settings">
          <ToolOutlined />
          <span>设置</span>
        </a-menu-item>
      </a-menu>
    </a-layout-sider>

    <a-layout>
      <a-layout-header class="app-header">
        <a-button
          type="text"
          class="collapse-btn"
          @click="collapsed = !collapsed"
        >
          <MenuUnfoldOutlined v-if="collapsed" />
          <MenuFoldOutlined v-else />
        </a-button>
        <div class="header-title">{{ currentTitle }}</div>
        <div class="header-actions">
          <a-button type="text" @click="refreshData" :loading="loading">
            <ReloadOutlined :spin="loading" />
          </a-button>
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
import { useRouter, useRoute } from 'vue-router'
import { useSkillStore } from '@/stores/skill'
import {
  DashboardOutlined,
  SettingOutlined,
  ToolOutlined,
  MenuUnfoldOutlined,
  MenuFoldOutlined,
  ReloadOutlined
} from '@ant-design/icons-vue'

const router = useRouter()
const route = useRoute()
const store = useSkillStore()

const collapsed = ref(false)
const selectedKeys = ref<string[]>(['dashboard'])

const loading = computed(() => store.loading)

const routeTitles: Record<string, string> = {
  Dashboard: '监控面板',
  Skills: 'Skill 配置',
  Settings: '设置'
}

const currentTitle = computed(() => {
  return routeTitles[route.name as string] || 'DevAI Skill Hub'
})

watch(
  () => route.name,
  (name) => {
    if (name) {
      selectedKeys.value = [name.toLowerCase()]
    }
  },
  { immediate: true }
)

function handleMenuClick({ key }: { key: string }) {
  router.push(`/${key}`)
}

async function refreshData() {
  await store.fetchAll()
}
</script>

<style scoped>
.app-layout {
  min-height: 100vh;
}

.app-sider {
  background: #001529;
}

.logo {
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 16px;
  font-weight: bold;
  padding: 0 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.app-header {
  background: #fff;
  padding: 0 16px;
  display: flex;
  align-items: center;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.1);
}

.collapse-btn {
  font-size: 18px;
}

.header-title {
  flex: 1;
  margin-left: 16px;
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
