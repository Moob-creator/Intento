# Phase 5: 用户交互流程设计

## 用户故事

### 故事 1: 查看今日工作总结
```
作为一名用户
我想查看今天完成的任务总结
以便了解今日的工作成果和待办事项
```

**交互流程:**
1. 用户按下 `⌘K` 打开 Command Palette
2. 输入 "today" 或 "summary"
3. 选择 "View Today's Summary"
4. 右侧滑出 SummaryPanel，展示今日总结
5. 查看 AI 生成的总结文本和统计数据

### 故事 2: 为特定 Tag 生成周总结
```
作为一名用户
我想为 "work" tag 生成本周总结
以便回顾本周的工作进展
```

**交互流程:**
1. 用户在 Sidebar 中找到 "work" tag
2. 右键点击 "work" tag
3. 在弹出菜单中选择 "📊 Generate Summary"
4. 弹出时间范围选择对话框
5. 选择 "Weekly"（本周）
6. 系统生成总结，SummaryPanel 滑出展示内容

### 故事 3: 浏览历史总结
```
作为一名用户
我想查看过去的总结记录
以便回顾历史工作情况和成长轨迹
```

**交互流程:**
1. 用户打开 SummaryPanel
2. 点击 "View History" 按钮
3. 展开历史总结时间线
4. 滚动浏览不同时期的总结
5. 点击某个历史总结查看详情

### 故事 4: 配置自动总结
```
作为一名用户
我想开启每日和每周自动总结
以便每天早上查看昨日成果
```

**交互流程:**
1. 用户按下 `⌘,` 打开设置面板
2. 滚动到 "Summaries" 部分
3. 勾选 "Enable automatic summaries"
4. 勾选 "Daily" 和 "Weekly" 频率
5. 设置保留时长为 6 个月
6. 关闭设置，系统开始定时生成总结

## 主要交互入口

### 入口 1: Sidebar Tag 右键菜单（主入口）

**设计理由:**
- Tag 是总结的主体，从 Tag 触发最符合用户心智模型
- 右键菜单不占用主界面空间
- 与现有 Sidebar 设计语言一致

**交互细节:**
```
┌─ Sidebar ─────────────┐
│ Tags                  │
│                       │
│ # All Tasks      (12) │
│ # work           (5)  │ ← 右键点击
│ # personal       (3)  │
│ # study          (4)  │
└───────────────────────┘

右键菜单：
┌──────────────────────────┐
│ 📊 Generate Summary      │ ← 新增
│ 📅 View Summary History  │ ← 新增
│ ─────────────────────── │
│ 🏷️  Rename Tag          │
│ 🎨 Customize Color       │
│ 🗑️  Delete Tag          │
└──────────────────────────┘
```

**点击 "Generate Summary" 后:**
1. 弹出时间范围选择对话框
2. 用户选择时间周期（日/周/月/半年/年）
3. 系统检查是否已有该时期的总结
   - 如果存在：直接展示
   - 如果不存在：生成新总结
4. SummaryPanel 滑出展示总结内容

### 入口 2: Command Palette（快捷入口）

**设计理由:**
- 符合现有 Command Palette 的设计语言
- 提供快速访问常用总结（今日、本周）
- 支持模糊搜索，输入效率高

**新增命令列表:**
```typescript
// 今日总结
{
  id: 'summary-today',
  label: 'View Today's Summary',
  icon: <FileText className="text-purple-500" size={18} />,
  keywords: ['summary', 'report', 'daily', 'today', '今天', '总结'],
  action: () => openSummaryPanel('daily', 'today'),
}

// 本周总结
{
  id: 'summary-week',
  label: 'View This Week's Summary',
  icon: <Calendar className="text-blue-500" size={18} />,
  keywords: ['summary', 'weekly', 'week', '本周', '周报'],
  action: () => openSummaryPanel('weekly', 'this-week'),
}

// 本月总结
{
  id: 'summary-month',
  label: 'View This Month's Summary',
  icon: <Calendar className="text-green-500" size={18} />,
  keywords: ['summary', 'monthly', 'month', '本月', '月报'],
  action: () => openSummaryPanel('monthly', 'this-month'),
}

// 查看历史
{
  id: 'summary-history',
  label: 'View Summary History',
  icon: <History className="text-gray-500" size={18} />,
  keywords: ['summary', 'history', '历史', '记录'],
  action: () => openSummaryPanel(null, 'history'),
}
```

### 入口 3: TopBar 快捷按钮（可选）

**设计理由:**
- 提供一键访问总结功能
- 与 AI、Settings 按钮并列，保持一致性
- 新用户更容易发现功能

**位置和样式:**
```tsx
// 在 TopBar.tsx 的 Right Actions 区域添加
<button
  onClick={() => setShowSummaryPanel(true)}
  className="p-2.5 text-purple-500 hover:bg-purple-50 rounded-lg transition-all duration-200"
  aria-label="Summaries"
  title="View Summaries (⌘R)"
>
  <FileText size={20} />
</button>
```

**快捷键绑定:**
- `⌘R` (Mac) / `Ctrl+R` (Windows/Linux) - 打开总结面板

## 总结面板交互

### 面板布局

```
┌─────────────────────────────────────────┐
│ 📊 Summary                         [X]  │ ← Header
├─────────────────────────────────────────┤
│ Tag: [All Tasks ▼]                      │ ← Filter Bar
│ [📅 Daily][📆 Weekly][🗓️ Monthly]...     │
├─────────────────────────────────────────┤
│                                         │
│ ## 🎉 本周工作总结                       │
│                                         │
│ 本周你完成了 12 个任务，完成率 80%...    │ ← Content
│                                         │
│ ### ✨ 亮点成就                          │
│ - 完成了高优先级项目 A                   │
│                                         │
│ [统计图表]                               │
│                                         │
├─────────────────────────────────────────┤
│ [⬇️ Export] [🔄 Regenerate] [📜 History]│ ← Footer
└─────────────────────────────────────────┘
```

### 交互细节

#### Tag 选择器
```tsx
<select
  value={selectedTag || 'all'}
  onChange={(e) => handleTagChange(e.target.value)}
  className="w-full px-3 py-2 rounded-lg border border-neutral-light"
>
  <option value="all">All Tags</option>
  <option value="work">🏢 work (5 tasks)</option>
  <option value="personal">🏠 personal (3 tasks)</option>
  <option value="study">📚 study (4 tasks)</option>
</select>
```

**交互行为:**
- 切换 Tag 时自动重新加载该 Tag 的总结
- 如果该 Tag + 时间范围的总结不存在，显示 "Generate" 按钮

#### 时间范围选择器
```tsx
<div className="flex gap-2 overflow-x-auto">
  {timeRanges.map(range => (
    <button
      key={range.value}
      onClick={() => setTimeRange(range.value)}
      className={`px-4 py-2 rounded-lg text-sm font-medium ${
        timeRange === range.value
          ? 'bg-primary text-white'
          : 'bg-neutral-light/40 text-neutral-dark/70 hover:bg-neutral-light/60'
      }`}
    >
      {range.icon} {range.label}
    </button>
  ))}
</div>
```

**时间范围选项:**
- 📅 Daily - 每日
- 📆 Weekly - 每周
- 🗓️ Monthly - 每月
- 📊 Semi-Annual - 每半年
- 🎉 Yearly - 每年

**交互行为:**
- 点击时自动加载对应时间范围的总结
- 当前选中的按钮高亮显示
- 如果总结不存在，显示 loading 状态并开始生成

#### 总结内容区

**加载状态:**
```tsx
{isLoading ? (
  <div className="flex items-center justify-center p-12">
    <div className="text-center">
      <Loader2 className="animate-spin text-primary mx-auto mb-3" size={32} />
      <p className="text-sm text-neutral-dark/60">Generating summary...</p>
    </div>
  </div>
) : (
  <SummaryContent data={summary} />
)}
```

**空状态:**
```tsx
{!summary && !isLoading && (
  <div className="text-center p-12">
    <FileText size={48} className="text-neutral-dark/20 mx-auto mb-4" />
    <p className="text-sm text-neutral-dark/60 mb-4">
      No summary available for this period
    </p>
    <button
      onClick={generateSummary}
      className="px-4 py-2 bg-primary text-white rounded-lg"
    >
      Generate Summary
    </button>
  </div>
)}
```

#### Footer 按钮

**Export 导出:**
- 点击后弹出格式选择对话框（Markdown / Plain Text）
- 调用系统保存文件对话框
- 保存成功后显示通知

**Regenerate 重新生成:**
- 点击后显示确认对话框
- 确认后删除当前总结，重新生成新的
- 适用于：任务数据有更新，想要刷新总结

**History 历史记录:**
- 点击后切换到历史浏览模式
- 展开时间线列表

## 历史总结浏览

### Timeline 视图

```
┌─────────────────────────────────────────┐
│ 📜 Summary History              [Back]  │
├─────────────────────────────────────────┤
│ [All Tags ▼] [All Types ▼]              │
├─────────────────────────────────────────┤
│                                         │
│ ┌─────────────────────────────────────┐ │
│ │ 📅 Daily • 2024-02-11                │ │
│ │ work tag                   2 days ago│ │
│ │ 完成了 5 个任务，完成率 100%...       │ │
│ └─────────────────────────────────────┘ │
│                                         │
│ ┌─────────────────────────────────────┐ │
│ │ 📆 Weekly • 2024-02-05 ~ 2024-02-11 │ │
│ │ All Tasks                   1 week ago│ │
│ │ 本周完成了 12 个任务...               │ │
│ └─────────────────────────────────────┘ │
│                                         │
│ ┌─────────────────────────────────────┐ │
│ │ 🗓️ Monthly • 2024-01                 │ │
│ │ personal tag              1 month ago│ │
│ │ 一月份完成了 30 个个人任务...         │ │
│ └─────────────────────────────────────┘ │
│                                         │
│              [Load More]                │
│                                         │
└─────────────────────────────────────────┘
```

### 交互行为

**点击历史记录卡片:**
- 切换回详情视图
- 显示该历史总结的完整内容

**筛选器:**
- Tag 筛选：只显示特定 tag 的总结
- Type 筛选：只显示特定时间周期的总结

**分页加载:**
- 初始加载最近 20 条记录
- 滚动到底部自动加载更多
- 或点击 "Load More" 按钮手动加载

## 键盘快捷键

| 快捷键 | 功能 |
|--------|------|
| `⌘K` → "summary" | 快速搜索总结相关命令 |
| `⌘R` | 打开/关闭总结面板 |
| `⌘,` | 打开设置（配置自动总结） |
| `ESC` | 关闭总结面板 |
| `⌘E` | 导出当前总结 |
| `⌘⇧R` | 重新生成总结 |

## 状态和反馈

### 成功状态
- ✅ 总结生成成功 → Toast 通知 "Summary generated successfully"
- ✅ 导出成功 → Toast 通知 "Summary exported to [path]"

### 错误状态
- ❌ 生成失败 → Toast 通知 "Failed to generate summary: [error]"
- ❌ 网络错误 → Toast 通知 "Network error, please try again"
- ❌ 无任务数据 → 显示提示 "No tasks found in this period"

### 加载状态
- 🔄 生成中 → Spinner + "Generating summary..."
- 🔄 导出中 → Button disabled + "Exporting..."

## 动画和过渡

### 面板滑入/滑出
```css
/* 滑入动画 */
@keyframes slideIn {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

/* 滑出动画 */
@keyframes slideOut {
  from {
    transform: translateX(0);
    opacity: 1;
  }
  to {
    transform: translateX(100%);
    opacity: 0;
  }
}
```

### 内容淡入
- 切换 Tag 或时间范围时，内容区淡入效果
- 历史记录加载时，逐个淡入动画

## 响应式设计

### 窗口宽度 < 1024px
- SummaryPanel 改为全屏覆盖模式
- 添加返回按钮关闭面板

### 窗口宽度 < 768px
- 时间范围选择器改为下拉菜单
- Footer 按钮改为垂直堆叠

## 可访问性

- 所有按钮都有 `aria-label`
- 键盘导航支持（Tab / Shift+Tab）
- 焦点可见性指示
- Screen reader 友好的语义化标签
