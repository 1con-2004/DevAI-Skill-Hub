<template>
  <div class="settings-page">
    <a-row :gutter="[16, 16]">
      <a-col :span="24">
        <h1>设置</h1>
      </a-col>
    </a-row>

    <a-row :gutter="[16, 16]">
      <a-col :span="24">
        <a-card title="Hook 配置">
          <a-descriptions :column="1" bordered size="small">
            <a-descriptions-item label="Hook 脚本路径">
              <a-input :value="hookScriptPath" readonly />
            </a-descriptions-item>
            <a-descriptions-item label="Claude Code 设置文件">
              <a-input :value="settingsPath" readonly />
            </a-descriptions-item>
          </a-descriptions>

          <a-divider>手动配置</a-divider>

          <a-alert
            message="配置步骤"
            type="info"
            show-icon
          >
            <template #description>
              <ol style="margin: 8px 0; padding-left: 20px">
                <li>打开 Claude Code 设置文件</li>
                <li>在 <code>hooks</code> 中添加配置</li>
                <li>保存文件并重启 Claude Code</li>
              </ol>
            </template>
          </a-alert>

          <a-card size="small" style="margin-top: 16px; background: #f5f5f5">
            <pre class="config-example">{{ configExample }}</pre>
          </a-card>

          <a-space style="margin-top: 16px">
            <a-button type="primary" @click="openSettingsFile">
              打开设置文件
            </a-button>
            <a-button @click="copyConfig">
              复制配置
            </a-button>
          </a-space>
        </a-card>
      </a-col>

      <a-col :span="24">
        <a-card title="显示设置">
          <a-form layout="inline">
            <a-form-item label="时区">
              <a-select
                v-model:value="selectedTimezone"
                style="width: 300px"
                @change="saveTimezone"
              >
                <a-select-option value="local">系统本地时间 (自动)</a-select-option>
                <a-select-option value="Asia/Shanghai">中国标准时间 (UTC+8)</a-select-option>
                <a-select-option value="America/New_York">美国东部时间 (UTC-5)</a-select-option>
                <a-select-option value="America/Los_Angeles">美国太平洋时间 (UTC-8)</a-select-option>
                <a-select-option value="Europe/London">伦敦时间 (UTC+0)</a-select-option>
                <a-select-option value="Europe/Paris">巴黎时间 (UTC+1)</a-select-option>
                <a-select-option value="Asia/Tokyo">东京时间 (UTC+9)</a-select-option>
                <a-select-option value="Asia/Singapore">新加坡时间 (UTC+8)</a-select-option>
                <a-select-option value="Australia/Sydney">悉尼时间 (UTC+10)</a-select-option>
              </a-select>
            </a-form-item>
            <a-form-item label="当前时间">
              <span style="font-family: monospace">{{ currentTime }}</span>
            </a-form-item>
          </a-form>
        </a-card>
      </a-col>
    </a-row>

    <a-row :gutter="[16, 16]">
      <a-col :xs="24" :lg="12">
        <a-card title="关于 DevAI Skill Hub">
          <a-descriptions :column="1" bordered size="small">
            <a-descriptions-item label="版本">1.0.0</a-descriptions-item>
            <a-descriptions-item label="技术栈">
              Tauri 2.0 + Vue 3 + SQLite
            </a-descriptions-item>
            <a-descriptions-item label="Hook 类型">
              PreToolUse / PostToolUse
            </a-descriptions-item>
          </a-descriptions>

          <a-divider>功能说明</a-divider>

          <a-list size="small" :data-source="features">
            <template #renderItem="{ item }">
              <a-list-item>
                <CheckCircleOutlined style="color: #52c41a; margin-right: 8px" />
                {{ item }}
              </a-list-item>
            </template>
          </a-list>
        </a-card>

        <a-card title="数据存储" style="margin-top: 16px">
          <a-descriptions :column="1" bordered size="small">
            <a-descriptions-item label="macOS">
              ~/Library/Application Support/com.devai.skill-hub/
            </a-descriptions-item>
            <a-descriptions-item label="Linux">
              ~/.local/share/com.devai.skill-hub/
            </a-descriptions-item>
            <a-descriptions-item label="Windows">
              %APPDATA%\com.devai.skill-hub\
            </a-descriptions-item>
          </a-descriptions>
        </a-card>
      </a-col>
    </a-row>

    <a-row style="margin-top: 16px">
      <a-col :span="24">
        <a-card title="Hook 工作原理">
          <a-steps :current="3" size="small">
            <a-step title="Claude Code 执行任务" description="用户输入或 AI 决策" />
            <a-step title="Hook 触发" description="PreToolUse 检测 Skill 调用" />
            <a-step title="记录到数据库" description="提取 Skill ID 并存储" />
            <a-step title="前端展示" description="监控面板可视化" />
          </a-steps>

          <a-divider />

          <a-alert
            message="监控目标"
            description="不是为了记录用户输入的 Skill 命令，而是理解 AI 的决策过程：
• AI 主动调用了哪些 Skill？
• AI 应该在什么场景下调用 Skill？"
            type="info"
          />
        </a-card>
      </a-col>
    </a-row>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { message } from 'ant-design-vue'
import { CheckCircleOutlined } from '@ant-design/icons-vue'
import { skillApi } from '@/api/skill'
import { openPath } from '@tauri-apps/plugin-opener'
import dayjs from 'dayjs'
import utc from 'dayjs/plugin/utc'
import timezone from 'dayjs/plugin/timezone'

dayjs.extend(utc)
dayjs.extend(timezone)

const hookScriptPath = ref('')
const settingsPath = ref('')
const selectedTimezone = ref('local')
const currentTime = ref('')
let timeInterval: number | null = null

function updateCurrentTime() {
  try {
    const now = dayjs.utc()
    if (selectedTimezone.value === 'local') {
      currentTime.value = now.local().format('YYYY-MM-DD HH:mm:ss')
    } else {
      currentTime.value = now.tz(selectedTimezone.value).format('YYYY-MM-DD HH:mm:ss')
    }
  } catch {
    currentTime.value = dayjs().format('YYYY-MM-DD HH:mm:ss')
  }
}

function saveTimezone() {
  localStorage.setItem('skill-hub-timezone', selectedTimezone.value)
  updateCurrentTime()
}

const configExample = computed(() => {
  return `{
  "hooks": {
    "pre_tool_use": "${hookScriptPath.value}",
    "post_tool_use": "${hookScriptPath.value}"
  }
}`
})

const features = [
  'Skill 自动识别与记录',
  'AI 触发 vs 手动触发区分',
  '多路径配置支持',
  '本地 SQLite 存储',
  '可视化监控面板',
  '团队配置同步（V1.1）'
]

async function openSettingsFile() {
  try {
    const path = await skillApi.getSettingsPath()
    await openPath(path)
  } catch (e) {
    message.error('无法打开设置文件: ' + String(e))
  }
}

async function copyConfig() {
  try {
    const text = configExample.value
    await navigator.clipboard.writeText(text)
    message.success('配置已复制到剪贴板')
  } catch (e) {
    message.error('复制失败: ' + String(e))
  }
}

onMounted(async () => {
  try {
    hookScriptPath.value = await skillApi.getHookScriptPath()
    settingsPath.value = await skillApi.getSettingsPath()

    // 加载保存的时区设置
    const savedTimezone = localStorage.getItem('skill-hub-timezone')
    if (savedTimezone) {
      selectedTimezone.value = savedTimezone
    }

    // 启动时钟
    updateCurrentTime()
    timeInterval = window.setInterval(updateCurrentTime, 1000)
  } catch (e) {
    console.error('Failed to get paths:', e)
  }
})

onUnmounted(() => {
  if (timeInterval) {
    clearInterval(timeInterval)
  }
})
</script>

<style scoped>
.settings-page {
  padding: 24px;
}

.config-example {
  margin: 0;
  padding: 12px;
  background: white;
  border-radius: 4px;
  font-family: monospace;
  font-size: 12px;
  overflow-x: auto;
}

code {
  background: #f0f0f0;
  padding: 2px 4px;
  border-radius: 2px;
}
</style>
