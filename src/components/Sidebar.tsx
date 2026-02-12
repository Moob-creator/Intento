import { useState } from 'react';
import { ChevronDown, Hash, Inbox, Package } from 'lucide-react';
import type { Task } from '../types/task';
import { ContextMenu } from './ContextMenu';

interface SidebarProps {
  tasks: Task[];
  selectedTag: string | null;
  onTagSelect: (tag: string | null) => void;
  isCollapsed: boolean;
  onToggleCollapse: () => void;
  onGenerateSummary?: (tag: string) => void;
  onViewSummaryHistory?: (tag: string) => void;
}

export function Sidebar({ tasks, selectedTag, onTagSelect, isCollapsed, onToggleCollapse, onGenerateSummary, onViewSummaryHistory }: SidebarProps) {
  const [contextMenu, setContextMenu] = useState<{ tag: string; x: number; y: number } | null>(null);

  // Extract all unique tags from tasks
  const allTags = Array.from(
    new Set(
      tasks.flatMap((task) => task.tags || [])
    )
  ).sort();

  // Count tasks per tag
  const getTagCount = (tag: string) => {
    return tasks.filter((task) => task.tags?.includes(tag)).length;
  };

  // Handle right-click on tag
  const handleTagRightClick = (e: React.MouseEvent, tag: string) => {
    e.preventDefault();
    setContextMenu({ tag, x: e.clientX, y: e.clientY });
  };

  if (isCollapsed) {
    return null;
  }

  return (
    <aside className="w-64 flex-shrink-0 border-r border-neutral-light/60 bg-background-card flex flex-col">
      {/* Header */}
      <div className="h-14 flex items-center justify-between px-4 border-b border-neutral-light/60">
        <div className="flex items-center gap-2">
          <Package size={20} className="text-primary flex-shrink-0" />
          <h1 className="text-base font-bold text-neutral-dark">Intento</h1>
        </div>
        <button
          onClick={onToggleCollapse}
          className="p-1.5 text-neutral-dark/40 hover:text-neutral-dark hover:bg-neutral-light/40 rounded-lg transition-all duration-200"
          title="Collapse sidebar"
        >
          <ChevronDown size={18} className="rotate-[-90deg]" />
        </button>
      </div>

      {/* Tag List */}
      <div className="flex-1 overflow-y-auto py-3">
        {/* All Tasks */}
        <button
          onClick={() => onTagSelect(null)}
          className={`w-full flex items-center gap-3 px-4 py-2.5 text-left transition-all duration-150 ${
            selectedTag === null
              ? 'bg-primary/10 border-l-2 border-primary text-primary'
              : 'hover:bg-neutral-light/30 text-neutral-dark'
          }`}
        >
          <Inbox size={18} className="flex-shrink-0" />
          <span className="text-sm font-medium flex-1">All Tasks</span>
          <span className="text-xs text-neutral-dark/40">
            {tasks.length}
          </span>
        </button>

        {/* Tags */}
        {allTags.length > 0 && (
          <div className="mt-4">
            <div className="px-4 py-2 text-xs font-semibold text-neutral-dark/40 uppercase tracking-wide">
              Tags
            </div>
            {allTags.map((tag) => (
              <button
                key={tag}
                onClick={() => onTagSelect(tag)}
                onContextMenu={(e) => handleTagRightClick(e, tag)}
                className={`w-full flex items-center gap-3 px-4 py-2.5 text-left transition-all duration-150 ${
                  selectedTag === tag
                    ? 'bg-primary/10 border-l-2 border-primary text-primary'
                    : 'hover:bg-neutral-light/30 text-neutral-dark'
                }`}
              >
                <Hash size={16} className="flex-shrink-0" />
                <span className="text-sm font-medium flex-1 truncate">{tag}</span>
                <span className="text-xs text-neutral-dark/40">
                  {getTagCount(tag)}
                </span>
              </button>
            ))}
          </div>
        )}

        {/* Empty state */}
        {allTags.length === 0 && (
          <div className="px-4 py-8 text-center">
            <Hash size={32} className="mx-auto text-neutral-dark/20 mb-2" />
            <p className="text-xs text-neutral-dark/40">
              No tags yet
            </p>
            <p className="text-xs text-neutral-dark/30 mt-1">
              Add tags to your tasks to organize them
            </p>
          </div>
        )}
      </div>

      {/* Context Menu */}
      {contextMenu && (
        <ContextMenu
          x={contextMenu.x}
          y={contextMenu.y}
          tag={contextMenu.tag}
          onClose={() => setContextMenu(null)}
          onGenerateSummary={() => {
            if (onGenerateSummary) {
              onGenerateSummary(contextMenu.tag);
            }
          }}
          onViewHistory={() => {
            if (onViewSummaryHistory) {
              onViewSummaryHistory(contextMenu.tag);
            }
          }}
        />
      )}
    </aside>
  );
}
