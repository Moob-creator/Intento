# Phase 5: 组件规格说明

## 组件架构图

```
src/components/
├── SummaryPanel.tsx          # 主面板容器
│   ├── SummaryHeader         # 头部（标题 + 关闭按钮）
│   ├── SummaryFilterBar      # 筛选栏（Tag + 时间范围）
│   ├── SummaryContent        # 内容展示区
│   └── SummaryFooter         # 底部操作按钮
│
├── TimeRangeSelector.tsx     # 时间范围选择器
├── TagSelector.tsx           # Tag 选择器
├── SummaryContent.tsx        # 总结内容渲染器
├── SummaryTimeline.tsx       # 历史总结时间线
└── ExportDialog.tsx          # 导出对话框
```

---

## 1. SummaryPanel (主面板)

### Props 接口

```typescript
interface SummaryPanelProps {
  isOpen: boolean;                    // 是否打开面板
  onClose: () => void;                // 关闭面板回调
  selectedTag?: string | null;        // 初始选中的 tag
  initialTimeRange?: SummaryType;     // 初始时间范围
  mode?: 'detail' | 'history';        // 展示模式
}
```

### 状态管理

```typescript
const [selectedTag, setSelectedTag] = useState<string | null>(null);
const [timeRange, setTimeRange] = useState<SummaryType>('weekly');
const [currentSummary, setCurrentSummary] = useState<Summary | null>(null);
const [isLoading, setIsLoading] = useState(false);
const [mode, setMode] = useState<'detail' | 'history'>('detail');
```

### 主要方法

```typescript
// 加载总结
const loadSummary = async (tag: string | null, type: SummaryType) => {
  setIsLoading(true);
  try {
    const summary = await invoke<Summary>('get_or_generate_summary', {
      tag,
      summaryType: type,
      periodStart: calculatePeriodStart(type),
      periodEnd: calculatePeriodEnd(type),
    });
    setCurrentSummary(summary);
  } catch (error) {
    toast.error('Failed to load summary');
  } finally {
    setIsLoading(false);
  }
};

// 重新生成总结
const regenerateSummary = async () => {
  if (!currentSummary) return;

  const confirmed = confirm('Regenerate this summary? Previous content will be lost.');
  if (!confirmed) return;

  setIsLoading(true);
  try {
    await invoke('delete_summary', { id: currentSummary.id });
    await loadSummary(selectedTag, timeRange);
    toast.success('Summary regenerated successfully');
  } catch (error) {
    toast.error('Failed to regenerate summary');
  } finally {
    setIsLoading(false);
  }
};

// 导出总结
const exportSummary = async (format: 'markdown' | 'text') => {
  if (!currentSummary) return;

  try {
    const content = await invoke<string>('export_summary', {
      id: currentSummary.id,
      format,
    });

    // 调用系统保存文件对话框
    const path = await save({
      defaultPath: `summary-${currentSummary.id}.${format === 'markdown' ? 'md' : 'txt'}`,
      filters: [{
        name: format === 'markdown' ? 'Markdown' : 'Text',
        extensions: [format === 'markdown' ? 'md' : 'txt'],
      }],
    });

    if (path) {
      await writeTextFile(path, content);
      toast.success(`Summary exported to ${path}`);
    }
  } catch (error) {
    toast.error('Failed to export summary');
  }
};
```

### 布局结构

```tsx
export function SummaryPanel({
  isOpen,
  onClose,
  selectedTag: initialTag,
  initialTimeRange = 'weekly',
  mode: initialMode = 'detail',
}: SummaryPanelProps) {
  // ... 状态和逻辑 ...

  if (!isOpen) return null;

  return (
    <aside
      className="fixed right-0 top-14 bottom-0 w-[480px] bg-background-card border-l border-neutral-light/60
                 flex flex-col shadow-2xl z-40 animate-slide-in"
    >
      {/* Header */}
      <div className="h-14 flex items-center justify-between px-6 border-b border-neutral-light/60">
        <div className="flex items-center gap-3">
          <FileText className="text-purple-500" size={20} />
          <h2 className="text-lg font-bold text-neutral-dark">Summary</h2>
        </div>
        <button
          onClick={onClose}
          className="p-1.5 hover:bg-neutral-light/40 rounded-lg transition-colors"
          aria-label="Close summary panel"
        >
          <X size={20} className="text-neutral-dark/60" />
        </button>
      </div>

      {/* Filter Bar */}
      {mode === 'detail' && (
        <div className="p-4 border-b border-neutral-light/60 bg-neutral-light/20 space-y-3">
          <TagSelector
            value={selectedTag}
            onChange={setSelectedTag}
            onTagChange={handleTagChange}
          />
          <TimeRangeSelector
            value={timeRange}
            onChange={handleTimeRangeChange}
          />
        </div>
      )}

      {/* Content */}
      <div className="flex-1 overflow-y-auto">
        {mode === 'detail' ? (
          isLoading ? (
            <LoadingState />
          ) : currentSummary ? (
            <SummaryContent summary={currentSummary} />
          ) : (
            <EmptyState onGenerate={() => loadSummary(selectedTag, timeRange)} />
          )
        ) : (
          <SummaryTimeline
            selectedTag={selectedTag}
            onSelectSummary={handleSelectHistorySummary}
          />
        )}
      </div>

      {/* Footer */}
      <div className="p-4 border-t border-neutral-light/60 flex gap-2">
        {mode === 'detail' ? (
          <>
            <button
              onClick={() => setShowExportDialog(true)}
              disabled={!currentSummary}
              className="flex-1 flex items-center justify-center gap-2 px-4 py-2.5
                       bg-neutral-light/60 hover:bg-neutral-light rounded-lg
                       text-neutral-dark font-medium text-sm transition-colors
                       disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <Download size={16} />
              Export
            </button>
            <button
              onClick={regenerateSummary}
              disabled={!currentSummary || isLoading}
              className="flex-1 flex items-center justify-center gap-2 px-4 py-2.5
                       bg-primary hover:bg-primary-dark rounded-lg
                       text-white font-medium text-sm transition-colors
                       disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <RefreshCw size={16} />
              Regenerate
            </button>
            <button
              onClick={() => setMode('history')}
              className="flex items-center justify-center gap-2 px-4 py-2.5
                       bg-neutral-light/60 hover:bg-neutral-light rounded-lg
                       text-neutral-dark font-medium text-sm transition-colors"
            >
              <History size={16} />
            </button>
          </>
        ) : (
          <button
            onClick={() => setMode('detail')}
            className="w-full flex items-center justify-center gap-2 px-4 py-2.5
                     bg-primary hover:bg-primary-dark rounded-lg
                     text-white font-medium text-sm transition-colors"
          >
            <ArrowLeft size={16} />
            Back to Current
          </button>
        )}
      </div>
    </aside>
  );
}
```

---

## 2. TimeRangeSelector (时间范围选择器)

### Props 接口

```typescript
interface TimeRangeSelectorProps {
  value: SummaryType;
  onChange: (value: SummaryType) => void;
}

type SummaryType = 'daily' | 'weekly' | 'monthly' | 'semi_annual' | 'yearly';
```

### 配置数据

```typescript
const TIME_RANGES = [
  { value: 'daily', label: 'Daily', icon: '📅', color: 'blue' },
  { value: 'weekly', label: 'Weekly', icon: '📆', color: 'green' },
  { value: 'monthly', label: 'Monthly', icon: '🗓️', color: 'purple' },
  { value: 'semi_annual', label: 'Semi-Annual', icon: '📊', color: 'orange' },
  { value: 'yearly', label: 'Yearly', icon: '🎉', color: 'red' },
] as const;
```

### 组件实现

```tsx
export function TimeRangeSelector({ value, onChange }: TimeRangeSelectorProps) {
  return (
    <div className="flex gap-2 overflow-x-auto pb-1">
      {TIME_RANGES.map((range) => (
        <button
          key={range.value}
          onClick={() => onChange(range.value)}
          className={`
            flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium
            transition-all duration-200 whitespace-nowrap
            ${value === range.value
              ? 'bg-primary text-white shadow-md'
              : 'bg-neutral-light/40 text-neutral-dark/70 hover:bg-neutral-light/60'
            }
          `}
        >
          <span>{range.icon}</span>
          <span>{range.label}</span>
        </button>
      ))}
    </div>
  );
}
```

---

## 3. TagSelector (Tag 选择器)

### Props 接口

```typescript
interface TagSelectorProps {
  value: string | null;
  onChange: (tag: string | null) => void;
  onTagChange: (tag: string | null) => void;
}
```

### 组件实现

```tsx
export function TagSelector({ value, onChange, onTagChange }: TagSelectorProps) {
  const { tasks } = useTaskStore();

  // 提取所有唯一的 tags
  const allTags = useMemo(() => {
    const tagSet = new Set<string>();
    tasks.forEach(task => {
      task.tags?.forEach(tag => tagSet.add(tag));
    });
    return Array.from(tagSet).sort();
  }, [tasks]);

  // 计算每个 tag 的任务数量
  const getTagCount = (tag: string) => {
    return tasks.filter(task => task.tags?.includes(tag)).length;
  };

  const handleChange = (newTag: string | null) => {
    onChange(newTag);
    onTagChange(newTag);
  };

  return (
    <div className="flex flex-col gap-1.5">
      <label className="text-xs font-medium text-neutral-dark/60 uppercase tracking-wide">
        Tag
      </label>
      <select
        value={value || 'all'}
        onChange={(e) => handleChange(e.target.value === 'all' ? null : e.target.value)}
        className="w-full px-3 py-2 bg-white border border-neutral-light/60 rounded-lg
                 text-sm text-neutral-dark focus:outline-none focus:ring-2 focus:ring-primary/30
                 transition-all duration-200 cursor-pointer"
      >
        <option value="all">All Tasks ({tasks.length})</option>
        {allTags.map(tag => (
          <option key={tag} value={tag}>
            {tag} ({getTagCount(tag)})
          </option>
        ))}
      </select>
    </div>
  );
}
```

---

## 4. SummaryContent (总结内容渲染器)

### Props 接口

```typescript
interface SummaryContentProps {
  summary: Summary;
}

interface Summary {
  id: number;
  summary_type: SummaryType;
  period_start: number;
  period_end: number;
  tag: string | null;
  content: string;        // Markdown 格式
  statistics: string;     // JSON 格式统计数据
  task_ids: number[];
  created_at: number;
}
```

### 组件实现

```tsx
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

export function SummaryContent({ summary }: SummaryContentProps) {
  const statistics = useMemo(() => {
    try {
      return JSON.parse(summary.statistics);
    } catch {
      return null;
    }
  }, [summary.statistics]);

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
            label="Completed"
            value={statistics.completed}
            total={statistics.total_tasks}
          />
          <StatCard
            icon={<Circle className="text-blue-500" />}
            label="In Progress"
            value={statistics.in_progress}
            total={statistics.total_tasks}
          />
          <StatCard
            icon={<TrendingUp className="text-purple-500" />}
            label="Completion Rate"
            value={`${(statistics.completion_rate * 100).toFixed(0)}%`}
          />
          <StatCard
            icon={<Target className="text-orange-500" />}
            label="High Priority"
            value={statistics.priority_distribution.high}
          />
        </div>
      )}

      {/* AI Generated Content */}
      <div className="prose prose-sm max-w-none">
        <ReactMarkdown
          remarkPlugins={[remarkGfm]}
          components={{
            h1: ({ children }) => (
              <h1 className="text-xl font-bold text-neutral-dark mb-3">{children}</h1>
            ),
            h2: ({ children }) => (
              <h2 className="text-lg font-semibold text-neutral-dark mb-2 mt-4">{children}</h2>
            ),
            h3: ({ children }) => (
              <h3 className="text-base font-semibold text-neutral-dark mb-2 mt-3">{children}</h3>
            ),
            p: ({ children }) => (
              <p className="text-sm text-neutral-dark/80 leading-relaxed mb-3">{children}</p>
            ),
            ul: ({ children }) => (
              <ul className="list-disc list-inside space-y-1 mb-3">{children}</ul>
            ),
            li: ({ children }) => (
              <li className="text-sm text-neutral-dark/80">{children}</li>
            ),
          }}
        >
          {summary.content}
        </ReactMarkdown>
      </div>

      {/* Meta Info */}
      <div className="mt-6 pt-4 border-t border-neutral-light/60 text-xs text-neutral-dark/40">
        Generated {formatRelativeTime(summary.created_at)} • {summary.task_ids.length} tasks included
      </div>
    </div>
  );
}

// 统计卡片组件
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
```

---

## 5. SummaryTimeline (历史总结时间线)

### Props 接口

```typescript
interface SummaryTimelineProps {
  selectedTag: string | null;
  onSelectSummary: (summary: Summary) => void;
}
```

### 组件实现

```tsx
export function SummaryTimeline({ selectedTag, onSelectSummary }: SummaryTimelineProps) {
  const [summaries, setSummaries] = useState<Summary[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [hasMore, setHasMore] = useState(true);
  const [page, setPage] = useState(0);

  const loadSummaries = async (pageNum: number) => {
    setIsLoading(true);
    try {
      const data = await invoke<Summary[]>('list_summaries', {
        tag: selectedTag,
        limit: 20,
        offset: pageNum * 20,
      });

      if (data.length < 20) {
        setHasMore(false);
      }

      setSummaries(prev => pageNum === 0 ? data : [...prev, ...data]);
    } catch (error) {
      toast.error('Failed to load summary history');
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    loadSummaries(0);
    setPage(0);
  }, [selectedTag]);

  const loadMore = () => {
    const nextPage = page + 1;
    setPage(nextPage);
    loadSummaries(nextPage);
  };

  if (summaries.length === 0 && !isLoading) {
    return (
      <div className="flex items-center justify-center p-12">
        <div className="text-center">
          <History size={48} className="text-neutral-dark/20 mx-auto mb-3" />
          <p className="text-sm text-neutral-dark/60">No summary history yet</p>
        </div>
      </div>
    );
  }

  return (
    <div className="p-4 space-y-3">
      {summaries.map(summary => (
        <button
          key={summary.id}
          onClick={() => onSelectSummary(summary)}
          className="w-full text-left p-4 bg-neutral-light/20 hover:bg-neutral-light/40
                   rounded-lg border border-neutral-light/60 transition-all duration-200
                   hover:shadow-md"
        >
          {/* Header */}
          <div className="flex items-center justify-between mb-2">
            <div className="flex items-center gap-2">
              {getSummaryTypeIcon(summary.summary_type)}
              <span className="text-xs font-semibold text-neutral-dark uppercase">
                {summary.summary_type}
              </span>
            </div>
            <span className="text-xs text-neutral-dark/60">
              {formatRelativeTime(summary.created_at)}
            </span>
          </div>

          {/* Period */}
          <div className="text-sm font-medium text-neutral-dark mb-1">
            {formatPeriodRange(summary.period_start, summary.period_end)}
          </div>

          {/* Tag */}
          {summary.tag && (
            <div className="flex items-center gap-1 mb-2">
              <Hash size={12} className="text-primary" />
              <span className="text-xs font-medium text-primary">{summary.tag}</span>
            </div>
          )}

          {/* Preview */}
          <p className="text-xs text-neutral-dark/60 line-clamp-2">
            {summary.content.substring(0, 120)}...
          </p>
        </button>
      ))}

      {/* Load More */}
      {hasMore && (
        <button
          onClick={loadMore}
          disabled={isLoading}
          className="w-full py-3 text-sm font-medium text-primary hover:bg-primary/10
                   rounded-lg transition-colors disabled:opacity-50"
        >
          {isLoading ? 'Loading...' : 'Load More'}
        </button>
      )}
    </div>
  );
}
```

---

## 6. ExportDialog (导出对话框)

### Props 接口

```typescript
interface ExportDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onExport: (format: 'markdown' | 'text') => Promise<void>;
}
```

### 组件实现

```tsx
export function ExportDialog({ isOpen, onClose, onExport }: ExportDialogProps) {
  const [format, setFormat] = useState<'markdown' | 'text'>('markdown');
  const [isExporting, setIsExporting] = useState(false);

  const handleExport = async () => {
    setIsExporting(true);
    try {
      await onExport(format);
      onClose();
    } finally {
      setIsExporting(false);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-white rounded-xl shadow-2xl w-[400px] p-6">
        <h3 className="text-lg font-bold text-neutral-dark mb-4">Export Summary</h3>

        <div className="space-y-3 mb-6">
          <label className="flex items-center gap-3 p-3 border border-neutral-light rounded-lg cursor-pointer hover:bg-neutral-light/20">
            <input
              type="radio"
              name="format"
              value="markdown"
              checked={format === 'markdown'}
              onChange={() => setFormat('markdown')}
              className="w-4 h-4 text-primary"
            />
            <div className="flex-1">
              <div className="font-medium text-sm">Markdown (.md)</div>
              <div className="text-xs text-neutral-dark/60">Preserves formatting and structure</div>
            </div>
          </label>

          <label className="flex items-center gap-3 p-3 border border-neutral-light rounded-lg cursor-pointer hover:bg-neutral-light/20">
            <input
              type="radio"
              name="format"
              value="text"
              checked={format === 'text'}
              onChange={() => setFormat('text')}
              className="w-4 h-4 text-primary"
            />
            <div className="flex-1">
              <div className="font-medium text-sm">Plain Text (.txt)</div>
              <div className="text-xs text-neutral-dark/60">Simple text format</div>
            </div>
          </label>
        </div>

        <div className="flex gap-3">
          <button
            onClick={onClose}
            disabled={isExporting}
            className="flex-1 px-4 py-2.5 bg-neutral-light/60 hover:bg-neutral-light rounded-lg
                     text-neutral-dark font-medium text-sm transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={handleExport}
            disabled={isExporting}
            className="flex-1 px-4 py-2.5 bg-primary hover:bg-primary-dark rounded-lg
                     text-white font-medium text-sm transition-colors
                     disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isExporting ? 'Exporting...' : 'Export'}
          </button>
        </div>
      </div>
    </div>
  );
}
```

---

## 辅助函数

```typescript
// 格式化总结类型
function formatSummaryType(type: SummaryType): string {
  const labels = {
    daily: 'Daily Summary',
    weekly: 'Weekly Summary',
    monthly: 'Monthly Summary',
    semi_annual: 'Semi-Annual Summary',
    yearly: 'Yearly Summary',
  };
  return labels[type];
}

// 格式化时间范围
function formatPeriodRange(start: number, end: number): string {
  const startDate = new Date(start * 1000);
  const endDate = new Date(end * 1000);

  return `${startDate.toLocaleDateString('zh-CN')} - ${endDate.toLocaleDateString('zh-CN')}`;
}

// 格式化相对时间
function formatRelativeTime(timestamp: number): string {
  const now = Date.now();
  const diff = now - timestamp * 1000;

  const minutes = Math.floor(diff / 60000);
  const hours = Math.floor(diff / 3600000);
  const days = Math.floor(diff / 86400000);

  if (minutes < 60) return `${minutes} minutes ago`;
  if (hours < 24) return `${hours} hours ago`;
  if (days < 7) return `${days} days ago`;
  if (days < 30) return `${Math.floor(days / 7)} weeks ago`;
  return `${Math.floor(days / 30)} months ago`;
}

// 获取总结类型图标
function getSummaryTypeIcon(type: SummaryType): React.ReactNode {
  const icons = {
    daily: <span>📅</span>,
    weekly: <span>📆</span>,
    monthly: <span>🗓️</span>,
    semi_annual: <span>📊</span>,
    yearly: <span>🎉</span>,
  };
  return icons[type];
}
```
