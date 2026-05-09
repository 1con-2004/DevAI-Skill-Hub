<template>
  <div class="settings-page">
    <a-row :gutter="[24, 24]">
      <!-- Hook 配置 -->
      <a-col :span="24">
        <div class="content-card">
          <h2 class="card-title">Hook 配置</h2>

          <div class="form-grid">
            <div class="form-item">
              <label class="form-label">Hook 脚本路径</label>
              <a-input :value="hookScriptPath" readonly placeholder="加载中..." />
            </div>
            <div class="form-item">
              <label class="form-label">Claude Code 设置文件</label>
              <a-input :value="settingsPath" readonly placeholder="加载中..." />
            </div>
          </div>

          <div class="section-divider">
            <span class="divider-text">手动配置</span>
          </div>

          <div class="config-box">
            <div class="config-steps">
              <div class="step-item">
                <span class="step-num">1</span>
                <span class="step-text">打开 Claude Code 设置文件</span>
              </div>
              <div class="step-item">
                <span class="step-num">2</span>
                <span class="step-text">在 <code>hooks</code> 中添加配置</span>
              </div>
              <div class="step-item">
                <span class="step-num">3</span>
                <span class="step-text">保存文件并重启 Claude Code</span>
              </div>
            </div>
          </div>

          <div class="code-block">
            <pre class="code-content">{{ configExample }}</pre>
          </div>

          <div class="action-buttons">
            <a-button type="primary" @click="openSettingsFile">
              <FileText :size="14" :stroke-width="1.75" />
              打开设置文件
            </a-button>
            <a-button @click="copyConfig">
              <Copy :size="14" :stroke-width="1.75" />
              复制配置
            </a-button>
          </div>
        </div>
      </a-col>

      <!-- 显示设置 + 关于 并排 -->
      <a-col :xs="24" :lg="12">
        <div class="content-card">
          <h2 class="card-title">显示设置</h2>

          <div class="timezone-row">
            <div class="form-item form-item--inline">
              <label class="form-label">时区</label>
              <a-select v-model:value="selectedTimezone" @change="saveTimezone">
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
            </div>
            <div class="current-time">
              <span class="time-label">当前时间</span>
              <span class="time-value">{{ currentTime }}</span>
            </div>
          </div>
        </div>
      </a-col>

      <a-col :xs="24" :lg="12">
        <div class="content-card">
          <h2 class="card-title">关于</h2>

          <div class="info-list">
            <div class="info-item">
              <span class="info-key">版本</span>
              <span class="info-value">1.0.0</span>
            </div>
            <div class="info-item">
              <span class="info-key">技术栈</span>
              <span class="info-value">Tauri 2.0 + Vue 3 + SQLite</span>
            </div>
            <div class="info-item">
              <span class="info-key">Hook 类型</span>
              <span class="info-value">PreToolUse / PostToolUse</span>
            </div>
          </div>

          <div class="section-divider">
            <span class="divider-text">功能说明</span>
          </div>

          <ul class="feature-list">
            <li v-for="feature in features" :key="feature.text" class="feature-item">
              <Check :size="14" :stroke-width="2" class="feature-icon" />
              <span class="feature-text">{{ feature.text }}</span>
              <span v-if="feature.planned" class="tag-planned">规划中</span>
            </li>
          </ul>
        </div>
      </a-col>

      <!-- 数据存储 -->
      <a-col :span="24">
        <div class="content-card">
          <h2 class="card-title">数据存储</h2>

          <div class="path-list">
            <div
              v-for="platform in storagePaths"
              :key="platform.os"
              class="path-item"
            >
              <span class="path-os">{{ platform.os }}</span>
              <span class="path-value">{{ platform.path }}</span>
              <button class="copy-btn" @click="copyPath(platform.path)" title="复制路径">
                <Copy :size="14" :stroke-width="1.75" />
              </button>
            </div>
          </div>
        </div>
      </a-col>

      <!-- Hook 工作原理 -->
      <a-col :span="24">
        <div class="content-card">
          <h2 class="card-title">Hook 工作原理</h2>

          <div class="timeline">
            <div v-for="(step, index) in workflowSteps" :key="index" class="timeline-item">
              <div class="timeline-connector" v-if="index > 0"></div>
              <div class="timeline-dot">
                <span class="dot-num">{{ index + 1 }}</span>
              </div>
              <div class="timeline-content">
                <div class="timeline-title">{{ step.title }}</div>
                <div class="timeline-desc">{{ step.desc }}</div>
              </div>
            </div>
          </div>

          <div class="info-box">
            <div class="info-box-title">监控目标</div>
            <p class="info-box-text">不是为了记录用户输入的 Skill 命令，而是理解 <strong>AI 的决策过程</strong>：</p>
            <ul class="info-box-list">
              <li>AI 主动调用了哪些 Skill？</li>
              <li>AI 应该在什么场景下调用 Skill？</li>
            </ul>
          </div>
        </div>
      </a-col>
    </a-row>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { message } from 'ant-design-vue'
import { FileText, Copy, Check } from 'lucide-vue-next'
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
  { text: 'Skill 自动识别与记录', planned: false },
  { text: 'AI 触发 vs 手动触发区分', planned: false },
  { text: '多路径配置支持', planned: false },
  { text: '本地 SQLite 存储', planned: false },
  { text: '可视化监控面板', planned: false },
  { text: '团队配置同步（V1.1）', planned: true }
]

const storagePaths = [
  { os: 'macOS', path: '~/Library/Application Support/com.devai.skill-hub/' },
  { os: 'Linux', path: '~/.local/share/com.devai.skill-hub/' },
  { os: 'Windows', path: '%APPDATA%\\com.devai.skill-hub\\' }
]

const workflowSteps = [
  { title: 'Claude Code 执行任务', desc: '用户输入或 AI 决策' },
  { title: 'Hook 触发', desc: 'PreToolUse 检测 Skill 调用' },
  { title: '记录到数据库', desc: '提取 Skill ID 并存储' },
  { title: '前端展示', desc: '监控面板可视化' }
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

async function copyPath(path: string) {
  try {
    await navigator.clipboard.writeText(path)
    message.success('路径已复制')
  } catch (e) {
    message.error('复制失败: ' + String(e))
  }
}

onMounted(async () => {
  try {
    hookScriptPath.value = await skillApi.getHookScriptPath()
    settingsPath.value = await skillApi.getSettingsPath()

    const savedTimezone = localStorage.getItem('skill-hub-timezone')
    if (savedTimezone) {
      selectedTimezone.value = savedTimezone
    }

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
  padding: 0;
}

/* ========== 通用卡片 ========== */
.content-card {
  background: #fff;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.card-title {
  font-size: 15px;
  font-weight: 600;
  color: #1e293b;
  margin: 0 0 20px 0;
}

/* ========== 表单样式 ========== */
.form-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
}

@media (max-width: 768px) {
  .form-grid {
    grid-template-columns: 1fr;
  }
}

.form-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-item--inline {
  flex-direction: row;
  align-items: center;
  gap: 12px;
}

.form-label {
  font-size: 14px;
  color: #334155;
  white-space: nowrap;
}

:deep(.ant-input) {
  height: 36px;
  border: 1px solid #e2e8f0;
  border-radius: 6px;
  font-size: 13px;
}

:deep(.ant-input:hover),
:deep(.ant-input:focus) {
  border-color: #3b82f6;
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
}

:deep(.ant-select-selector) {
  height: 36px !important;
  border: 1px solid #e2e8f0 !important;
  border-radius: 6px !important;
}

:deep(.ant-select-selection-item) {
  line-height: 34px !important;
  font-size: 13px;
}

/* ========== 分隔线 ========== */
.section-divider {
  display: flex;
  align-items: center;
  margin: 24px 0;
}

.section-divider::before,
.section-divider::after {
  content: '';
  flex: 1;
  height: 1px;
  background: #e2e8f0;
}

.divider-text {
  padding: 0 16px;
  font-size: 13px;
  font-weight: 600;
  color: #64748b;
}

/* ========== 配置步骤框 ========== */
.config-box {
  background: #eff6ff;
  border-radius: 6px;
  padding: 16px 20px;
  margin-bottom: 16px;
}

.config-steps {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.step-item {
  display: flex;
  align-items: center;
  gap: 12px;
}

.step-num {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: #3b82f6;
  color: #fff;
  font-size: 11px;
  font-weight: 600;
  flex-shrink: 0;
}

.step-text {
  font-size: 13px;
  color: #334155;
}

.step-text code {
  background: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  font-size: 12px;
}

/* ========== 代码块 ========== */
.code-block {
  background: #f8fafc;
  border-radius: 6px;
  padding: 16px;
  margin-bottom: 16px;
  overflow-x: auto;
}

.code-content {
  margin: 0;
  font-family: 'JetBrains Mono', 'Fira Code', 'Monaco', monospace;
  font-size: 12px;
  color: #334155;
  line-height: 1.6;
  white-space: pre;
}

/* ========== 按钮 ========== */
.action-buttons {
  display: flex;
  gap: 12px;
}

:deep(.ant-btn) {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  height: 36px;
  padding: 0 16px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
}

:deep(.ant-btn-primary) {
  background: #3b82f6;
  border-color: #3b82f6;
}

:deep(.ant-btn-primary:hover) {
  background: #2563eb !important;
  border-color: #2563eb !important;
}

/* ========== 时区设置 ========== */
.timezone-row {
  display: flex;
  align-items: center;
  gap: 24px;
  flex-wrap: wrap;
}

.timezone-row .form-item--inline {
  flex: 1;
  min-width: 280px;
}

.current-time {
  display: flex;
  align-items: center;
  gap: 8px;
}

.time-label {
  font-size: 14px;
  color: #64748b;
}

.time-value {
  font-family: 'JetBrains Mono', monospace;
  font-size: 13px;
  color: #334155;
}

/* ========== 信息列表 ========== */
.info-list {
  background: #f8fafc;
  border-radius: 6px;
  padding: 12px 16px;
}

.info-item {
  display: flex;
  padding: 8px 0;
  border-bottom: 1px solid #e2e8f0;
}

.info-item:last-child {
  border-bottom: none;
}

.info-key {
  width: 80px;
  font-size: 13px;
  color: #64748b;
  flex-shrink: 0;
}

.info-value {
  font-size: 13px;
  color: #334155;
}

/* ========== 功能列表 ========== */
.feature-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.feature-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 0;
  font-size: 13px;
  color: #334155;
}

.feature-icon {
  color: #16a34a;
  flex-shrink: 0;
}

.feature-text {
  flex: 1;
}

.tag-planned {
  font-size: 11px;
  color: #94a3b8;
  background: #f1f5f9;
  padding: 2px 8px;
  border-radius: 4px;
}

/* ========== 路径列表 ========== */
.path-list {
  display: flex;
  flex-direction: column;
}

.path-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  background: #f8fafc;
  border-radius: 6px;
  margin-bottom: 8px;
  transition: background 0.15s ease;
}

.path-item:last-child {
  margin-bottom: 0;
}

.path-item:hover {
  background: #f1f5f9;
}

.path-os {
  width: 80px;
  font-size: 13px;
  font-weight: 500;
  color: #334155;
  flex-shrink: 0;
}

.path-value {
  flex: 1;
  font-family: 'JetBrains Mono', monospace;
  font-size: 12px;
  color: #64748b;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.copy-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: #94a3b8;
  cursor: pointer;
  transition: all 0.15s ease;
  opacity: 0;
  margin-left: 12px;
}

.path-item:hover .copy-btn {
  opacity: 1;
}

.copy-btn:hover {
  background: #e2e8f0;
  color: #3b82f6;
}

/* ========== 时间线 ========== */
.timeline {
  display: flex;
  justify-content: space-between;
  margin-bottom: 24px;
  position: relative;
}

@media (max-width: 768px) {
  .timeline {
    flex-direction: column;
    gap: 0;
  }
}

.timeline-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  position: relative;
  text-align: center;
}

.timeline-connector {
  position: absolute;
  top: 16px;
  right: 50%;
  width: 100%;
  height: 2px;
  background: #e2e8f0;
  z-index: 0;
}

@media (max-width: 768px) {
  .timeline-connector {
    top: 0;
    right: auto;
    left: 16px;
    width: 2px;
    height: 100%;
  }

  .timeline-item {
    flex-direction: row;
    text-align: left;
    padding-left: 40px;
    min-height: 60px;
  }

  .timeline-connector {
    display: none;
  }

  .timeline-item:not(:first-child) {
    border-left: 2px solid #e2e8f0;
    margin-left: 14px;
    padding-left: 26px;
  }
}

.timeline-dot {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: #3b82f6;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1;
  transition: transform 0.2s ease;
}

.timeline-item:hover .timeline-dot {
  transform: scale(1.1);
}

.dot-num {
  color: #fff;
  font-size: 13px;
  font-weight: 600;
}

.timeline-content {
  margin-top: 12px;
}

@media (max-width: 768px) {
  .timeline-content {
    margin-top: 0;
    margin-left: 12px;
  }
}

.timeline-title {
  font-size: 14px;
  font-weight: 600;
  color: #334155;
  margin-bottom: 4px;
}

.timeline-desc {
  font-size: 12px;
  color: #64748b;
}

/* ========== 信息框 ========== */
.info-box {
  background: #eff6ff;
  border-radius: 6px;
  padding: 16px 20px;
}

.info-box-title {
  font-size: 13px;
  font-weight: 600;
  color: #334155;
  margin-bottom: 8px;
}

.info-box-text {
  font-size: 13px;
  color: #334155;
  margin: 0 0 8px 0;
  line-height: 1.6;
}

.info-box-text strong {
  color: #3b82f6;
}

.info-box-list {
  margin: 0;
  padding-left: 20px;
  font-size: 13px;
  color: #334155;
  line-height: 1.8;
}
</style>
