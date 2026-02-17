/**
 * Date and time formatting utilities
 */

/**
 * Format a deadline timestamp as a relative time string
 * @param timestamp - Unix timestamp in seconds
 * @returns Relative time string (e.g., "Today", "Tomorrow", "In 2 days", "Overdue")
 */
export function formatDeadline(timestamp: number, includeOverdue: boolean = true): string {
  const deadlineDate = new Date(timestamp * 1000);
  const now = new Date();

  // Set both dates to midnight for accurate day comparison
  const deadlineMidnight = new Date(
    deadlineDate.getFullYear(),
    deadlineDate.getMonth(),
    deadlineDate.getDate()
  );
  const todayMidnight = new Date(
    now.getFullYear(),
    now.getMonth(),
    now.getDate()
  );

  const diffTime = deadlineMidnight.getTime() - todayMidnight.getTime();
  const diffDays = Math.round(diffTime / (1000 * 60 * 60 * 24));

  if (diffDays < 0 && includeOverdue) return 'Overdue';
  if (diffDays === 0) return 'Today';
  if (diffDays === 1) return 'Tomorrow';
  if (diffDays <= 7) return `In ${diffDays} days`;
  return deadlineDate.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
}

/**
 * Format a timestamp as a relative time string (for past dates)
 * @param timestamp - Unix timestamp in seconds
 * @returns Relative time string (e.g., "刚刚", "5 分钟前", "2 小时前")
 */
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

/**
 * Format a date for display in detail panels
 * @param timestamp - Unix timestamp in seconds
 * @returns Formatted date string (e.g., "2026/02/13 07:59")
 */
export function formatDateTime(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  const formattedDate = date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit'
  });
  const hours = date.getHours().toString().padStart(2, '0');
  const minutes = date.getMinutes().toString().padStart(2, '0');
  return `${formattedDate} ${hours}:${minutes}`;
}

/**
 * Format a date (no time) for display
 * @param timestamp - Unix timestamp in seconds
 * @returns Formatted date string (e.g., "2026/02/13")
 */
export function formatDate(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit'
  });
}

/**
 * Get the number of days between today and a deadline
 * @param timestamp - Unix timestamp in seconds
 * @returns Number of days (negative if overdue, 0 for today, positive for future)
 */
export function getDaysUntilDeadline(timestamp: number): number {
  const deadlineDate = new Date(timestamp * 1000);
  const now = new Date();

  const deadlineMidnight = new Date(
    deadlineDate.getFullYear(),
    deadlineDate.getMonth(),
    deadlineDate.getDate()
  );
  const todayMidnight = new Date(
    now.getFullYear(),
    now.getMonth(),
    now.getDate()
  );

  const diffTime = deadlineMidnight.getTime() - todayMidnight.getTime();
  return Math.round(diffTime / (1000 * 60 * 60 * 24));
}
