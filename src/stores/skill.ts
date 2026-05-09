import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { skillApi, type SkillExecution, type SkillPath, type SkillStats } from '@/api/skill'

const REFRESH_INTERVAL = 5000 // 5秒刷新一次

export const useSkillStore = defineStore('skill', () => {
  // State
  const executions = ref<SkillExecution[]>([])
  const stats = ref<SkillStats[]>([])
  const paths = ref<SkillPath[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  let refreshTimer: number | null = null

  // Getters
  const userPaths = computed(() => paths.value.filter(p => p.path_type === 'user'))
  const projectPaths = computed(() => paths.value.filter(p => p.path_type === 'project'))

  const statsBySkill = computed(() => {
    const grouped: Record<string, { total: number; user: number; ai: number }> = {}
    for (const s of stats.value) {
      if (!grouped[s.skill_id]) {
        grouped[s.skill_id] = { total: 0, user: 0, ai: 0 }
      }
      grouped[s.skill_id].total += s.count
      if (s.trigger_type === 'user') {
        grouped[s.skill_id].user += s.count
      } else {
        grouped[s.skill_id].ai += s.count
      }
    }
    return Object.entries(grouped)
      .map(([skill_id, data]) => ({ skill_id, ...data }))
      .sort((a, b) => b.total - a.total)
  })

  // Actions
  async function fetchExecutions(limit = 100) {
    loading.value = true
    error.value = null
    try {
      executions.value = await skillApi.getExecutions(limit)
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function fetchStats(days = 7) {
    loading.value = true
    error.value = null
    try {
      stats.value = await skillApi.getStats(days)
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function fetchPaths() {
    loading.value = true
    error.value = null
    try {
      paths.value = await skillApi.getPaths()
    } catch (e) {
      error.value = String(e)
    } finally {
      loading.value = false
    }
  }

  async function addPath(path: string, pathType: string, projectName?: string) {
    loading.value = true
    error.value = null
    try {
      await skillApi.addPath(path, pathType, projectName)
      await fetchPaths()
    } catch (e) {
      error.value = String(e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function removePath(path: string) {
    loading.value = true
    error.value = null
    try {
      await skillApi.removePath(path)
      await fetchPaths()
    } catch (e) {
      error.value = String(e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function togglePath(path: string, enabled: boolean) {
    try {
      await skillApi.togglePath(path, enabled)
      const p = paths.value.find(p => p.path === path)
      if (p) p.enabled = enabled
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  async function fetchAll() {
    await Promise.all([fetchExecutions(), fetchStats(), fetchPaths()])
  }

  function startAutoRefresh() {
    if (refreshTimer) return
    fetchAll() // 立即执行一次
    refreshTimer = window.setInterval(() => {
      fetchAll()
    }, REFRESH_INTERVAL)
  }

  function stopAutoRefresh() {
    if (refreshTimer) {
      clearInterval(refreshTimer)
      refreshTimer = null
    }
  }

  return {
    // State
    executions,
    stats,
    paths,
    loading,
    error,
    // Getters
    userPaths,
    projectPaths,
    statsBySkill,
    // Actions
    fetchExecutions,
    fetchStats,
    fetchPaths,
    addPath,
    removePath,
    togglePath,
    fetchAll,
    startAutoRefresh,
    stopAutoRefresh
  }
})
