# DevAI Skill Hub

面向开发团队的 **Skill 标准化管控平台**。通过 Claude Code Hooks 机制实现 Skill 检测与监控，专注 Skill 规范管理。

## 核心功能

- **Skill 自动检测**：通过 Claude Code PreToolUse Hook 实时监控 Skill 调用
- **AI 决策分析**：区分 AI 自动触发 vs 用户手动触发的 Skill
- **可视化监控面板**：展示 Skill 使用排行、执行记录、趋势分析
- **多路径配置**：支持用户级和项目级 Skill 路径管理

## 技术栈

| 层级 | 技术 |
|-----|------|
| 桌面客户端 | Tauri 2.0 + Vue 3 + TypeScript |
| UI 组件 | Ant Design Vue |
| 状态管理 | Pinia + Vue Router |
| 本地存储 | SQLite (rusqlite) |
| Hook 脚本 | Python 3 |

## 项目结构

```
DevAI-Skill-Hub/
├── src/                      # 前端 Vue 代码
│   ├── api/                  # Tauri 命令调用封装
│   ├── components/           # 通用组件
│   ├── router/               # 路由配置
│   ├── stores/               # Pinia 状态管理
│   └── views/                # 页面视图
│       ├── Dashboard.vue      # 监控面板
│       ├── Skills.vue         # Skill 配置管理
│       └── Settings.vue       # 设置页面
├── src-tauri/                # Rust 后端代码
│   └── src/
│       ├── db/               # SQLite 数据库模块
│       ├── commands/          # Tauri 命令
│       ├── lib.rs             # 入口
│       └── main.rs            # 主程序
├── scripts/
│   └── skill_hook.py         # Claude Code Hook 脚本
└── CLAUDE.md                 # 项目开发说明
```

## 快速开始

### 1. 安装依赖

```bash
pnpm install
```

### 2. 配置 Claude Code Hook

将以下配置添加到 `~/.claude/settings.json`：

```json
{
  "hooks": {
    "UserPromptExpansion": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "/Users/apple/.claude/hooks/skill-hook.py"
          }
        ]
      }
    ],
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "/Users/apple/.claude/hooks/skill-hook.py"
          }
        ]
      }
    ]
  }
}
```

**说明**：
- `UserPromptExpansion`：捕获用户直接输入的 `/skill-name` 命令
- `PreToolUse (Bash)`：从 Bash 命令中分析 skill 调用
- 通过 transcript 分析自动区分 `user` 和 `ai_auto` 触发类型

### 3. 启动开发模式

```bash
pnpm tauri dev
```

### 4. 构建发布版本

```bash
pnpm tauri build
```

## 使用流程

1. 启动 DevAI Skill Hub 桌面客户端
2. 在 Claude Code 中正常使用，Hook 自动检测 Skill 调用
3. 在监控面板查看 Skill 使用数据

## 数据存储

| 平台 | 路径 |
|-----|------|
| macOS | `~/Library/Application Support/com.devai.skill-hub/` |
| Linux | `~/.local/share/com.devai.skill-hub/` |
| Windows | `%APPDATA%\com.devai.skill-hub\` |

## 开发说明

- 详见 [CLAUDE.md](./CLAUDE.md)
- 详见 [PRD](./DevAI%20Skill%20Hub%20产品需求文档（PRD）.md)
