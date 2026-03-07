# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 🚨 重要约束

### 文档创建规则
**IMPORTANT**: 除非用户明确要求"输出到文档"、"写入文档"或"创建文档"，否则：
- ❌ 不要主动创建新的 Markdown 文档
- ❌ 不要生成报告、总结、指南等文档文件
- ✅ 直接在对话中提供答案和信息
- ✅ 只在用户明确请求时才创建文档

**例外情况**（可以创建文档）：
1. 用户明确说"写到文档里"、"保存到文件"
2. 创建测试文件（`.test.ts`, `.test.tsx`, `.spec.ts`）
3. 创建配置文件（`.config.ts`, `.yaml`, `.json`）
4. 创建代码文件（`.ts`, `.tsx`, `.rs` 等）

**原因**: 项目中文档已经很多，避免文档泛滥。

## Project Overview

**Intento (无忧记)** is an intelligent todo list desktop application that uses AI to parse natural language and image inputs into structured tasks, and automatically generates daily/weekly/monthly summaries.

**Tech Stack:** Rust + Tauri 2.0 + React 19 + TypeScript + Zustand + ADK-Rust (AI SDK)

## Development Commands

### Frontend (React + Vite)
```bash
npm run dev              # Start Vite dev server (frontend only)
npm run build            # Build frontend for production
npm run preview          # Preview production build
```

### Tauri Application
```bash
npm run tauri:dev        # Start full Tauri app in dev mode (recommended)
npm run tauri:build      # Build production app for current platform
npm run build:all        # Build both frontend and Tauri app
npm run build:mac        # Build universal macOS binary
npm run build:win        # Build Windows x86_64 binary
```

### Rust Backend Testing
```bash
cd src-tauri
cargo test               # Run all Rust unit tests
cargo test --lib         # Run library tests only
cargo build              # Compile Rust backend
cargo clippy             # Lint Rust code
```

## Architecture Overview

### Tauri Backend (Rust) - `src-tauri/src/`

**Core Modules:**
- `db/` - SQLite database layer with migration support
  - `models.rs` - Data models (Task, Summary, ContextCache, etc.)
  - Connection is thread-safe via `Arc<Mutex<Connection>>`
  - Uses `PRAGMA user_version` for migration tracking
  - **Bundle ID separation**:
    - Debug mode: `com.intento.app.debug` → `~/Library/Application Support/com.intento.app.debug/intento.db`
    - Release mode: `com.intento.app` → `~/Library/Application Support/com.intento.app/intento.db`
    - Complete isolation: different Bundle IDs = truly separate apps
    - Can run both versions simultaneously without conflicts

- `ai/` - AI integration using ADK-Rust
  - `client.rs` - OpenAI/Claude API client wrapper
  - `prompts.rs` - System prompts for task parsing
  - `task_operations.rs` - Tool-based operation extraction (create/update/delete/complete)
  - Supports text and image (vision) input parsing

- `commands/` - Tauri command handlers exposed to frontend
  - `task.rs` - CRUD operations: `create_task`, `list_tasks`, `update_task`, `delete_task`
  - `ai.rs` - AI parsing: `parse_text_input`, `parse_image_input`, `parse_image_for_operations`
  - `summary.rs` - Summary generation and retrieval
  - `notification.rs` - Desktop notification triggers
  - `settings.rs` - User settings management

- `scheduler/` - Background cron jobs using tokio-cron-scheduler
  - Deadline reminders (checks hourly)
  - Automatic daily/weekly/monthly summary generation

- `summary/` - Summary generation system
  - `generator.rs` - AI-powered summary content generation
  - `period.rs` - Time period calculations (daily/weekly/monthly/etc.)
  - `scheduler_jobs.rs` - Scheduled summary generation logic

- `window.rs` - macOS-specific window customization (traffic light button positioning)

**State Management in main.rs:**
```rust
app.manage(database.clone());           // Shared Database handle
app.manage(AiClientState::new());       // AI client lazy initialization
app.manage(TaskScheduler);              // Background scheduler
```

**Database Schema (SQLite):**
- `tasks` table: id, title, description, status (todo/doing/done), priority, deadline, tags (JSON), attachments (JSON), reminder_time, is_deleted (soft delete)
- `summaries` table: id, summary_type (daily/weekly/monthly/etc.), period_start/end, content, statistics (JSON), task_ids (JSON), tag/tag_filter
- `context_cache` table: AI parsing cache with TTL support
- `settings` table: Auto-summary and notification settings
- Migrations in `src-tauri/migrations/v*.sql`

### Frontend (React + TypeScript) - `src/`

**State Management (Zustand):**
- `store/taskStore.ts` - Central task state with Tauri command integration
  - Actions: `loadTasks`, `createTask`, `updateTask`, `deleteTask`, `selectTask`
  - Uses `@tauri-apps/api/core` `invoke()` to call Rust commands

**Component Architecture:**
- `App.tsx` - Main layout with sidebar, top bar, and content panels
- `components/TaskList.tsx` - Task list with filtering
- `components/TaskDetailPanel.tsx` - Selected task detail view
- `components/TaskConfirmDialog.tsx` - AI parsed task confirmation (single operation)
- `components/TaskOperationsConfirmDialog.tsx` - Multi-operation confirmation from image
- `components/CommandPalette.tsx` - Keyboard-driven command interface (Cmd+K)
- `components/SummaryPanel.tsx` - View/generate summaries
- `components/SettingsPanel.tsx` - User settings UI
- `components/CalendarView.tsx` - Calendar-based task view
- `components/AdvancedFilterPanel.tsx` - Multi-criteria task filtering

**AI Input Flow:**
1. User types text or pastes image → Frontend captures input
2. Call `parse_text_input` or `parse_image_for_operations` command
3. Backend uses ADK-Rust to call OpenAI/Claude API
4. Returns `ParsedTask` or `ImageParseResult` with operations
5. Show confirmation dialog → User approves → Create/update tasks

**Keyboard Shortcuts:**
- Defined in `hooks/useKeyboardShortcuts.ts`
- Cmd+K: Command palette
- Cmd+N: New task input
- ESC: Close dialogs

## Environment Setup

### Required Environment Variables
Create `.env` file in project root:
```bash
OPENAI_API_KEY=sk-xxx           # For OpenAI (gpt-4o, gpt-4o-mini)
ANTHROPIC_API_KEY=sk-ant-xxx    # For Claude (claude-3-5-sonnet)
```

The app loads `.env` automatically in debug mode via `dotenv::dotenv()`.

### AI Provider Configuration
The app supports switching between:
- OpenAI (gpt-4o, gpt-4o-mini)
- Anthropic Claude (claude-3-5-sonnet-20241022)
- Moonshot Kimi (moonshot-v1-8k)

Provider is selected via settings UI and stored in database.

## Key Design Patterns

### Rust-Frontend Communication
All commands use Result<T, String> and serialize errors:
```rust
#[tauri::command]
pub async fn create_task(
    db: State<'_, Database>,
    title: String,
    // ...
) -> Result<i64, String> {
    db.create_task(/* ... */)
        .map_err(|e| e.to_string())
}
```

Frontend handles with try-catch on invoke():
```typescript
try {
  const taskId = await invoke<number>('create_task', { title, ... });
} catch (error) {
  console.error('Failed to create task:', error);
}
```

### AI Tool Use Pattern
Image parsing uses OpenAI function calling to extract operations:
```rust
// Define tools for AI
let tools = vec![
    create_task_tool(),
    update_task_tool(),
    delete_task_tool(),
    complete_task_tool(),
];

// AI responds with tool_calls
let operations = parse_tool_calls_from_response(response);
```

### Soft Delete
All tables use `is_deleted` flag instead of hard deletion for data safety.

### TypeScript Type Alignment
Frontend types in `src/types/task.ts` must match Rust models exactly:
```typescript
// Must match src-tauri/src/db/models.rs Task struct
interface Task {
  id?: number;
  title: string;
  status: 'todo' | 'doing' | 'done';  // Matches TaskStatus enum
  // ...
}
```

## Development Workflow

### Adding a New Tauri Command
1. Define command function in `src-tauri/src/commands/<module>.rs`
2. Export in `src-tauri/src/commands/mod.rs`
3. Register in `main.rs` `invoke_handler![]` macro
4. Call from frontend via `invoke('<command_name>', { params })`
5. Add TypeScript types in `src/types/`

### Database Migrations
1. Create `src-tauri/migrations/vN_description.sql`
2. Add migration check in `db/mod.rs` `run_migrations()`
3. Increment version with `PRAGMA user_version = N;`

### Testing AI Integration
Use `commands::ai::ai_health_check()` to verify API connectivity without consuming credits.

## Platform-Specific Notes

### macOS
- Traffic light buttons are repositioned via `window.rs` using objc2 bindings
- Requires Xcode Command Line Tools for building

### Windows
- Use `--target x86_64-pc-windows-msvc` for builds
- Notification permissions handled by Tauri automatically

## 项目目录规范（2026-02-25 整理）

### 根目录只保留
- `CLAUDE.md`、`readme.md` 以及项目配置文件（package.json / vite.config.ts / tailwind.config.js / tsconfig.json / .env 等）
- **禁止在根目录放脚本和 MD 文档**

### Shell 脚本 → `tools/scripts/`
| 文件 | 用途 |
|------|------|
| `build_mac.sh` | 打 macOS DMG 安装包（universal binary），完成后打印 DMG 路径 |
| `open_release.sh` | 打开上次构建的 release bundle 目录 |
| `run_intent_test.sh` | 意图识别 AI 测试 |
| `test_db_separation.sh` | 验证 debug/release 数据库隔离 |
| `test.sh` | 通用测试 runner |

> 新增脚本一律放 `tools/scripts/`，不要放根目录。

---

## Documentation Organization

### Documentation Structure

All project documentation is organized in the `docs/` directory with the following structure:

```
docs/
├── README.md                        # Documentation index and navigation
├── CHANGELOG.md                     # Version history and updates
├── ROADMAP.md                       # Product roadmap and future plans
│
├── specs/                           # Product and technical specifications
│   ├── prd-v3.md                   # Product requirements document
│   ├── development-plan.md         # Development phases and tasks
│   ├── database-schema.md          # Database design
│   ├── tech-stack.md               # Technology stack decisions
│   └── frontend-reference.md       # Frontend architecture
│
├── user-guide/                      # User documentation
│   ├── README.md                   # Complete user guide
│   └── features/                   # Feature-specific guides
│       ├── image-parsing.md
│       └── notifications.md
│
├── progress/                        # ⭐ Current development tracking
│   ├── PROGRESS_REPORT.md          # Detailed progress analysis
│   └── NEXT_TASKS.md               # Prioritized task list
│
└── archive/                         # Historical documentation
    ├── TODO.md                     # Old task list (archived)
    ├── progress/                   # Old progress reports
    ├── technical/                  # Old technical docs
    └── phase5/                     # Phase-specific archives
```

### Documentation Guidelines

**When to Create/Update Documentation:**

1. **Progress Reports** → `docs/progress/`
   - Create when: Major milestone completed, quarterly review
   - Update: `PROGRESS_REPORT.md` with current status
   - Include: PRD coverage, code stats, next steps

2. **Task Planning** → `docs/progress/`
   - Create when: Planning next development phase
   - Update: `NEXT_TASKS.md` with prioritized tasks
   - Include: P0/P1/P2 tasks, time estimates, acceptance criteria

3. **Technical Specs** → `docs/specs/`
   - Create when: Designing new architecture/feature
   - Files: `database-schema.md`, `tech-stack.md`, etc.
   - Keep: Only active/current specifications

4. **User Guides** → `docs/user-guide/`
   - Create when: New user-facing feature released
   - Update: After any UI/UX changes
   - Keep: Always current with latest version

5. **Archive** → `docs/archive/`
   - Move here: Completed phase docs, old progress reports
   - Never delete: Keep for historical reference
   - Organize by: Phase or category

**Documentation Best Practices:**

- ✅ Use clear, concise language
- ✅ Include code examples and screenshots
- ✅ Keep documents focused (one topic per file)
- ✅ Update dates and version numbers
- ✅ Cross-reference related documents
- ❌ Don't create duplicate documentation
- ❌ Don't mix current and historical docs
- ❌ Don't leave TODO sections indefinitely

**File Naming Convention:**
- Progress: `PROGRESS_REPORT.md`, `NEXT_TASKS.md`
- Specs: `lowercase-with-dashes.md`
- User docs: `lowercase-with-dashes.md`
- Archives: Keep original names with context

## Project Status (2026-02-19)

**Completed Phases:**
- Phase 0: Project initialization ✅
- Phase 1: Database and backend infrastructure ✅
- Phase 2: Basic task management CRUD ✅
- Phase 3: AI text/image parsing ✅
- Phase 4: Notification system ✅
- Phase 5: Summary generation (daily/weekly/monthly) ✅

**Current Status:** MVP (v0.1.0) Complete - 100%

**Next Steps (P0 - Must Complete):**
1. Context caching mechanism (PRD 4.2) - 1-2 days
2. Quarterly/yearly summaries (PRD 4.6.3/4.6.4) - 1-2 days

See `docs/progress/NEXT_TASKS.md` for detailed task breakdown.

**Current Development:**
- Completing PRD v3 remaining features
- Preparing for v0.2.0 release
