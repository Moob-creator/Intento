# Intento (无忧记)

<div align="center">

**一款智能 Todo 桌面应用，通过 AI 帮助你快速管理任务，自动生成总结**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/tauri-2.0-24C8DB.svg)](https://tauri.app)
[![React](https://img.shields.io/badge/react-19-61DAFB.svg)](https://react.dev)

[功能特性](#-功能特性) • [快速开始](#-快速开始) • [使用文档](#-使用文档) • [开发指南](#-开发指南) • [路线图](#-路线图)

![Intento Screenshot](docs/assets/screenshot-placeholder.png)
<!-- TODO: 添加实际截图 -->

</div>

---

## 💡 项目动机

在快节奏的工作中，我们常常面临这些问题：

- **📝 信息碎片化** - 聊天、会议中口头提到的任务，转化为 todo 打断工作流
- **🧠 记忆负担重** - 脑子记不住所有待办，容易遗漏重要事项
- **📊 缺少回顾** - 忙碌一天后不记得做了什么，写总结要从记忆中找

**Intento** 正是为了解决这些痛点而生：

✨ **AI 快速记录** - 自然语言或截图即可创建任务，不打断工作流
📸 **图片识别** - 从聊天截图直接提取任务信息
🤖 **自动总结** - AI 自动生成每日/周/月总结，轻松回顾成果
⌨️ **键盘优先** - Command Palette (⌘K) 驱动，效率极致

---

## ✨ 功能特性

### 🎯 核心功能

| 功能 | 描述 |
|------|------|
| **⌨️ Command Palette** | ⌘K 打开通用命令中心，搜索、操作、导航一键完成 |
| **✍️ 智能任务创建** | 自然语言描述或粘贴截图，AI 自动提取任务信息 |
| **📸 图片识别** | 从聊天截图识别多种操作（创建/更新/完成/删除） |
| **🔔 智能提醒** | 可配置的截止日期提前提醒（15分钟~1天） |
| **📊 自动总结** | 每日/周/月自动生成工作总结，回顾成果 |
| **📅 日历视图** | 月视图可视化任务时间分布，快速定位 |
| **🔍 高级筛选** | 按状态、优先级、标签、日期多维度筛选 |
| **🎨 温暖设计** | 柔和色系、圆角设计，舒适的视觉体验 |

### 🚀 技术亮点

- **⚡ 高性能** - Rust + Tauri 2.0 原生性能
- **🔒 数据安全** - SQLite 本地存储，数据不上传
- **🤖 AI 集成** - 支持 OpenAI/Claude/Kimi 多种模型
- **🎯 类型安全** - React 19 + TypeScript 全栈类型安全
- **📦 跨平台** - macOS / Windows / Linux 一键构建

---

## 🚀 快速开始

### 环境要求

- **Node.js**: 18.0+
- **Rust**: 1.75+
- **pnpm**: 8.0+ (推荐) 或 npm

### 安装依赖

```bash
# 克隆项目
git clone https://github.com/your-username/Intento.git
cd Intento

# 安装前端依赖
npm install
# 或使用 pnpm
pnpm install
```

### 配置 AI API Key

创建项目根目录的 `.env` 文件：

```bash
# 选择一个 AI 提供商配置
OPENAI_API_KEY=sk-xxx              # OpenAI (推荐 gpt-4o)
ANTHROPIC_API_KEY=sk-ant-xxx       # Anthropic Claude
MOONSHOT_API_KEY=sk-xxx            # Moonshot Kimi
```

### 开发模式

```bash
# 启动开发服务器（前端热更新 + Rust 后端）
npm run tauri:dev
```

### 构建应用

```bash
# 构建当前平台
npm run tauri:build

# 或构建特定平台
npm run build:mac      # macOS Universal Binary
npm run build:win      # Windows x86_64
```

---

## 📖 使用文档

### 用户文档

- **[用户指南](./docs/user-guide/README.md)** - 完整的使用手册
  - 快速开始
  - 核心功能介绍
  - 键盘快捷键
  - 工作流程建议
  - 常见问题

- **功能详解**
  - [图片识别功能](./docs/user-guide/features/image-parsing.md) - 从图片创建和管理任务
  - [通知设置](./docs/user-guide/features/notifications.md) - 配置智能提醒系统

### 开发文档

- **[产品需求文档 (PRD)](./docs/specs/prd-v3.md)** - 产品定位、功能需求、用户场景
- **[开发计划](./docs/specs/development-plan.md)** - 完整的开发阶段和任务拆分
- **[数据库设计](./docs/specs/database-schema.md)** - SQLite 数据库架构
- **[技术选型](./docs/specs/tech-stack.md)** - 技术栈研究和决策
- **[CLAUDE.md](./CLAUDE.md)** - Claude Code 开发指引

### 项目规划

- **[产品路线图](./docs/ROADMAP.md)** - 未来功能和版本规划
- **[更新日志](./docs/CHANGELOG.md)** - 版本历史和更新记录

---

## 🎮 使用演示

### 创建任务

**方式 1: AI 文本输入**
```
按 ⌘/ → 输入"明天下午3点前完成季度报告，高优先级" → 确认
```

**方式 2: 图片识别**
```
按 ⌘/ → 粘贴截图 → AI 识别任务 → 确认
```

**方式 3: 手动创建**
```
按 ⌘N → 填写表单 → 保存
```

### 查找任务

```
按 ⌘K → 输入关键词 → 选择任务 → 回车打开
```

### 查看总结

```
按 ⌘R → 选择日期范围 → 查看 AI 生成的总结
```

### 快捷键参考

| 快捷键 | 功能 |
|--------|------|
| `⌘K` / `Ctrl+K` | 打开 Command Palette |
| `⌘N` / `Ctrl+N` | 新建任务 |
| `⌘/` / `Ctrl+/` | AI 添加任务 |
| `⌘R` / `Ctrl+R` | 查看总结 |
| `⌘,` / `Ctrl+,` | 打开设置 |
| `⌘W` | 隐藏窗口（点击 Dock 图标可恢复） |
| `ESC` | 关闭面板 |

---

## 🛠️ 开发指南

### 项目结构

```
Intento/
├── src/                          # React 前端
│   ├── components/              # React 组件
│   ├── store/                   # Zustand 状态管理
│   ├── types/                   # TypeScript 类型定义
│   └── App.tsx                  # 应用入口
│
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── commands/            # Tauri Commands
│   │   ├── db/                  # 数据库层
│   │   ├── ai/                  # AI 集成
│   │   ├── scheduler/           # 定时任务
│   │   ├── summary/             # 总结生成
│   │   └── main.rs              # 应用入口
│   ├── migrations/              # 数据库迁移
│   └── Cargo.toml
│
├── docs/                         # 项目文档
│   ├── user-guide/              # 用户文档
│   ├── specs/                   # 技术规格
│   └── archive/                 # 历史文档
│
└── CLAUDE.md                     # Claude Code 指引
```

### 技术栈详解

#### 前端
- **React 19** - UI 框架
- **TypeScript** - 类型安全
- **Tailwind CSS** - 样式系统
- **Zustand** - 状态管理
- **Vite** - 构建工具

#### 后端
- **Rust** - 高性能系统语言
- **Tauri 2.0** - 跨平台桌面框架
- **SQLite + Rusqlite** - 本地数据库
- **ADK-Rust** - AI 集成库
- **tokio-cron-scheduler** - 定时任务

### 开发命令

```bash
# 前端开发
npm run dev              # Vite dev server
npm run build            # 构建前端

# Tauri 应用
npm run tauri:dev        # 开发模式
npm run tauri:build      # 构建应用

# Rust 测试
cd src-tauri
cargo test               # 运行测试
cargo clippy             # 代码检查
```

### 添加新功能

1. **定义 Tauri Command** - `src-tauri/src/commands/`
2. **添加数据模型** - `src-tauri/src/db/models.rs`
3. **实现业务逻辑** - 相应的 Rust 模块
4. **创建前端组件** - `src/components/`
5. **集成状态管理** - `src/store/`
6. **更新文档** - `docs/`

详见：[开发计划](./docs/specs/development-plan.md)

---

## 🗺️ 路线图

### v0.1.0 - MVP 版本 ✅ (当前)

- ✅ Command Palette 驱动界面
- ✅ AI 文本和图片识别
- ✅ 智能提醒系统
- ✅ 自动总结生成
- ✅ 日历视图（月视图）
- ✅ Cmd+W 隐藏窗口，点击 Dock 图标恢复

### v0.2.0 - 增强版 🔄 (计划中)

- [ ] 日历周视图
- [ ] 任务拖拽重新排期
- [ ] 操作撤销 (Undo)
- [ ] 上下文缓存机制
- [ ] 性能优化

### v1.0.0 - 正式版 🚀 (长期规划)

- [ ] 深色模式
- [ ] 多语言支持
- [ ] 时间追踪
- [ ] 完整测试覆盖

详见：[产品路线图](./docs/ROADMAP.md)

---

## 🤝 贡献指南

欢迎贡献代码、报告 Bug、提出功能建议！

### 贡献方式

1. **Fork 项目**
2. **创建分支** (`git checkout -b feature/AmazingFeature`)
3. **提交更改** (`git commit -m 'Add some AmazingFeature'`)
4. **推送分支** (`git push origin feature/AmazingFeature`)
5. **提交 Pull Request**

### 开发规范

- 遵循现有代码风格
- 添加必要的测试
- 更新相关文档
- 提交信息清晰明确

---

## 📄 许可证

本项目采用 [MIT License](LICENSE) 开源协议。

---

## 🙏 致谢

### 技术栈

- [Tauri](https://tauri.app) - 跨平台桌面应用框架
- [React](https://react.dev) - UI 构建库
- [Rust](https://www.rust-lang.org) - 系统编程语言
- [ADK-Rust](https://github.com/your-adk-repo) - AI 开发套件

### 灵感来源

- [Raycast](https://raycast.com) - Command Palette 设计灵感
- [Linear](https://linear.app) - 简洁的任务管理理念
- [Things](https://culturedcode.com/things/) - 优雅的交互设计

---

## 📞 联系方式

- **作者**: @wangshuo
- **项目地址**: [GitHub](https://github.com/your-username/Intento)
- **问题反馈**: [Issues](https://github.com/your-username/Intento/issues)
- **功能建议**: [Discussions](https://github.com/your-username/Intento/discussions)

---

<div align="center">

**如果这个项目对你有帮助，请给个 ⭐️ Star！**

Made with ❤️ by [@wangshuo](https://github.com/your-username)

[⬆ 回到顶部](#intento-无忧记)

</div>
