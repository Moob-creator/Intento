# Intento 技术调研报告

**日期：** 2026-02-09
**调研目标：** 为 Intento（智能 Todo 桌面应用）选择合适的技术栈

---

## 一、核心技术要求

- **桌面框架：** Rust + Tauri
- **前端框架：** React
- **本地存储：** 轻量级数据库
- **AI 能力：** 调用云端大模型 API（OpenAI/Claude）
- **定时任务：** 支持每日自动总结

---

## 二、技术选型方案

### 2.1 Tauri 桌面框架

**版本信息：**
- **当前稳定版本：** v2.10.2（2026年2月4日发布）
- **GitHub Stars：** 102,000+
- **官网：** https://tauri.app

**核心特性：**
- 跨平台支持：Linux, macOS, Windows, Android, iOS
- 极小应用体积：最小可达 600KB（使用系统原生 WebView）
- 内置打包工具（.exe, .app, .dmg, .deb, .AppImage）
- 内置自动更新器
- 系统托盘图标和原生通知
- GitHub Action CI/CD 支持

**与 React 集成：**
```bash
# 使用官方脚手架创建项目
npm create tauri-app@latest
# 选择 React + TypeScript
```

**Rust 后端与前端通信：**

1. **Commands（类型安全的函数调用）**
   ```rust
   #[tauri::command]
   fn parse_task(input: String) -> Result<Task, String> {
       // 调用 AI API 解析任务
       Ok(task)
   }
   ```

   前端调用：
   ```typescript
   import { invoke } from '@tauri-apps/api/core';

   const task = await invoke('parse_task', {
     input: '明天下午记得发测试包'
   });
   ```

2. **Event System（异步事件广播）**
   ```rust
   // Rust 发送事件
   app.emit_all("task_completed", payload).unwrap();
   ```

   ```typescript
   // 前端监听事件
   import { listen } from '@tauri-apps/api/event';

   listen('task_completed', (event) => {
     console.log('任务完成:', event.payload);
   });
   ```

---

### 2.2 本地存储方案

#### 方案对比

| 数据库 | 版本 | 月下载量 | Stars | 最后更新 | 适用场景 | 推荐度 |
|--------|------|----------|-------|----------|----------|--------|
| **rusqlite** | 0.38.0 | 2,737,017 | N/A | 2025-12-20 | SQLite 绑定 | ⭐⭐⭐⭐⭐ |
| **diesel** | 2.3.6 | 838,400 | N/A | 2026-01-23 | ORM 查询构建器 | ⭐⭐⭐⭐ |
| **sled** | 1.0.0-alpha | 414,183 | 8,900 | 2024-10-11 | 嵌入式数据库 | ⭐⭐⭐ |
| **redb** | 3.1.0 | 239,822 | 4,200 | 2025-09-25 | 纯 Rust 键值存储 | ⭐⭐⭐⭐ |

#### 推荐方案：rusqlite

**理由：**
- Rust 生态最成熟的 SQLite 绑定
- 月下载量最高（270万+），生态最完善
- 100% 文档覆盖率
- 支持丰富的扩展（chrono, serde_json, uuid 等）
- 完整的事务支持
- 适合需要关系型查询的场景

**示例代码：**
```rust
use rusqlite::{Connection, Result};

fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL,
            deadline INTEGER,
            created_at INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}
```

**数据库结构设计：**
```sql
-- 任务表
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL CHECK(status IN ('todo', 'doing', 'done')),
    deadline INTEGER,  -- Unix timestamp
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    context TEXT  -- JSON 格式的上下文信息
);

-- 总结表
CREATE TABLE summaries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL CHECK(type IN ('daily', 'monthly', 'quarterly', 'yearly')),
    date TEXT NOT NULL,  -- YYYY-MM-DD 格式
    content TEXT NOT NULL,  -- AI 生成的总结内容
    tasks_snapshot TEXT,  -- JSON 格式的任务快照
    generated_at INTEGER NOT NULL
);

-- 上下文缓存表（用于 AI 理解）
CREATE TABLE context_cache (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_input TEXT NOT NULL,
    parsed_intent TEXT NOT NULL,
    timestamp INTEGER NOT NULL
);
```

---

### 2.3 Rust 大模型 API 调用库

#### 推荐方案：ADK-Rust（Agent Development Kit）⭐⭐⭐⭐⭐

**项目地址：** https://github.com/zavora-ai/adk-rust
**版本：** 0.3.0（2026年2月8日更新）
**GitHub Stars：** 98
**总下载量：** 820+（adk-model）
**状态：** 生产就绪且积极维护

**为什么选择 ADK-Rust：**
- ✅ **统一 API**：一套代码支持多个 LLM 提供商（OpenAI、Claude、Gemini、DeepSeek 等）
- ✅ **类型安全**：Rust 编译期类型检查，避免运行时错误
- ✅ **模块化架构**：20+ 核心组件，按需引入（adk-core、adk-agent、adk-model 等）
- ✅ **生产级功能**：会话管理、内存系统、认证、遥测
- ✅ **专注智能体开发**：不仅是 API 绑定，提供完整的 Agent 框架
- ✅ **丰富示例**：80+ 可用示例和交互式 CLI
- ✅ **本地推理支持**：支持 Ollama、mistral.rs 本地模型

**支持的 LLM 提供商：**

| 提供商 | 模型示例 | 特性标志 |
|--------|----------|----------|
| **Gemini** | gemini-2.5-flash, gemini-2.5-pro | 默认启用 |
| **OpenAI** | gpt-4o, gpt-4o-mini | `openai` |
| **Anthropic** | claude-sonnet-4 | `anthropic` |
| **DeepSeek** | deepseek-chat, deepseek-reasoner | `deepseek` |
| **Groq** | llama-3.3-70b | `groq` |
| **Ollama** | llama3.2, qwen2.5（本地） | `ollama` |
| **mistral.rs** | Phi-3, Mistral（本地） | `mistral-rs` |

**核心优势对比：**

| 特性 | ADK-Rust | async-openai | claude-sdk |
|------|----------|--------------|------------|
| 多提供商支持 | ✅ 7 个 | ❌ 仅 OpenAI | ❌ 仅 Claude |
| 统一 API | ✅ | ❌ | ❌ |
| Agent 框架 | ✅ | ❌ | ❌ |
| 会话管理 | ✅ | ❌ | ✅ |
| 工具调用 | ✅ | ✅ | ✅ |
| 本地模型 | ✅ | ❌ | ❌ |
| 生产级功能 | ✅ | 部分 | 部分 |

**示例代码：**

**1. OpenAI 调用示例：**
```rust
use adk_model::openai::{OpenAIClient, OpenAIConfig};
use adk_agent::llm::LlmAgentBuilder;
use std::sync::Arc;

async fn parse_task_with_openai(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;

    // 创建 OpenAI 客户端
    let model = OpenAIClient::new(OpenAIConfig::new(api_key, "gpt-4o"))?;

    // 构建 LLM Agent
    let agent = LlmAgentBuilder::new("task_parser")
        .instruction("你是一个任务解析助手，从用户输入中提取任务标题、描述和截止时间。")
        .model(Arc::new(model))
        .build()?;

    // 执行任务
    let response = agent.run(input).await?;
    Ok(response.content)
}
```

**2. Claude 调用示例：**
```rust
use adk_model::anthropic::{AnthropicClient, AnthropicConfig};
use adk_agent::llm::LlmAgentBuilder;
use std::sync::Arc;

async fn parse_task_with_claude(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")?;

    // 创建 Claude 客户端
    let model = AnthropicClient::new(AnthropicConfig::new(
        api_key,
        "claude-sonnet-4"
    ))?;

    // 构建 LLM Agent
    let agent = LlmAgentBuilder::new("task_parser")
        .instruction("你是任务解析助手")
        .model(Arc::new(model))
        .build()?;

    let response = agent.run(input).await?;
    Ok(response.content)
}
```

**3. 统一抽象（可切换提供商）：**
```rust
use adk_model::Model;
use adk_agent::llm::LlmAgentBuilder;
use std::sync::Arc;

// 通用函数，接受任何实现 Model trait 的客户端
async fn parse_task(
    model: Arc<dyn Model>,
    input: &str
) -> Result<String, Box<dyn std::error::Error>> {
    let agent = LlmAgentBuilder::new("task_parser")
        .instruction("你是任务解析助手")
        .model(model)
        .build()?;

    let response = agent.run(input).await?;
    Ok(response.content)
}

// 使用时可以轻松切换
let openai_model = Arc::new(OpenAIClient::new(...)?);
let claude_model = Arc::new(AnthropicClient::new(...)?);

// 相同的代码，不同的提供商
parse_task(openai_model, "明天记得发测试包").await?;
parse_task(claude_model, "明天记得发测试包").await?;
```

**4. 图片识别示例：**
```rust
use adk_model::openai::{OpenAIClient, OpenAIConfig};
use adk_agent::llm::LlmAgentBuilder;

async fn parse_screenshot(image_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let model = OpenAIClient::new(OpenAIConfig::new(
        std::env::var("OPENAI_API_KEY")?,
        "gpt-4o"  // 支持视觉
    ))?;

    let agent = LlmAgentBuilder::new("image_parser")
        .instruction("从截图中提取任务信息，包括标题、描述、截止时间。")
        .model(Arc::new(model))
        .build()?;

    // ADK-Rust 支持多模态输入
    let response = agent.run_with_image(image_path).await?;
    Ok(response.content)
}
```

**Cargo.toml 配置：**
```toml
[dependencies]
# 核心库
adk-core = "0.3"
adk-agent = "0.3"
adk-model = { version = "0.3", features = ["openai", "anthropic"] }

# 如需更多提供商
# adk-model = { version = "0.3", features = ["openai", "anthropic", "deepseek", "ollama"] }

# 异步运行时
tokio = { version = "1", features = ["full"] }
```

**为什么 ADK-Rust 最适合 Intento 项目：**

1. **灵活性**：初期使用 OpenAI，未来可以无缝切换到 Claude 或本地模型
2. **成本优化**：支持多提供商，可以根据价格和性能选择
3. **离线能力**：集成 Ollama 后可以支持完全离线的任务解析
4. **类型安全**：Rust 编译期检查，减少运行时错误
5. **生产就绪**：内置会话管理、错误处理、重试逻辑
6. **社区活跃**：2026年2月仍在积极更新（最新提交 2 天前）

---

#### 备选方案

如果只需要单个提供商，可以考虑：

**async-openai（仅 OpenAI）**
- 版本：0.32.4
- 月下载量：724,076
- 仅支持 OpenAI API

**claude-sdk（仅 Claude）**
- 原生 Rust 实现
- 支持 Prompt Caching
- 仅支持 Anthropic Claude

---

### 2.4 定时任务方案

#### Rust 定时任务库：tokio-cron-scheduler

**推荐库：** tokio-cron-scheduler
**版本：** 0.15.1
**GitHub Stars：** 700
**月下载量：** 214,142
**最后更新：** 2025-10-28

**特性：**
- 基于 tokio 异步运行时
- 支持标准 cron 语法
- 支持英文到 cron 转换
- MIT/Apache 双许可证

**示例代码：**
```rust
use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scheduler = JobScheduler::new().await?;

    // 每天 18:00 生成日报
    let daily_summary = Job::new_async("0 0 18 * * *", |_uuid, _l| {
        Box::pin(async {
            println!("开始生成每日总结...");
            // 1. 从数据库读取今日任务
            // 2. 调用 AI API 生成总结
            // 3. 保存到 summaries 表
            // 4. 发送桌面通知
        })
    })?;

    // 每月 1 日生成月报
    let monthly_summary = Job::new_async("0 0 0 1 * *", |_uuid, _l| {
        Box::pin(async {
            println!("开始生成月度总结...");
        })
    })?;

    scheduler.add(daily_summary).await?;
    scheduler.add(monthly_summary).await?;
    scheduler.start().await?;

    // 保持运行
    tokio::signal::ctrl_c().await?;
    Ok(())
}
```

**常用 Cron 表达式：**
```
0 0 18 * * *      # 每天 18:00
0 0 0 1 * *       # 每月 1 日 00:00
0 0 0 1 */3 *     # 每季度第一天 00:00
0 0 0 1 1 *       # 每年 1 月 1 日 00:00
*/30 * * * * *    # 每 30 秒
```

---

### 2.5 React 前端技术栈

#### React 版本
- **推荐版本：** React 19.2.4（2026年1月26日发布）
- **GitHub Stars：** 234,000+
- 配合 **TypeScript** 使用

#### 状态管理方案

| 方案 | 版本 | Stars | 特点 | 推荐度 | 适用场景 |
|------|------|-------|------|--------|----------|
| **Zustand** | latest | N/A | 轻量、无样板代码 | ⭐⭐⭐⭐⭐ | 中小型应用 |
| **Jotai** | 2.17.1 | 21,000 | 原子化状态 | ⭐⭐⭐⭐⭐ | 分散式状态管理 |
| **Redux Toolkit** | 2.11.2 | 11,200 | 适合大型应用 | ⭐⭐⭐⭐ | 企业级应用 |

**推荐 Zustand（最适合 Todo 应用）：**

```typescript
// store.ts
import { create } from 'zustand';

interface Task {
  id: number;
  title: string;
  status: 'todo' | 'doing' | 'done';
  deadline?: number;
}

interface TaskStore {
  tasks: Task[];
  addTask: (task: Task) => void;
  updateTask: (id: number, updates: Partial<Task>) => void;
  deleteTask: (id: number) => void;
}

const useTaskStore = create<TaskStore>((set) => ({
  tasks: [],
  addTask: (task) => set((state) => ({
    tasks: [...state.tasks, task]
  })),
  updateTask: (id, updates) => set((state) => ({
    tasks: state.tasks.map(t =>
      t.id === id ? { ...t, ...updates } : t
    )
  })),
  deleteTask: (id) => set((state) => ({
    tasks: state.tasks.filter(t => t.id !== id)
  })),
}));

export default useTaskStore;
```

**优势：**
- Hooks 优先，无需 Provider 包裹
- 极简 API，几乎零样板代码
- 高性能，选择性更新
- 解决 React 常见问题（zombie child、context loss 等）

#### UI 组件库

| 库 | 版本 | Stars | 最后更新 | 特点 | 推荐度 |
|---|------|-------|----------|------|--------|
| **shadcn/ui** | 3.8.4 | 106,000 | 2026-02-06 | 可复制可定制 | ⭐⭐⭐⭐⭐ |
| **Ant Design** | 6.2.3 | 97,500 | 2026-02-02 | 企业级组件库 | ⭐⭐⭐⭐ |
| **Material-UI** | 7.3.7 | 97,800 | 2026-01-08 | Material Design | ⭐⭐⭐⭐ |

**推荐 shadcn/ui（最适合桌面应用）：**

**特点：**
- 组件可以复制到项目中，完全可控
- 基于 Radix UI（无障碍性好）+ Tailwind CSS（样式灵活）
- 不像传统组件库那样臃肿
- 非常适合定制化需求

**安装使用：**
```bash
npx shadcn@latest init
npx shadcn@latest add button card checkbox dialog input
```

**示例组件：**
```tsx
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

function TaskCard({ task }: { task: Task }) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>{task.title}</CardTitle>
      </CardHeader>
      <CardContent>
        <p>{task.description}</p>
        <Button onClick={() => completeTask(task.id)}>
          完成
        </Button>
      </CardContent>
    </Card>
  );
}
```

#### 构建工具
- **推荐：** Vite 7.3.1（2026年1月7日）
- Tauri 官方模板默认使用 Vite
- 极快的冷启动、HMR（热模块替换）

---

## 三、推荐技术栈总结

### 前端技术栈
```json
{
  "framework": "React 19.2.4 + TypeScript",
  "build-tool": "Vite 7.3.1",
  "state-management": "Zustand (推荐) / Jotai",
  "ui-library": "shadcn/ui 3.8.4 (推荐) / Ant Design 6.2.3",
  "styling": "Tailwind CSS (配合 shadcn/ui)"
}
```

### 后端技术栈（Rust）
```toml
[dependencies]
# 桌面框架
tauri = "2.10"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 异步运行时
tokio = { version = "1", features = ["full"] }

# 数据库
rusqlite = { version = "0.38", features = ["bundled", "serde_json"] }

# AI 客户端 - ADK-Rust（推荐）
adk-core = "0.3"
adk-agent = "0.3"
adk-model = { version = "0.3", features = ["openai", "anthropic"] }

# 定时任务
tokio-cron-scheduler = "0.15"

# 日期时间
chrono = "0.4"

# 错误处理
anyhow = "1.0"
thiserror = "2.0"
```

---

## 四、项目初始化步骤

### 4.1 创建项目

```bash
# 使用 Tauri 官方脚手架
npm create tauri-app@latest

# 选择以下选项：
# - Project name: intento
# - Choose your package manager: npm / yarn / pnpm
# - Choose your UI template: React
# - Add TypeScript? Yes
```

### 4.2 安装前端依赖

```bash
cd intento

# 安装状态管理
npm install zustand

# 安装 shadcn/ui
npx shadcn@latest init
# 选择：
# - Style: Default
# - Base color: Slate
# - CSS variables: Yes

# 添加常用组件
npx shadcn@latest add button card checkbox dialog input textarea
npx shadcn@latest add dropdown-menu select calendar
```

### 4.3 配置 Rust 依赖

编辑 `src-tauri/Cargo.toml`：

```toml
[dependencies]
tauri = { version = "2.10", features = [] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
rusqlite = { version = "0.38", features = ["bundled", "serde_json", "chrono"] }

# ADK-Rust AI 客户端（统一多提供商）
adk-core = "0.3"
adk-agent = "0.3"
adk-model = { version = "0.3", features = ["openai", "anthropic"] }

tokio-cron-scheduler = "0.15"
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
thiserror = "2.0"
```

### 4.4 项目结构建议

```
intento/
├── src/                    # 前端代码
│   ├── components/         # React 组件
│   │   ├── ui/            # shadcn/ui 组件
│   │   ├── TaskList.tsx
│   │   ├── TaskInput.tsx
│   │   └── SummaryView.tsx
│   ├── stores/            # Zustand stores
│   │   └── taskStore.ts
│   ├── lib/               # 工具函数
│   │   └── tauri.ts       # Tauri API 封装
│   ├── App.tsx
│   └── main.tsx
├── src-tauri/             # Rust 后端代码
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/      # Tauri commands
│   │   │   ├── mod.rs
│   │   │   ├── task.rs
│   │   │   └── ai.rs
│   │   ├── db/            # 数据库模块
│   │   │   ├── mod.rs
│   │   │   ├── models.rs
│   │   │   └── schema.rs
│   │   ├── ai/            # AI 模块
│   │   │   ├── mod.rs
│   │   │   ├── openai.rs
│   │   │   └── claude.rs
│   │   └── scheduler/     # 定时任务模块
│   │       ├── mod.rs
│   │       └── jobs.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
└── package.json
```

---

## 五、开发建议

### 5.1 开发流程

1. **本地开发模式：**
   ```bash
   npm run tauri dev
   ```

2. **构建生产版本：**
   ```bash
   npm run tauri build
   ```

### 5.2 调试建议

- **前端调试：** 使用浏览器 DevTools（Tauri 自动打开）
- **Rust 调试：**
  ```rust
  println!("Debug info: {:?}", data);  // 输出到终端
  ```

### 5.3 性能优化

- 使用 Tauri 的 `invoke` 时避免频繁调用，批量处理数据
- SQLite 使用事务提升批量插入性能
- AI API 调用添加缓存机制，避免重复调用

### 5.4 安全建议

- **API Key 管理：** 不要硬编码在代码中，使用环境变量或配置文件
- **数据库加密：** 考虑使用 SQLCipher（rusqlite 支持）
- **输入验证：** 所有用户输入都要在 Rust 端验证

---

## 六、CI/CD 建议

Tauri 官方提供 GitHub Actions 模板，支持自动构建多平台安装包：

```yaml
# .github/workflows/build.yml
name: Build
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    strategy:
      matrix:
        platform: [macos-latest, ubuntu-latest, windows-latest]
    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: npm install
      - run: npm run tauri build
```

---

## 七、估算成本

### AI API 成本（以 OpenAI 为例）

| 操作 | 模型 | Token 数 | 成本 |
|------|------|----------|------|
| 解析任务 | GPT-4o mini | 500 | $0.0001 |
| 图片识别 | GPT-4o | 1000 | $0.005 |
| 每日总结 | GPT-4o | 2000 | $0.01 |
| 月度总结 | GPT-4o | 5000 | $0.025 |

**月度成本估算（个人用户）：**
- 每日 10 次任务解析：$0.001 × 10 × 30 = $0.30
- 每日 1 次图片识别：$0.005 × 30 = $0.15
- 每日 1 次总结：$0.01 × 30 = $0.30
- 每月 1 次月度总结：$0.025
- **总计：约 $0.80/月**

---

## 八、总结

这套技术栈具有以下优势：

1. **现代化：** 所有技术都是 2026 年初最新稳定版
2. **主流：** 都是社区最活跃、下载量最大的库
3. **轻量级：** 符合产品定位，不过度设计
4. **类型安全：** Rust + TypeScript 双重保障
5. **可维护性：** 清晰的模块划分，易于扩展
6. **性能优秀：** Tauri 体积小、启动快，SQLite 读写高效

可以立即开始开发 MVP 版本！
