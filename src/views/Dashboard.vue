<template>
  <div class="dashboard">
    <a-row :gutter="[16, 16]">
      <a-col :span="24">
        <h1>Skill 监控面板</h1>
      </a-col>
    </a-row>

    <a-row :gutter="[16, 16]">
      <a-col :xs="24" :sm="12" :md="6" v-for="stat in topStats" :key="stat.skill_id">
        <a-card class="stat-card">
          <a-statistic
            :title="stat.skill_id"
            :value="stat.total"
            :value-style="{ color: '#1890ff' }"
          >
            <template #suffix>
              <span class="stat-suffix">次</span>
            </template>
          </a-statistic>
          <div class="stat-detail">
            <a-tag color="blue">AI触发 {{ stat.ai }}</a-tag>
            <a-tag color="green">手动 {{ stat.user }}</a-tag>
          </div>
        </a-card>
      </a-col>
    </a-row>

    <a-row :gutter="[16, 16]" style="margin-top: 16px">
      <a-col :xs="24" :lg="12">
        <a-card title="Skill 使用排行" :loading="loading">
          <a-list
            :data-source="statsBySkill"
            :loading="loading"
            size="small"
          >
            <template #renderItem="{ item, index }">
              <a-list-item>
                <a-list-item-meta>
                  <template #title>
                    <a-space>
                      <span class="skill-rank">#{{ index + 1 }}</span>
                      <span class="skill-name">{{ item.skill_id }}</span>
                    </a-space>
                  </template>
                  <template #description>
                    <a-space>
                      <a-tag v-if="item.ai > 0" color="blue">AI {{ item.ai }}</a-tag>
                      <a-tag v-if="item.user > 0" color="green">手动 {{ item.user }}</a-tag>
                    </a-space>
                  </template>
                </a-list-item-meta>
                <template #actions>
                  <a-button type="link" size="small">
                    {{ item.total }} 次
                  </a-button>
                </template>
              </a-list-item>
            </template>
            <template #emptyText>
              <a-empty description="暂无数据，请先使用 Claude Code">
                <template #image>
                  <InboxOutlined style="font-size: 48px; color: #ccc" />
                </template>
              </a-empty>
            </template>
          </a-list>
        </a-card>
      </a-col>

      <a-col :xs="24" :lg="12">
        <a-card title="最近执行记录" :loading="loading">
          <a-list
            :data-source="recentExecutions"
            :loading="loading"
            size="small"
          >
            <template #renderItem="{ item }">
              <a-list-item>
                <a-list-item-meta>
                  <template #title>
                    <a-space>
                      <a-tag color="processing">{{ item.skill_id }}</a-tag>
                      <a-tag v-if="item.trigger_type === 'user'" color="success">手动</a-tag>
                      <a-tag v-else color="blue">AI</a-tag>
                    </a-space>
                  </template>
                  <template #description>
                    <span class="exec-time">{{ formatTime(item.timestamp) }}</span>
                    <span v-if="item.args" class="exec-args">{{ item.args }}</span>
                  </template>
                </a-list-item-meta>
              </a-list-item>
            </template>
            <template #emptyText>
              <a-empty description="暂无执行记录" />
            </template>
          </a-list>
        </a-card>
      </a-col>
    </a-row>

    <a-row style="margin-top: 16px">
      <a-col :span="24">
        <a-card title="执行趋势">
          <a-tabs v-model:activeKey="trendDays">
            <a-tab-pane key="7" tab="近7天" />
            <a-tab-pane key="30" tab="近30天" />
          </a-tabs>
          <div class="trend-chart">
            <a-empty description="趋势图待实现">
              <template #image>
                <BarChartOutlined style="font-size: 48px; color: #ccc" />
              </template>
            </a-empty>
          </div>
        </a-card>
      </a-col>
    </a-row>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useSkillStore } from '@/stores/skill'
import { InboxOutlined, BarChartOutlined } from '@ant-design/icons-vue'
import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'
import utc from 'dayjs/plugin/utc'
import timezone from 'dayjs/plugin/timezone'

dayjs.extend(relativeTime)
dayjs.extend(utc)
dayjs.extend(timezone)

const store = useSkillStore()
const trendDays = ref('7')

function formatTime(timestamp: string) {
  const tz = localStorage.getItem('skill-hub-timezone') || 'local'
  try {
    if (tz === 'local') {
      return dayjs(timestamp).format('YYYY-MM-DD HH:mm:ss')
    }
    return dayjs(timestamp).tz(tz).format('YYYY-MM-DD HH:mm:ss')
  } catch {
    return dayjs(timestamp).format('YYYY-MM-DD HH:mm:ss')
  }
}

const loading = computed(() => store.loading)
const statsBySkill = computed(() => store.statsBySkill)
const recentExecutions = computed(() => store.executions.slice(0, 10))
const topStats = computed(() => store.statsBySkill.slice(0, 4))

onMounted(async () => {
  await store.fetchAll()
})
</script>

<style scoped>
.dashboard {
  padding: 24px;
}

.stat-card {
  height: 100%;
}

.stat-card :deep(.ant-statistic-title) {
  font-weight: 600;
}

.stat-suffix {
  font-size: 14px;
  color: #999;
  margin-left: 4px;
}

.stat-detail {
  margin-top: 8px;
}

.skill-rank {
  font-weight: bold;
  color: #1890ff;
  min-width: 30px;
}

.skill-name {
  font-weight: 600;
}

.exec-time {
  color: #999;
  font-size: 12px;
}

.exec-args {
  margin-left: 8px;
  color: #666;
  font-size: 12px;
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  display: inline-block;
}

.trend-chart {
  height: 300px;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
