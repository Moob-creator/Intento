// Phase 5: Summary type definitions

export type SummaryType = 'daily' | 'weekly' | 'monthly' | 'semi_annual' | 'yearly';

export interface Summary {
  id: number;
  summary_type: SummaryType;
  period_start: number;        // Unix timestamp
  period_end: number;          // Unix timestamp
  tag: string | null;
  tag_filter: string[] | null;
  content: string;             // Markdown format
  statistics: string;          // JSON string
  task_ids: number[];
  created_at: number;          // Unix timestamp
  is_deleted: boolean;
}

export interface SummaryStatistics {
  total_tasks: number;
  completed: number;
  in_progress: number;
  todo: number;
  completion_rate: number;
  priority_distribution: PriorityDistribution;
  time_stats?: TimeStats;
}

export interface PriorityDistribution {
  high: number;
  medium: number;
  low: number;
}

export interface TimeStats {
  avg_completion_time_hours?: number;
  overdue_count: number;
}

// Helper: Parse statistics string to object
export function parseSummaryStatistics(summary: Summary): SummaryStatistics | null {
  try {
    return JSON.parse(summary.statistics);
  } catch {
    return null;
  }
}

// Helper: Format period range
export function formatPeriodRange(start: number, end: number): string {
  const startDate = new Date(start * 1000);
  const endDate = new Date(end * 1000);

  const options: Intl.DateTimeFormatOptions = {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  };

  return `${startDate.toLocaleDateString('zh-CN', options)} - ${endDate.toLocaleDateString('zh-CN', options)}`;
}

// Helper: Format relative time
export function formatRelativeTime(timestamp: number): string {
  const now = Date.now();
  const diff = now - timestamp * 1000;

  const minutes = Math.floor(diff / 60000);
  const hours = Math.floor(diff / 3600000);
  const days = Math.floor(diff / 86400000);

  if (minutes < 1) return '刚刚';
  if (minutes < 60) return `${minutes} 分钟前`;
  if (hours < 24) return `${hours} 小时前`;
  if (days < 7) return `${days} 天前`;
  if (days < 30) return `${Math.floor(days / 7)} 周前`;
  if (days < 365) return `${Math.floor(days / 30)} 个月前`;
  return `${Math.floor(days / 365)} 年前`;
}

// Helper: Format summary type display name
export function formatSummaryType(type: SummaryType): string {
  const labels = {
    daily: '每日总结',
    weekly: '每周总结',
    monthly: '每月总结',
    semi_annual: '半年总结',
    yearly: '年度总结',
  };
  return labels[type];
}

// Helper: Get summary type icon
export function getSummaryTypeIcon(type: SummaryType): string {
  const icons = {
    daily: '📅',
    weekly: '📆',
    monthly: '🗓️',
    semi_annual: '📊',
    yearly: '🎉',
  };
  return icons[type];
}
