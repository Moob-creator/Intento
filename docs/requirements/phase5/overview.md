# Phase 5: 自动总结功能 - 总览

## 功能概述

为 Intento 任务管理应用设计一个按 tag 维度进行自动总结的功能，支持多个时间维度（每天、每周、每月、每半年、每年），总结内容持久化存储，可在设置中开启/关闭。

## 核心目标

1. **按 Tag 维度总结**: 可以为特定 tag 或全局任务生成总结
2. **多时间周期**: 支持每日、每周、每月、每半年、每年的总结
3. **自动化生成**: 定时任务自动触发生成，无需手动干预
4. **持久化存储**: 总结内容保存到数据库，支持历史回顾
5. **可配置**: 在设置面板中可开启/关闭，选择生成频率
6. **非侵入式 UI**: 自然融入现有界面，不显得突兀

## 功能特性

### 核心功能
- ✅ 按 tag 维度生成任务总结
- ✅ 支持 5 种时间周期（日/周/月/半年/年）
- ✅ AI 生成温暖、鼓励性的总结文本
- ✅ 自动计算统计数据（完成率、优先级分布等）
- ✅ 定时任务自动生成总结
- ✅ 手动触发生成总结

### 用户交互
- ✅ Sidebar Tag 右键菜单入口
- ✅ Command Palette 快捷命令
- ✅ 侧边滑出面板展示总结
- ✅ 历史总结浏览
- ✅ 导出总结（Markdown/Text）

### 设置和配置
- ✅ 开启/关闭自动总结
- ✅ 选择生成频率
- ✅ 设置保留时长
- ✅ 配置持久化

## 技术栈

### 前端
- React 19 + TypeScript
- Tailwind CSS（Notion 风格设计）
- Zustand 状态管理
- lucide-react 图标库

### 后端
- Rust + Tauri 2.0
- SQLite 数据库
- tokio-cron-scheduler 定时任务
- AI Client（Kimi API）

## 项目结构

```
docs/phase5/
├── overview.md                    # 本文档 - 总览
├── ui-design/
│   ├── user-flows.md             # 用户交互流程
│   ├── component-specs.md        # 组件规格说明
│   └── ui-mockups.md             # UI 设计草图
├── backend-design/
│   ├── data-model.md             # 数据模型设计
│   ├── summary-generation.md    # 总结生成逻辑
│   ├── scheduler-integration.md # 定时任务集成
│   └── api-commands.md           # Tauri Commands API
├── implementation/
│   ├── phase5.1-core.md         # Phase 5.1 核心功能实现
│   ├── phase5.2-scheduler.md    # Phase 5.2 定时任务
│   ├── phase5.3-settings.md     # Phase 5.3 设置集成
│   ├── phase5.4-export.md       # Phase 5.4 导出功能
│   └── phase5.5-optimization.md # Phase 5.5 优化增强
└── testing/
    ├── test-plan.md              # 测试计划
    └── test-cases.md             # 测试用例
```

## 实现优先级

### Phase 5.1: 核心总结功能 (P0 - 必须实现)
**预计时间: 3-4 天**

- 数据库迁移（添加 tag 字段到 summaries 表）
- SummaryGenerator 核心逻辑
- AI Prompt 设计和测试
- `generate_summary` / `list_summaries` commands
- SummaryPanel 基础 UI
- Sidebar Tag 右键菜单集成

### Phase 5.2: 定时任务集成 (P0)
**预计时间: 1-2 天**

- Scheduler 添加每日/周/月总结 Job
- 避免重复生成逻辑
- 错误处理和日志

### Phase 5.3: 设置面板集成 (P1)
**预计时间: 1 天**

- Settings UI 设计
- 配置项持久化（localStorage 或数据库）
- 根据设置控制定时任务

### Phase 5.4: 导出功能 (P1)
**预计时间: 1 天**

- `export_summary` command
- Markdown 格式化
- 文件保存对话框（Tauri Dialog API）

### Phase 5.5: 历史浏览和优化 (P2)
**预计时间: 1-2 天**

- SummaryTimeline 组件
- 分页加载历史总结
- 搜索/筛选功能
- 性能优化（虚拟滚动）

## 关键文件清单

### 后端关键文件

1. **src-tauri/src/db/models.rs**
   - 扩展 Summary 模型，添加 tag 和 tag_filter 字段
   - 扩展 SummaryType 枚举（添加 Weekly 和 SemiAnnual）

2. **src-tauri/src/summary/mod.rs** (新建)
   - SummaryGenerator 核心逻辑
   - 任务查询和统计计算
   - AI 总结生成

3. **src-tauri/src/ai/prompts.rs**
   - 添加 summary_generation_prompt 函数
   - 设计温暖、鼓励性的 Prompt

4. **src-tauri/src/commands/summary.rs** (新建)
   - generate_summary command
   - list_summaries command
   - get_summary command
   - delete_summary command
   - export_summary command

5. **src-tauri/src/scheduler/mod.rs**
   - 添加每日/周/月总结定时任务

### 前端关键文件

1. **src/components/SummaryPanel.tsx** (新建)
   - 总结展示主面板
   - Tag 选择器集成
   - 时间范围选择器集成

2. **src/components/TimeRangeSelector.tsx** (新建)
   - 时间维度选择组件
   - 支持 5 种时间周期切换

3. **src/components/SummaryContent.tsx** (新建)
   - Markdown 总结内容渲染
   - 统计数据可视化

4. **src/components/SummaryTimeline.tsx** (新建)
   - 历史总结时间线
   - 总结列表展示

5. **src/components/Sidebar.tsx**
   - 添加 Tag 右键菜单
   - 集成总结功能入口

6. **src/components/SettingsPanel.tsx**
   - 添加总结设置部分
   - 开关、频率、保留时长配置

7. **src/store/taskStore.ts**
   - 添加 summary 相关状态管理
   - 添加 summary 操作方法

## 下一步行动

1. 阅读详细的设计文档（ui-design/ 和 backend-design/）
2. 确认设计方案符合需求
3. 开始实现 Phase 5.1 核心功能

## 相关文档

- [用户交互流程](./ui-design/user-flows.md)
- [数据模型设计](./backend-design/data-model.md)
- [总结生成逻辑](./backend-design/summary-generation.md)
- [Phase 5.1 实现指南](./implementation/phase5.1-core.md)
