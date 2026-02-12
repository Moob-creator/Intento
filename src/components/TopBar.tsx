import { Search, Sparkles, Settings, PanelLeftClose, PanelLeft, FileText } from 'lucide-react';

interface TopBarProps {
  onSearchClick: () => void;
  onAIClick: () => void;
  onSettingsClick: () => void;
  onSidebarToggle: () => void;
  onSummaryClick?: () => void;  // ✨ Phase 5
  sidebarCollapsed: boolean;
}

export function TopBar({ onSearchClick, onAIClick, onSettingsClick, onSidebarToggle, onSummaryClick, sidebarCollapsed }: TopBarProps) {
  return (
    <header
      data-tauri-drag-region
      className="h-14 flex items-center justify-between px-6 pl-20 border-b border-neutral-light/60 bg-white/80 backdrop-blur-sm"
    >
      {/* Left - Sidebar toggle button */}
      <button
        onClick={onSidebarToggle}
        className="p-2.5 text-neutral-dark/40 hover:text-neutral-dark hover:bg-neutral-light/40 rounded-lg transition-all duration-200"
        style={{ WebkitAppRegion: 'no-drag' } as React.CSSProperties}
        aria-label={sidebarCollapsed ? '展开侧栏' : '收起侧栏'}
        title={sidebarCollapsed ? '展开侧栏' : '收起侧栏'}
      >
        {sidebarCollapsed ? (
          <PanelLeft size={20} />
        ) : (
          <PanelLeftClose size={20} />
        )}
      </button>

      {/* Center - Search bar */}
      <button
        onClick={onSearchClick}
        className="flex-1 max-w-md mx-8 flex items-center gap-3 px-4 py-2 bg-neutral-light/30 hover:bg-neutral-light/50
                   rounded-lg transition-all duration-200 group"
        style={{ WebkitAppRegion: 'no-drag' } as React.CSSProperties}
      >
        <Search size={16} className="text-neutral-dark/40 group-hover:text-neutral-dark/60" />
        <span className="text-sm text-neutral-dark/40 group-hover:text-neutral-dark/60">
          Search tasks...
        </span>
        <kbd className="ml-auto px-2 py-0.5 text-xs font-medium text-neutral-dark/60 bg-white border border-neutral-light rounded">
          ⌘K
        </kbd>
      </button>

      {/* Right - Action buttons */}
      <div
        className="flex items-center gap-2"
        style={{ WebkitAppRegion: 'no-drag' } as React.CSSProperties}
      >
        {/* ✨ Phase 5: Summary button */}
        {onSummaryClick && (
          <button
            onClick={onSummaryClick}
            className="p-2.5 text-purple-500 hover:bg-purple-50 rounded-lg transition-all duration-200 group"
            aria-label="Summaries"
            title="View Summaries (⌘R)"
          >
            <FileText size={20} className="group-hover:scale-110 transition-transform duration-200" />
          </button>
        )}
        <button
          onClick={onAIClick}
          className="p-2.5 text-amber-500 hover:bg-amber-50 rounded-lg transition-all duration-200 group"
          aria-label="AI Add Task"
          title="AI Add Task (⌘/)"
        >
          <Sparkles size={20} className="group-hover:scale-110 transition-transform duration-200" />
        </button>
        <button
          onClick={onSettingsClick}
          className="p-2.5 text-neutral-dark/40 hover:text-neutral-dark hover:bg-neutral-light/40 rounded-lg transition-all duration-200"
          aria-label="Settings"
          title="Settings (⌘,)"
        >
          <Settings size={20} />
        </button>
      </div>
    </header>
  );
}
