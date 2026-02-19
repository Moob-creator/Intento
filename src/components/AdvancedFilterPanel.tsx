import { useState, useEffect } from 'react';
import { Filter, X, ChevronDown, Calendar, Tag, AlertCircle, Clock, Search } from 'lucide-react';
import type { Task, TaskStatus, TaskPriority } from '../types/task';

interface FilterOptions {
  status: TaskStatus | 'all';
  priorities: TaskPriority[];
  tags: string[];
  deadlineRange: {
    start: number | null;
    end: number | null;
  };
  searchQuery: string;
  showOverdueOnly: boolean;
  showCompletedToday: boolean;
}

interface AdvancedFilterPanelProps {
  isOpen: boolean;
  onClose: () => void;
  tasks: Task[];
  currentFilters: FilterOptions;
  onApplyFilters: (filters: FilterOptions) => void;
}

export function AdvancedFilterPanel({
  isOpen,
  onClose,
  tasks,
  currentFilters,
  onApplyFilters,
}: AdvancedFilterPanelProps) {
  const [filters, setFilters] = useState<FilterOptions>(currentFilters);
  const [expandedSections, setExpandedSections] = useState({
    status: true,
    priority: true,
    tags: true,
    deadline: false,
    quick: false,
  });

  // Extract all unique tags from tasks
  const allTags = Array.from(new Set(tasks.flatMap((t) => t.tags || [])))
    .sort()
    .filter(Boolean);

  // Update local state when currentFilters change
  useEffect(() => {
    setFilters(currentFilters);
  }, [currentFilters]);

  if (!isOpen) return null;

  const toggleSection = (section: keyof typeof expandedSections) => {
    setExpandedSections((prev) => ({
      ...prev,
      [section]: !prev[section],
    }));
  };

  const handleStatusChange = (status: TaskStatus | 'all') => {
    setFilters((prev) => ({ ...prev, status }));
  };

  const handlePriorityToggle = (priority: TaskPriority) => {
    setFilters((prev) => ({
      ...prev,
      priorities: prev.priorities.includes(priority)
        ? prev.priorities.filter((p) => p !== priority)
        : [...prev.priorities, priority],
    }));
  };

  const handleTagToggle = (tag: string) => {
    setFilters((prev) => ({
      ...prev,
      tags: prev.tags.includes(tag)
        ? prev.tags.filter((t) => t !== tag)
        : [...prev.tags, tag],
    }));
  };

  const handleDeadlineRangeChange = (type: 'start' | 'end', value: string) => {
    const timestamp = value ? new Date(value).getTime() / 1000 : null;
    setFilters((prev) => ({
      ...prev,
      deadlineRange: {
        ...prev.deadlineRange,
        [type]: timestamp,
      },
    }));
  };

  const handleQuickFilterToggle = (filter: 'showOverdueOnly' | 'showCompletedToday') => {
    setFilters((prev) => ({
      ...prev,
      [filter]: !prev[filter],
    }));
  };

  const handleReset = () => {
    const resetFilters: FilterOptions = {
      status: 'all',
      priorities: [],
      tags: [],
      deadlineRange: { start: null, end: null },
      searchQuery: '',
      showOverdueOnly: false,
      showCompletedToday: false,
    };
    setFilters(resetFilters);
    onApplyFilters(resetFilters);
  };

  const handleApply = () => {
    onApplyFilters(filters);
    onClose();
  };

  const getActiveFilterCount = () => {
    let count = 0;
    if (filters.status !== 'all') count++;
    if (filters.priorities.length > 0) count += filters.priorities.length;
    if (filters.tags.length > 0) count += filters.tags.length;
    if (filters.deadlineRange.start || filters.deadlineRange.end) count++;
    if (filters.showOverdueOnly) count++;
    if (filters.showCompletedToday) count++;
    return count;
  };

  const activeCount = getActiveFilterCount();

  return (
    <>
      {/* Backdrop */}
      <div
        className="fixed inset-0 bg-black/30 backdrop-blur-sm z-40 animate-fade-in"
        onClick={onClose}
      />

      {/* Panel */}
      <div className="fixed right-0 top-0 h-full w-full max-w-md bg-background-warm shadow-2xl z-50 flex flex-col animate-slide-left">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-5 border-b border-neutral-light/60 bg-white">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-primary/10 rounded-lg">
              <Filter className="text-primary" size={20} />
            </div>
            <div>
              <h2 className="text-neutral-dark font-bold text-lg">Advanced Filters</h2>
              {activeCount > 0 && (
                <p className="text-sm text-primary font-medium">
                  {activeCount} {activeCount === 1 ? 'filter' : 'filters'} active
                </p>
              )}
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-2 text-neutral-dark/40 hover:text-neutral-dark hover:bg-neutral-light/40 rounded-lg transition-all duration-200"
            aria-label="Close filters"
          >
            <X size={20} />
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-6 py-4 space-y-5">
          {/* Search */}
          <div className="space-y-2">
            <label className="text-sm font-semibold text-neutral-dark flex items-center gap-2">
              <Search size={16} />
              Search Tasks
            </label>
            <input
              type="text"
              value={filters.searchQuery}
              onChange={(e) => setFilters((prev) => ({ ...prev, searchQuery: e.target.value }))}
              placeholder="Search by title or description..."
              className="w-full px-4 py-2.5 rounded-xl bg-white border border-neutral-light/60
                       text-neutral-dark placeholder:text-neutral-dark/40
                       focus:outline-none focus:border-primary/40 focus:ring-2 focus:ring-primary/10
                       transition-all duration-200"
            />
          </div>

          {/* Quick Filters */}
          <div className="space-y-2">
            <button
              onClick={() => toggleSection('quick')}
              className="w-full flex items-center justify-between text-sm font-semibold text-neutral-dark"
            >
              <span className="flex items-center gap-2">
                <Clock size={16} />
                Quick Filters
              </span>
              <ChevronDown
                size={16}
                className={`transition-transform duration-200 ${
                  expandedSections.quick ? 'rotate-180' : ''
                }`}
              />
            </button>
            {expandedSections.quick && (
              <div className="space-y-2 pt-2">
                <button
                  onClick={() => handleQuickFilterToggle('showOverdueOnly')}
                  className={`w-full px-4 py-2.5 rounded-xl text-sm font-medium text-left
                            transition-all duration-200 ${
                              filters.showOverdueOnly
                                ? 'bg-red-100 text-red-700 border-2 border-red-300'
                                : 'bg-white text-neutral-dark/70 border border-neutral-light/60 hover:bg-neutral-light/30'
                            }`}
                >
                  Overdue Tasks Only
                </button>
                <button
                  onClick={() => handleQuickFilterToggle('showCompletedToday')}
                  className={`w-full px-4 py-2.5 rounded-xl text-sm font-medium text-left
                            transition-all duration-200 ${
                              filters.showCompletedToday
                                ? 'bg-green-100 text-green-700 border-2 border-green-300'
                                : 'bg-white text-neutral-dark/70 border border-neutral-light/60 hover:bg-neutral-light/30'
                            }`}
                >
                  Completed Today
                </button>
              </div>
            )}
          </div>

          {/* Status Filter */}
          <div className="space-y-2">
            <button
              onClick={() => toggleSection('status')}
              className="w-full flex items-center justify-between text-sm font-semibold text-neutral-dark"
            >
              <span className="flex items-center gap-2">
                <AlertCircle size={16} />
                Status
              </span>
              <ChevronDown
                size={16}
                className={`transition-transform duration-200 ${
                  expandedSections.status ? 'rotate-180' : ''
                }`}
              />
            </button>
            {expandedSections.status && (
              <div className="grid grid-cols-2 gap-2 pt-2">
                {(['all', 'todo', 'doing', 'done'] as const).map((status) => (
                  <button
                    key={status}
                    onClick={() => handleStatusChange(status)}
                    className={`px-4 py-2.5 rounded-xl text-sm font-medium transition-all duration-200 ${
                      filters.status === status
                        ? status === 'all'
                          ? 'bg-neutral-dark text-white shadow-md'
                          : status === 'todo'
                          ? 'bg-blue-500 text-white shadow-md'
                          : status === 'doing'
                          ? 'bg-amber-500 text-white shadow-md'
                          : 'bg-emerald-500 text-white shadow-md'
                        : 'bg-white text-neutral-dark/70 border border-neutral-light/60 hover:bg-neutral-light/30'
                    }`}
                  >
                    {status === 'all' ? 'All' : status === 'todo' ? 'To Do' : status === 'doing' ? 'Doing' : 'Done'}
                  </button>
                ))}
              </div>
            )}
          </div>

          {/* Priority Filter */}
          <div className="space-y-2">
            <button
              onClick={() => toggleSection('priority')}
              className="w-full flex items-center justify-between text-sm font-semibold text-neutral-dark"
            >
              <span className="flex items-center gap-2">
                <AlertCircle size={16} />
                Priority
              </span>
              <ChevronDown
                size={16}
                className={`transition-transform duration-200 ${
                  expandedSections.priority ? 'rotate-180' : ''
                }`}
              />
            </button>
            {expandedSections.priority && (
              <div className="space-y-2 pt-2">
                {(['high', 'medium', 'low'] as TaskPriority[]).map((priority) => (
                  <button
                    key={priority}
                    onClick={() => handlePriorityToggle(priority)}
                    className={`w-full px-4 py-2.5 rounded-xl text-sm font-medium text-left
                              transition-all duration-200 flex items-center gap-2 ${
                                filters.priorities.includes(priority)
                                  ? priority === 'high'
                                    ? 'bg-red-100 text-red-700 border-2 border-red-300'
                                    : priority === 'medium'
                                    ? 'bg-amber-100 text-amber-700 border-2 border-amber-300'
                                    : 'bg-gray-100 text-gray-700 border-2 border-gray-300'
                                  : 'bg-white text-neutral-dark/70 border border-neutral-light/60 hover:bg-neutral-light/30'
                              }`}
                  >
                    <div
                      className={`w-4 h-4 rounded border-2 flex items-center justify-center ${
                        filters.priorities.includes(priority)
                          ? 'bg-current border-current'
                          : 'border-neutral-light'
                      }`}
                    >
                      {filters.priorities.includes(priority) && (
                        <svg
                          className="w-3 h-3 text-white"
                          fill="none"
                          viewBox="0 0 24 24"
                          stroke="currentColor"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={3}
                            d="M5 13l4 4L19 7"
                          />
                        </svg>
                      )}
                    </div>
                    {priority.charAt(0).toUpperCase() + priority.slice(1)} Priority
                  </button>
                ))}
              </div>
            )}
          </div>

          {/* Tags Filter */}
          {allTags.length > 0 && (
            <div className="space-y-2">
              <button
                onClick={() => toggleSection('tags')}
                className="w-full flex items-center justify-between text-sm font-semibold text-neutral-dark"
              >
                <span className="flex items-center gap-2">
                  <Tag size={16} />
                  Tags {filters.tags.length > 0 && `(${filters.tags.length})`}
                </span>
                <ChevronDown
                  size={16}
                  className={`transition-transform duration-200 ${
                    expandedSections.tags ? 'rotate-180' : ''
                  }`}
                />
              </button>
              {expandedSections.tags && (
                <div className="flex flex-wrap gap-2 pt-2">
                  {allTags.map((tag) => (
                    <button
                      key={tag}
                      onClick={() => handleTagToggle(tag)}
                      className={`px-3 py-1.5 rounded-full text-sm font-medium
                                transition-all duration-200 ${
                                  filters.tags.includes(tag)
                                    ? 'bg-primary text-white shadow-md'
                                    : 'bg-accent-peach text-primary-dark hover:bg-primary/20'
                                }`}
                    >
                      #{tag}
                    </button>
                  ))}
                </div>
              )}
            </div>
          )}

          {/* Deadline Range Filter */}
          <div className="space-y-2">
            <button
              onClick={() => toggleSection('deadline')}
              className="w-full flex items-center justify-between text-sm font-semibold text-neutral-dark"
            >
              <span className="flex items-center gap-2">
                <Calendar size={16} />
                Deadline Range
              </span>
              <ChevronDown
                size={16}
                className={`transition-transform duration-200 ${
                  expandedSections.deadline ? 'rotate-180' : ''
                }`}
              />
            </button>
            {expandedSections.deadline && (
              <div className="space-y-3 pt-2">
                <div>
                  <label className="text-xs text-neutral-dark/60 font-medium mb-1 block">
                    From
                  </label>
                  <input
                    type="date"
                    value={
                      filters.deadlineRange.start
                        ? new Date(filters.deadlineRange.start * 1000).toISOString().split('T')[0]
                        : ''
                    }
                    onChange={(e) => handleDeadlineRangeChange('start', e.target.value)}
                    className="w-full px-3 py-2 rounded-lg bg-white border border-neutral-light/60
                             text-neutral-dark text-sm
                             focus:outline-none focus:border-primary/40 focus:ring-2 focus:ring-primary/10
                             transition-all duration-200"
                  />
                </div>
                <div>
                  <label className="text-xs text-neutral-dark/60 font-medium mb-1 block">
                    To
                  </label>
                  <input
                    type="date"
                    value={
                      filters.deadlineRange.end
                        ? new Date(filters.deadlineRange.end * 1000).toISOString().split('T')[0]
                        : ''
                    }
                    onChange={(e) => handleDeadlineRangeChange('end', e.target.value)}
                    className="w-full px-3 py-2 rounded-lg bg-white border border-neutral-light/60
                             text-neutral-dark text-sm
                             focus:outline-none focus:border-primary/40 focus:ring-2 focus:ring-primary/10
                             transition-all duration-200"
                  />
                </div>
              </div>
            )}
          </div>
        </div>

        {/* Footer Actions */}
        <div className="px-6 py-4 border-t border-neutral-light/60 bg-white space-y-2">
          <button
            onClick={handleApply}
            className="w-full px-6 py-3 rounded-xl bg-gradient-to-r from-primary to-primary-dark
                     text-white font-semibold text-base
                     hover:shadow-lg hover:scale-[1.02] active:scale-[0.98]
                     transition-all duration-200"
          >
            Apply Filters
          </button>
          {activeCount > 0 && (
            <button
              onClick={handleReset}
              className="w-full px-6 py-2.5 rounded-xl bg-neutral-light/40 text-neutral-dark
                       font-medium text-sm hover:bg-neutral-light/60
                       transition-all duration-200"
            >
              Reset All Filters
            </button>
          )}
        </div>
      </div>
    </>
  );
}
