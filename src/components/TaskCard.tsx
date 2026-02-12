import { useState } from 'react';
import { Clock, Check, Edit2, Trash2, X } from 'lucide-react';
import type { Task } from '../types/task';

interface TaskCardProps {
  task: Task;
  isSelected?: boolean;
  onClick: () => void;
  onStatusChange?: (taskId: number, newStatus: Task['status']) => void;
  onEdit?: (task: Task) => void;
  onDelete?: (taskId: number) => void;
}

export function TaskCard({
  task,
  isSelected = false,
  onClick,
  onStatusChange,
  onEdit,
  onDelete,
}: TaskCardProps) {
  const [confirmDelete, setConfirmDelete] = useState(false);
  const isOverdue = task.deadline && task.deadline < Date.now() / 1000 && task.status !== 'done';
  const isDone = task.status === 'done';

  const statusConfig = {
    todo: { label: 'To Do', color: 'text-gray-700 bg-gray-100' },
    doing: { label: 'Doing', color: 'text-blue-700 bg-blue-100' },
    done: { label: 'Done', color: 'text-green-700 bg-green-100' },
  };

  const formatDeadline = (timestamp: number) => {
    const date = new Date(timestamp * 1000);
    const now = new Date();
    const diffTime = date.getTime() - now.getTime();
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));

    if (diffDays < 0) return 'Overdue';
    if (diffDays === 0) return 'Today';
    if (diffDays === 1) return 'Tomorrow';
    if (diffDays <= 7) return `In ${diffDays} days`;
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
  };

  const handleStatusToggle = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (onStatusChange && task.id) {
      const newStatus = task.status === 'done' ? 'todo' : 'done';
      onStatusChange(task.id, newStatus);
    }
  };

  const handleEdit = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (onEdit) {
      onEdit(task);
    }
  };

  const handleDelete = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (!confirmDelete) {
      setConfirmDelete(true);
      return;
    }
    if (onDelete && task.id) {
      onDelete(task.id);
    }
  };

  const handleCancelDelete = (e: React.MouseEvent) => {
    e.stopPropagation();
    setConfirmDelete(false);
  };

  return (
    <div
      onClick={onClick}
      className={`
        group relative flex flex-col gap-2 p-5 rounded-xl cursor-pointer
        transition-all duration-200
        ${isDone ? 'opacity-60' : ''}
        ${
          isSelected
            ? 'bg-primary/10 border-2 border-primary ring-2 ring-primary/20 shadow-warm'
            : 'bg-background-card border border-neutral-light/60 hover:shadow-soft hover:border-primary/20'
        }
      `}
    >
      {/* Quick actions - show on hover */}
      <div className="absolute top-3 right-3 flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
        {onStatusChange && (
          <button
            onClick={handleStatusToggle}
            className="p-1.5 bg-white text-green-600 hover:bg-green-50 rounded-lg shadow-sm transition-all duration-150"
            title={isDone ? 'Mark as todo' : 'Mark as done'}
          >
            <Check size={16} />
          </button>
        )}
        {onEdit && (
          <button
            onClick={handleEdit}
            className="p-1.5 bg-white text-blue-600 hover:bg-blue-50 rounded-lg shadow-sm transition-all duration-150"
            title="Edit task"
          >
            <Edit2 size={16} />
          </button>
        )}
        {onDelete && !confirmDelete && (
          <button
            onClick={handleDelete}
            className="p-1.5 bg-white text-red-600 hover:bg-red-50 rounded-lg shadow-sm transition-all duration-150"
            title="Delete task"
          >
            <Trash2 size={16} />
          </button>
        )}
        {confirmDelete && (
          <div className="flex items-center gap-1">
            <button
              onClick={handleCancelDelete}
              className="p-1.5 bg-white text-gray-500 hover:bg-gray-50 rounded-lg shadow-sm transition-all duration-150"
              title="Cancel"
            >
              <X size={16} />
            </button>
            <button
              onClick={handleDelete}
              className="px-2.5 py-1 bg-red-600 text-white hover:bg-red-700 rounded-lg shadow-sm transition-all duration-150 text-xs font-medium"
              title="Confirm delete"
            >
              Delete
            </button>
          </div>
        )}
      </div>

      {/* Header with title and status */}
      <div className="flex justify-between items-start gap-4 pr-20">
        <h3 className={`text-neutral-dark font-semibold text-base leading-snug ${isDone ? 'line-through' : ''}`}>
          {task.title}
        </h3>

        {/* Priority badge */}
        {task.priority === 'high' && !isDone && (
          <span className="text-xs font-semibold uppercase px-2.5 py-1 rounded-full text-red-800 bg-red-100 shrink-0">
            High
          </span>
        )}
      </div>

      {/* Description */}
      {task.description && (
        <p className={`text-sm text-neutral-dark/60 leading-relaxed line-clamp-2 ${isDone ? 'line-through' : ''}`}>
          {task.description}
        </p>
      )}

      {/* Footer with deadline and metadata */}
      <div className="flex items-center flex-wrap gap-4 mt-2">
        {/* Status badge */}
        <span className={`text-xs font-semibold uppercase px-2.5 py-1 rounded-full ${statusConfig[task.status].color}`}>
          {statusConfig[task.status].label}
        </span>

        {task.deadline && !isDone && (
          <div className={`flex items-center gap-1.5 text-sm ${isOverdue ? 'text-red-600' : 'text-neutral-dark/60'}`}>
            <Clock size={16} />
            <span>{formatDeadline(task.deadline)}</span>
          </div>
        )}

        {isDone && task.completed_at && (
          <div className="flex items-center gap-1.5 text-sm text-neutral-dark/60">
            <Clock size={16} />
            <span>Completed: {new Date(task.completed_at * 1000).toLocaleDateString('en-US', { month: 'short', day: 'numeric' })}</span>
          </div>
        )}

        {/* Tags */}
        {task.tags && task.tags.length > 0 && (
          <div className="flex items-center gap-1.5 ml-auto">
            {task.tags.slice(0, 3).map((tag, index) => (
              <span
                key={index}
                className="text-xs px-2 py-0.5 bg-accent-peach text-primary-dark rounded-full"
              >
                {tag}
              </span>
            ))}
            {task.tags.length > 3 && (
              <span className="text-xs text-neutral-dark/60">+{task.tags.length - 3}</span>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
