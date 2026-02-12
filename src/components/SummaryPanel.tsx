import { useState, useEffect } from 'react';
import { FileText, X, Download, RefreshCw, Loader2 } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import type { Summary, SummaryType } from '../types/summary';
import { TimeRangeSelector } from './TimeRangeSelector';
import { SummaryContent } from './SummaryContent';
import { useTaskStore } from '../store/taskStore';

interface SummaryPanelProps {
  isOpen: boolean;
  onClose: () => void;
  selectedTag?: string | null;
  initialTimeRange?: SummaryType;
}

export function SummaryPanel({
  isOpen,
  onClose,
  selectedTag: initialTag,
  initialTimeRange = 'weekly',
}: SummaryPanelProps) {
  const { tasks } = useTaskStore();
  const [selectedTag, setSelectedTag] = useState<string | null>(initialTag || null);
  const [timeRange, setTimeRange] = useState<SummaryType>(initialTimeRange);
  const [currentSummary, setCurrentSummary] = useState<Summary | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Calculate period based on time range
  const getPeriod = (type: SummaryType): { start: number; end: number } => {
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());

    switch (type) {
      case 'daily':
        return {
          start: Math.floor(today.getTime() / 1000),
          end: Math.floor((today.getTime() + 86400000 - 1) / 1000),
        };
      case 'weekly': {
        const dayOfWeek = today.getDay();
        const monday = new Date(today);
        monday.setDate(today.getDate() - (dayOfWeek === 0 ? 6 : dayOfWeek - 1));
        return {
          start: Math.floor(monday.getTime() / 1000),
          end: Math.floor((monday.getTime() + 7 * 86400000 - 1) / 1000),
        };
      }
      case 'monthly': {
        const firstDay = new Date(now.getFullYear(), now.getMonth(), 1);
        const lastDay = new Date(now.getFullYear(), now.getMonth() + 1, 0, 23, 59, 59);
        return {
          start: Math.floor(firstDay.getTime() / 1000),
          end: Math.floor(lastDay.getTime() / 1000),
        };
      }
      case 'semi_annual': {
        const firstMonth = now.getMonth() < 6 ? 0 : 6;
        const firstDay = new Date(now.getFullYear(), firstMonth, 1);
        const lastDay = new Date(now.getFullYear(), firstMonth + 6, 0, 23, 59, 59);
        return {
          start: Math.floor(firstDay.getTime() / 1000),
          end: Math.floor(lastDay.getTime() / 1000),
        };
      }
      case 'yearly': {
        const firstDay = new Date(now.getFullYear(), 0, 1);
        const lastDay = new Date(now.getFullYear(), 11, 31, 23, 59, 59);
        return {
          start: Math.floor(firstDay.getTime() / 1000),
          end: Math.floor(lastDay.getTime() / 1000),
        };
      }
    }
  };

  // Load summary when tag or time range changes
  useEffect(() => {
    if (isOpen) {
      loadSummary();
    }
  }, [selectedTag, timeRange, isOpen]);

  const loadSummary = async () => {
    setIsLoading(true);
    setError(null);

    try {
      const period = getPeriod(timeRange);
      const summary = await invoke<Summary>('get_or_generate_summary', {
        tag: selectedTag,
        summaryType: timeRange,
        periodStart: period.start,
        periodEnd: period.end,
      });

      setCurrentSummary(summary);
    } catch (err) {
      console.error('Failed to load summary:', err);
      setError(err as string);
    } finally {
      setIsLoading(false);
    }
  };

  const handleTimeRangeChange = (newRange: SummaryType) => {
    setTimeRange(newRange);
  };

  const handleTagChange = (tag: string | null) => {
    setSelectedTag(tag);
  };

  const handleRegenerate = async () => {
    if (!currentSummary) return;

    const confirmed = confirm('重新生成此总结？之前的内容将丢失。');
    if (!confirmed) return;

    await loadSummary();
  };

  const handleExport = async () => {
    if (!currentSummary) return;

    try {
      const content = await invoke<string>('export_summary', {
        id: currentSummary.id,
        format: 'markdown',
      });

      // Create download link
      const blob = new Blob([content], { type: 'text/markdown' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `summary-${currentSummary.id}.md`;
      a.click();
      URL.revokeObjectURL(url);
    } catch (err) {
      console.error('Failed to export summary:', err);
      alert('导出失败: ' + err);
    }
  };

  if (!isOpen) return null;

  // Extract unique tags from tasks
  const allTags = Array.from(new Set(tasks.flatMap(task => task.tags || []))).sort();

  return (
    <aside
      className="fixed right-0 top-14 bottom-0 w-[480px] bg-background-card border-l border-neutral-light/60
                 flex flex-col shadow-2xl z-40 animate-slide-in"
    >
      {/* Header */}
      <div className="h-14 flex items-center justify-between px-6 border-b border-neutral-light/60">
        <div className="flex items-center gap-3">
          <FileText className="text-purple-500" size={20} />
          <h2 className="text-lg font-bold text-neutral-dark">总结</h2>
        </div>
        <button
          onClick={onClose}
          className="p-1.5 hover:bg-neutral-light/40 rounded-lg transition-colors"
          aria-label="关闭总结面板"
        >
          <X size={20} className="text-neutral-dark/60" />
        </button>
      </div>

      {/* Filter Bar */}
      <div className="p-4 border-b border-neutral-light/60 bg-neutral-light/20 space-y-3">
        {/* Tag Selector */}
        <div className="flex flex-col gap-1.5">
          <label className="text-xs font-medium text-neutral-dark/60 uppercase tracking-wide">
            标签
          </label>
          <select
            value={selectedTag || 'all'}
            onChange={(e) => handleTagChange(e.target.value === 'all' ? null : e.target.value)}
            className="w-full px-3 py-2 bg-white border border-neutral-light/60 rounded-lg
                     text-sm text-neutral-dark focus:outline-none focus:ring-2 focus:ring-primary/30
                     transition-all duration-200 cursor-pointer"
          >
            <option value="all">所有任务 ({tasks.length})</option>
            {allTags.map(tag => (
              <option key={tag} value={tag}>
                {tag} ({tasks.filter(t => t.tags?.includes(tag)).length})
              </option>
            ))}
          </select>
        </div>

        {/* Time Range Selector */}
        <TimeRangeSelector value={timeRange} onChange={handleTimeRangeChange} />
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto">
        {isLoading ? (
          <div className="flex items-center justify-center p-12">
            <div className="text-center">
              <Loader2 className="animate-spin text-primary mx-auto mb-3" size={32} />
              <p className="text-sm text-neutral-dark/60">正在生成总结...</p>
            </div>
          </div>
        ) : error ? (
          <div className="flex items-center justify-center p-12">
            <div className="text-center">
              <p className="text-sm text-red-500 mb-4">{error}</p>
              <button
                onClick={loadSummary}
                className="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary-dark"
              >
                重试
              </button>
            </div>
          </div>
        ) : currentSummary ? (
          <SummaryContent summary={currentSummary} />
        ) : (
          <div className="flex items-center justify-center p-12">
            <div className="text-center">
              <FileText size={48} className="text-neutral-dark/20 mx-auto mb-4" />
              <p className="text-sm text-neutral-dark/60 mb-4">
                此时段暂无总结
              </p>
              <button
                onClick={loadSummary}
                className="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary-dark"
              >
                生成总结
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Footer */}
      <div className="p-4 border-t border-neutral-light/60 flex gap-2">
        <button
          onClick={handleExport}
          disabled={!currentSummary}
          className="flex-1 flex items-center justify-center gap-2 px-4 py-2.5
                   bg-neutral-light/60 hover:bg-neutral-light rounded-lg
                   text-neutral-dark font-medium text-sm transition-colors
                   disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <Download size={16} />
          导出
        </button>
        <button
          onClick={handleRegenerate}
          disabled={!currentSummary || isLoading}
          className="flex-1 flex items-center justify-center gap-2 px-4 py-2.5
                   bg-primary hover:bg-primary-dark rounded-lg
                   text-white font-medium text-sm transition-colors
                   disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <RefreshCw size={16} />
          重新生成
        </button>
      </div>
    </aside>
  );
}
