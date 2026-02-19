# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

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

## Project Status (2026-02-19)

**Completed Phases:**
- Phase 1: Database and backend infrastructure ✅
- Phase 2: Basic task management CRUD ✅
- Phase 3: AI text/image parsing ✅
- Phase 4: Notification system ✅
- Phase 5: Summary generation (daily/weekly/monthly) ✅

**Current Development:**
- UI refinements and user experience improvements
- Advanced filtering and search capabilities
- Notification settings customization
