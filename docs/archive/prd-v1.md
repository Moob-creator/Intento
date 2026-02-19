基于你的 PRD 需求以及“**不部署本地模型，利用 OpenAI 能力**”的核心限制，这是最终确定的技术栈总结。

该架构的核心理念是：**前端极简渲染 + Rust 高性能管家 + OpenAI 云端大脑**。

### 1\. 总体架构图

```mermaid
graph TD
    User[用户] --> |截图/文本| UI[前端 (React)]
    UI --> |Base64/指令| Core[后端 (Rust/Tauri)]
    
    subgraph "本地桌面环境"
    UI
    Core
    DB[(SQLite 数据库)]
    end
    
    Core <--> |读写任务/总结| DB
    Core <--> |HTTP 请求 (图片+上下文)| LLM_Framework[rig-core]
    LLM_Framework --> |OpenAI| Cloud[Cloud LLMs]
    LLM_Framework --> |Ollama etc.| Local[Local LLMs]
    
    subgraph "云端智能 (Agent)"
    Cloud --> |GPT-4o| Vision[视觉识别]
    Cloud --> |GPT-4o| Logic[语义理解 & 总结]
    end
```

-----

### 2\. 详细技术栈清单

| 分层 | 技术选型 | 关键作用 & 理由 |
| :--- | :--- | :--- |
| **桌面框架** | **Tauri v2** | **轻量级分发的核心**。相比 Electron，它生成的安装包极小（\<10MB），启动快，内存占用低，完美符合 PRD 中“工具保持轻量”的要求。 |
| **后端语言** | **Rust** | **性能与安全**。负责文件系统操作、数据库交互、以及作为“代理”向 OpenAI 发起请求（避免前端直接暴露 Key 或处理跨域）。 |
| **前端框架** | **React + Vite** | **构建 UI**。配合 TypeScript 保证代码健壮性。Vite 提供极速的开发体验。 |
| **UI 组件库** | **shadcn/ui** | **现代化界面**。基于 Tailwind CSS，代码即组件，无需安装庞大的依赖包，方便定制出 PRD 要求的“简洁、不打扰”的界面。 |
| **数据存储** | **SQLite** | **本地数据主权**。无需安装服务，单文件存储。完美支持 PRD 中的“历史回顾”和“总结归档”查询需求（如按月聚合任务）。 |
| **ORM/驱动** | **sqlx** (Rust) | **类型安全的数据库操作**。支持异步（Async），保证在大量读写（如生成年度总结）时界面不卡顿。 |
| **AI 模型** | **OpenAI / Ollama 等** | **云端大脑**。通过 `rig-core` 框架接入。
**1. 视觉能力**：利用 GPT-4o 等多模态模型看懂截图。
**2. 逻辑能力**：处理任务拆解、时间推断。
**3. 总结能力**：生成日报、周报。 |
| **LLM 集成框架** | **rig-core** (Rust) | **统一的多模型客户端**。在 Rust 端提供对 OpenAI、Anthropic、Ollama 等多种模型的统一接口，方便未来切换或增加模型。 |
| **本地部署** | **Ollama** (通过 rig-core 支持) | **可选本地模型支持**。当用户希望完全离线使用或保护隐私时，可通过 rig-core 集成本地 Ollama。 |

-----

### 3\. 关键流程解决方案

#### 针对“图片/截图处理” (无需本地模型)

  * **方案**：**OpenAI GPT-4o Vision**。
  * **流程**：
    1.  前端获取图片 -\> 转为 Base64 字符串。
    2.  Rust 接收 Base64 -\> 构造 JSON Payload (包含 `image_url` 字段)。
    3.  OpenAI 直接返回识别后的结构化 JSON（如 `{"task": "修复Crash", "due": "周三"}`）。
  * **优势**：无需在用户电脑上安装庞大的 PyTorch 或 Tesseract OCR 库，安装包体积保持最小。

#### 针对“自动总结与档案” (PRD 4.6)

  * **方案**：**SQLite + Cron (Rust)**。
  * **流程**：
    1.  Rust 后端在后台静默运行定时检查。
    2.  从 SQLite 拉取当天的 Completed Tasks。
    3.  发送给 OpenAI 进行自然语言润色。
    4.  生成的总结存回 SQLite 的 `daily_summaries` 表，供前端随时渲染时间轴。
    5.  若计划时间段内应用未运行，则在下次启动或下次定时检查时补跑缺失区间的总结生成任务，保证档案完整（与 PRD 中“宽松补偿”一致）。

#### 针对“应用分发”

  * **方案**：**Tauri Action / Build**。
  * **结果**：直接编译出 `.msi` (Windows) 和 `.dmg` (macOS) 安装包。用户下载即用，无需配置 Python 环境或 Docker。
