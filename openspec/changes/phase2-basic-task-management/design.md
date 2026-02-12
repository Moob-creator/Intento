## Context

Phase 1 is complete with:
- Backend: Rust/Tauri infrastructure with SQLite database, complete Task CRUD operations in `db/mod.rs`, and Tauri commands in `commands/task.rs` already registered in `main.rs`
- Frontend: Basic React app in `App.tsx` with hardcoded task management logic (no Zustand, no shadcn/ui components)
- Database: Tasks table with full schema (status, priority, tags, attachments, etc.) already migrated

**Current State:**
The existing `App.tsx` demonstrates working end-to-end functionality but lacks:
1. Proper state management (uses local component state)
2. Professional UI components (inline styles, no shadcn/ui)
3. Form validation and error handling
4. Modular component architecture

**Constraints:**
- Must preserve existing Tauri command signatures in `commands/task.rs` (they're already working)
- shadcn/ui must be properly installed and configured
- Zustand store must be the single source of truth for task state

## Goals / Non-Goals

**Goals:**
- Refactor `App.tsx` into modular components using shadcn/ui for professional UI
- Implement Zustand store to replace local state management
- Create reusable `TaskList` component with status filtering
- Create reusable `TaskForm` component with validation for create/edit operations
- Maintain full backward compatibility with existing Tauri commands

**Non-Goals:**
- Modifying the Rust backend or database schema (Phase 1 is complete and working)
- Advanced features like task search, sorting, or bulk operations
- Real-time synchronization or offline support
- User authentication or multi-user support

## Decisions

### 1. State Management: Zustand Over Redux
**Decision:** Use Zustand for global state management.

**Rationale:**
- Simpler API than Redux (no actions/reducers boilerplate)
- Built-in TypeScript support
- Better performance for small to medium apps
- Easier to test and debug

**Alternatives Considered:**
- Redux: Too heavy for current requirements
- Context API: No built-in optimizations, causes unnecessary re-renders
- Jotai/Recoil: Less mature ecosystem

### 2. Component Architecture: Single-File Components
**Decision:** Create separate `TaskList.tsx` and `TaskForm.tsx` components, but keep state logic in Zustand store.

**Rationale:**
- Clear separation of concerns (presentation vs logic)
- Components are purely presentational and easily testable
- Store handles all business logic and backend communication
- Easier to maintain and refactor

**Structure:**
```
src/
├── stores/
│   └── taskStore.ts          # Zustand store with all task logic
├── components/
│   ├── TaskList.tsx           # Display and filter tasks
│   └── TaskForm.tsx           # Create/edit task form
└── App.tsx                    # Main layout and routing
```

### 3. UI Framework: shadcn/ui Components
**Decision:** Use shadcn/ui for all UI components (Dialog, Button, Input, Select, etc.).

**Rationale:**
- Copy-paste components (no runtime dependency)
- Built on Radix UI (accessibility by default)
- Fully customizable with Tailwind CSS
- Follows the warm, soft design aesthetic from project requirements

**Key Components to Use:**
- Dialog for task form modal
- Button for actions
- Input/Textarea for form fields
- Select for status/priority dropdowns
- Card for task list items

### 4. Form Handling: React Hook Form + Zod
**Decision:** Use React Hook Form for form state and Zod for validation.

**Rationale:**
- Type-safe validation schemas
- Automatic TypeScript inference
- Minimal re-renders
- Easy integration with shadcn/ui form components

**Validation Rules:**
- Title: Required, 1-200 characters
- Description: Optional, max 1000 characters
- Status: One of "todo" | "doing" | "done"
- Priority: One of "low" | "medium" | "high"
- Deadline: Optional, must be future timestamp

### 5. Error Handling: Toast Notifications
**Decision:** Use shadcn/ui Toast component for error/success feedback.

**Rationale:**
- Non-intrusive user feedback
- Consistent with modern UX patterns
- Better than alert() or status text

**Error Scenarios:**
- Backend command failures → Toast error with message
- Validation errors → Inline form errors
- Network/IPC errors → Toast error with retry option

## Risks / Trade-offs

**[Risk]** shadcn/ui installation might conflict with existing CSS
→ **Mitigation:** Follow official installation guide, test on clean branch first

**[Risk]** Zustand store might grow complex with more features
→ **Mitigation:** Keep store focused on task management only, create separate stores for future features

**[Risk]** Form validation might not match backend validation
→ **Mitigation:** Use same validation rules as Rust backend (max lengths, required fields)

**[Risk]** Breaking changes to existing test UI
→ **Mitigation:** This is expected and acceptable - Phase 2 is replacing the test UI with production-ready components

**[Trade-off]** More files and abstractions vs simpler single-file approach
→ **Accepted:** Better maintainability and scalability outweigh initial complexity

**[Trade-off]** Additional dependencies (Zustand, React Hook Form, Zod)
→ **Accepted:** Industry-standard libraries with active maintenance and small bundle size

## Migration Plan

### Step 1: Setup (No Breaking Changes)
1. Install dependencies: `zustand`, `react-hook-form`, `zod`, shadcn/ui
2. Initialize shadcn/ui with `npx shadcn@latest init`
3. Add required shadcn/ui components: `Button`, `Dialog`, `Input`, `Select`, `Card`, `Toast`

### Step 2: Create Store (Additive)
1. Create `src/stores/taskStore.ts` with Zustand store
2. Implement actions: `fetchTasks`, `addTask`, `updateTask`, `deleteTask`
3. Add loading/error states

### Step 3: Create Components (Additive)
1. Create `src/components/TaskList.tsx` (status filtering, task cards)
2. Create `src/components/TaskForm.tsx` (Dialog-based form with validation)
3. Test components independently

### Step 4: Refactor App.tsx (Breaking Change)
1. Replace local state with Zustand store
2. Replace inline forms with `TaskForm` component
3. Replace table with `TaskList` component
4. Remove old inline styles

### Rollback Strategy
Each step is git-committed separately. If issues arise:
- Step 1-3: Simply don't import new code
- Step 4: Revert single commit to restore old UI

### Testing Checklist
- [ ] Create task via form
- [ ] Update task status
- [ ] Delete task
- [ ] Filter tasks by status
- [ ] Form validation (empty title, max lengths)
- [ ] Error handling (backend failures)
- [ ] Loading states display correctly

## Open Questions

None - Phase 1 provides all backend functionality needed. Frontend architecture decisions are finalized.
