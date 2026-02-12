import { useState, useEffect, useRef, useMemo } from 'react';
import {
  Sparkles,
  Plus,
  BarChart3,
  Calendar,
  Clock,
  Settings,
  HelpCircle,
  Search,
  CheckCircle2,
  Circle,
  Loader2,
} from 'lucide-react';
import type { Task } from '../types/task';

interface CommandPaletteProps {
  isOpen: boolean;
  onClose: () => void;
  tasks: Task[];
  onAIAdd: () => void;
  onNewTask: () => void;
  onShowStats: () => void;
  onShowSettings: () => void;
  onTestNotification: () => void;
  onTodayTasks: () => void;
  onDueSoon: () => void;
  onTaskSelect: (task: Task) => void;
}

interface Command {
  id: string;
  label: string;
  icon: React.ReactNode;
  action: () => void;
  category: 'action' | 'filter' | 'navigation';
  keywords?: string[];
}

export function CommandPalette({
  isOpen,
  onClose,
  tasks,
  onAIAdd,
  onNewTask,
  onShowStats,
  onShowSettings,
  onTestNotification,
  onTodayTasks,
  onDueSoon,
  onTaskSelect,
}: CommandPaletteProps) {
  const [search, setSearch] = useState('');
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);
  const listRef = useRef<HTMLDivElement>(null);
  const isComposingRef = useRef(false);
  const compositionEndTimeRef = useRef(0);

  // Commands list
  const commands: Command[] = useMemo(
    () => [
      {
        id: 'ai-add',
        label: 'AI',
        icon: <Sparkles className="text-amber-500" size={18} />,
        action: () => {
          onAIAdd();
          onClose();
        },
        category: 'action',
        keywords: ['ai', 'add', 'create', 'new', 'task'],
      },
      {
        id: 'new-task',
        label: 'New Task',
        icon: <Plus className="text-primary" size={18} />,
        action: () => {
          onNewTask();
          onClose();
        },
        category: 'action',
        keywords: ['new', 'create', 'add', 'task'],
      },
      {
        id: 'statistics',
        label: 'View Statistics',
        icon: <BarChart3 className="text-blue-500" size={18} />,
        action: () => {
          onShowStats();
          onClose();
        },
        category: 'navigation',
        keywords: ['stats', 'statistics', 'analytics', 'chart', 'data'],
      },
      {
        id: 'today',
        label: "Today's Tasks",
        icon: <Calendar className="text-green-500" size={18} />,
        action: () => {
          onTodayTasks();
          onClose();
        },
        category: 'filter',
        keywords: ['today', 'calendar', 'date'],
      },
      {
        id: 'due-soon',
        label: 'Due Soon',
        icon: <Clock className="text-orange-500" size={18} />,
        action: () => {
          onDueSoon();
          onClose();
        },
        category: 'filter',
        keywords: ['due', 'soon', 'deadline', 'expiring'],
      },
      {
        id: 'settings',
        label: 'Settings',
        icon: <Settings className="text-neutral-dark/60" size={18} />,
        action: () => {
          onShowSettings();
          onClose();
        },
        category: 'navigation',
        keywords: ['settings', 'preferences', 'config'],
      },
      {
        id: 'help',
        label: 'Help & Support',
        icon: <HelpCircle className="text-neutral-dark/60" size={18} />,
        action: () => {
          onClose();
        },
        category: 'navigation',
        keywords: ['help', 'support', 'docs', 'documentation'],
      },
    ],
    [onAIAdd, onNewTask, onShowStats, onShowSettings, onTestNotification, onTodayTasks, onDueSoon, onClose]
  );

  // Filter commands and tasks based on search
  const filteredCommands = useMemo(() => {
    if (!search.trim()) return commands;

    const query = search.toLowerCase();
    return commands.filter(
      (cmd) =>
        cmd.label.toLowerCase().includes(query) ||
        cmd.keywords?.some((kw) => kw.toLowerCase().includes(query))
    );
  }, [commands, search]);

  const filteredTasks = useMemo(() => {
    if (!search.trim()) return [];

    const query = search.toLowerCase();
    return tasks.filter(
      (task) =>
        task.title.toLowerCase().includes(query) ||
        (task.description && task.description.toLowerCase().includes(query)) ||
        task.tags?.some((tag) => tag.toLowerCase().includes(query))
    );
  }, [tasks, search]);

  const totalItems = filteredCommands.length + filteredTasks.length;

  // Handle keyboard navigation
  useEffect(() => {
    if (!isOpen) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      // Skip all shortcut handling during IME composition,
      // or within a short window after composition ends.
      // When the user presses Escape/Enter to dismiss the IME,
      // compositionend fires before keydown, so we use a
      // timestamp-based cooldown to absorb that trailing key.
      if (
        isComposingRef.current ||
        e.isComposing ||
        e.keyCode === 229 ||
        Date.now() - compositionEndTimeRef.current < 200
      ) {
        return;
      }

      switch (e.key) {
        case 'ArrowDown':
          e.preventDefault();
          setSelectedIndex((prev) => (prev + 1) % totalItems);
          break;
        case 'ArrowUp':
          e.preventDefault();
          setSelectedIndex((prev) => (prev - 1 + totalItems) % totalItems);
          break;
        case 'Enter':
          e.preventDefault();
          if (selectedIndex < filteredCommands.length) {
            filteredCommands[selectedIndex].action();
          } else {
            const taskIndex = selectedIndex - filteredCommands.length;
            if (filteredTasks[taskIndex]) {
              onTaskSelect(filteredTasks[taskIndex]);
              onClose();
            }
          }
          break;
        case 'Escape':
          e.preventDefault();
          // If search has text, clear it first; otherwise close
          if (search.trim()) {
            setSearch('');
            setSelectedIndex(0);
          } else {
            onClose();
          }
          break;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, search, selectedIndex, totalItems, filteredCommands, filteredTasks, onTaskSelect, onClose]);

  // Focus input when opened
  useEffect(() => {
    if (isOpen) {
      inputRef.current?.focus();
      setSearch('');
      setSelectedIndex(0);
    }
  }, [isOpen]);

  // Scroll selected item into view
  useEffect(() => {
    if (listRef.current) {
      const selectedElement = listRef.current.querySelector(`[data-index="${selectedIndex}"]`);
      if (selectedElement) {
        selectedElement.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
      }
    }
  }, [selectedIndex]);

  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-start justify-center pt-[15vh] bg-black/40 backdrop-blur-sm"
      onClick={onClose}
    >
      <div
        className="w-full max-w-2xl bg-white rounded-xl shadow-2xl overflow-hidden"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Search input */}
        <div className="flex items-center gap-4 px-5 py-4 border-b border-neutral-light/60">
          <Search size={20} className="text-neutral-dark/40 flex-shrink-0" />
          <input
            ref={inputRef}
            type="text"
            value={search}
            onChange={(e) => {
              setSearch(e.target.value);
              setSelectedIndex(0);
            }}
            onCompositionStart={() => { isComposingRef.current = true; }}
            onCompositionEnd={() => {
              isComposingRef.current = false;
              compositionEndTimeRef.current = Date.now();
            }}
            placeholder="Search tasks or run a command..."
            className="flex-1 text-base text-neutral-dark placeholder:text-neutral-dark/40 bg-transparent outline-none border-none shadow-none focus:outline-none focus:ring-0 [&:focus-visible]:outline-none"
          />
          <kbd className="px-3 py-1 text-xs font-medium text-neutral-dark/60 bg-neutral-light/40 rounded flex-shrink-0">
            ESC
          </kbd>
        </div>

        {/* Results */}
        <div ref={listRef} className="max-h-[60vh] overflow-y-auto">
          {/* Commands */}
          {filteredCommands.length > 0 && (
            <div className="py-3">
              <div className="px-5 py-2 text-xs font-semibold text-neutral-dark/60 uppercase tracking-wide">
                Commands
              </div>
              {filteredCommands.map((command, index) => (
                <button
                  key={command.id}
                  data-index={index}
                  onClick={command.action}
                  className={`w-full flex items-center gap-4 px-5 py-3.5 text-left transition-all duration-150 ${
                    index === selectedIndex
                      ? 'bg-primary/10 border-l-2 border-primary'
                      : 'hover:bg-neutral-light/30'
                  }`}
                >
                  <div className="flex-shrink-0">{command.icon}</div>
                  <span className="text-sm font-medium text-neutral-dark">{command.label}</span>
                </button>
              ))}
            </div>
          )}

          {/* Tasks */}
          {filteredTasks.length > 0 && (
            <div className="py-3 border-t border-neutral-light/60">
              <div className="px-5 py-2 text-xs font-semibold text-neutral-dark/60 uppercase tracking-wide">
                Tasks ({filteredTasks.length})
              </div>
              {filteredTasks.map((task, index) => {
                const globalIndex = filteredCommands.length + index;
                return (
                  <button
                    key={task.id}
                    data-index={globalIndex}
                    onClick={() => {
                      onTaskSelect(task);
                      onClose();
                    }}
                    className={`w-full flex items-center gap-4 px-5 py-3.5 text-left transition-all duration-150 ${
                      globalIndex === selectedIndex
                        ? 'bg-primary/10 border-l-2 border-primary'
                        : 'hover:bg-neutral-light/30'
                    }`}
                  >
                    <div className="flex-shrink-0">
                      {task.status === 'done' ? (
                        <CheckCircle2 size={18} className="text-green-500" />
                      ) : task.status === 'doing' ? (
                        <Loader2 size={18} className="text-blue-500" />
                      ) : (
                        <Circle size={18} className="text-neutral-dark/40" />
                      )}
                    </div>
                    <div className="flex-1 min-w-0">
                      <div className="text-sm font-medium text-neutral-dark truncate">
                        {task.title}
                      </div>
                      {task.description && (
                        <div className="text-xs text-neutral-dark/60 truncate mt-0.5">
                          {task.description}
                        </div>
                      )}
                    </div>
                    {task.priority === 'high' && (
                      <span className="px-2 py-0.5 text-xs font-medium text-red-600 bg-red-50 rounded">
                        High
                      </span>
                    )}
                  </button>
                );
              })}
            </div>
          )}

          {/* Empty state */}
          {filteredCommands.length === 0 && filteredTasks.length === 0 && search.trim() && (
            <div className="py-12 text-center">
              <p className="text-neutral-dark/60">No results found</p>
              <p className="text-sm text-neutral-dark/40 mt-1">Try a different search term</p>
            </div>
          )}
        </div>

        {/* Footer hint */}
        <div className="flex items-center justify-between px-4 py-2 border-t border-neutral-light/60 bg-neutral-light/20 text-xs text-neutral-dark/60">
          <div className="flex items-center gap-4">
            <span className="flex items-center gap-1">
              <kbd className="px-1.5 py-0.5 bg-white rounded border border-neutral-light">↑</kbd>
              <kbd className="px-1.5 py-0.5 bg-white rounded border border-neutral-light">↓</kbd>
              to navigate
            </span>
            <span className="flex items-center gap-1">
              <kbd className="px-1.5 py-0.5 bg-white rounded border border-neutral-light">↵</kbd>
              to select
            </span>
          </div>
          <span className="flex items-center gap-1">
            <kbd className="px-1.5 py-0.5 bg-white rounded border border-neutral-light">⌘K</kbd>
            to open
          </span>
        </div>
      </div>
    </div>
  );
}
