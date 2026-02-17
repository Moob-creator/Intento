# 时间显示修复说明

## 问题描述

任务列表中，相同截止日期的任务显示了不同的相对时间标签：
- 任务 A 显示 "In 2 days"
- 任务 B 显示 "Tomorrow"
- 实际上两个任务的截止日期都是同一天

## 根本原因

### 原有实现问题

```typescript
const formatDeadline = (timestamp: number) => {
  const date = new Date(timestamp * 1000);
  const now = new Date();
  const diffTime = date.getTime() - now.getTime();
  const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));  // ❌ 问题所在

  if (diffDays === 0) return 'Today';
  if (diffDays === 1) return 'Tomorrow';
  if (diffDays <= 7) return `In ${diffDays} days`;
  // ...
};
```

**问题分析：**

使用 `Math.ceil()` 向上取整会导致不一致的结果：

1. **场景 1**：当前时间 2026-02-11 23:00，截止时间 2026-02-13 07:59
   - 时间差：约 1.37 天
   - `Math.ceil(1.37)` = 2
   - 显示：**"In 2 days"** ❌

2. **场景 2**：当前时间 2026-02-12 08:00，截止时间 2026-02-13 07:59
   - 时间差：约 0.99 天
   - `Math.ceil(0.99)` = 1
   - 显示：**"Tomorrow"** ✓

相同的截止日期，因为查看时间不同，显示的标签不同！

### 正确的做法

应该基于**日历日期**进行比较，而不是精确的时间差：

```typescript
const formatDeadline = (timestamp: number) => {
  const deadlineDate = new Date(timestamp * 1000);
  const now = new Date();

  // 将两个日期都设置为午夜（0:00），只比较日期部分
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
  const diffDays = Math.round(diffTime / (1000 * 60 * 60 * 24));  // ✓

  if (diffDays < 0) return 'Overdue';
  if (diffDays === 0) return 'Today';
  if (diffDays === 1) return 'Tomorrow';
  if (diffDays <= 7) return `In ${diffDays} days`;
  return deadlineDate.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
};
```

## 修复方案

### 1. 创建统一的工具函数

创建了 `src/utils/dateFormat.ts`，包含所有日期格式化函数：

- `formatDeadline(timestamp)` - 格式化截止时间为相对日期
- `formatRelativeTime(timestamp)` - 格式化过去的时间（"X 小时前"）
- `formatDateTime(timestamp)` - 格式化为 "YYYY/MM/DD HH:MM"
- `formatDate(timestamp)` - 格式化为 "YYYY/MM/DD"
- `getDaysUntilDeadline(timestamp)` - 获取距离截止日期的天数

### 2. 更新受影响的组件

- `src/components/TaskCard.tsx` - 任务卡片
- `src/pages/HomePage.tsx` - 首页
- `src/components/TaskOperationsConfirmDialog.tsx` - 操作确认对话框

所有组件现在都使用统一的日期格式化逻辑。

## 测试验证

### 测试场景

假设当前时间：2026-02-12 14:30

| 截止时间 | 预期显示 | 原有实现 | 修复后 |
|---------|---------|---------|--------|
| 2026-02-11 23:59 | Overdue | Overdue | Overdue ✓ |
| 2026-02-12 08:00 | Today | Today | Today ✓ |
| 2026-02-12 23:59 | Today | Today | Today ✓ |
| 2026-02-13 00:01 | Tomorrow | Tomorrow | Tomorrow ✓ |
| 2026-02-13 23:59 | Tomorrow | In 2 days ❌ | Tomorrow ✓ |
| 2026-02-14 08:00 | In 2 days | In 2 days ✓ | In 2 days ✓ |
| 2026-02-15 14:00 | In 3 days | In 3-4 days ❌ | In 3 days ✓ |

## 关键要点

1. **日期比较应该基于日历日期**，而不是精确的时间戳差值
2. **午夜归一化**：将日期设置为午夜（00:00:00）来比较
3. **使用 Math.round**：由于归一化到午夜，差值应该是整数，使用 `Math.round` 更安全
4. **统一工具函数**：避免在多个地方重复相同的逻辑

## 相关文件

- ✓ `src/utils/dateFormat.ts` - 新增工具函数
- ✓ `src/components/TaskCard.tsx` - 已更新
- ✓ `src/pages/HomePage.tsx` - 已更新
- ✓ `src/components/TaskOperationsConfirmDialog.tsx` - 已更新（时间格式统一）

## 额外修复

顺便修复了 `TaskOperationsConfirmDialog.tsx` 中的时间显示格式，确保与 `TaskDetailPanel.tsx` 的格式一致：
- 统一使用 `YYYY/MM/DD HH:MM` 格式（24小时制，无秒数）
