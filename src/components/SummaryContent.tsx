import { useMemo } from 'react';
import { Calendar, CheckCircle, Circle, TrendingUp, Target, Hash } from 'lucide-react';
import type { Summary } from '../types/summary';
import { parseSummaryStatistics, formatPeriodRange, formatRelativeTime, formatSummaryType } from '../types/summary';

interface SummaryContentProps {
  summary: Summary;
}

export function SummaryContent({ summary }: SummaryContentProps) {
  const statistics = useMemo(() => parseSummaryStatistics(summary), [summary.statistics]);

  return (
    <div className="p-6">
      {/* Period Header */}
      <div className="mb-6 pb-4 border-b border-neutral-light/60">
        <div className="flex items-center gap-2 mb-2">
          <Calendar size={16} className="text-primary" />
          <span className="text-xs font-medium text-neutral-dark/60 uppercase tracking-wide">
            {formatSummaryType(summary.summary_type)}
          </span>
        </div>
        <h3 className="text-lg font-bold text-neutral-dark">
          {formatPeriodRange(summary.period_start, summary.period_end)}
        </h3>
        {summary.tag && (
          <div className="flex items-center gap-1.5 mt-2">
            <Hash size={14} className="text-primary" />
            <span className="text-sm font-medium text-primary">{summary.tag}</span>
          </div>
        )}
      </div>

      {/* Statistics Cards */}
      {statistics && (
        <div className="grid grid-cols-2 gap-3 mb-6">
          <StatCard
            icon={<CheckCircle className="text-green-500" />}
            label="已完成"
            value={statistics.completed}
            total={statistics.total_tasks}
          />
          <StatCard
            icon={<Circle className="text-blue-500" />}
            label="进行中"
            value={statistics.in_progress}
            total={statistics.total_tasks}
          />
          <StatCard
            icon={<TrendingUp className="text-purple-500" />}
            label="完成率"
            value={`${(statistics.completion_rate * 100).toFixed(0)}%`}
          />
          <StatCard
            icon={<Target className="text-orange-500" />}
            label="高优先级"
            value={statistics.priority_distribution.high}
          />
        </div>
      )}

      {/* AI Generated Content */}
      <div className="prose prose-sm max-w-none">
        <div
          className="summary-content text-sm text-neutral-dark/80 leading-relaxed"
          dangerouslySetInnerHTML={{ __html: formatMarkdown(summary.content) }}
        />
      </div>

      {/* Meta Info */}
      <div className="mt-6 pt-4 border-t border-neutral-light/60 text-xs text-neutral-dark/40">
        生成于 {formatRelativeTime(summary.created_at)} • 包含 {summary.task_ids?.length || 0} 个任务
      </div>
    </div>
  );
}

// Stat card component
function StatCard({ icon, label, value, total }: {
  icon: React.ReactNode;
  label: string;
  value: number | string;
  total?: number;
}) {
  return (
    <div className="p-3 bg-neutral-light/30 rounded-lg">
      <div className="flex items-center gap-2 mb-1">
        {icon}
        <span className="text-xs font-medium text-neutral-dark/60">{label}</span>
      </div>
      <div className="text-2xl font-bold text-neutral-dark">
        {value}
        {total && <span className="text-sm font-normal text-neutral-dark/40">/{total}</span>}
      </div>
    </div>
  );
}

// Simple markdown formatting (basic support)
function formatMarkdown(content: string): string {
  return content
    // Headers
    .replace(/^### (.*$)/gim, '<h3 class="text-base font-semibold text-neutral-dark mb-2 mt-3">$1</h3>')
    .replace(/^## (.*$)/gim, '<h2 class="text-lg font-semibold text-neutral-dark mb-2 mt-4">$1</h2>')
    .replace(/^# (.*$)/gim, '<h1 class="text-xl font-bold text-neutral-dark mb-3">$1</h1>')
    // Bold
    .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
    // Lists
    .replace(/^\- (.*$)/gim, '<li class="ml-4">$1</li>')
    // Paragraphs
    .replace(/\n\n/g, '</p><p class="mb-3">')
    // Wrap in paragraph
    .replace(/^(.+)$/gim, '<p class="mb-3">$1</p>')
    // Clean up empty paragraphs
    .replace(/<p class="mb-3"><\/p>/g, '');
}
