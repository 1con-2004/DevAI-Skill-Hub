# DevAI Skill Hub 产品需求文档（PRD）

# 一、文档基础信息

|项目名称|DevAI Skill Hub|版本号|V1.0.0|
|---|---|---|---|
|产品定位|基于Claude Code Hooks的Skill标准化管控平台|||
|目标用户<br>|1. 团队管理员（Skill规范制定者）；2. 开发团队成员（ClaudeCode用户）|||
|核心目标|1. 通过Claude Code Hooks实现Skill自动检测；2. 标准化Skill定义，支持团队同步规范；3. 专注Skill监控，不做流量代理|||
|技术方案 Claude Code Hooks（非MITM代理）|||
|开发优先级|第一阶段：本地Hook检测版（核心必做）；第二阶段：团队云端版（扩展必做）|||
|文档作者||更新时间|2026-05-09|

# 二、产品背景与痛点

## 2.1 产品背景

当前开发团队使用 Claude Code 等 AI 工具时，存在以下问题：
- 团队内无统一的 AI 任务（Skill）拆分规范，流程混乱
- 无法统计各 Skill 的使用情况，难以后评估和优化
- 新人入职不了解团队 Skill 规范，配置成本高

基于此，开发 DevAI Skill Hub，通过 Claude Code Hooks 机制实现 Skill 检测与监控，专注 Skill 规范管理，不做流量劫持。

## 2.2 核心痛点

- **监控层面**：无法统计各 Skill 的使用频率和执行情况
- **规范层面**：无统一 Skill 规范，成员自定义任务拆分，流程混乱
- **协作层面**：无法同步团队 Skill 规范，新人配置成本高
- **兼容性**：MITM 代理方案会与 Clash、tingly-box 等工具冲突

# 三、产品核心定位

DevAI Skill Hub 是一款「基于 Claude Code Hooks 的 Skill 检测工具」，不依赖流量代理，核心定位为：

面向开发团队的 **Skill 监控工具** + **Skill 规范管理平台**，实现"Claude Code 无感接入、团队一键同步规范、Skill 使用可追踪"。

## 与 MITM 代理方案对比

| 对比项 | MITM 代理方案 | DevAI (Hooks 方案) |
|--------|---------------|-------------------|
| 系统代理 | 需要修改 | ✅ 不需要 |
| Clash | 可能冲突 | ✅ 无影响 |
| tingly-box | 可能冲突 | ✅ 无影响 |
| Token 统计 | 支持 | ❌ 不支持 |
| Skill 检测 | 支持 | ✅ 支持 |
| 开发复杂度 | 高 | ✅ 低 |

# 四、核心功能需求（分阶段）

## 第一阶段：本地 Hook 检测版（核心必做）

### 4.1.1 Claude Code Hooks 配置

- 提供 Hook 脚本（Shell/Python），用户配置到 Claude Code settings.json
- 支持的 Hook 类型：
  - `pre_tool_use`：工具调用前识别 Skill
  - `post_tool_use`：工具调用后记录结果
  - `on_error`：错误时记录信息

### 4.1.2 Skill 自动识别

- 从 Claude Code 上下文（system prompt、对话历史）中提取 Skill ID
- 支持的 Skill 类型：
  自动检测是什么Skill并记录

### 4.1.3 本地 Skill 监控

- 自动记录每一次 Skill 执行：时间、Skill ID、用户输入摘要、执行结果
- 本地 SQLite 存储，支持按时间、Skill 类型筛选查询
- 本地监控面板：展示 Skill 调用排行、使用频率

### 4.1.4 本地 Skill 配置管理

- 支持本地添加、编辑、删除 Skill 规范
- 支持导入/导出 Skill 配置（JSON 格式）
- 内置默认 Skill 模板

## 第二阶段：团队云端版（扩展必做）

### 4.2.1 云端配置中心（核心）

- 团队管理员可上传、编辑、发布团队统一的 Skill 规范
- Skill 规范支持版本化管理（如 V1.0、V1.1）
- 支持团队成员权限管理

### 4.2.2 团队配置一键同步

- 本地客户端支持"一键同步团队配置"
- 支持查看同步记录

### 4.2.3 团队 Skill 监控看板

- 云端看板：展示全团队 Skill 使用数据
- Skill 调用量排行、各成员使用情况

# 五、用户流程

## 5.1 个人用户流程（本地版）

1. 下载 DevAI Skill Hub 本地客户端
2. 配置 Claude Code Hooks（复制脚本路径到 settings.json）
3. 正常使用 Claude Code，Hooks 自动检测并记录 Skill
4. 客户端自动展示 Skill 监控数据
5. 用户可编辑本地 Skill 配置

## 5.2 团队用户流程（云端+本地）

### 5.2.1 管理员流程

1. 搭建云端配置中心，创建团队空间
2. 在云端编辑团队统一的 Skill 规范
3. 发布 Skill 规范版本
4. 通过云端看板监控团队 Skill 使用情况

### 5.2.2 普通成员流程

1. 配置 Claude Code Hooks
2. 点击"同步团队配置"，拉取云端 Skill 规范
3. 正常使用 Claude Code，Hooks 自动按团队规范识别 Skill
4. 查看本地监控面板，了解个人 Skill 使用情况

# 六、兼容性需求

- **系统兼容性**：支持 macOS 12 及以上、Windows 10 及以上
- **AI 工具兼容性**：Claude Code（主要支持）、Cursor（Hooks 方案待验证）
- **工具兼容性**：与 Clash、tingly-box 等工具无冲突
- **网络**：无需特殊网络配置，Hooks 在本地执行

# 七、非功能需求

## 7.1 性能需求

- Hook 执行延迟：≤50ms，不影响 Claude Code 响应速度
- 本地存储：支持至少 6 个月的 Skill 日志
- 同步速度：团队配置同步（≤50KB）耗时≤1 秒

## 7.2 安全需求

- 数据安全：本地日志仅存储在用户本地
- 隐私：不记录敏感的用户输入内容，仅记录 Skill 类型和执行状态
- Hook 脚本：可审查，开源透明

## 7.3 易用性需求

- 安装简单：复制脚本路径到配置文件
- 操作简洁：监控面板直观，Skill 配置同步仅需 1 步
- 零冲突：不修改系统代理，不影响其他工具

# 八、产品原型核心页面（简要说明）

## 8.1 本地客户端页面

- 首页（监控面板）：展示 Skill 调用排行、使用频率趋势
- Skill 管理页：展示本地所有 Skill 规范，支持添加、编辑、删除
- 设置页：配置 Hook 脚本路径、同步设置

## 8.2 云端配置中心页面

- 仪表盘：团队 Skill 使用数据
- Skill 规范管理：团队 Skill 列表、版本管理、编辑/发布
- 成员管理：添加/删除成员、分配权限

# 九、迭代规划

## V1.0.0（本地 Hook 版）

核心目标：通过 Claude Code Hooks 实现 Skill 检测与本地监控

迭代周期：1-2 周

核心功能：Hooks 配置、Skill 识别、本地监控面板、本地 Skill 配置

## V1.1.0（团队基础版）

核心目标：搭建云端配置中心，实现团队配置同步

迭代周期：2 周

核心功能：云端配置中心、Skill 规范版本管理、团队配置同步

## V1.2.0（团队完善版）

核心目标：完善团队监控、协作功能

迭代周期：2 周

核心功能：团队看板、成员反馈、协作编辑

# 十、风险与应对方案

|风险类型|具体风险|应对方案|
|---|---|---|
|兼容性风险|Claude Code 版本更新可能导致 Hook API 变化|持续关注 Claude Code 更新，提供兼容性适配|
|用户接受度风险|配置 Hook 需要手动修改配置文件|提供一键配置脚本，降低配置门槛|
|数据完整性风险|Hook 执行失败可能导致 Skill 记录丢失|本地缓存机制，断网时记录到本地文件|

# 十一、附录

## 11.1 关键术语定义

- **Skill**：AI 任务的标准化拆分，指独立的、可复用的 AI 能力单元（如 docs-updater、explore-code）
- **Claude Code Hooks**：Claude Code 提供的钩子机制，可在工具调用前后执行自定义脚本
- **Skill 检测**：通过解析 Claude Code 上下文，识别当前执行任务对应的 Skill 类型

## 11.2 Claude Code Hooks 配置示例

```json
{
  "hooks": {
    "pre_tool_use": "/path/to/devai-skill-hub/scripts/skill-detector.sh"
  }
}
```

## 11.3 参考

- Claude Code Hooks 官方文档
- 现有团队 AI 开发流程中的 dev-utility、explore-code 等 Skill 标准

> （注：文档部分内容可能由 AI 生成）
