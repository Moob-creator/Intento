import { useState, useEffect, useRef } from 'react';
import { Trash2, Calendar, X, Hash, Circle, Loader2, CheckCircle2 } from 'lucide-react';
import { DateTimePicker } from './DateTimePicker';
import { CustomSelect, type SelectOption } from './CustomSelect';
import type { Task, TaskStatus, TaskPriority } from '../types/task';

interface TaskDetailPanelProps {
  task: Task | null;
  onSave: (updates: Partial<Task>) => Promise<void>;
  onDelete: (id: number) => Promise<void>;
  onCancel: () => void;
}

export function TaskDetailPanel({ task, onSave, onDelete, onCancel }: TaskDetailPanelProps) {
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [status, setStatus] = useState<TaskStatus>('todo');
  const [priority, setPriority] = useState<TaskPriority>('medium');
  const [deadlineDate, setDeadlineDate] = useState('');
  const [deadlineTime, setDeadlineTime] = useState('');
  const [tags, setTags] = useState<string[]>([]);
  const [tagInput, setTagInput] = useState('');
  const [showDateTimePicker, setShowDateTimePicker] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);
  const dateTimePickerRef = useRef<HTMLDivElement>(null);

  // Status options with icons
  const statusOptions: SelectOption[] = [
    {
      value: 'todo',
      label: 'To Do',
      icon: <Circle size={18} className="text-neutral-dark/60" />,
    },
    {
      value: 'doing',
      label: 'Doing',
      icon: <Loader2 size={18} className="text-blue-500" />,
    },
    {
      value: 'done',
      label: 'Done',
      icon: <CheckCircle2 size={18} className="text-green-500" />,
    },
  ];

  // Priority options with colors
  const priorityOptions: SelectOption[] = [
    {
      value: 'low',
      label: 'Low',
      icon: <div className="w-2 h-2 rounded-full bg-blue-400" />,
    },
    {
      value: 'medium',
      label: 'Medium',
      icon: <div className="w-2 h-2 rounded-full bg-amber-400" />,
    },
    {
      value: 'high',
      label: 'High',
      icon: <div className="w-2 h-2 rounded-full bg-rose-400" />,
    },
  ];

  // Update form when task changes
  useEffect(() => {
    if (task) {
      setTitle(task.title);
      setDescription(task.description || '');
      setStatus(task.status);
      setPriority(task.priority);
      setTags(task.tags || []);
      if (task.deadline) {
        const deadlineDateTime = new Date(task.deadline * 1000);
        setDeadlineDate(deadlineDateTime.toISOString().split('T')[0]);
        const hours = deadlineDateTime.getHours().toString().padStart(2, '0');
        const minutes = deadlineDateTime.getMinutes().toString().padStart(2, '0');
        setDeadlineTime(`${hours}:${minutes}`);
      } else {
        setDeadlineDate('');
        setDeadlineTime('');
      }
    } else {
      // Reset form for new task
      setTitle('');
      setDescription('');
      setStatus('todo');
      setPriority('medium');
      setTags([]);
      setDeadlineDate('');
      setDeadlineTime('');
    }
  }, [task]);

  // Close date time picker when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dateTimePickerRef.current && !dateTimePickerRef.current.contains(event.target as Node)) {
        setShowDateTimePicker(false);
      }
    };

    if (showDateTimePicker) {
      document.addEventListener('mousedown', handleClickOutside);
      return () => document.removeEventListener('mousedown', handleClickOutside);
    }
  }, [showDateTimePicker]);

  const handleSave = async () => {
    if (!title.trim()) {
      alert('Please enter a task title');
      return;
    }

    setIsSaving(true);
    try {
      const updates: Partial<Task> = {
        title: title.trim(),
        description: description.trim() || undefined,
        status,
        priority,
        tags: tags.length > 0 ? tags : undefined,
        deadline: deadlineDate && deadlineTime
          ? Math.floor(new Date(`${deadlineDate}T${deadlineTime}`).getTime() / 1000)
          : deadlineDate
          ? Math.floor(new Date(deadlineDate).getTime() / 1000)
          : undefined,
      };

      await onSave(updates);
    } finally {
      setIsSaving(false);
    }
  };

  const handleDelete = async () => {
    if (!task?.id) return;

    const confirmed = confirm('Are you sure you want to delete this task?');
    if (!confirmed) return;

    setIsDeleting(true);
    try {
      await onDelete(task.id);
    } finally {
      setIsDeleting(false);
    }
  };

  if (!task && title === '') {
    return (
      <aside className="w-96 flex-shrink-0 border-l border-neutral-light/60 bg-background-card flex items-center justify-center">
        <div className="text-center px-6">
          <p className="text-neutral-dark/40 text-sm">
            Select a task to view details or click "New Task" to create one
          </p>
        </div>
      </aside>
    );
  }

  return (
    <aside className="w-96 flex-shrink-0 border-l border-neutral-light/60 bg-background-card flex flex-col">
      {/* Form content */}
      <div className="p-6 flex-1 flex flex-col gap-6 overflow-y-auto">
        {/* Title */}
        <div className="flex flex-col gap-1.5">
          <label htmlFor="task-title" className="text-sm font-medium text-neutral-dark/60">
            Title
          </label>
          <input
            id="task-title"
            type="text"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            placeholder="Enter task title..."
            className="w-full px-1 py-2 text-lg font-semibold
                     text-neutral-dark placeholder:text-neutral-dark/40
                     border-none focus:outline-none bg-transparent
                     transition-all duration-200"
          />
        </div>

        {/* Description */}
        <div className="flex flex-col gap-1.5">
          <label htmlFor="task-desc" className="text-sm font-medium text-neutral-dark/60">
            Description
          </label>
          <textarea
            id="task-desc"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            placeholder="Add description..."
            rows={5}
            className="w-full px-1 py-2
                     text-neutral-dark placeholder:text-neutral-dark/40
                     border-none focus:outline-none bg-transparent
                     transition-all duration-200 resize-none"
          />
        </div>

        {/* Status */}
        <CustomSelect
          label="Status"
          value={status}
          options={statusOptions}
          onChange={(value) => setStatus(value as TaskStatus)}
        />

        {/* Priority */}
        <CustomSelect
          label="Priority"
          value={priority}
          options={priorityOptions}
          onChange={(value) => setPriority(value as TaskPriority)}
        />

        {/* Due Date & Time */}
        <div className="flex flex-col gap-1.5">
          <label className="text-sm font-medium text-neutral-dark/60">
            Due Date & Time
          </label>
          <div className="relative" ref={dateTimePickerRef}>
            <button
              type="button"
              onClick={() => setShowDateTimePicker(!showDateTimePicker)}
              className="w-full px-1 py-2
                       text-neutral-dark border-none text-left bg-transparent
                       focus:outline-none
                       transition-all duration-200 flex items-center gap-3"
            >
              <Calendar size={18} className="text-primary flex-shrink-0" />
              <span className={deadlineDate ? '' : 'text-neutral-dark/40'}>
                {deadlineDate && deadlineTime
                  ? `${new Date(deadlineDate).toLocaleDateString('zh-CN', {
                      year: 'numeric',
                      month: '2-digit',
                      day: '2-digit'
                    })} ${deadlineTime}`
                  : deadlineDate
                  ? new Date(deadlineDate).toLocaleDateString('zh-CN', {
                      year: 'numeric',
                      month: '2-digit',
                      day: '2-digit'
                    })
                  : '选择日期和时间...'}
              </span>
              {deadlineDate && (
                <button
                  type="button"
                  onClick={(e) => {
                    e.stopPropagation();
                    setDeadlineDate('');
                    setDeadlineTime('');
                  }}
                  className="ml-auto text-neutral-dark/40 hover:text-red-500 transition-colors"
                >
                  清除
                </button>
              )}
            </button>

            {/* Date Time Picker */}
            {showDateTimePicker && (
              <DateTimePicker
                value={{ date: deadlineDate, time: deadlineTime }}
                onChange={({ date, time }) => {
                  setDeadlineDate(date);
                  setDeadlineTime(time);
                }}
                onClose={() => setShowDateTimePicker(false)}
              />
            )}
          </div>
        </div>

        {/* Tags */}
        <div className="flex flex-col gap-1.5">
          <label className="text-sm font-medium text-neutral-dark/60">
            Tags
          </label>
          <div className="w-full py-2 min-h-[44px]">
            <div className="flex flex-wrap gap-2">
              {tags.map((tag, index) => (
                <span
                  key={index}
                  className="inline-flex items-center gap-1.5 px-2.5 py-1 bg-primary/10 text-primary rounded-lg text-xs font-medium"
                >
                  <Hash size={12} />
                  {tag}
                  <button
                    type="button"
                    onClick={() => setTags(tags.filter((_, i) => i !== index))}
                    className="hover:text-red-500 transition-colors"
                  >
                    <X size={12} />
                  </button>
                </span>
              ))}
              <input
                type="text"
                value={tagInput}
                onChange={(e) => setTagInput(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === 'Enter' && tagInput.trim()) {
                    e.preventDefault();
                    const newTag = tagInput.trim();
                    if (!tags.includes(newTag)) {
                      setTags([...tags, newTag]);
                    }
                    setTagInput('');
                  } else if (e.key === 'Backspace' && !tagInput && tags.length > 0) {
                    setTags(tags.slice(0, -1));
                  }
                }}
                placeholder={tags.length === 0 ? 'Add tags...' : ''}
                className="flex-1 min-w-[120px] bg-transparent outline-none text-sm text-neutral-dark placeholder:text-neutral-dark/40"
              />
            </div>
          </div>
          <p className="text-xs text-neutral-dark/40">
            Press Enter to add a tag, Backspace to remove
          </p>
        </div>
      </div>

      {/* Action buttons */}
      <div className="p-6 border-t border-neutral-light/60 flex items-center justify-between gap-4">
        {/* Delete button */}
        {task?.id && (
          <button
            onClick={handleDelete}
            disabled={isDeleting}
            className="p-2 text-neutral-dark/40 hover:text-red-500 transition-colors duration-200 disabled:opacity-50"
            title="Delete task"
          >
            <Trash2 size={20} />
          </button>
        )}

        {/* Save/Cancel buttons */}
        <div className="flex items-center gap-3 ml-auto">
          <button
            onClick={onCancel}
            disabled={isSaving || isDeleting}
            className="px-5 py-2.5 rounded-lg bg-neutral-light/60 text-neutral-dark
                     font-medium text-sm transition-all duration-200
                     hover:bg-neutral-light hover:shadow-soft
                     disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Cancel
          </button>
          <button
            onClick={handleSave}
            disabled={isSaving || isDeleting}
            className="px-5 py-2.5 rounded-lg bg-primary text-white
                     font-medium text-sm transition-all duration-200
                     hover:bg-primary-dark hover:shadow-warm
                     disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isSaving ? 'Saving...' : 'Save'}
          </button>
        </div>
      </div>
    </aside>
  );
}
