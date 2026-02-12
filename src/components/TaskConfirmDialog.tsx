import { useState, useEffect } from 'react';
import { X, Calendar, Plus, X as XIcon } from 'lucide-react';
import type { ParsedTask } from '../types/task';

interface TaskConfirmDialogProps {
  isOpen: boolean;
  parsedTask: ParsedTask | null;
  onConfirm: (task: ParsedTask) => Promise<void>;
  onCancel: () => void;
  isLoading: boolean;
  error: string | null;
}

export function TaskConfirmDialog({
  isOpen,
  parsedTask,
  onConfirm,
  onCancel,
  isLoading,
  error,
}: TaskConfirmDialogProps) {
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [deadline, setDeadline] = useState('');
  const [priority, setPriority] = useState<'low' | 'medium' | 'high'>('medium');
  const [tags, setTags] = useState<string[]>([]);
  const [newTag, setNewTag] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);

  // Reset form when dialog opens with new parsed data
  useEffect(() => {
    if (isOpen && parsedTask) {
      setTitle(parsedTask.title);
      setDescription(parsedTask.description || '');
      setDeadline(parsedTask.deadline || '');
      setPriority(parsedTask.priority || 'medium');
      setTags(parsedTask.tags || []);
      setNewTag('');
    }
  }, [isOpen, parsedTask]);

  const handleAddTag = () => {
    const trimmed = newTag.trim();
    if (trimmed && !tags.includes(trimmed)) {
      setTags([...tags, trimmed]);
      setNewTag('');
    }
  };

  const handleRemoveTag = (tagToRemove: string) => {
    setTags(tags.filter((t) => t !== tagToRemove));
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      handleAddTag();
    }
  };

  const handleConfirm = async () => {
    if (!title.trim()) {
      return;
    }

    setIsSubmitting(true);
    try {
      const confirmedTask: ParsedTask = {
        title: title.trim(),
        description: description.trim() || undefined,
        deadline: deadline || undefined,
        priority,
        tags: tags.length > 0 ? tags : undefined,
      };
      await onConfirm(confirmedTask);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleCancel = () => {
    if (!isSubmitting) {
      onCancel();
    }
  };

  if (!isOpen) {
    return null;
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-stone-900/30 backdrop-blur-sm"
        onClick={handleCancel}
        aria-hidden="true"
      />

      {/* Modal */}
      <div className="relative w-full max-w-2xl rounded-2xl bg-white shadow-2xl shadow-stone-300/50 flex flex-col max-h-[90vh]">
        {/* Close button */}
        <button
          onClick={handleCancel}
          disabled={isSubmitting}
          className="absolute right-4 top-4 p-2 rounded-lg text-stone-400 hover:text-stone-600 hover:bg-stone-100 transition-all duration-200 disabled:opacity-50"
          aria-label="Close"
        >
          <X size={20} />
        </button>

        {/* Content */}
        <div className="flex flex-col p-6 sm:p-8 overflow-y-auto">
          {/* Header */}
          <div className="flex flex-col gap-1 pb-4">
            <h2 className="text-3xl font-bold text-stone-800 tracking-tight">
              Review Task
            </h2>
            <p className="text-sm text-stone-500 leading-relaxed">
              Confirm and edit the details before adding to your tasks
            </p>
          </div>

          {/* Error message */}
          {error && (
            <div className="mb-4 p-3 rounded-lg bg-rose-50 border border-rose-200">
              <p className="text-sm text-rose-800">{error}</p>
            </div>
          )}

          {/* Form Fields */}
          <div className="flex flex-col gap-5">
            {/* Task Title */}
            <div className="flex flex-col gap-2">
              <label htmlFor="task-title" className="text-base font-medium text-stone-700">
                Task Title
              </label>
              <input
                id="task-title"
                type="text"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                placeholder="Enter task title..."
                autoFocus
                className="w-full px-4 py-3 rounded-xl bg-amber-50/50 text-stone-800
                         placeholder:text-stone-400 border-2 border-amber-100
                         focus:outline-none focus:border-amber-400/50 focus:bg-amber-50/80
                         transition-all duration-200 text-base font-semibold"
              />
            </div>

            {/* Description */}
            <div className="flex flex-col gap-2">
              <label htmlFor="task-desc" className="text-base font-medium text-stone-700">
                Description (optional)
              </label>
              <textarea
                id="task-desc"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                placeholder="Add more details..."
                rows={4}
                className="w-full px-4 py-3 rounded-xl bg-amber-50/50 text-stone-800
                         placeholder:text-stone-400 border-2 border-amber-100 resize-none
                         focus:outline-none focus:border-amber-400/50 focus:bg-amber-50/80
                         transition-all duration-200 text-base leading-relaxed"
              />
            </div>

            {/* Deadline and Priority */}
            <div className="flex flex-col sm:flex-row w-full gap-4">
              {/* Due Date */}
              <div className="flex flex-col flex-1 gap-2">
                <label htmlFor="task-deadline" className="text-base font-medium text-stone-700">
                  Due Date
                </label>
                <div className="relative">
                  <input
                    id="task-deadline"
                    type="datetime-local"
                    value={deadline}
                    onChange={(e) => setDeadline(e.target.value)}
                    className="w-full px-4 py-3 pr-10 rounded-xl bg-amber-50/50 text-stone-800
                             placeholder:text-stone-400 border-2 border-amber-100
                             focus:outline-none focus:border-amber-400/50 focus:bg-amber-50/80
                             transition-all duration-200 text-base"
                  />
                  <Calendar
                    size={20}
                    className="absolute right-3 top-1/2 -translate-y-1/2 text-stone-400 pointer-events-none"
                  />
                </div>
              </div>

              {/* Priority */}
              <div className="flex flex-col flex-1 gap-2">
                <label htmlFor="task-priority" className="text-base font-medium text-stone-700">
                  Priority
                </label>
                <select
                  id="task-priority"
                  value={priority}
                  onChange={(e) => setPriority(e.target.value as 'low' | 'medium' | 'high')}
                  className="w-full px-4 py-3 rounded-xl bg-amber-50/50 text-stone-800
                           border-2 border-amber-100 cursor-pointer
                           focus:outline-none focus:border-amber-400/50 focus:bg-amber-50/80
                           transition-all duration-200 text-base appearance-none"
                >
                  <option value="low">Low</option>
                  <option value="medium">Medium</option>
                  <option value="high">High</option>
                </select>
              </div>
            </div>

            {/* Tags */}
            <div className="flex flex-col gap-2">
              <label htmlFor="task-tags" className="text-base font-medium text-stone-700">
                Tags
              </label>
              <div className="flex gap-2">
                <input
                  id="task-tags"
                  type="text"
                  value={newTag}
                  onChange={(e) => setNewTag(e.target.value)}
                  onKeyDown={handleKeyDown}
                  placeholder="Add a tag..."
                  className="flex-1 px-4 py-2.5 rounded-xl bg-amber-50/50 text-stone-800
                           placeholder:text-stone-400 border-2 border-amber-100
                           focus:outline-none focus:border-amber-400/50 focus:bg-amber-50/80
                           transition-all duration-200 text-base"
                />
                <button
                  onClick={handleAddTag}
                  disabled={!newTag.trim() || tags.includes(newTag.trim())}
                  className="px-4 py-2.5 rounded-xl bg-stone-100 text-stone-600
                           hover:bg-stone-200 transition-colors duration-200
                           disabled:opacity-50 disabled:cursor-not-allowed"
                  aria-label="Add tag"
                >
                  <Plus size={18} />
                </button>
              </div>

              {/* Tags display */}
              {tags.length > 0 && (
                <div className="flex flex-wrap gap-2 mt-2">
                  {tags.map((tag) => (
                    <span
                      key={tag}
                      className="flex items-center gap-1 px-3 py-1.5 rounded-lg
                               bg-amber-100/80 text-amber-800 text-sm font-medium
                               transition-colors duration-200"
                    >
                      {tag}
                      <button
                        onClick={() => handleRemoveTag(tag)}
                        className="ml-1 rounded-full hover:bg-amber-200/80 p-0.5"
                        aria-label={`Remove ${tag}`}
                      >
                        <XIcon size={14} />
                      </button>
                    </span>
                  ))}
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Footer Actions */}
        <div className="flex flex-col-reverse sm:flex-row gap-3 p-6 pt-4 mt-2 border-t border-amber-100/50">
          <button
            onClick={handleCancel}
            disabled={isSubmitting || isLoading}
            className="flex items-center justify-center w-full sm:w-auto
                     h-12 px-6 rounded-xl bg-stone-100 text-stone-700
                     font-bold text-base transition-all duration-200
                     hover:bg-stone-200 hover:shadow-sm
                     disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Cancel
          </button>
          <button
            onClick={handleConfirm}
            disabled={isSubmitting || isLoading || !title.trim()}
            className="flex items-center justify-center gap-2 w-full sm:w-auto
                     h-12 px-6 rounded-xl bg-gradient-to-r from-orange-400 to-rose-400
                     text-white font-bold text-base transition-all duration-200
                     hover:from-orange-500 hover:to-rose-500 hover:shadow-lg
                     disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isSubmitting || isLoading ? (
              <>
                <svg
                  className="animate-spin h-5 w-5"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                >
                  <circle
                    className="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    strokeWidth="4"
                  />
                  <path
                    className="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                  />
                </svg>
                Adding...
              </>
            ) : (
              'Add Task'
            )}
          </button>
        </div>
      </div>
    </div>
  );
}
