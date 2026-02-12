import { useEffect, useRef } from 'react';
import { FileText, History } from 'lucide-react';

interface ContextMenuProps {
  x: number;
  y: number;
  onClose: () => void;
  onGenerateSummary: () => void;
  onViewHistory: () => void;
  tag: string;
}

export function ContextMenu({
  x,
  y,
  onClose,
  onGenerateSummary,
  onViewHistory,
  tag,
}: ContextMenuProps) {
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        onClose();
      }
    };

    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        onClose();
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    document.addEventListener('keydown', handleEscape);

    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
      document.removeEventListener('keydown', handleEscape);
    };
  }, [onClose]);

  // Adjust position if menu would go off-screen
  useEffect(() => {
    if (menuRef.current) {
      const rect = menuRef.current.getBoundingClientRect();
      const viewportWidth = window.innerWidth;
      const viewportHeight = window.innerHeight;

      let adjustedX = x;
      let adjustedY = y;

      if (rect.right > viewportWidth) {
        adjustedX = viewportWidth - rect.width - 10;
      }

      if (rect.bottom > viewportHeight) {
        adjustedY = viewportHeight - rect.height - 10;
      }

      menuRef.current.style.left = `${adjustedX}px`;
      menuRef.current.style.top = `${adjustedY}px`;
    }
  }, [x, y]);

  return (
    <div
      ref={menuRef}
      className="fixed z-50 min-w-[200px] bg-white border border-neutral-light/60 rounded-lg shadow-xl py-1
                 animate-fade-in"
      style={{ left: x, top: y }}
    >
      {/* Tag Header */}
      <div className="px-3 py-2 border-b border-neutral-light/40">
        <span className="text-xs font-semibold text-neutral-dark/60">
          #{tag}
        </span>
      </div>

      {/* Menu Items */}
      <button
        onClick={() => {
          onGenerateSummary();
          onClose();
        }}
        className="w-full flex items-center gap-3 px-3 py-2.5 text-left hover:bg-purple-50
                   text-neutral-dark transition-colors"
      >
        <FileText size={16} className="text-purple-500" />
        <span className="text-sm font-medium">生成总结</span>
      </button>

      <button
        onClick={() => {
          onViewHistory();
          onClose();
        }}
        className="w-full flex items-center gap-3 px-3 py-2.5 text-left hover:bg-neutral-light/40
                   text-neutral-dark transition-colors"
      >
        <History size={16} className="text-neutral-dark/60" />
        <span className="text-sm font-medium">查看历史</span>
      </button>
    </div>
  );
}
