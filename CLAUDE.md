# DevAI Skill Hub 项目说明

## 项目概述

面向开发团队的 **Skill 标准化管控平台**。通过 Claude Code Hooks 机制实现 Skill 检测与监控，专注 Skill 规范管理，不做流量代理和 Token 统计。

## 核心定位

- **不同于 MITM 代理**：不劫持流量，不修改系统代理
- **不冲突 tingly-box**：与现有 Agent 网关共存
- **不冲突 Clash**：无需修改网络设置
- **专注 Skill**：识别、监控、管理 Skill 规范

## 监控目标

**不是为了记录用户输入的 Skill 命令，而是理解 AI 的决策过程：**
- AI **主动调用**了哪些 Skill？为什么调用？
- AI **应该调用但没有调用** Skill？为什么没调用？

### 监控时机（触发记录的条件）

| 场景 | 说明 | 触发条件 |
|-----|------|---------|
| **Skill 被调用** | AI 实际执行了 `/xxx` Skill | PreToolUse Hook 检测到 `tool_name == "Skill"` |
| **Thinking 提及 Skill** | AI 在思考过程中提到考虑/应该用某个 Skill | 解析 Thinking 内容 |
| **Skill 被忽略** | AI 没有调用某个本该调用的 Skill | 基于任务描述 + Thinking 双重判断 |

### 判断逻辑（双重判断）

```
用户任务描述 + AI Thinking 内容
            ↓
判断1: 基于任务描述推断应该用什么 Skill
判断2: 基于 Thinking 分析是否有提及
            ↓
分析结果:
├── Skill 被调用 → 记录为 'invoked'
├── Thinking 提到但未调用 → 记录为 'considered_but_not_used'
└── 完全未考虑 → 记录为 'not_considered'
```

## 技术方案

### Claude Code Hooks 方案

```
Claude Code 执行任务
       ↓
   Hooks 触发 (PreToolUse)
       ↓
检测 tool_name == "Skill"
       ↓
解析 tool_input JSON → 提取 skill 字段
       ↓
读取 transcript_path 获取完整上下文
       ↓
双重判断 → 记录到本地 SQLite
       ↓
   前端展示监控
```

### Claude Code Skill 调用机制

当 AI 调用 Skill 时，PreToolUse Hook 收到的数据：

```json
{
  "tool_name": "Skill",
  "tool_input": "{\"skill\": \"dev-utility\", \"args\": \"重构代码\"}",
  "session_id": "abc123",
  "transcript_path": "/path/to/session.jsonl",
  "cwd": "/project/path"
}
```

### Claude Code Skill 查找路径

| 优先级 | 路径 | 说明 |
|-------|------|------|
| 1 | `项目目录/.claw/skills/` | 项目级 |
| 2 | `项目目录/.claude/skills/` | Claude 项目级 |
| 3 | `~/.claw/skills/` | 用户级 |
| 4 | `~/.claude/skills/` | Claude 用户级 |

### Skill 命令格式

| 命令 | 含义 | 示例 |
|-----|------|------|
| `/` | Skill 调用 | `/dev-utility`、`/docs-updater` |
| `@` | Agent 调用 | `@dev-utility`（**不需要处理**）|

> 注意：`/` 是 Skill 调用，`@` 是 Agent 调用。本项目只监控 Skill。

### Claude Code Hooks 类型

| Hook | 触发时机 | 用途 |
|------|----------|------|
| `pre_tool_use` | 工具调用前 | 识别即将执行的 Skill，记录触发类型 |
| `post_tool_use` | 工具调用后 | 记录 Skill 执行结果（成功/失败） |
| `on_error` | 错误发生时 | 记录失败信息 |

## 技术栈

- **Skill 检测**: Claude Code Hooks (Shell/Python 脚本)
- **本地存储**: SQLite (轻量、零配置)
- **桌面客户端**: Tauri 2.0 + Vue 3 + TypeScript + Ant Design Vue
- **云端 (V1.1)**: Go + PostgreSQL（暂不需要 Redis）

## 数据存储

### SQLite 数据库表结构

```sql
-- Skill 执行记录（核心表）
CREATE TABLE skill_executions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    skill_id TEXT NOT NULL,              -- 如 "dev-utility"
    args TEXT,                           -- Skill 参数
    trigger_type TEXT NOT NULL,          -- 'user' | 'ai_auto'
    context_summary TEXT,                -- 任务摘要（AI 分析）
    thinking_hint TEXT,                  -- Thinking 中提到的 Skill
    status TEXT DEFAULT 'invoked',       -- 'invoked' | 'considered_but_not_used' | 'not_considered'
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Skill 路径配置
CREATE TABLE skill_paths (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    path_type TEXT NOT NULL,             -- 'user' | 'project'
    project_name TEXT,                   -- 所属项目（可选）
    enabled BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 会话上下文（用于分析）
CREATE TABLE session_context (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    user_prompt TEXT NOT NULL,
    skill_mentioned_in_thinking TEXT,   -- AI Thinking 中提到的 Skill
    skills_invoked TEXT,                 -- 实际调用的 Skill（逗号分隔）
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Skill 规范定义
CREATE TABLE skill_definitions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    skill_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    version TEXT DEFAULT '1.0.0',
    config JSON,                        -- Skill 配置
    is_team_skill BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### 多 Skill 记录

一个 prompt 中出现多个 Skill 时，每个 Skill 作为独立监控 item：

```
用户: /explore 研究代码 && /dev-utility 重构
    ↓
记录: skill_executions 表新增 2 条记录
    ├── skill_id: "explore", status: "invoked"
    └── skill_id: "dev-utility", status: "invoked"
```

## 模块说明

### backend/

| 模块 | 说明 |
|------|------|
| `skill` | Skill 识别引擎，从 prompt 提取 Skill ID |
| `storage` | SQLite 存储，存储 Skill 日志和配置 |
| `sync` | 云端同步客户端 (V1.1) |

### frontend/

| 目录 | 说明 |
|------|------|
| `views` | 页面：Skill 监控、Skill 管理、团队配置 |
| `components` | 通用组件 |
| `stores` | 状态管理 (Pinia) |
| `api` | Tauri 命令调用封装 |

### scripts/

## Skill 路径配置

用户可以配置多个 Skill 路径，支持：
- **用户级路径**：如 `~/.claude/skills/`
- **项目级路径**：如 `./.claude/skills/`（每个项目可不同）

### 配置管理
- 前端提供输入框让用户引入 Skill 路径
- 支持多路径引入和管理
- 支持删除已添加的路径

## 与 MITM 方案的对比

| 对比项 | MITM 代理方案 | Hooks 方案 (当前) |
|--------|---------------|-------------------|
| 系统代理 | 需要修改 | ✅ 不需要 |
| Clash | 可能冲突 | ✅ 无影响 |
| tingly-box | 可能冲突 | ✅ 无影响 |
| 开发复杂度 | 高 | ✅ 低 |
| Token 统计 | 支持 | ❌ 不支持 |
| Skill 检测 | 支持 | ✅ 支持 |

## 团队协作（V1.1）

### 权限体系
- **管理员**：可编辑团队 Skill 规范、发布版本、管理成员
- **普通成员**：只能查看和同步团队配置

### 版本策略
- 管理员发布新版本后，成员可选择是否同步
- 支持版本回滚

### Hook 配置
- 支持一键自动配置（程序自动修改 settings.json）
- 同时提供手动配置方案（用户复制脚本路径）

## 开发规范

- Go 代码遵循 `go fmt` 和 `golangci-lint`
- Vue 组件使用 Composition API + `<script setup>`
- Claude Code Hooks 脚本使用 Shell + Python

## Hook 脚本实现示例

```python
#!/usr/bin/env python3
import json
import sqlite3
import re
import sys
from datetime import datetime
from pathlib import Path

DB_PATH = Path.home() / ".devai-skill-hub" / "skills.db"

def extract_skill_id(tool_input: str) -> str | None:
    """从 Skill 工具输入中提取 Skill ID"""
    try:
        data = json.loads(tool_input)
        skill = data.get("skill", "")
        # 去除 / 前缀
        return skill.strip("/") if skill else None
    except json.JSONDecodeError:
        return None

def detect_trigger_type(session_id: str, skill_id: str) -> str:
    """判断触发类型：用户手动还是 AI 自动"""
    # 读取会话历史判断
    # 如果用户 prompt 中包含 /skill_id 则为 'user'
    # 否则为 'ai_auto'
    return 'ai_auto'

def process_hook_input():
    """处理 Hook 输入"""
    input_data = json.loads(sys.stdin.read())

    if input_data.get("tool_name") != "Skill":
        return

    skill_id = extract_skill_id(input_data.get("tool_input", ""))
    if not skill_id:
        return

    # 确保数据库目录存在
    DB_PATH.parent.mkdir(parents=True, exist_ok=True)

    # 记录到数据库
    conn = sqlite3.connect(DB_PATH)
    conn.execute("""
        INSERT INTO skill_executions
        (session_id, skill_id, trigger_type, timestamp)
        VALUES (?, ?, ?, ?)
    """, (
        input_data["session_id"],
        skill_id,
        detect_trigger_type(input_data["session_id"], skill_id),
        datetime.now().isoformat()
    ))
    conn.commit()
    conn.close()

if __name__ == "__main__":
    process_hook_input()
```

## 相关文档

- [PRD](./DevAI%20Skill%20Hub%20产品需求文档（PRD）.md)
