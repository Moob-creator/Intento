# 通知系统工作原理

## 📋 目录
1. [系统架构](#系统架构)
2. [核心工作流程](#核心工作流程)
3. [时间线示例](#时间线示例)
4. [关键问题解答](#关键问题解答)
5. [测试验证](#测试验证)

---

## 系统架构

```
┌─────────────────────────────────────────────────────────────┐
│                     应用启动（main.rs）                      │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ├─→ 初始化数据库
                       ├─→ 创建 TaskScheduler
                       └─→ 启动后台调度器
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              TaskScheduler（每5分钟执行）                    │
│  Cron: "0 */5 * * * *"                                      │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│         db.get_tasks_needing_reminder()                     │
│  查询条件：                                                  │
│  - reminder_time IS NOT NULL                                │
│  - reminder_time <= now（已到提醒时间）                      │
│  - reminder_time > now - 5分钟（容错窗口）                   │
│  - status != 'done'                                         │
│  - is_deleted = 0                                           │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│               遍历需要提醒的任务                             │
│  ├─→ 构建通知内容                                           │
│  ├─→ 发送系统通知                                           │
│  └─→ db.clear_reminder(task_id) ← 关键！防止重复           │
└─────────────────────────────────────────────────────────────┘
```

---

## 核心工作流程

### 1. 创建任务时（task.rs:create_task）

```rust
用户创建任务
  ↓
设置 deadline = 2026-02-17 17:30
  ↓
自动计算 reminder_time：
  ├─ 如果距离截止时间 > 1小时
  │  → reminder_time = deadline - 1小时
  │  → 例如：17:30到期 → 16:30提醒
  │
  ├─ 如果距离截止时间 5分钟~1小时
  │  → reminder_time = deadline - 5分钟
  │  → 例如：16:40到期 → 16:35提醒
  │
  └─ 如果距离截止时间 < 5分钟
     → reminder_time = now + 1分钟
     → 例如：16:32到期（现在16:30）→ 16:31提醒
  ↓
任务保存到数据库
```

**代码实现**：
```rust
if let Some(deadline_ts) = deadline {
    let now = chrono::Utc::now().timestamp();
    let time_until_deadline = deadline_ts - now;

    task.reminder_time = if time_until_deadline > 3600 {
        Some(deadline_ts - 3600)  // 提前1小时
    } else if time_until_deadline > 300 {
        Some(deadline_ts - 300)   // 提前5分钟
    } else {
        Some(now + 60)            // 1分钟后
    };
}
```

### 2. 调度器检查（scheduler/mod.rs:add_deadline_reminder_job）

```
每5分钟触发
  ↓
调用 db.get_tasks_needing_reminder()
  ↓
SQL查询：
  SELECT * FROM tasks
  WHERE reminder_time IS NOT NULL
    AND reminder_time <= now
    AND reminder_time > now - 300  ← 5分钟容错窗口
    AND status != 'done'
    AND is_deleted = 0
  ↓
返回需要提醒的任务列表
  ↓
遍历每个任务：
  ├─→ 发送通知
  └─→ 调用 db.clear_reminder(task_id)
      └─→ UPDATE tasks SET reminder_time = NULL WHERE id = ?
          ↑
          关键步骤：清除后不会再被查询到，避免重复通知
```

### 3. 通知发送

```rust
title = "⏰ Task Reminder: {任务标题}"
body = "Deadline: {截止时间}\nPriority: {优先级}"
  ↓
send_notification_internal()
  ↓
tauri_plugin_notification 发送系统通知
  ↓
macOS 通知中心显示通知
```

---

## 时间线示例

### 场景A：正常任务（提前1小时提醒）

```
[16:00] 用户创建任务
  - title: "完成报告"
  - deadline: 2026-02-17 17:30
  - 计算: time_until_deadline = 90分钟 > 60分钟
  - reminder_time: 2026-02-17 16:30 ✅

数据库状态：
  id=1, title="完成报告", deadline=1708178400, reminder_time=1708174800

[16:00-16:25] 调度器检查
  - 查询条件: reminder_time <= now
  - 16:30 > 16:00 ❌ 不满足
  - 不发送通知

[16:30] 调度器检查 ← 正好到提醒时间
  - now = 16:30:00
  - 查询条件: reminder_time <= 16:30 AND reminder_time > 16:25
  - 16:30 在范围内 ✅
  - 发送通知："⏰ Task Reminder: 完成报告"
  - 执行: UPDATE tasks SET reminder_time = NULL WHERE id = 1

[16:35] 调度器检查
  - 查询: reminder_time IS NOT NULL
  - 任务1的 reminder_time = NULL ❌
  - 不再发送通知 ✅ 避免重复

[17:30] 任务到期
  - UI显示为 "Overdue"
  - 不再发送通知（reminder_time已清除）
```

### 场景B：紧急任务（提前5分钟提醒）

```
[16:20] 用户创建任务
  - title: "线上会议"
  - deadline: 2026-02-17 16:35
  - 计算: time_until_deadline = 15分钟 (300~3600秒范围)
  - reminder_time: 2026-02-17 16:30 ✅

[16:25] 调度器检查
  - 16:30 > 16:25 ❌ 还没到

[16:30] 调度器检查
  - 16:30 <= 16:30 ✅ 到了！
  - 发送通知："⏰ Task Reminder: 线上会议"
  - 清除 reminder_time

[16:35] 任务到期
  - 已提前5分钟提醒过
```

### 场景C：超紧急任务（1分钟后提醒）

```
[16:30] 用户创建任务
  - title: "立即处理"
  - deadline: 2026-02-17 16:32
  - 计算: time_until_deadline = 2分钟 < 5分钟
  - reminder_time: 2026-02-17 16:31 ✅

[16:31] 下一次检查还没到（要等16:35）

[16:35] 调度器检查
  - now = 16:35
  - 查询: reminder_time <= 16:35 AND reminder_time > 16:30
  - 16:31 在范围内 ✅
  - 发送通知（虽然任务已在16:32过期，但容错窗口内）
```

### 场景D：在两次检查之间到期的任务

**这是旧系统的致命缺陷，新系统完美解决！**

```
[16:22] 用户创建任务
  - deadline: 16:27
  - 计算: time_until_deadline = 5分钟
  - reminder_time: 16:22 ✅ （提前5分钟 = 16:22）

[16:25] 调度器检查
  - 查询: reminder_time <= 16:25 AND reminder_time > 16:20
  - 16:22 在范围内 ✅
  - 发送通知 ✓
  - 清除 reminder_time

[16:27] 任务到期
  - 已经在16:25收到提醒

✅ 旧系统会错过（16:20检查时未创建，16:30检查时已过期）
✅ 新系统在16:25检查时准确捕获！
```

---

## 关键问题解答

### Q1: 为什么检查频率是5分钟？

**答**：平衡精确度和性能
- ✅ 5分钟足够捕获大部分场景（配合5分钟容错窗口）
- ✅ 避免过于频繁的数据库查询
- ✅ 对于紧急任务，用户设置时reminder_time已经计算好了

### Q2: 5分钟容错窗口是什么意思？

**答**：查询条件 `reminder_time > now - 300`

```
假设 now = 16:30:00

查询范围：16:25:00 < reminder_time <= 16:30:00

这意味着：
- 16:26 设置的提醒 → ✅ 会被捕获
- 16:28 设置的提醒 → ✅ 会被捕获
- 16:20 设置的提醒 → ❌ 超出窗口（已在上次检查处理）
```

**作用**：
1. 避免错过在检查间隔期间到达的提醒
2. 防止误捕获过期太久的提醒

### Q3: 如何避免重复通知？

**答**：发送后立即清除 `reminder_time`

```rust
// 发送通知后
if send_notification_success {
    db.clear_reminder(task_id);  // SET reminder_time = NULL
}

// 下次检查时
// WHERE reminder_time IS NOT NULL  ← 因为是NULL，不会被查到
```

### Q4: 如果用户修改了截止时间怎么办？

**答**：update_task会重新计算 reminder_time

```rust
// 在 update_task 中
if let Some(d) = deadline {
    task.deadline = Some(d);

    // 重新计算提醒时间
    let time_until_deadline = d - now;
    task.reminder_time = calculate_reminder_time(d, time_until_deadline);
}
```

### Q5: 完成任务后会继续提醒吗？

**答**：不会，查询条件排除了已完成的任务

```sql
WHERE status != 'done'  ← 已完成的任务不会被查询
```

### Q6: 删除任务后呢？

**答**：同样不会

```sql
WHERE is_deleted = 0  ← 软删除的任务不会被查询
```

---

## 测试验证

### 测试用例1：常规任务提醒

```bash
# 1. 创建任务（1小时后到期）
curl -X POST http://localhost:1420/create_task \
  -d "title=测试任务&deadline=$(date -v+1H +%s)"

# 2. 检查数据库
sqlite3 ~/Library/Application\ Support/com.intento.app/intento.db \
  "SELECT title, datetime(reminder_time, 'unixepoch', 'localtime')
   FROM tasks WHERE title='测试任务';"

# 预期输出：reminder_time = 当前时间
```

### 测试用例2：紧急任务提醒

```bash
# 创建6分钟后到期的任务
deadline=$(date -v+6M +%s)
# reminder_time 应该是 deadline - 5分钟 = 1分钟后
```

### 测试用例3：验证不重复

```bash
# 1. 创建任务
# 2. 等待第一次提醒
# 3. 检查数据库 reminder_time 是否为NULL
# 4. 等待下一个5分钟检查周期
# 5. 确认没有再次收到通知
```

### 手动测试

```typescript
// 在浏览器控制台
import { invoke } from '@tauri-apps/api/core';

// 创建测试任务（5分钟后到期）
const deadline = Math.floor(Date.now() / 1000) + 300;
await invoke('create_task', {
  title: '测试通知',
  deadline: deadline,
  priority: 'high'
});

// 等待提醒...
```

---

## 总结

新的通知系统通过以下机制实现了可靠的任务提醒：

1. ✅ **精确提醒时间**：使用 reminder_time 字段预先计算
2. ✅ **避免错过**：5分钟容错窗口 + 5分钟检查频率
3. ✅ **防止重复**：发送后立即清除 reminder_time
4. ✅ **智能策略**：根据时间紧迫程度自动调整提醒时机
5. ✅ **状态过滤**：自动排除已完成/已删除的任务

**与旧系统对比**：

| 特性 | 旧系统 | 新系统 |
|------|--------|--------|
| 检查频率 | 15分钟 | 5分钟 |
| 提醒逻辑 | 未来24h窗口 | reminder_time精确匹配 |
| 重复通知 | ❌ 每次都通知 | ✅ 只通知一次 |
| 错过风险 | ⚠️ 中等 | ✅ 极低 |
| 时机准确性 | ⚠️ 不精确 | ✅ 精确 |
