<template>
  <div class="dashboard">
    <!-- 主内容区 -->
    <main class="main-content">
      <a-row :gutter="[24, 24]">
        <!-- Skill 使用排行 -->
        <a-col :span="24">
          <div class="content-card">
            <div class="card-header">
              <h2 class="card-title">Skill 使用排行</h2>
            </div>
            <a-table
              :columns="rankColumns"
              :data-source="statsBySkill"
              :loading="loading"
              :pagination="false"
              row-key="skill_id"
              size="middle"
            >
              <template #bodyCell="{ column, record, index }">
                <template v-if="column.key === 'rank'">
                  <span class="rank-badge">#{{ index + 1 }}</span>
                </template>
                <template v-if="column.key === 'skill_id'">
                  <span class="skill-name">{{ record.skill_id }}</span>
                </template>
                <template v-if="column.key === 'tags'">
                  <span class="tag tag-ai" v-if="record.ai > 0">AI {{ record.ai }}</span>
                  <span class="tag tag-manual" v-if="record.user > 0" style="margin-left: 6px">手动 {{ record.user }}</span>
                </template>
                <template v-if="column.key === 'total'">
                  <span class="total-count">{{ record.total }}</span>
                </template>
              </template>
              <template #emptyText>
                <div class="empty-state">
                  <Package :size="40" :stroke-width="1" class="empty-icon" />
                  <p class="empty-text">暂无使用数据</p>
                </div>
              </template>
            </a-table>
          </div>
        </a-col>
      </a-row>
    </main>

    <!-- 右侧边栏 - 最近执行记录 -->
    <aside class="sidebar">
      <div class="sidebar-header">
        <h2 class="sidebar-title">最新执行记录</h2>
        <span class="record-count">{{ executions.length }} 条</span>
      </div>
      <div class="sidebar-content">
        <div
          v-for="(item, index) in executions"
          :key="index"
          class="record-item"
        >
          <div class="record-header">
            <span :class="['tag', item.trigger_type === 'user' ? 'tag-manual' : 'tag-ai']">
              {{ item.trigger_type === 'user' ? '手动' : 'AI' }}
            </span>
            <span class="record-time">{{ formatTime(item.timestamp) }}</span>
          </div>
          <div class="record-skill">{{ item.skill_id }}</div>
          <div v-if="item.args" class="record-args">{{ item.args }}</div>
        </div>
        <div v-if="executions.length === 0" class="empty-state">
          <ListTodo :size="40" :stroke-width="1" class="empty-icon" />
          <p class="empty-text">暂无执行记录</p>
        </div>
      </div>
    </aside>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue'
import { useSkillStore } from '@/stores/skill'
import { Package, ListTodo } from 'lucide-vue-next'
import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'
import utc from 'dayjs/plugin/utc'
import timezone from 'dayjs/plugin/timezone'

dayjs.extend(relativeTime)
dayjs.extend(utc)
dayjs.extend(timezone)

const store = useSkillStore()

const rankColumns = [
  { title: '排名', key: 'rank', width: 80, align: 'center' },
  { title: 'Skill', key: 'skill_id', dataIndex: 'skill_id' },
  { title: '触发方式', key: 'tags', width: 180 },
  { title: '总次数', key: 'total', dataIndex: 'total', width: 100, align: 'right' }
]

function formatTime(timestamp: string) {
  const tz = localStorage.getItem('skill-hub-timezone') || 'local'
  try {
    if (tz === 'local') {
      return dayjs(timestamp).format('MM-DD HH:mm')
    }
    return dayjs.utc(timestamp).tz(tz).format('MM-DD HH:mm')
  } catch {
    return dayjs(timestamp).format('MM-DD HH:mm')
  }
}

const loading = computed(() => store.loading)
const statsBySkill = computed(() => store.statsBySkill)
const executions = computed(() => store.executions.slice(0, 20))

onMounted(() => {
  store.startAutoRefresh()
})

onUnmounted(() => {
  store.stopAutoRefresh()
})
</script>

<style scoped>
.dashboard {
  display: flex;
  gap: 24px;
  padding: 0;
  min-height: calc(100vh - 64px - 32px);
}

/* ========== 主内容区 ========== */
.main-content {
  flex: 1;
  min-width: 0;
}

/* ========== 通用卡片 ========== */
.content-card {
  background: #fff;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.card-title {
  font-size: 15px;
  font-weight: 600;
  color: #1e293b;
  margin: 0;
}

/* ========== 表格样式 ========== */
:deep(.ant-table) {
  font-size: 14px;
}

:deep(.ant-table-thead > tr > th) {
  background: #f8fafc;
  font-weight: 500;
  color: #64748b;
  font-size: 13px;
  border-bottom: 1px solid #e2e8f0;
}

:deep(.ant-table-tbody > tr > td) {
  border-bottom: 1px solid #f1f5f9;
}

:deep(.ant-table-tbody > tr:hover > td) {
  background: #f8fafc !important;
}

.rank-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: 50%;
  background: #3b82f6;
  color: #fff;
  font-size: 12px;
  font-weight: 600;
}

.skill-name {
  font-weight: 500;
  color: #334155;
}

.total-count {
  font-weight: 600;
  color: #3b82f6;
}

/* ========== 右侧边栏 ========== */
.sidebar {
  width: 320px;
  flex-shrink: 0;
  background: #fff;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  display: flex;
  flex-direction: column;
  max-height: calc(100vh - 64px - 64px);
  position: sticky;
  top: 16px;
}

.sidebar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid #e2e8f0;
}

.sidebar-title {
  font-size: 15px;
  font-weight: 600;
  color: #1e293b;
  margin: 0;
}

.record-count {
  font-size: 12px;
  color: #94a3b8;
  background: #f1f5f9;
  padding: 2px 8px;
  border-radius: 4px;
}

.sidebar-content {
  flex: 1;
  overflow-y: auto;
  padding: 8px 0;
}

.sidebar-content::-webkit-scrollbar {
  width: 4px;
}

.sidebar-content::-webkit-scrollbar-thumb {
  background: #e2e8f0;
  border-radius: 2px;
}

.record-item {
  padding: 12px 20px;
  border-bottom: 1px solid #f1f5f9;
  transition: background 0.15s ease;
}

.record-item:last-child {
  border-bottom: none;
}

.record-item:hover {
  background: #f8fafc;
}

.record-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 6px;
}

.record-time {
  font-size: 12px;
  color: #94a3b8;
}

.record-skill {
  font-size: 14px;
  font-weight: 500;
  color: #334155;
}

.record-args {
  font-size: 12px;
  color: #64748b;
  margin-top: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* ========== 标签 ========== */
.tag {
  display: inline-flex;
  align-items: center;
  padding: 2px 8px;
  border-radius: 999px;
  font-size: 11px;
  font-weight: 500;
}

.tag-ai {
  background: #eff6ff;
  color: #3b82f6;
}

.tag-manual {
  background: #f0fdf4;
  color: #16a34a;
}

/* ========== 空状态 ========== */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 48px 24px;
  text-align: center;
}

.empty-icon {
  color: #cbd5e1;
  margin-bottom: 12px;
}

.empty-text {
  color: #94a3b8;
  font-size: 14px;
  margin: 0;
}
</style>
