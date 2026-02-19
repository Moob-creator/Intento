# Task Management UI - Code Examples

## Component Usage Examples

### Using the Task Store

```typescript
import { useTaskStore } from './store/taskStore';

function MyComponent() {
  // Access state and actions from store
  const {
    tasks,           // Task[] - all tasks
    selectedTask,    // Task | null - currently selected task
    isLoading,       // boolean - loading state
    error,           // string | null - error message
    loadTasks,       // () => Promise<void>
    createTask,      // (task) => Promise<void>
    updateTask,      // (id, updates) => Promise<void>
    deleteTask,      // (id) => Promise<void>
    selectTask,      // (task) => void
  } = useTaskStore();

  // Load tasks on mount
  useEffect(() => {
    loadTasks();
  }, [loadTasks]);

  // Create a new task
  const handleCreate = async () => {
    await createTask({
      title: 'New Task',
      description: 'Task description',
      status: 'todo',
      priority: 'medium',
    });
  };

  // Update a task
  const handleUpdate = async (taskId: number) => {
    await updateTask(taskId, {
      status: 'done',
    });
  };

  return (
    <div>
      {isLoading && <p>Loading...</p>}
      {error && <p>Error: {error}</p>}
      {tasks.map(task => (
        <div key={task.id}>{task.title}</div>
      ))}
    </div>
  );
}
```

### Using TaskSearchBar

```typescript
import { useState } from 'react';
import { TaskSearchBar } from './components/TaskSearchBar';

function SearchExample() {
  const [query, setQuery] = useState('');

  return (
    <TaskSearchBar
      value={query}
      onChange={setQuery}
      placeholder="Search tasks..."
    />
  );
}
```

### Using StatusFilter

```typescript
import { useState } from 'react';
import { StatusFilter } from './components/StatusFilter';
import type { TaskStatus } from './types/task';

function FilterExample() {
  const [status, setStatus] = useState<TaskStatus | 'all'>('all');

  return (
    <StatusFilter
      activeStatus={status}
      onStatusChange={setStatus}
    />
  );
}
```

### Using TaskCard

```typescript
import { TaskCard } from './components/TaskCard';
import type { Task } from './types/task';

function CardExample() {
  const task: Task = {
    id: 1,
    title: 'Complete documentation',
    description: 'Write comprehensive docs',
    status: 'doing',
    priority: 'high',
    deadline: Date.now() / 1000 + 86400, // Tomorrow
    created_at: Date.now() / 1000,
    updated_at: Date.now() / 1000,
  };

  const handleClick = () => {
    console.log('Task clicked:', task.id);
  };

  return (
    <TaskCard
      task={task}
      isSelected={false}
      onClick={handleClick}
    />
  );
}
```

### Using TaskList

```typescript
import { TaskList } from './components/TaskList';
import type { Task } from './types/task';

function ListExample() {
  const tasks: Task[] = [
    // ... array of tasks
  ];

  const [selectedId, setSelectedId] = useState<number | null>(null);

  const handleTaskClick = (task: Task) => {
    setSelectedId(task.id ?? null);
  };

  return (
    <TaskList
      tasks={tasks}
      selectedTaskId={selectedId}
      onTaskClick={handleTaskClick}
      isLoading={false}
    />
  );
}
```

### Using TaskDetailPanel

```typescript
import { TaskDetailPanel } from './components/TaskDetailPanel';
import type { Task } from './types/task';

function DetailExample() {
  const [task, setTask] = useState<Task | null>(null);

  const handleSave = async (updates: Partial<Task>) => {
    console.log('Saving task:', updates);
    // Call API or store action here
  };

  const handleDelete = async (id: number) => {
    console.log('Deleting task:', id);
    // Call API or store action here
  };

  const handleCancel = () => {
    setTask(null);
  };

  return (
    <TaskDetailPanel
      task={task}
      onSave={handleSave}
      onDelete={handleDelete}
      onCancel={handleCancel}
    />
  );
}
```

## Filtering and Searching Example

```typescript
import { useMemo } from 'react';
import type { Task, TaskStatus } from './types/task';

function useFilteredTasks(
  tasks: Task[],
  searchQuery: string,
  statusFilter: TaskStatus | 'all'
) {
  return useMemo(() => {
    let filtered = tasks;

    // Apply status filter
    if (statusFilter !== 'all') {
      filtered = filtered.filter(task => task.status === statusFilter);
    }

    // Apply search filter
    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(task =>
        task.title.toLowerCase().includes(query) ||
        (task.description && task.description.toLowerCase().includes(query))
      );
    }

    return filtered;
  }, [tasks, searchQuery, statusFilter]);
}

// Usage
function FilteredTaskList() {
  const { tasks } = useTaskStore();
  const [searchQuery, setSearchQuery] = useState('');
  const [statusFilter, setStatusFilter] = useState<TaskStatus | 'all'>('all');

  const filteredTasks = useFilteredTasks(tasks, searchQuery, statusFilter);

  return (
    <div>
      <TaskSearchBar value={searchQuery} onChange={setSearchQuery} />
      <StatusFilter activeStatus={statusFilter} onStatusChange={setStatusFilter} />
      <TaskList tasks={filteredTasks} ... />
    </div>
  );
}
```

## Creating a Task with Tauri

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { Task } from './types/task';

async function createTaskExample() {
  try {
    const taskId = await invoke<number>('create_task', {
      title: 'My New Task',
      description: 'This is a description',
      priority: 'high',
      deadline: Math.floor(Date.now() / 1000) + 86400, // Tomorrow
      tags: ['important', 'urgent'],
    });

    console.log('Created task with ID:', taskId);
    return taskId;
  } catch (error) {
    console.error('Failed to create task:', error);
    throw error;
  }
}
```

## Updating a Task with Tauri

```typescript
import { invoke } from '@tauri-apps/api/core';

async function updateTaskExample(taskId: number) {
  try {
    await invoke('update_task', {
      id: taskId,
      title: 'Updated Title',
      description: null, // Don't update description
      status: 'done',
      priority: null, // Don't update priority
      deadline: null,
      tags: ['completed'],
      completedAt: Math.floor(Date.now() / 1000),
    });

    console.log('Task updated successfully');
  } catch (error) {
    console.error('Failed to update task:', error);
    throw error;
  }
}
```

## Listing Tasks with Tauri

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { Task } from './types/task';

async function listTasksExample() {
  try {
    // Get all tasks
    const allTasks = await invoke<Task[]>('list_tasks', { status: null });
    console.log('All tasks:', allTasks);

    // Get only "doing" tasks
    const doingTasks = await invoke<Task[]>('list_tasks', { status: 'doing' });
    console.log('Doing tasks:', doingTasks);

    return allTasks;
  } catch (error) {
    console.error('Failed to list tasks:', error);
    throw error;
  }
}
```

## Deleting a Task with Tauri

```typescript
import { invoke } from '@tauri-apps/api/core';

async function deleteTaskExample(taskId: number) {
  // Show confirmation dialog
  const confirmed = window.confirm('Are you sure you want to delete this task?');

  if (!confirmed) {
    return;
  }

  try {
    await invoke('delete_task', { id: taskId });
    console.log('Task deleted successfully');
  } catch (error) {
    console.error('Failed to delete task:', error);
    throw error;
  }
}
```

## Custom Hook for Task Operations

```typescript
import { useTaskStore } from './store/taskStore';
import type { Task } from './types/task';

export function useTask(taskId?: number) {
  const { tasks, updateTask, deleteTask } = useTaskStore();

  const task = taskId
    ? tasks.find(t => t.id === taskId)
    : null;

  const markAsDone = async () => {
    if (!taskId) return;
    await updateTask(taskId, { status: 'done' });
  };

  const markAsTodo = async () => {
    if (!taskId) return;
    await updateTask(taskId, { status: 'todo' });
  };

  const markAsDoing = async () => {
    if (!taskId) return;
    await updateTask(taskId, { status: 'doing' });
  };

  const remove = async () => {
    if (!taskId) return;
    const confirmed = window.confirm('Delete this task?');
    if (confirmed) {
      await deleteTask(taskId);
    }
  };

  const isOverdue = task?.deadline
    ? task.deadline < Date.now() / 1000 && task.status !== 'done'
    : false;

  return {
    task,
    markAsDone,
    markAsTodo,
    markAsDoing,
    remove,
    isOverdue,
  };
}

// Usage
function TaskActions({ taskId }: { taskId: number }) {
  const { task, markAsDone, isOverdue } = useTask(taskId);

  if (!task) return null;

  return (
    <div>
      <h3>{task.title}</h3>
      {isOverdue && <span>⚠️ Overdue</span>}
      <button onClick={markAsDone}>Mark as Done</button>
    </div>
  );
}
```

## Date Formatting Utility

```typescript
export function formatDeadline(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  const now = new Date();
  const diffTime = date.getTime() - now.getTime();
  const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));

  if (diffDays < 0) return 'Overdue';
  if (diffDays === 0) return 'Today';
  if (diffDays === 1) return 'Tomorrow';
  if (diffDays <= 7) return `In ${diffDays} days`;

  return date.toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric'
  });
}

export function formatCompletionDate(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  return date.toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
    year: date.getFullYear() !== new Date().getFullYear()
      ? 'numeric'
      : undefined
  });
}

// Usage
const task: Task = { /* ... */ deadline: Date.now() / 1000 + 86400 };
console.log(formatDeadline(task.deadline!)); // "Tomorrow"
```

## Error Boundary for Tauri Calls

```typescript
import { useState } from 'react';

export function useTauriCommand<T>(
  commandName: string,
  onSuccess?: (result: T) => void,
  onError?: (error: string) => void
) {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const execute = async (args?: Record<string, unknown>) => {
    setIsLoading(true);
    setError(null);

    try {
      const result = await invoke<T>(commandName, args);
      onSuccess?.(result);
      return result;
    } catch (err) {
      const errorMessage = String(err);
      setError(errorMessage);
      onError?.(errorMessage);
      throw err;
    } finally {
      setIsLoading(false);
    }
  };

  return { execute, isLoading, error };
}

// Usage
function CreateTaskButton() {
  const { execute, isLoading, error } = useTauriCommand<number>(
    'create_task',
    (taskId) => console.log('Created task:', taskId),
    (error) => alert(`Error: ${error}`)
  );

  const handleCreate = () => {
    execute({
      title: 'New Task',
      description: null,
      priority: 'medium',
      deadline: null,
      tags: null,
    });
  };

  return (
    <div>
      <button onClick={handleCreate} disabled={isLoading}>
        {isLoading ? 'Creating...' : 'Create Task'}
      </button>
      {error && <p className="text-red-500">{error}</p>}
    </div>
  );
}
```

## Tailwind Custom Classes

```typescript
// Example of custom utility classes you might add

// In your component
<div className="task-card">
  {/* Content */}
</div>

// In App.css or a separate CSS file
.task-card {
  @apply flex flex-col gap-2 p-4 rounded-xl bg-white border border-neutral-light/60;
  @apply hover:shadow-soft hover:border-primary/20 transition-all duration-200;
  @apply cursor-pointer;
}

.task-card.selected {
  @apply bg-primary/10 border-2 border-primary ring-2 ring-primary/20 shadow-warm;
}

.task-card.completed {
  @apply opacity-60;
}
```

## Type Guards

```typescript
import type { Task, TaskStatus, TaskPriority } from './types/task';

export function isValidTaskStatus(value: string): value is TaskStatus {
  return ['todo', 'doing', 'done'].includes(value);
}

export function isValidTaskPriority(value: string): value is TaskPriority {
  return ['low', 'medium', 'high'].includes(value);
}

export function isTaskOverdue(task: Task): boolean {
  return !!(
    task.deadline &&
    task.deadline < Date.now() / 1000 &&
    task.status !== 'done'
  );
}

export function isTaskCompleted(task: Task): boolean {
  return task.status === 'done';
}

// Usage
const task: Task = { /* ... */ };

if (isTaskOverdue(task)) {
  console.log('This task is overdue!');
}

if (isTaskCompleted(task)) {
  console.log('This task is completed!');
}
```

---

**Note**: These examples demonstrate best practices for using the task management components and integrating with Tauri backend. Adapt them to your specific needs and extend as necessary.

**Last Updated**: 2026-02-09
