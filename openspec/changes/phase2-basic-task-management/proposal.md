## Why

Intento needs a foundational task management system to allow users to organize their work. Phase 1 established the core infrastructure (database, IPC communication, state management patterns), and now Phase 2 builds the first user-facing feature on top of that foundation. This enables users to create, track, and manage tasks with basic CRUD operations and status tracking.

## What Changes

- Add Tauri commands for task CRUD operations (`create_task`, `update_task`, `delete_task`, `get_tasks`)
- Implement Zustand store for client-side task state management with automatic backend synchronization
- Create React UI components for task list display with status filtering (Todo/Doing/Done)
- Create React UI components for task creation and editing via dialog forms
- Add task data model in Rust backend with SQLite persistence
- Integrate task management with existing database infrastructure from Phase 1

## Capabilities

### New Capabilities
- `task-crud`: Backend Tauri commands for creating, reading, updating, and deleting tasks with SQLite persistence
- `task-state-management`: Frontend Zustand store managing task state and synchronization with backend
- `task-list-ui`: React component displaying filterable task list with status toggles and delete actions
- `task-form-ui`: React dialog-based form component for creating and editing tasks with validation

### Modified Capabilities
<!-- No existing capabilities being modified - this is all new functionality -->

## Impact

**Backend (Rust/Tauri):**
- New file: `src-tauri/src/commands/task.rs` with 4 command functions
- Modified: `src-tauri/src/main.rs` to register task commands
- New file: `src-tauri/src/db/models.rs` (or extend existing) for Task model
- Modified: `src-tauri/src/db/mod.rs` to add task database operations

**Frontend (React/TypeScript):**
- New file: `src/stores/taskStore.ts` with Zustand store
- New file: `src/components/TaskList.tsx` for task list display
- New file: `src/components/TaskForm.tsx` for task creation/editing
- Integration with existing shadcn/ui components (Dialog, Button, Input, etc.)

**Database:**
- New table: `tasks` with columns for id, title, description, status, deadline, created_at, updated_at

**Dependencies:**
- No new external dependencies required (leverages existing Zustand, shadcn/ui, Tauri APIs)
