<template>
  <div class="skills-page">
    <a-row :gutter="[16, 16]">
      <a-col :span="24">
        <h1>Skill 配置</h1>
        <p class="subtitle">配置 Skill 路径以监控 Claude Code 的 Skill 调用</p>
      </a-col>
    </a-row>

    <a-row :gutter="[16, 16]">
      <a-col :span="24">
        <a-card title="添加 Skill 路径">
          <a-form layout="inline" @finish="handleAddPath">
            <a-form-item
              label="路径类型"
              name="pathType"
              :rules="[{ required: true, message: '请选择路径类型' }]"
            >
              <a-select v-model:value="newPath.pathType" style="width: 120px">
                <a-select-option value="user">用户级</a-select-option>
                <a-select-option value="project">项目级</a-select-option>
              </a-select>
            </a-form-item>

            <a-form-item
              label="路径"
              name="path"
              :rules="[{ required: true, message: '请输入 Skill 路径' }]"
            >
              <a-input
                v-model:value="newPath.path"
                placeholder="如 ~/.claude/skills 或 ./skills"
                style="width: 300px"
              />
            </a-form-item>

            <a-form-item v-if="newPath.pathType === 'project'" label="项目名称" name="projectName">
              <a-input
                v-model:value="newPath.projectName"
                placeholder="可选，用于标识项目"
                style="width: 150px"
              />
            </a-form-item>

            <a-form-item>
              <a-space>
                <a-button type="primary" html-type="submit" :loading="loading">
                  添加路径
                </a-button>
                <a-button @click="detectPaths">
                  自动检测
                </a-button>
              </a-space>
            </a-form-item>
          </a-form>
        </a-card>
      </a-col>
    </a-row>

    <a-row :gutter="[16, 16]" style="margin-top: 16px">
      <a-col :xs="24" :lg="12">
        <a-card title="用户级路径">
          <a-list
            :data-source="userPaths"
            :loading="loading"
          >
            <template #renderItem="{ item }">
              <a-list-item>
                <template #actions>
                  <a-switch
                    :checked="item.enabled"
                    size="small"
                    @change="(checked: boolean) => handleToggle(item.path, checked)"
                  />
                  <a-button type="text" danger size="small" @click="handleRemove(item.path)">
                    删除
                  </a-button>
                </template>
                <a-list-item-meta>
                  <template #title>
                    <FolderOutlined /> {{ item.path }}
                  </template>
                  <template #description>
                    添加于 {{ formatDate(item.created_at) }}
                  </template>
                </a-list-item-meta>
              </a-list-item>
            </template>
            <template #emptyText>
              <a-empty description="暂无用户级路径配置">
                <template #image>
                  <FolderOutlined style="font-size: 48px; color: #ccc" />
                </template>
              </a-empty>
            </template>
          </a-list>
        </a-card>
      </a-col>

      <a-col :xs="24" :lg="12">
        <a-card title="项目级路径">
          <a-list
            :data-source="projectPaths"
            :loading="loading"
          >
            <template #renderItem="{ item }">
              <a-list-item>
                <template #actions>
                  <a-switch
                    :checked="item.enabled"
                    size="small"
                    @change="(checked: boolean) => handleToggle(item.path, checked)"
                  />
                  <a-button type="text" danger size="small" @click="handleRemove(item.path)">
                    删除
                  </a-button>
                </template>
                <a-list-item-meta>
                  <template #title>
                    <FolderOutlined /> {{ item.path }}
                    <a-tag v-if="item.project_name" style="margin-left: 8px">{{ item.project_name }}</a-tag>
                  </template>
                  <template #description>
                    添加于 {{ formatDate(item.created_at) }}
                  </template>
                </a-list-item-meta>
              </a-list-item>
            </template>
            <template #emptyText>
              <a-empty description="暂无项目级路径配置">
                <template #image>
                  <FolderOutlined style="font-size: 48px; color: #ccc" />
                </template>
              </a-empty>
            </template>
          </a-list>
        </a-card>
      </a-col>
    </a-row>

    <a-row style="margin-top: 16px">
      <a-col :span="24">
        <a-card title="常见 Skill 路径参考">
          <aDescriptions :column="2" bordered size="small">
            <aDescriptions-item label="macOS 用户级">
              <code>~/.claude/skills/</code>
            </aDescriptions-item>
            <aDescriptions-item label="macOS 项目级">
              <code>项目目录/.claude/skills/</code>
            </aDescriptions-item>
            <aDescriptions-item label="Linux 用户级">
              <code>~/.claude/skills/</code>
            </aDescriptions-item>
            <aDescriptions-item label="Linux 项目级">
              <code>项目目录/.claude/skills/</code>
            </aDescriptions-item>
          </aDescriptions>
        </a-card>
      </a-col>
    </a-row>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { message } from 'ant-design-vue'
import { useSkillStore } from '@/stores/skill'
import { FolderOutlined } from '@ant-design/icons-vue'
import dayjs from 'dayjs'
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

function formatDate(date: string) {
  return dayjs(date).format('YYYY-MM-DD HH:mm')
}

async function handleAddPath() {
  try {
    await store.addPath(
      newPath.value.path,
      newPath.value.pathType,
      newPath.value.projectName || undefined
    )
    message.success('路径添加成功')
    newPath.value.path = ''
    newPath.value.projectName = ''
  } catch (e) {
    message.error('添加失败: ' + String(e))
  }
}

async function handleRemove(path: string) {
  try {
    await store.removePath(path)
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
  const home = await window.invoke('get_settings_path') as string
  const userPath = home.replace('/settings.json', '/skills')

  if (newPath.value.pathType === 'user') {
    newPath.value.path = userPath
  } else {
    newPath.value.path = './skills'
  }
  message.info('已填充建议路径，请根据实际情况修改')
}

onMounted(async () => {
  await store.fetchPaths()
})
</script>

<style scoped>
.skills-page {
  padding: 24px;
}

.subtitle {
  color: #999;
  margin-top: -8px;
}

code {
  background: #f5f5f5;
  padding: 2px 6px;
  border-radius: 4px;
  font-family: monospace;
}
</style>
