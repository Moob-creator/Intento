# 更新日志

本文档记录 Intento 的版本历史和重要更新。

格式遵循 [Keep a Changelog](https://keepachangelog.com/)。

---

## [Unreleased]

### 计划中
- 日历周视图
- 任务拖拽重新排期
- 操作撤销功能
- 上下文缓存机制

---

## [0.1.0] - 2026-02-19

### 🎉 首个 MVP 版本发布

这是 Intento 的首个可用版本，包含了核心的任务管理和 AI 功能。

### ✨ 新增功能

#### 核心功能
- **Command Palette** - ⌘K 驱动的极简界面
- **任务管理** - 完整的 CRUD 操作
- **智能排序** - 按优先级、状态、日期自动排序
- **快捷键系统** - 全键盘操作支持

#### AI 能力
- **文本解析** - 自然语言创建任务
- **图片识别** - 从截图提取任务信息
- **多操作支持** - 创建/更新/完成/删除/批量操作
- **智能提取** - 自动提取标题、描述、优先级、截止时间、标签

#### 通知系统
- **截止日期提醒** - 可配置的提前提醒时间
- **桌面通知** - 系统级通知支持
- **灵活时间** - 15分钟到1天前的多种选项
- **测试功能** - 验证通知设置

#### 总结功能
- **每日总结** - 自动生成每日工作总结
- **每周总结** - 周度成果回顾
- **每月总结** - 月度产出分析
- **半年度/年度总结** - 长期成长轨迹
- **总结中心** - Timeline 形式查看历史

#### 可视化
- **日历视图** - 月视图展示任务分布
- **统计面板** - 任务完成率和分布统计
- **优先级指示** - 彩色点标识优先级
- **今日高亮** - 当前日期特殊标识

#### 高级功能
- **高级筛选** - 多维度任务筛选
- **标签系统** - 支持多标签分类
- **搜索功能** - Command Palette 内快速搜索
- **任务详情** - 侧边面板完整编辑

### 🎨 设计特色
- **温暖色系** - 柔和的珊瑚色、桃色、米色主题
- **圆角设计** - 8-12px 圆角，柔和视觉
- **流畅动画** - 200-300ms 的平滑过渡
- **响应式布局** - 适配不同屏幕尺寸

### 🛠️ 技术栈
- **前端**: React 19 + TypeScript + Tailwind CSS
- **后端**: Rust + Tauri 2.0
- **数据库**: SQLite + Rusqlite
- **AI**: ADK-Rust (OpenAI/Anthropic/Kimi)
- **状态管理**: Zustand
- **调度器**: tokio-cron-scheduler

### 📱 平台支持
- macOS (Universal Binary)
- Windows (x86_64)
- Linux (主流发行版)

### 📚 文档
- 完整的用户指南
- 图片识别使用指南
- 通知设置指南
- 开发文档
- API 文档

---

## [阶段性更新] - 开发历程

### Phase 5 - 自动总结功能 (2026-02-11 ~ 2026-02-12)

#### Added
- 创建 summary 模块
- 实现数据聚合和 AI 总结
- generate_summary/get_summary/get_summaries commands
- 基于 tokio-cron-scheduler 的调度器
- 支持每日/每周/每月/半年度/年度总结
- SummaryPanel 组件
- CustomSelect 主题统一
- Toast 通知系统
- 设置面板集成
- 自动总结开关配置

### Phase 4 - 智能提醒系统 (2026-02-10 ~ 2026-02-11)

#### Added
- TaskScheduler 基于 tokio-cron-scheduler
- 每小时检查即将到期任务
- 自动触发通知
- send_notification/check_expiring_tasks/test_notification commands
- 支持多种通知类型
- 跨平台通知支持
- get_expiring_tasks 数据库方法

### 界面重构 (2026-02-11)

#### Added
- Command Palette (⌘K)
- 模糊搜索任务
- 快捷操作菜单
- 键盘导航
- 极简顶部栏设计
- 全局键盘快捷键系统
- 统计面板（Statistics Panel）
- 设置面板（Settings Panel）
- 任务卡片 hover 快捷操作
- 智能任务排序（优先级 > 状态 > 日期）

#### Removed
- 传统侧边栏（释放 30% 屏幕空间）

### Phase 3 - AI 能力集成 (2026-02-09 ~ 2026-02-10)

#### Added
- AI 客户端封装
- 支持 OpenAI/Anthropic/Kimi
- 统一的 parse_input 接口
- 错误处理和重试逻辑
- parse_text_input Tauri command
- AI 解析任务标题、描述、截止时间
- TaskConfirmDialog 组件
- 展示 AI 解析结果并允许编辑

### Phase 2 - 基础任务管理 (2026-02-09)

#### Added
- 任务管理 Tauri Commands
- Zustand 任务状态管理
- 任务列表组件
- 任务卡片组件（支持 hover 快捷操作）
- 状态筛选（通过 Command Palette）
- 搜索功能（通过 Command Palette）
- TaskDetailPanel 侧滑面板
- 表单验证

### Phase 1 - 核心基础设施 (2026-02-09)

#### Added
- SQLite 数据库配置
- 数据库 Schema 设计
- 3 张核心表：tasks, summaries, context_cache
- 完整的索引优化
- 软删除机制
- JSON 字段支持
- Task 结构体及 TaskStatus、Priority 枚举
- Summary 结构体及 SummaryType 枚举
- ContextCache 结构体及 CacheType 枚举
- 完整的 Serde 序列化支持
- Task CRUD 操作
- Summary 操作
- Cache 操作
- 单元测试覆盖

### Phase 0 - 项目初始化 (2026-02-08)

#### Added
- Tauri 项目搭建
- React + TypeScript 前端配置
- Tailwind CSS 配置
- ADK-Rust 依赖集成
- 基础项目结构

---

## 版本标签说明

- `Added` - 新增功能
- `Changed` - 功能变更
- `Deprecated` - 即将废弃
- `Removed` - 已移除功能
- `Fixed` - Bug 修复
- `Security` - 安全相关

---

## 链接

- [产品路线图](./ROADMAP.md)
- [用户指南](./user-guide/README.md)
- [开发文档](./specs/development-plan.md)

---

**最后更新:** 2026-02-19
