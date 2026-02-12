## ADDED Requirements

### Requirement: Zustand store provides task state
The system SHALL provide a Zustand store that maintains an in-memory array of tasks as the single source of truth for UI state.

#### Scenario: Initial state on mount
- **WHEN** application loads
- **THEN** store initializes with empty tasks array and loading false

#### Scenario: Store exposes task state
- **WHEN** component accesses store via `useTaskStore()`
- **THEN** component receives reactive `tasks` array that triggers re-renders on changes

### Requirement: Fetch tasks from backend
The system SHALL provide a `fetchTasks` action that loads all tasks from the backend via `list_tasks` command.

#### Scenario: Fetch tasks successfully
- **WHEN** `fetchTasks()` is called
- **THEN** store sets loading true, invokes `list_tasks`, updates tasks array, and sets loading false

#### Scenario: Fetch tasks on error
- **WHEN** `fetchTasks()` is called and backend returns error
- **THEN** store logs error to console, sets loading false, and keeps existing tasks array unchanged

#### Scenario: Fetch tasks with status filter
- **WHEN** `fetchTasks("todo")` is called
- **THEN** store invokes `list_tasks` with status "todo" and updates tasks array with filtered results

### Requirement: Add new task to store
The system SHALL provide an `addTask` action that creates a task on backend and updates local state.

#### Scenario: Add task successfully
- **WHEN** `addTask({ title: "Test", description: "Desc" })` is called
- **THEN** store invokes `create_task`, receives new task ID, and appends new task object to tasks array

#### Scenario: Add task on error
- **WHEN** `addTask()` is called and backend returns error
- **THEN** store logs error and does not modify tasks array

#### Scenario: Add task updates UI immediately
- **WHEN** `addTask()` succeeds
- **THEN** all components subscribed to `tasks` state re-render with new task visible

### Requirement: Update existing task in store
The system SHALL provide an `updateTask` action that updates a task on backend and updates local state.

#### Scenario: Update task successfully
- **WHEN** `updateTask(123, { status: "done" })` is called
- **THEN** store invokes `update_task`, and updates matching task in tasks array with new fields

#### Scenario: Update task on error
- **WHEN** `updateTask()` is called and backend returns error
- **THEN** store logs error and does not modify tasks array

#### Scenario: Update task optimistic UI
- **WHEN** `updateTask()` succeeds
- **THEN** UI reflects updated task immediately without refetching all tasks

### Requirement: Delete task from store
The system SHALL provide a `deleteTask` action that soft-deletes a task on backend and removes from local state.

#### Scenario: Delete task successfully
- **WHEN** `deleteTask(123)` is called
- **THEN** store invokes `delete_task` and removes task with id 123 from tasks array

#### Scenario: Delete task on error
- **WHEN** `deleteTask()` is called and backend returns error
- **THEN** store logs error and does not modify tasks array

#### Scenario: Delete task updates UI immediately
- **WHEN** `deleteTask()` succeeds
- **THEN** deleted task is immediately removed from UI without refetching

### Requirement: Store provides loading state
The system SHALL provide a `loading` boolean in the store to indicate backend operations in progress.

#### Scenario: Loading state during fetch
- **WHEN** `fetchTasks()` is in progress
- **THEN** store sets `loading` to true, and sets to false when complete or errored

#### Scenario: UI shows loading indicator
- **WHEN** store `loading` is true
- **THEN** components can display loading spinner or skeleton UI
