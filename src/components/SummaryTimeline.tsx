import { useState } from 'react';
import { Clock, Calendar, Tag, Trash2, Eye } from 'lucide-react';
import type { Summary } from '../types/summary';

interface SummaryTimelineProps {
  summaries: Summary[];
  onSelectSummary: (summary: Summary) => void;
  onDeleteSummary: (id: number) => void;
  isLoading?: boolean;
}

export function SummaryTimeline({
  summaries,
  onSelectSummary,
  onDeleteSummary,
  isLoading = false,
}: SummaryTimelineProps) {
  const [hoveredId, setHoveredId] = useState<number | null>(null);

  // Format timestamp to readable date
  const formatDate = (timestamp: number): string => {
    const date = new Date(timestamp * 1000);
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  // Get type label with emoji
  const getTypeLabel = (type: string): string => {
    const labels: Record<string, string> = {
      daily: '📅 每日',
      weekly: '📆 每周',
      monthly: '🗓️ 每月',
      semi_annual: '📊 半年',
      yearly: '🎉 年度',
    };
    return labels[type] || type;
  };

  // Get type color
  const getTypeColor = (type: string): string => {
    const colors: Record<string, string> = {
      daily: 'bg-blue-100 text-blue-700',
      weekly: 'bg-green-100 text-green-700',
      monthly: 'bg-purple-100 text-purple-700',
      semi_annual: 'bg-orange-100 text-orange-700',
      yearly: 'bg-rose-100 text-rose-700',
    };
    return colors[type] || 'bg-neutral-light text-neutral-dark';
  };

  // Format period range
  const formatPeriodRange = (start: number, end: number): string => {
    const startDate = new Date(start * 1000);
    const endDate = new Date(end * 1000);
    return `${startDate.toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' })} - ${endDate.toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' })}`;
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center p-12">
        <div className="animate-pulse text-neutral-dark/40">
          <Clock size={32} className="mx-auto mb-3" />
          <p className="text-sm">加载历史总结...</p>
        </div>
      </div>
    );
  }

  if (summaries.length === 0) {
    return (
      <div className="flex items-center justify-center p-12">
        <div className="text-center text-neutral-dark/40">
          <Calendar size={48} className="mx-auto mb-4" />
          <p className="text-sm">暂无历史总结</p>
          <p className="text-xs mt-2">生成总结后将在此显示</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-3 p-4">
      {summaries.map((summary) => (
        <div
          key={summary.id}
          className="group relative bg-white rounded-xl border-2 border-neutral-light/60
                   hover:border-primary/40 hover:shadow-md transition-all duration-200
                   cursor-pointer"
          onMouseEnter={() => setHoveredId(summary.id || null)}
          onMouseLeave={() => setHoveredId(null)}
          onClick={() => onSelectSummary(summary)}
        >
          {/* Header */}
          <div className="flex items-start justify-between p-4 pb-3">
            <div className="flex items-center gap-3 flex-1">
              <span
                className={`px-3 py-1 rounded-full text-xs font-medium ${getTypeColor(summary.summary_type)}`}
              >
                {getTypeLabel(summary.summary_type)}
              </span>

              {summary.tag && (
                <div className="flex items-center gap-1.5 text-xs text-neutral-dark/60">
                  <Tag size={14} />
                  <span>{summary.tag}</span>
                </div>
              )}
            </div>

            {/* Actions */}
            <div
              className={`flex items-center gap-2 transition-opacity duration-200 ${
                hoveredId === summary.id ? 'opacity-100' : 'opacity-0'
              }`}
            >
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  onSelectSummary(summary);
                }}
                className="p-1.5 hover:bg-blue-50 rounded-lg transition-colors"
                aria-label="查看详情"
              >
                <Eye size={16} className="text-blue-500" />
              </button>
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  if (summary.id && confirm('确定删除这个总结吗？')) {
                    onDeleteSummary(summary.id);
                  }
                }}
                className="p-1.5 hover:bg-rose-50 rounded-lg transition-colors"
                aria-label="删除"
              >
                <Trash2 size={16} className="text-rose-500" />
              </button>
            </div>
          </div>

          {/* Period and Date */}
          <div className="px-4 pb-4">
            <div className="flex items-center gap-4 text-xs text-neutral-dark/60">
              <div className="flex items-center gap-1.5">
                <Calendar size={14} />
                <span>{formatPeriodRange(summary.period_start, summary.period_end)}</span>
              </div>
              <div className="flex items-center gap-1.5">
                <Clock size={14} />
                <span>{formatDate(summary.created_at)}</span>
              </div>
            </div>
          </div>

          {/* Preview - first line of content */}
          <div className="px-4 pb-4 border-t border-neutral-light/40 pt-3">
            <p className="text-sm text-neutral-dark/70 line-clamp-2">
              {summary.content
                .replace(/^#+\s+/gm, '')
                .replace(/\*\*/g, '')
                .split('\n')
                .find((line) => line.trim().length > 0) || '无内容预览'}
            </p>
          </div>
        </div>
      ))}
    </div>
  );
}
