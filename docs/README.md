# Intento 文档中心

欢迎来到 Intento (无忧记) 项目文档。这里包含了产品规格、开发指南、用户手册等所有文档。

## 📚 文档导航

### 👤 用户文档
**位置:** `user-guide/`

- **[用户指南](./user-guide/README.md)** - 完整的使用手册
  - 快速开始
  - 核心功能介绍
  - 键盘快捷键
  - 工作流程建议
  - 常见问题

- **功能详解**
  - **[图片识别功能](./user-guide/features/image-parsing.md)** - 从图片创建和管理任务
  - **[通知设置](./user-guide/features/notifications.md)** - 配置智能提醒系统

### 📋 产品规格
**位置:** `specs/`

- **[产品需求文档 (PRD v3)](./specs/prd-v3.md)** - 产品定位、功能需求、用户场景
- **[开发计划](./specs/development-plan.md)** - 完整的开发阶段和任务拆分
- **[数据库设计](./specs/database-schema.md)** - SQLite 数据库架构
- **[技术选型](specs/tech-stack.md)** - 技术栈研究和决策
- **[前端参考](./specs/frontend-reference.md)** - 前端架构和组件设计

### 🗺️ 项目规划

- **[产品路线图](./ROADMAP.md)** - 未来功能和版本规划
- **[更新日志](./CHANGELOG.md)** - 版本历史和更新记录

### 📦 历史文档
**位置:** `archive/`

归档的旧版本文档和已完成阶段的详细设计，供参考和追溯使用。

---

## 🚀 快速链接

### 新用户
1. 阅读 [用户指南](./user-guide/README.md) 了解基本操作
2. 查看 [图片识别](./user-guide/features/image-parsing.md) 学习 AI 功能
3. 配置 [通知设置](./user-guide/features/notifications.md) 开启提醒

### 开发者
1. 查看 [开发计划](./specs/development-plan.md) 了解项目结构
2. 阅读 [数据库设计](./specs/database-schema.md) 理解数据模型
3. 参考 [技术选型](specs/tech-stack.md) 了解技术决策
4. 查看 [产品路线图](./ROADMAP.md) 了解待开发功能

### 产品经理
1. 阅读 [PRD v3](./specs/prd-v3.md) 了解产品定位
2. 查看 [产品路线图](./ROADMAP.md) 了解未来规划
3. 参考 [开发计划](./specs/development-plan.md) 了解实施进度

---

## 📂 文档结构

```
docs/
├── README.md                        # 本文档 - 文档导航
├── ROADMAP.md                       # 产品路线图
├── CHANGELOG.md                     # 更新日志
│
├── user-guide/                      # 用户文档
│   ├── README.md                   # 用户指南主页
│   └── features/                   # 功能详解
│       ├── image-parsing.md        # 图片识别
│       └── notifications.md        # 通知设置
│
├── specs/                           # 产品和技术规格
│   ├── prd-v3.md                   # 产品需求文档
│   ├── development-plan.md         # 开发计划
│   ├── database-schema.md          # 数据库设计
│   ├── tech-stack.md               # 技术选型
│   └── frontend-reference.md       # 前端参考
│
└── archive/                         # 历史文档归档
    ├── prd-v1.md                   # 旧版 PRD
    ├── prd-v3.1-summary.md
    ├── phase1-summary.md           # Phase 1 总结
    ├── progress.md                 # 旧版进度跟踪
    ├── TODO.md                     # 旧版待办清单
    ├── phase5/                     # Phase 5 详细设计
    └── front-reference-pages/      # 前端参考页面
```

---

## 🎯 项目概览

### 产品定位
Intento (无忧记) 是一款智能 Todo 桌面应用，通过 AI 帮助用户快速管理任务，自动生成总结。

### 核心功能
- ✅ **智能任务管理** - Command Palette 驱动的极简界面
- 🤖 **AI 文本解析** - 自然语言创建任务
- 📸 **图片识别** - 从截图提取任务信息
- 🔔 **智能提醒** - 截止日期自动提醒
- 📊 **自动总结** - 每日/周/月总结生成
- 📅 **日历视图** - 可视化任务时间分布
- 🎨 **温暖设计** - 柔和色系，舒适体验

### 技术栈
- **前端:** React 19 + TypeScript + Tailwind CSS + Zustand
- **后端:** Rust + Tauri 2.0
- **数据库:** SQLite + Rusqlite
- **AI:** ADK-Rust (支持 OpenAI/Anthropic/Kimi)
- **调度:** tokio-cron-scheduler

---

## 📊 项目状态

### 当前版本
**v0.1.0** - MVP 版本

### 已完成阶段
- ✅ Phase 0: 项目初始化
- ✅ Phase 1: 核心基础设施
- ✅ Phase 2: 基础任务管理
- ✅ Phase 3: AI 能力集成
- ✅ Phase 4: 智能提醒系统
- ✅ Phase 5: 自动总结功能

### 进行中
- 🔄 UI 优化和用户体验改进
- 🔄 高级筛选和搜索功能
- 🔄 日历视图增强

详见：[产品路线图](./ROADMAP.md)

---

## 🤝 贡献指南

### 文档更新

当修改功能或添加新功能时，请同步更新相关文档：

1. **用户可见功能** → 更新 `user-guide/`
2. **技术变更** → 更新 `specs/` 中的对应文档
3. **新版本发布** → 更新 `CHANGELOG.md`
4. **未来规划** → 更新 `ROADMAP.md`

### 文档规范

- 使用 Markdown 格式
- 保持简洁清晰的语言
- 添加示例和截图（如适用）
- 更新文档目录结构
- 标注更新日期

---

## 📞 获取帮助

### 文档问题
如果文档有错误或不清晰的地方，请：
1. 创建 Issue 描述问题
2. 提出改进建议
3. 提交 PR 修复

### 功能问题
- 查看 [用户指南](./user-guide/README.md)
- 查看 [常见问题](./user-guide/README.md#常见问题)
- 搜索已有 Issues

---

## 📝 文档维护

### 更新频率
- **用户文档**: 每次功能更新后同步
- **技术文档**: 架构变更时更新
- **路线图**: 每月/季度更新
- **更新日志**: 每次版本发布时更新

### 维护者
@wangshuo

### 最后更新
2026-02-19

---

## 🔗 相关链接

- **项目地址**: `/Users/wangshuo/codes/projects/Intento`
- **官网**: (待添加)
- **GitHub**: (待添加)

---

**欢迎使用 Intento！** 🎉
