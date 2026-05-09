import { invoke } from '@tauri-apps/api/core'

export interface SkillExecution {
  id: number
  session_id: string
  skill_id: string
  args: string | null
  trigger_type: string
  context_summary: string | null
  thinking_hint: string | null
  status: string
  timestamp: string
  created_at: string
}

export interface SkillPath {
  id: number
  path: string
  path_type: string
  project_name: string | null
  enabled: boolean
  created_at: string
}

export interface SkillStats {
  skill_id: string
  count: number
  trigger_type: string
}

export const skillApi = {
  // 获取 Skill 执行记录
  getExecutions(limit?: number): Promise<SkillExecution[]> {
    return invoke('get_skill_executions', { limit })
  },

  // 获取 Skill 统计数据
  getStats(days?: number): Promise<SkillStats[]> {
    return invoke('get_skill_stats', { days })
  },

  // 获取 Skill 路径配置
  getPaths(): Promise<SkillPath[]> {
    return invoke('get_skill_paths')
  },

  // 添加 Skill 路径
  addPath(path: string, pathType: string, projectName?: string): Promise<number> {
    return invoke('add_skill_path', { path, pathType, projectName })
  },

  // 删除 Skill 路径
  removePath(path: string): Promise<void> {
    return invoke('remove_skill_path', { path })
  },

  // 切换路径启用状态
  togglePath(path: string, enabled: boolean): Promise<void> {
    return invoke('toggle_skill_path', { path, enabled })
  },

  // 手动记录 Skill 执行
  recordExecution(
    sessionId: string,
    skillId: string,
    args?: string,
    triggerType?: string,
    status?: string
  ): Promise<number> {
    return invoke('record_skill_execution', {
      sessionId,
      skillId,
      args,
      triggerType,
      status
    })
  },

  // 获取 Hook 脚本路径
  getHookScriptPath(): Promise<string> {
    return invoke('get_hook_script_path')
  },

  // 获取 Claude Code settings.json 路径
  getSettingsPath(): Promise<string> {
    return invoke('get_settings_path')
  },

  // 获取某个路径下的 Skill 列表
  getSkillsInPath(path: string): Promise<string[]> {
    return invoke('get_skills_in_path', { path })
  }
}
