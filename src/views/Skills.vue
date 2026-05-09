<template>
  <div class="skills-page">
    <!-- 添加 Skill 路径表单 -->
    <section class="form-card">
      <div class="form-row">
        <div class="form-field">
          <label class="form-label">路径类型</label>
          <a-select v-model:value="newPath.pathType" class="custom-select">
            <a-select-option value="user">用户级</a-select-option>
            <a-select-option value="project">项目级</a-select-option>
          </a-select>
        </div>
        <div class="form-field form-field--wide">
          <label class="form-label">路径</label>
          <a-input
            v-model:value="newPath.path"
            placeholder="如 ~/.claude/skills 或 ./skills"
            class="custom-input"
          />
        </div>
        <div v-if="newPath.pathType === 'project'" class="form-field">
          <label class="form-label">项目名称</label>
          <a-input
            v-model:value="newPath.projectName"
            placeholder="可选"
            class="custom-input"
          />
        </div>
        <div class="form-actions">
          <button class="btn btn-primary" @click="handleAddPath" :disabled="loading">
            <Plus :size="16" :stroke-width="2" />
            添加路径
          </button>
          <button class="btn btn-secondary" @click="detectPaths">
            <Search :size="16" :stroke-width="2" />
            自动检测
          </button>
        </div>
      </div>
    </section>

    <!-- 路径列表 -->
    <section class="lists-section">
      <a-row :gutter="[24, 24]">
        <!-- 用户级路径 -->
        <a-col :xs="24" :lg="12">
          <div class="list-card">
            <h2 class="list-title">
              <User :size="18" :stroke-width="1.75" />
              用户级路径
            </h2>
            <div class="path-list" v-if="userPaths.length > 0">
              <div
                v-for="item in userPaths"
                :key="item.path"
                class="path-item"
              >
                <div class="path-info">
                  <div class="path-row">
                    <FolderOpen :size="16" :stroke-width="1.75" class="path-icon" />
                    <span class="path-text">{{ item.path }}</span>
                  </div>
                  <div class="path-skills">
                    <span
                      v-for="skill in pathSkills[item.path] || []"
                      :key="skill"
                      class="skill-tag"
                    >
                      {{ skill }}
                    </span>
                    <span v-if="!pathSkills[item.path]?.length" class="no-skills">无 Skill</span>
                  </div>
                </div>
                <div class="path-actions">
                  <a-switch
                    :checked="item.enabled"
                    size="small"
                    @change="(checked: boolean) => handleToggle(item.path, checked)"
                  />
                  <button class="btn-icon btn-icon--danger" @click="handleRemove(item.path)">
                    <Trash2 :size="16" :stroke-width="1.75" />
                  </button>
                </div>
              </div>
            </div>
            <div v-else class="empty-state">
              <FolderOpen :size="40" :stroke-width="1" class="empty-icon" />
              <p class="empty-text">暂无用户级路径配置</p>
              <p class="empty-hint">点击上方「添加路径」或「自动检测」添加</p>
            </div>
          </div>
        </a-col>

        <!-- 项目级路径 -->
        <a-col :xs="24" :lg="12">
          <div class="list-card">
            <h2 class="list-title">
              <FolderCog :size="18" :stroke-width="1.75" />
              项目级路径
            </h2>
            <div class="path-list" v-if="projectPaths.length > 0">
              <div
                v-for="item in projectPaths"
                :key="item.path"
                class="path-item"
              >
                <div class="path-info">
                  <div class="path-row">
                    <FolderOpen :size="16" :stroke-width="1.75" class="path-icon" />
                    <span class="path-text">{{ item.path }}</span>
                    <span v-if="item.project_name" class="project-tag">{{ item.project_name }}</span>
                  </div>
                  <div class="path-skills">
                    <span
                      v-for="skill in pathSkills[item.path] || []"
                      :key="skill"
                      class="skill-tag skill-tag--green"
                    >
                      {{ skill }}
                    </span>
                    <span v-if="!pathSkills[item.path]?.length" class="no-skills">无 Skill</span>
                  </div>
                </div>
                <div class="path-actions">
                  <a-switch
                    :checked="item.enabled"
                    size="small"
                    @change="(checked: boolean) => handleToggle(item.path, checked)"
                  />
                  <button class="btn-icon btn-icon--danger" @click="handleRemove(item.path)">
                    <Trash2 :size="16" :stroke-width="1.75" />
                  </button>
                </div>
              </div>
            </div>
            <div v-else class="empty-state">
              <FolderCog :size="40" :stroke-width="1" class="empty-icon" />
              <p class="empty-text">暂无项目级路径配置</p>
              <p class="empty-hint">点击上方「添加路径」或「自动检测」添加</p>
            </div>
          </div>
        </a-col>
      </a-row>
    </section>

    <!-- 常见路径参考 -->
    <section class="ref-card">
      <h2 class="ref-title">
        <Info :size="16" :stroke-width="1.75" />
        常见 Skill 路径参考
      </h2>
      <div class="ref-grid">
        <div class="ref-item">
          <span class="ref-label">macOS / Linux 用户级</span>
          <div class="ref-path">
            <code>~/.claude/skills/</code>
            <button class="btn-copy" @click="copyPath('~/.claude/skills/')">
              <Copy :size="14" :stroke-width="1.75" />
            </button>
          </div>
        </div>
        <div class="ref-item">
          <span class="ref-label">项目级</span>
          <div class="ref-path">
            <code>项目目录/.claude/skills/</code>
            <button class="btn-copy" @click="copyPath('项目目录/.claude/skills/')">
              <Copy :size="14" :stroke-width="1.75" />
            </button>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive, onMounted } from 'vue'
import { message } from 'ant-design-vue'
import { useSkillStore } from '@/stores/skill'
import {
  Plus, Search, User, FolderOpen, FolderCog, Trash2, Info, Copy
} from 'lucide-vue-next'
import { skillApi } from '@/api/skill'

const store = useSkillStore()
const loading = computed(() => store.loading)
const userPaths = computed(() => store.userPaths)
const projectPaths = computed(() => store.projectPaths)

const newPath = ref({
  pathType: 'user' as 'user' | 'project',
  path: '',
  projectName: ''
})

const pathSkills = reactive<Record<string, string[]>>({})

async function loadSkillsForPath(path: string) {
  try {
    const skills = await skillApi.getSkillsInPath(path)
    pathSkills[path] = skills
  } catch {
    pathSkills[path] = []
  }
}

async function loadAllPathSkills() {
  for (const p of userPaths.value) {
    await loadSkillsForPath(p.path)
  }
  for (const p of projectPaths.value) {
    await loadSkillsForPath(p.path)
  }
}

async function handleAddPath() {
  if (!newPath.value.path) {
    message.warning('请输入路径')
    return
  }
  try {
    await store.addPath(
      newPath.value.path,
      newPath.value.pathType,
      newPath.value.projectName || undefined
    )
    message.success('路径添加成功')
    newPath.value.path = ''
    newPath.value.projectName = ''
    await loadSkillsForPath(newPath.value.path)
  } catch (e) {
    message.error('添加失败: ' + String(e))
  }
}

async function handleRemove(path: string) {
  try {
    await store.removePath(path)
    delete pathSkills[path]
    message.success('路径已删除')
  } catch (e) {
    message.error('删除失败: ' + String(e))
  }
}

async function handleToggle(path: string, enabled: boolean) {
  try {
    await store.togglePath(path, enabled)
    message.success(`路径已${enabled ? '启用' : '禁用'}`)
  } catch (e) {
    message.error('操作失败: ' + String(e))
  }
}

async function detectPaths() {
  const home = await skillApi.getSettingsPath()
  const userPath = home.replace('/settings.json', '/skills')
  newPath.value.path = userPath
  message.info('已填充建议路径')
}

async function copyPath(text: string) {
  try {
    await navigator.clipboard.writeText(text)
    message.success('已复制')
  } catch {
    message.error('复制失败')
  }
}

onMounted(async () => {
  await store.fetchPaths()
  await loadAllPathSkills()
})
</script>

<style scoped>
.skills-page {
  padding: 0;
}

/* ========== 通用卡片 ========== */
.form-card, .list-card, .ref-card {
  background: #fff;
  border-radius: 8px;
  padding: 24px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  margin-bottom: 24px;
}

/* ========== 表单样式 ========== */
.form-row {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
  align-items: flex-end;
}

.form-field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-field--wide {
  flex: 1;
  min-width: 200px;
}

.form-label {
  font-size: 14px;
  color: #334155;
  font-weight: 500;
}

.form-actions {
  display: flex;
  gap: 8px;
}

/* 自定义输入框/下拉框 */
.custom-input :deep(.ant-input),
.custom-select :deep(.ant-select-selector) {
  height: 36px !important;
  border: 1px solid #e2e8f0 !important;
  border-radius: 6px !important;
  font-size: 13px !important;
  transition: all 0.2s !important;
}

.custom-input :deep(.ant-input:hover),
.custom-select :deep(.ant-select:not(.ant-select-disabled):hover .ant-select-selector) {
  border-color: #cbd5e1 !important;
}

.custom-input :deep(.ant-input:focus),
.custom-select :deep(.ant-select-focused .ant-select-selector) {
  border-color: #3b82f6 !important;
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1) !important;
}

/* ========== 按钮样式 ========== */
.btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  border: none;
}

.btn-primary {
  background: #3b82f6;
  color: #fff;
}

.btn-primary:hover {
  background: #2563eb;
}

.btn-primary:disabled {
  background: #93c5fd;
  cursor: not-allowed;
}

.btn-secondary {
  background: #fff;
  color: #334155;
  border: 1px solid #e2e8f0;
}

.btn-secondary:hover {
  background: #f8fafc;
  border-color: #cbd5e1;
}

/* ========== 列表标题 ========== */
.list-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 15px;
  font-weight: 600;
  color: #1e293b;
  margin: 0 0 20px 0;
}

.list-title svg {
  color: #64748b;
}

/* ========== 路径列表 ========== */
.path-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.path-item {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: 16px;
  border-radius: 8px;
  background: #f8fafc;
  transition: all 0.2s ease;
}

.path-item:hover {
  background: #f1f5f9;
}

.path-info {
  flex: 1;
  min-width: 0;
}

.path-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.path-icon {
  color: #64748b;
  flex-shrink: 0;
}

.path-text {
  font-size: 14px;
  color: #475569;
  word-break: break-all;
}

.project-tag {
  font-size: 11px;
  padding: 2px 8px;
  background: #f1f5f9;
  color: #64748b;
  border-radius: 4px;
  flex-shrink: 0;
}

.path-skills {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.skill-tag {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  background: #eff6ff;
  color: #3b82f6;
  font-size: 12px;
  border-radius: 999px;
}

.skill-tag--green {
  background: #f0fdf4;
  color: #22c55e;
}

.no-skills {
  font-size: 12px;
  color: #94a3b8;
}

.path-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

/* ========== 图标按钮 ========== */
.btn-icon {
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
  transition: all 0.2s ease;
}

.btn-icon:hover {
  background: #f1f5f9;
  color: #64748b;
}

.btn-icon--danger:hover {
  background: #fef2f2;
  color: #ef4444;
}

/* ========== 空状态 ========== */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
  text-align: center;
}

.empty-icon {
  color: #cbd5e1;
  margin-bottom: 16px;
}

.empty-text {
  font-size: 14px;
  color: #64748b;
  margin: 0;
}

.empty-hint {
  font-size: 13px;
  color: #94a3b8;
  margin: 4px 0 0 0;
}

/* ========== 路径参考 ========== */
.ref-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 15px;
  font-weight: 600;
  color: #1e293b;
  margin: 0 0 20px 0;
}

.ref-title svg {
  color: #64748b;
}

.ref-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 16px;
}

.ref-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 16px;
  background: #f8fafc;
  border-radius: 6px;
}

.ref-label {
  font-size: 12px;
  color: #64748b;
  font-weight: 500;
}

.ref-path {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.ref-path code {
  font-size: 13px;
  color: #475569;
  background: #fff;
  padding: 6px 10px;
  border-radius: 4px;
  border: 1px solid #e2e8f0;
  flex: 1;
  font-family: 'SF Mono', Monaco, monospace;
}

.btn-copy {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: #94a3b8;
  cursor: pointer;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.btn-copy:hover {
  background: #e2e8f0;
  color: #3b82f6;
}
</style>
