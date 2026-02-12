import { TaskCard } from './TaskCard';
import type { Task } from '../types/task';

interface TaskListProps {
  tasks: Task[];
  selectedTaskId: number | null;
  onTaskClick: (task: Task) => void;
  onStatusChange?: (taskId: number, newStatus: Task['status']) => void;
  onEdit?: (task: Task) => void;
  onDelete?: (taskId: number) => void;
  isLoading?: boolean;
}

export function TaskList({
  tasks,
  selectedTaskId,
  onTaskClick,
  onStatusChange,
  onEdit,
  onDelete,
  isLoading = false,
}: TaskListProps) {
  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="animate-pulse text-neutral-dark/40">Loading tasks...</div>
      </div>
    );
  }

  if (tasks.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-16 text-center">
        <div className="w-16 h-16 mb-4 bg-gradient-to-br from-primary/20 to-primary-dark/20 rounded-2xl flex items-center justify-center">
          <span className="text-3xl">📝</span>
        </div>
        <p className="text-neutral-dark text-lg font-medium mb-2">No tasks yet</p>
        <p className="text-neutral-dark/60 text-sm max-w-sm">
          Get started by creating a new task or use AI to quickly add tasks with natural language
        </p>
        <div className="mt-6 flex gap-3">
          <kbd className="px-3 py-1.5 text-sm font-medium text-neutral-dark/80 bg-white border border-neutral-light rounded-lg">
            ⌘K
          </kbd>
          <span className="text-neutral-dark/60">to open command palette</span>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-4">
      {tasks.map((task) => (
        <TaskCard
          key={task.id}
          task={task}
          isSelected={task.id === selectedTaskId}
          onClick={() => onTaskClick(task)}
          onStatusChange={onStatusChange}
          onEdit={onEdit}
          onDelete={onDelete}
        />
      ))}
    </div>
  );
}
