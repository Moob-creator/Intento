# Task Management UI - Component Architecture

## Component Hierarchy

```
App.tsx (Root)
├── Sidebar
│   ├── User Profile Section
│   ├── Navigation Links
│   │   ├── Home
│   │   ├── Tasks (active)
│   │   ├── Summary
│   │   └── Settings
│   ├── New Task Button
│   └── Help & Support Link
│
└── Main Content Area
    ├── Header
    │   ├── Title ("Tasks")
    │   └── Action Buttons
    │       ├── Notifications Icon
    │       └── Messages Icon
    │
    └── Content (Split Layout)
        ├── Task List Section (Left)
        │   ├── Search & Filter Bar
        │   │   ├── TaskSearchBar
        │   │   └── StatusFilter
        │   │
        │   └── TaskList
        │       └── TaskCard (multiple)
        │           ├── Title
        │           ├── Status Badge
        │           ├── Description
        │           └── Metadata (deadline, icons)
        │
        └── Task Detail Panel (Right)
            ├── Form Fields
            │   ├── Title Input
            │   ├── Description Textarea
            │   ├── Status Select
            │   ├── Priority Select
            │   └── Due Date Picker
            │
            └── Action Buttons
                ├── Delete Button
                ├── Cancel Button
                └── Save Button
```

## Data Flow

```
┌─────────────────────────────────────────────────┐
│              Zustand Store                      │
│            (taskStore.ts)                       │
│                                                 │
│  State:                                         │
│  - tasks: Task[]                                │
│  - selectedTask: Task | null                    │
│  - isLoading: boolean                           │
│  - error: string | null                         │
│                                                 │
│  Actions:                                       │
│  - loadTasks()                                  │
│  - createTask()                                 │
│  - updateTask()                                 │
│  - deleteTask()                                 │
│  - selectTask()                                 │
│  - clearError()                                 │
└────────────┬────────────────────────────────────┘
             │
             │ (subscribes)
             ↓
┌─────────────────────────────────────────────────┐
│             App.tsx                             │
│                                                 │
│  Local State:                                   │
│  - searchQuery: string                          │
│  - statusFilter: TaskStatus | 'all'             │
│                                                 │
│  Computed:                                      │
│  - filteredTasks (useMemo)                      │
│                                                 │
│  Handlers:                                      │
│  - handleNewTask()                              │
│  - handleSaveTask()                             │
│  - handleDeleteTask()                           │
│  - handleCancelEdit()                           │
└────────────┬────────────────────────────────────┘
             │
             │ (passes props)
             ↓
┌─────────────────────────────────────────────────┐
│         Child Components                        │
│                                                 │
│  TaskSearchBar                                  │
│  - value, onChange                              │
│                                                 │
│  StatusFilter                                   │
│  - activeStatus, onStatusChange                 │
│                                                 │
│  TaskList                                       │
│  - tasks, selectedTaskId, onTaskClick           │
│  │                                              │
│  └─→ TaskCard                                   │
│      - task, isSelected, onClick                │
│                                                 │
│  TaskDetailPanel                                │
│  - task, onSave, onDelete, onCancel             │
└─────────────────────────────────────────────────┘
```

## Tauri Backend Integration

```
Frontend (React)          Tauri IPC           Backend (Rust)
─────────────────────────────────────────────────────────────

[taskStore.ts]
    │
    │ loadTasks()
    ├──────────────> invoke('list_tasks')
    │                    │
    │                    └──────────> [task.rs]
    │                                list_tasks()
    │                                    │
    │                                    └──> [Database]
    │                                         query tasks
    │                                             │
    │                <──────────────────────────┘
    │ tasks: Task[]
    │
    │ createTask()
    ├──────────────> invoke('create_task')
    │                    │
    │                    └──────────> [task.rs]
    │                                create_task()
    │                                    │
    │                                    └──> [Database]
    │                                         insert task
    │                                             │
    │                <──────────────────────────┘
    │ taskId: number
    │
    │ updateTask()
    ├──────────────> invoke('update_task')
    │                    │
    │                    └──────────> [task.rs]
    │                                update_task()
    │                                    │
    │                                    └──> [Database]
    │                                         update task
    │                                             │
    │                <──────────────────────────┘
    │ success
    │
    │ deleteTask()
    ├──────────────> invoke('delete_task')
    │                    │
    │                    └──────────> [task.rs]
    │                                delete_task()
    │                                    │
    │                                    └──> [Database]
    │                                         soft delete
    │                                             │
    │                <──────────────────────────┘
    │ success
    └───
```

## User Interaction Flow

### Creating a New Task

```
1. User clicks "New Task" button in sidebar
   ↓
2. App.tsx: handleNewTask()
   - Creates temporary empty Task object
   - Calls selectTask(newTask)
   ↓
3. Store updates selectedTask
   ↓
4. TaskDetailPanel renders with empty form
   ↓
5. User fills in title, description, etc.
   ↓
6. User clicks "Save"
   ↓
7. App.tsx: handleSaveTask()
   - Calls createTask() from store
   ↓
8. Store: createTask()
   - Invokes Tauri backend
   - Waits for task ID
   - Reloads task list
   - Selects newly created task
   ↓
9. UI updates with new task in list
   Detail panel shows created task
```

### Editing an Existing Task

```
1. User clicks on a task card
   ↓
2. TaskCard: onClick() triggered
   ↓
3. TaskList passes event up to App
   ↓
4. App.tsx: selectTask(task)
   ↓
5. Store updates selectedTask
   ↓
6. TaskDetailPanel renders with task data
   - Form fields populated with current values
   ↓
7. User modifies fields (e.g., status, description)
   ↓
8. User clicks "Save"
   ↓
9. App.tsx: handleSaveTask()
   - Detects task.id exists
   - Calls updateTask(id, updates)
   ↓
10. Store: updateTask()
    - Invokes Tauri backend
    - Reloads task list
    - Updates selectedTask
    ↓
11. UI updates with modified task
    Task card reflects changes
```

### Filtering and Searching

```
1. User types in search bar OR clicks status filter
   ↓
2. Local state updates:
   - searchQuery changed
   - OR statusFilter changed
   ↓
3. useMemo recalculates filteredTasks
   - Applies status filter first
   - Then applies search filter
   ↓
4. TaskList receives new filtered array
   ↓
5. TaskCard components re-render
   - Only matching tasks displayed
   ↓
6. Empty state shown if no matches
```

## State Management Strategy

### Why Zustand?

1. **Simple API**: Less boilerplate than Redux
2. **Performance**: Hook-based, efficient re-renders
3. **TypeScript**: Excellent type inference
4. **Async handling**: Easy to work with promises
5. **No context wrapping**: Direct store access

### Store Organization

- **State**: Minimal, normalized data
- **Actions**: Clear, single-purpose functions
- **Side effects**: Handled within actions
- **Error handling**: Centralized in store
- **Loading states**: Track async operations

## Styling Approach

### Tailwind Configuration

Custom theme extends Tailwind with warm colors:
- Primary palette (coral, peach, terracotta)
- Background colors (soft whites, cream)
- Neutral colors (warm grays, beige)
- Custom shadows (soft, warm)
- Rounded corners (8px, 12px, 16px)

### CSS Organization

1. **Global styles** (App.css): Base resets, scrollbar, animations
2. **Tailwind classes**: Component-level styling
3. **Inline styles**: Dynamic values only

### Design Tokens

Consistent use of:
- `text-sm`, `text-base`, `text-lg` for typography
- `px-3`, `px-4`, `py-2`, `py-2.5` for spacing
- `rounded-lg`, `rounded-xl` for borders
- `transition-all duration-200` for animations
- `hover:`, `focus:` for interactive states

---

**Last Updated**: 2026-02-09
