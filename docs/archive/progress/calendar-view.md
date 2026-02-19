# 日历视图功能实现文档

> 实现时间：2026-02-12
> 版本：v0.1.0
> 状态：月视图已完成，周视图待实现

---

## 📋 功能概述

日历视图是 Intento 的任务可视化功能，让用户以日历形式查看和管理任务。采用**方案 C（混合方案）**：
- **月视图**：快速浏览整月任务分布
- **周视图**（待实现）：查看近期任务详情

---

## ✅ 已完成功能（月视图）

### 1. 核心组件

#### CalendarView.tsx
**路径**: `src/components/CalendarView.tsx`
**代码量**: ~350 lines

**功能特性**:
- 7×6 日历网格布局（42 天，确保显示完整月份）
- 周一到周日标题行
- 月份导航（上一月、下一月、Today 按钮）
- 基于任务 `deadline` 字段过滤和显示
- 支持标签筛选（selectedTag prop）

**核心逻辑**:
```typescript
// 生成日历网格
const calendarGrid = useMemo(() => {
  const year = currentDate.getFullYear();
  const month = currentDate.getMonth();

  // 从周一开始计算
  const firstDay = new Date(year, month, 1);
  const firstDayOfWeek = firstDay.getDay();
  const daysToSubtract = firstDayOfWeek === 0 ? 6 : firstDayOfWeek - 1;

  // 生成 42 天（6 周）
  const grid: DayCell[] = [];
  for (let i = 0; i < 42; i++) {
    // 计算日期和任务...
  }

  return grid;
}, [currentDate, tasks, selectedTag]);
```

### 2. 日期单元格（DayCell）

**显示元素**:
1. **日期数字** - 当前月/非当前月区分颜色
2. **优先级指示点** - 红/黄/蓝代表 high/medium/low
3. **任务数量角标** - 超过 3 个任务时显示数字
4. **今天高亮** - 紫色边框（ring-2 ring-primary）
5. **逾期标记** - 红色边框（ring-2 ring-red-400）

**交互逻辑**:
- 点击有任务的日期打开任务抽屉
- Hover 效果：放大（scale-105）+ 阴影

```typescript
const hasOverdue = tasks.some(
  t => t.deadline && t.deadline < now && t.status !== 'done'
);
```

### 3. 任务抽屉（Drawer）

**设计**:
- 从底部滑入（animate-slide-up）
- 最大高度 70vh，内容可滚动
- 半透明背景遮罩（backdrop-blur-sm）

**内容显示**:
- 日期标题（格式：February 12, 2026）
- 任务卡片列表，按优先级着色：
  - High: `bg-red-50 border-red-400`
  - Medium: `bg-amber-50 border-amber-400`
  - Low: `bg-blue-50 border-blue-400`
- 每个任务显示：标题、描述、状态、标签

**交互**:
- 点击任务卡片打开 TaskDetailPanel
- 点击背景或关闭按钮关闭抽屉

### 4. 无截止日期任务区域

**位置**: 日历下方，用边框分隔
**显示**: 横向排列的任务按钮
**特点**:
- 显示优先级指示点
- 任务标题截断（truncate）
- 最大宽度 max-w-xs

```typescript
const noDeadlineTasks = useMemo(() => {
  return tasks.filter(task => {
    if (!task.deadline) {
      if (selectedTag && !task.tags?.includes(selectedTag)) return false;
      return true;
    }
    return false;
  });
}, [tasks, selectedTag]);
```

### 5. TopBar 视图切换

**修改文件**: `src/components/TopBar.tsx`

**新增 Props**:
```typescript
type ViewMode = 'list' | 'calendar';

interface TopBarProps {
  viewMode?: ViewMode;
  onViewModeChange?: (mode: ViewMode) => void;
}
```

**UI 实现**:
- 搜索框右侧添加切换按钮组
- List 图标和 Calendar 图标
- Toggle 样式：选中时白色背景 + 阴影
- 未选中时灰色 + hover 效果

### 6. App.tsx 集成

**状态管理**:
```typescript
const [viewMode, setViewMode] = useState<ViewMode>('list');
```

**条件渲染**:
```typescript
{viewMode === 'list' ? (
  <TaskList ... />
) : (
  <CalendarView
    tasks={tasks}
    onTaskClick={selectTask}
    selectedTag={selectedTag}
  />
)}
```

**注意**:
- Quick filter buttons 只在 list 模式显示
- Calendar 模式使用完整 tasks 数组（不应用 statusFilter 和 timeFilter）
- Tag 筛选对两种视图都生效

---

## 🎨 设计规范

### 颜色主题
- **主色**: `#A78BFA` (紫色，primary)
- **高优先级**: `#F87171` (红色，red-400)
- **中优先级**: `#FBBF24` (黄色，amber-400)
- **低优先级**: `#60A5FA` (蓝色，blue-400)
- **逾期**: `#F87171` (红色边框)

### 圆角和阴影
- 日期单元格: `rounded-xl`
- 任务抽屉: `rounded-t-3xl`
- 阴影: hover 时 `shadow-lg`

### 动画
- 抽屉滑入: `animate-slide-up`（App.css 已定义）
- 背景遮罩: `animate-fade-in`
- 单元格 hover: `scale-105` + `duration-200`

---

## 📋 待实现功能

### Phase 6.6: 日历视图增强

#### 1. 周视图 (Task 6.6.1)
**优先级**: P2
**预计时间**: 1-2 天

**功能需求**:
- [ ] 创建 WeekView 组件
- [ ] 显示当前周（周一到周日）
- [ ] 每天一列显示任务卡片
- [ ] 支持左右箭头切换周
- [ ] 任务条显示：优先级着色 + 标题截断
- [ ] 最多显示 5 个任务，超出可滚动
- [ ] TopBar 添加第三个切换按钮（List/Month/Week）

**UI 布局草图**:
```
┌─────────────────────────────────────────────┐
│  Week of Feb 12 - Feb 18, 2026   [< Today >]│
├───────┬───────┬───────┬───────┬───────┬─────┤
│  Mon  │  Tue  │  Wed  │  Thu  │  Fri  │ Sat │
│   12  │   13  │   14  │   15  │   16  │  17 │
├───────┼───────┼───────┼───────┼───────┼─────┤
│ ▬ 任务1│ ▬ 任务3│       │ ▬ 任务5│       │     │
│ ▬ 任务2│ ▬ 任务4│       │       │       │     │
│       │+2 more│       │       │       │     │
└───────┴───────┴───────┴───────┴───────┴─────┘
```

**技术实现**:
```typescript
// WeekView.tsx
const weekStart = useMemo(() => {
  const date = new Date(currentDate);
  const day = date.getDay();
  const diff = day === 0 ? -6 : 1 - day; // Monday = 1
  date.setDate(date.getDate() + diff);
  return date;
}, [currentDate]);

const weekDays = useMemo(() => {
  const days = [];
  for (let i = 0; i < 7; i++) {
    const date = new Date(weekStart);
    date.setDate(weekStart.getDate() + i);

    const dayTasks = tasks.filter(task => {
      if (!task.deadline) return false;
      const taskDate = new Date(task.deadline * 1000);
      return taskDate.toDateString() === date.toDateString();
    });

    days.push({ date, tasks: dayTasks });
  }
  return days;
}, [weekStart, tasks]);
```

---

#### 2. 任务拖拽排期 (Task 6.6.2)
**优先级**: P2
**预计时间**: 1 天

**功能需求**:
- [ ] 日历单元格支持拖放
- [ ] 拖拽任务到新日期自动更新 deadline
- [ ] 拖拽预览效果（半透明 ghost）
- [ ] 拖拽取消（拖到外部区域）

**技术方案**:
```typescript
// 使用原生 HTML5 Drag & Drop API
const handleDragStart = (e: DragEvent, task: Task) => {
  e.dataTransfer.effectAllowed = 'move';
  e.dataTransfer.setData('application/json', JSON.stringify({
    taskId: task.id,
    taskTitle: task.title
  }));

  // 自定义拖拽预览
  const dragImage = document.createElement('div');
  dragImage.className = 'opacity-75 bg-white p-2 rounded-lg shadow-lg';
  dragImage.textContent = task.title;
  document.body.appendChild(dragImage);
  e.dataTransfer.setDragImage(dragImage, 0, 0);
  setTimeout(() => document.body.removeChild(dragImage), 0);
};

const handleDragOver = (e: DragEvent) => {
  e.preventDefault();
  e.dataTransfer.dropEffect = 'move';
};

const handleDrop = async (e: DragEvent, targetDate: Date) => {
  e.preventDefault();
  const data = JSON.parse(e.dataTransfer.getData('application/json'));

  const newDeadline = Math.floor(targetDate.getTime() / 1000);
  await updateTask(data.taskId, { deadline: newDeadline });

  showToast(`任务已移动到 ${targetDate.toLocaleDateString()}`, 'success');
};
```

**视觉反馈**:
- 拖拽源：`opacity-50`
- 拖拽目标高亮：`ring-2 ring-primary`
- 无效目标：`cursor-not-allowed`

---

#### 3. 日历批量操作 (Task 6.6.3)
**优先级**: P3
**预计时间**: 1 天

**功能需求**:
- [ ] Shift + 点击选择日期范围
- [ ] 右键菜单：批量完成、批量移动、批量删除
- [ ] 确认对话框（防止误操作）
- [ ] 支持撤销（可选）

**实现思路**:
```typescript
const [selectedDates, setSelectedDates] = useState<Date[]>([]);
const [lastClickedDate, setLastClickedDate] = useState<Date | null>(null);

const handleDateClick = (date: Date, shiftKey: boolean) => {
  if (shiftKey && lastClickedDate) {
    // 计算范围
    const range = getDateRange(lastClickedDate, date);
    setSelectedDates(range);
  } else {
    setSelectedDates([date]);
    setLastClickedDate(date);
  }
};

const handleBatchComplete = async () => {
  const tasksInRange = tasks.filter(task =>
    selectedDates.some(date => isSameDay(date, task.deadline))
  );

  if (confirm(`确认完成 ${tasksInRange.length} 个任务？`)) {
    for (const task of tasksInRange) {
      await updateTask(task.id, {
        status: 'done',
        completed_at: Math.floor(Date.now() / 1000)
      });
    }
    showToast(`已完成 ${tasksInRange.length} 个任务`, 'success');
  }
};
```

---

#### 4. 日历导出 (Task 6.6.4)
**优先级**: P3
**预计时间**: 1 天

**功能需求**:
- [ ] 导出为图片（PNG/JPG）
- [ ] 导出为 iCalendar 格式（.ics）
- [ ] 选择日期范围导出
- [ ] 自定义样式（颜色主题）

**技术方案**:

**图片导出**（使用 html2canvas）:
```typescript
import html2canvas from 'html2canvas';

const handleExportImage = async () => {
  const calendarElement = document.getElementById('calendar-view');
  if (!calendarElement) return;

  const canvas = await html2canvas(calendarElement, {
    backgroundColor: '#f9fafb',
    scale: 2, // 高清
  });

  const link = document.createElement('a');
  link.download = `intento-calendar-${new Date().toISOString().split('T')[0]}.png`;
  link.href = canvas.toDataURL('image/png');
  link.click();

  showToast('日历已导出为图片', 'success');
};
```

**iCalendar 导出**:
```typescript
const generateICS = (tasks: Task[]): string => {
  let ics = [
    'BEGIN:VCALENDAR',
    'VERSION:2.0',
    'PRODID:-//Intento//Calendar//EN',
    'CALSCALE:GREGORIAN',
  ];

  tasks.forEach(task => {
    if (!task.deadline) return;

    const date = new Date(task.deadline * 1000);
    const dateStr = date.toISOString().replace(/[-:]/g, '').split('.')[0] + 'Z';

    ics.push(
      'BEGIN:VEVENT',
      `UID:${task.id}@intento.app`,
      `DTSTAMP:${dateStr}`,
      `DTSTART:${dateStr}`,
      `SUMMARY:${task.title}`,
      `DESCRIPTION:${task.description || ''}`,
      `STATUS:${task.status === 'done' ? 'COMPLETED' : 'NEEDS-ACTION'}`,
      `PRIORITY:${task.priority === 'high' ? '1' : task.priority === 'medium' ? '5' : '9'}`,
      'END:VEVENT'
    );
  });

  ics.push('END:VCALENDAR');
  return ics.join('\r\n');
};

const handleExportICS = () => {
  const icsContent = generateICS(tasks);
  const blob = new Blob([icsContent], { type: 'text/calendar' });
  const link = document.createElement('a');
  link.download = `intento-tasks-${new Date().toISOString().split('T')[0]}.ics`;
  link.href = URL.createObjectURL(blob);
  link.click();

  showToast('任务已导出为 iCalendar 格式', 'success');
};
```

---

## 🐛 已知问题

### 当前问题
- [ ] 跨月任务显示可能有边界问题（需要更多测试）
- [ ] 大量任务（>50/天）时单元格可能拥挤

### 性能优化
- [ ] 虚拟滚动优化（历史月份加载）
- [ ] 任务抽屉懒加载
- [ ] useMemo 优化计算密集型操作

---

## 📊 代码统计

### 新增文件
| 文件 | 代码量 | 说明 |
|------|--------|------|
| `src/components/CalendarView.tsx` | ~350 lines | 月视图主组件 |

### 修改文件
| 文件 | 新增代码 | 说明 |
|------|---------|------|
| `src/components/TopBar.tsx` | +30 lines | 视图切换按钮 |
| `src/App.tsx` | +20 lines | 视图模式状态和条件渲染 |
| `src/components/Sidebar.tsx` | -1 line | 移除未使用 import |
| `src/components/SummaryContent.tsx` | -1 line | 移除未使用 import |

**总计**: ~400 lines

---

## 🎯 使用指南

### 基本操作

1. **切换到日历视图**
   - 点击 TopBar 搜索框右侧的日历图标
   - 或在设置中设置默认视图

2. **导航月份**
   - 点击左右箭头切换月份
   - 点击 "Today" 快速回到当前月

3. **查看任务**
   - 点击有任务的日期打开抽屉
   - 在抽屉中点击任务卡片查看详情

4. **按标签筛选**
   - 在 Sidebar 选择标签
   - 日历视图自动过滤任务

5. **查看无截止日期任务**
   - 滚动到日历下方
   - 点击任务按钮查看详情

### 视觉提示

| 元素 | 含义 |
|------|------|
| 紫色边框 | 今天 |
| 红色边框 | 有逾期任务 |
| 红点 | 高优先级任务 |
| 黄点 | 中优先级任务 |
| 蓝点 | 低优先级任务 |
| (5) | 该日有 5 个任务 |
| 灰色日期 | 非当前月 |

---

## 🚀 未来规划

### v0.2.0 (2 周内)
- ✅ 月视图（已完成）
- [ ] 周视图
- [ ] 任务拖拽排期

### v0.3.0 (1 个月内)
- [ ] 日历批量操作
- [ ] 日历导出功能
- [ ] 性能优化

### v1.0.0 (正式版)
- [ ] 移动端适配
- [ ] 触摸手势支持
- [ ] 日历小部件（macOS Widget）

---

## 📝 开发笔记

### 技术选择

**为什么不用第三方日历库？**
- 现有库（react-big-calendar, fullcalendar）过于复杂
- 自定义样式困难，与 Warm & Soft 主题不匹配
- Bundle 体积大（~100KB+）
- 自实现可控性更强，代码量仅 350 行

**周一作为起始日的原因？**
- 符合国际标准（ISO 8601）
- 更符合中国用户习惯
- 工作周的自然开始

### 性能考虑

**为什么用 42 天（6 周）网格？**
- 确保显示完整月份
- 避免动态高度导致的布局跳动
- 提供一致的视觉体验

**useMemo 优化点**:
- `calendarGrid` - 避免每次渲染重新计算
- `noDeadlineTasks` - 任务筛选缓存

---

## 🔗 相关文档

- [Phase 5 进度文档](../src-tauri/docs/phase5/PROGRESS.md)
- [开发待办清单](./TODO.md)
- [开发计划](../../specs/development-plan.md)
- [设计规范](../specs/ui-ux-design.md)

---

**文档维护者**: @wangshuo
**最后更新**: 2026-02-12
**实现状态**: 月视图已完成，周视图待实现
