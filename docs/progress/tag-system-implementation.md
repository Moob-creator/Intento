# Tag System Implementation - Notion 风格标签系统

## 📅 日期
2024-03-12

## 🎯 实现内容

### 1. ✅ 日期时间选择器改进

**问题**: 原有的日期选择器只能选择到天，无法选择具体时间，且不够美观。

**解决方案**: 实现了一个全新的自定义日期时间选择器

**功能特性**:
- ✅ 支持日期选择
- ✅ 支持时间选择（30分钟间隔）
- ✅ 美观的下拉式界面
- ✅ 12小时制显示（12:00 AM, 12:30 AM, etc.）
- ✅ 快捷操作按钮：
  - "Now" - 设置为当前时间（向上取整到下一个30分钟）
  - "Tomorrow 9 AM" - 设置为明天上午9点
- ✅ 点击外部自动关闭
- ✅ 显示已选择的日期和时间
- ✅ "Clear" 按钮清除日期时间

**修改文件**:
- `src/components/TaskDetailPanel.tsx`

**代码变更**:
```typescript
// 新增状态
const [deadlineDate, setDeadlineDate] = useState('');
const [deadlineTime, setDeadlineTime] = useState('');
const [showDateTimePicker, setShowDateTimePicker] = useState(false);
const dateTimePickerRef = useRef<HTMLDivElement>(null);

// 48个时间选项（24小时 × 2）
Array.from({ length: 48 }, (_, i) => {
  const hours = Math.floor(i / 2);
  const minutes = (i % 2) * 30;
  const timeValue = `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}`;
  // ...
})
```

### 2. ✅ Notion 风格左侧边栏（Sidebar）

**问题**: 需要一个类似 Notion 的标签管理系统，能够展示所有 tags 并进行筛选。

**解决方案**: 创建了一个可折叠的左侧边栏组件

**功能特性**:
- ✅ 显示所有唯一的 tags
- ✅ 显示每个 tag 下的任务数量
- ✅ "All Tasks" 选项显示所有任务
- ✅ 点击 tag 筛选该 tag 的任务
- ✅ 可折叠/展开（类似 Notion）
- ✅ 选中状态高亮显示
- ✅ 空状态提示（当没有 tags 时）
- ✅ Hash (#) 图标标识 tags
- ✅ Inbox 图标标识 All Tasks

**新建文件**:
- `src/components/Sidebar.tsx`

**UI 结构**:
```
┌─────────────────────┐
│ Tags            [▼] │ ← Header with collapse button
├─────────────────────┤
│ [📥] All Tasks   12 │ ← All tasks option
│                     │
│ TAGS                │ ← Section header
│ [#] Work         5  │ ← Tag with count
│ [#] Personal     3  │
│ [#] Urgent       2  │
└─────────────────────┘
```

**宽度**:
- 展开: 256px (w-64)
- 折叠: 48px (w-12)

### 3. ✅ TaskDetailPanel 添加 Tags 输入

**问题**: 创建/编辑任务时需要能够添加 tags。

**解决方案**: 在 TaskDetailPanel 添加了 tags 输入功能

**功能特性**:
- ✅ Tag 输入框（类似 chips input）
- ✅ 按 Enter 添加 tag
- ✅ 按 Backspace 删除最后一个 tag
- ✅ 点击 X 按钮删除特定 tag
- ✅ Tag 显示带有 Hash (#) 图标
- ✅ Tag 带有主题色背景
- ✅ 防止重复 tag
- ✅ 自动去除空格
- ✅ 提示信息：「Press Enter to add a tag, Backspace to remove」

**修改文件**:
- `src/components/TaskDetailPanel.tsx`

**代码实现**:
```typescript
// 新增状态
const [tags, setTags] = useState<string[]>([]);
const [tagInput, setTagInput] = useState('');

// Tags UI
<div className="flex flex-wrap gap-2">
  {tags.map((tag, index) => (
    <span className="inline-flex items-center gap-1.5 px-2.5 py-1 bg-primary/10 text-primary rounded-lg">
      <Hash size={12} />
      {tag}
      <button onClick={() => setTags(tags.filter((_, i) => i !== index))}>
        <X size={12} />
      </button>
    </span>
  ))}
  <input
    onKeyDown={(e) => {
      if (e.key === 'Enter' && tagInput.trim()) {
        // Add tag
      } else if (e.key === 'Backspace' && !tagInput && tags.length > 0) {
        // Remove last tag
      }
    }}
  />
</div>
```

### 4. ✅ App.tsx 集成 Tag 筛选

**问题**: 需要在主应用中集成 tag 筛选逻辑。

**解决方案**: 更新了 App.tsx 的筛选逻辑和布局

**功能特性**:
- ✅ Tag 筛选状态管理
- ✅ Sidebar 折叠状态管理
- ✅ 同时支持状态筛选 + tag 筛选
- ✅ 标题动态显示当前选中的 tag
- ✅ 任务数量统计包含 tag 信息

**修改文件**:
- `src/App.tsx`

**代码变更**:
```typescript
// 新增状态
const [selectedTag, setSelectedTag] = useState<string | null>(null);
const [sidebarCollapsed, setSidebarCollapsed] = useState(false);

// 更新过滤逻辑
const sortedTasks = useMemo(() => {
  let filtered = tasks;

  // Filter by status
  if (statusFilter !== 'all') {
    filtered = filtered.filter(task => task.status === statusFilter);
  }

  // Filter by tag
  if (selectedTag) {
    filtered = filtered.filter(task => task.tags?.includes(selectedTag));
  }

  return [...filtered].sort(...);
}, [tasks, statusFilter, selectedTag]);

// 布局添加 Sidebar
<main className="flex-1 flex overflow-hidden">
  <Sidebar
    tasks={tasks}
    selectedTag={selectedTag}
    onTagSelect={setSelectedTag}
    isCollapsed={sidebarCollapsed}
    onToggleCollapse={() => setSidebarCollapsed(!sidebarCollapsed)}
  />
  {/* Task list section */}
</main>
```

## 📊 功能对比

### 日期时间选择器

**之前**:
```
[Due Date]
[____________________] ← type="date" 只能选择日期
```

**之后**:
```
[Due Date & Time]
┌──────────────────────────────────┐
│ 📅 Jan 15, 2024 at 14:30        │ ← 点击打开
└──────────────────────────────────┘
    ↓ 打开后
┌──────────────────────────────────┐
│ Date                             │
│ [2024-01-15]                     │
│                                  │
│ 🕐 Time (30-minute intervals)   │
│ [12:00 AM] [12:30 AM] [1:00 AM] │
│ [1:30 AM]  [2:00 AM]  [2:30 AM] │
│ ...                              │
│                                  │
│ [Now] [Tomorrow 9 AM]      [Done]│
└──────────────────────────────────┘
```

### Tag 系统

**之前**:
- ❌ 无标签系统
- ❌ 只能通过状态筛选

**之后**:
```
┌─────────────┬──────────────────────────────────┐
│ Tags    [▼] │ # Work                           │
│             │                                  │
│ 📥 All  12  │ All / To Do / Doing / Done       │
│             │ 5 tasks • Work                   │
│ TAGS        │                                  │
│ # Work   5  │ [Task 1 - #Work #Urgent]        │
│ # Personal 3│ [Task 2 - #Work]                │
│ # Urgent  2 │ [Task 3 - #Work #Important]     │
└─────────────┴──────────────────────────────────┘
```

## 🎨 视觉效果

### Sidebar 设计

**展开状态**:
- 宽度: 256px
- 圆角: rounded-lg
- 边框: border-neutral-light/60
- 背景: bg-background-card

**折叠状态**:
- 宽度: 48px
- 只显示折叠按钮（ChevronRight 图标）

**Tag 项目**:
- 选中: `bg-primary/10` + `border-l-2 border-primary`
- 未选中: `hover:bg-neutral-light/30`
- 图标: Hash (#) 16px
- 数量显示: 右侧灰色小字

### Tags 输入设计

**Tag Chip**:
- 背景: `bg-primary/10`
- 文字: `text-primary`
- 图标: Hash (#) 12px
- 删除按钮: X 图标，hover 变红色
- 圆角: `rounded-lg`
- 内边距: `px-2.5 py-1`

**输入框**:
- 背景: `bg-neutral-light/60`
- 聚焦: `focus-within:ring-2 ring-primary/30`
- 最小高度: 44px
- Flex wrap: 自动换行

## 🔄 用户交互流程

### 添加 Tag

1. 在 TaskDetailPanel 的 Tags 字段输入标签名
2. 按 Enter 键添加
3. Tag 显示为带有 # 图标的 chip
4. 可以继续添加更多 tags
5. 点击 X 删除不需要的 tag
6. 点击 Save 保存任务

### 使用 Tag 筛选

1. 查看左侧 Sidebar 的 tag 列表
2. 点击任意 tag（例如 "Work"）
3. 主内容区域标题显示 "# Work"
4. 任务列表只显示带有 "Work" tag 的任务
5. 可以继续使用状态筛选按钮（All/To Do/Doing/Done）
6. 两种筛选同时生效（AND 逻辑）
7. 点击 "All Tasks" 返回查看所有任务

### 折叠/展开 Sidebar

1. 点击 Sidebar 右上角的折叠按钮
2. Sidebar 折叠到 48px 宽度
3. 只显示一个展开按钮（ChevronRight）
4. 点击展开按钮恢复正常宽度
5. 状态保持在整个会话中

## 📝 技术细节

### Tag 数据结构

```typescript
// Task 类型已经包含 tags 字段
interface Task {
  id?: number;
  title: string;
  description?: string;
  status: "todo" | "doing" | "done";
  priority: "low" | "medium" | "high";
  deadline?: number; // Unix timestamp
  created_at: number;
  updated_at: number;
  completed_at?: number;
  tags?: string[]; // ✅ 已存在
}
```

### 筛选逻辑

```typescript
// 双重筛选：状态 + 标签
const sortedTasks = useMemo(() => {
  let filtered = tasks;

  // 1. 状态筛选
  if (statusFilter !== 'all') {
    filtered = filtered.filter(task => task.status === statusFilter);
  }

  // 2. 标签筛选
  if (selectedTag) {
    filtered = filtered.filter(task => task.tags?.includes(selectedTag));
  }

  return [...filtered].sort(...);
}, [tasks, statusFilter, selectedTag]);
```

### 标签提取

```typescript
// 从所有任务中提取唯一的标签
const allTags = Array.from(
  new Set(
    tasks.flatMap((task) => task.tags || [])
  )
).sort();

// 计算每个标签的任务数量
const getTagCount = (tag: string) => {
  return tasks.filter((task) => task.tags?.includes(tag)).length;
};
```

### 时间戳处理

```typescript
// 确保使用整数时间戳
deadline: deadlineDate && deadlineTime
  ? Math.floor(new Date(`${deadlineDate}T${deadlineTime}`).getTime() / 1000)
  : deadlineDate
  ? Math.floor(new Date(deadlineDate).getTime() / 1000)
  : undefined
```

## ✅ 编译状态

- ✅ **TypeScript**: 编译通过，无错误
- ✅ **Vite Build**: 构建成功
- ✅ **Bundle Size**: 273.97 KB (gzip: 78.82 KB)
- ✅ **CSS Size**: 32.52 KB (gzip: 6.34 KB)

## 🚀 使用示例

### 示例 1: 创建带标签的任务

1. 点击 "New Task" 或使用快捷键 ⌘N
2. 填写任务标题和描述
3. 在 Tags 字段输入 "Work"，按 Enter
4. 继续输入 "Urgent"，按 Enter
5. 选择日期时间
6. 点击 Save
7. 任务卡片显示 `#Work #Urgent` 标签

### 示例 2: 使用 Tag 筛选

1. 查看左侧 Sidebar
2. 看到 "Work (5)" - 表示有 5 个 Work 标签的任务
3. 点击 "Work"
4. 主标题变为 "# Work"
5. 任务列表只显示带 Work 标签的 5 个任务
6. 点击 "To Do" 按钮
7. 现在只显示状态为 To Do 且带 Work 标签的任务

### 示例 3: 选择日期和时间

1. 在任务编辑面板点击 "Select date and time..."
2. 在日期选择器中选择日期
3. 在时间网格中选择时间（例如 "2:30 PM"）
4. 或者点击 "Tomorrow 9 AM" 快捷按钮
5. 点击 "Done" 关闭选择器
6. 按钮显示 "Jan 16, 2024 at 09:00"

## 📈 改进总结

### 日期时间选择器
- ✅ 从只能选择日期 → 可选择具体到30分钟的时间
- ✅ 从简陋的 HTML input → 美观的自定义下拉选择器
- ✅ 新增快捷操作（Now, Tomorrow 9 AM）
- ✅ 新增清除功能
- ✅ 更好的视觉反馈

### Tag 系统
- ✅ 完整的标签管理系统（添加、删除、显示）
- ✅ Notion 风格的左侧边栏
- ✅ 可折叠的 Sidebar
- ✅ Tag 筛选功能
- ✅ Tag 数量统计
- ✅ 双重筛选（状态 + 标签）
- ✅ 美观的 UI 设计

### 用户体验
- ✅ 更强大的任务组织能力
- ✅ 更灵活的筛选方式
- ✅ 更直观的信息展示
- ✅ 类似 Notion 的熟悉交互
- ✅ 响应式布局支持

## 🎯 设计原则

### 1. 类 Notion 体验
- 左侧边栏可折叠
- Tag 图标统一使用 Hash (#)
- 选中状态高亮显示
- 简洁的设计语言

### 2. 信息架构
```
Sidebar (Tags) → Main Content (Filtered Tasks) → Detail Panel (Task Info)
     ↓                    ↓                              ↓
  Tag 选择           状态 + Tag 筛选                  Tag 编辑
```

### 3. 交互一致性
- 所有按钮使用统一的圆角和内边距
- 选中状态统一使用 primary 色
- Hover 效果统一使用透明度变化
- 图标大小统一（18px 或 16px）

---

**版本**: Tag System v1
**状态**: ✅ 完成并测试通过
**影响**: 布局 + 功能 + 用户体验
