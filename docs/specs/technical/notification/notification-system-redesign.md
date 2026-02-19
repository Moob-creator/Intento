# 通知系统重新设计方案

## 当前问题总结

### 问题1：时间窗口错过
- **现象**：即使每15分钟检查一次，在两次检查之间到期的任务会错过通知
- **例子**：16:20创建任务设置16:25到期，16:30检查时任务已过期

### 问题2：重复通知
- **现象**：同一任务在每次检查时都会发送通知
- **例子**：16:00、16:15、16:30都会为同一个17:00到期的任务发送通知

### 问题3：通知时机不合理
- **现象**：只在"未来24小时内到期"时提醒，但用户真正需要的是"到期时"提醒
- **问题**：用户可能在16:00收到通知说"任务将在明天16:00到期"，但到真正到期时反而没有提醒

## 设计方案

### 方案A：多级提醒系统（推荐）

#### 提醒时间点
1. **提前24小时**：第一次提醒（可选）
2. **提前1小时**：重要提醒
3. **到期时刻**：最终提醒（±5分钟容差）
4. **逾期1小时**：逾期提醒（如果仍未完成）

#### 实现步骤

##### 1. 数据库增强

添加新字段记录通知历史：
```sql
ALTER TABLE tasks ADD COLUMN notification_24h_sent BOOLEAN DEFAULT 0;
ALTER TABLE tasks ADD COLUMN notification_1h_sent BOOLEAN DEFAULT 0;
ALTER TABLE tasks ADD COLUMN notification_due_sent BOOLEAN DEFAULT 0;
ALTER TABLE tasks ADD COLUMN notification_overdue_sent BOOLEAN DEFAULT 0;
```

或者使用JSON字段：
```sql
ALTER TABLE tasks ADD COLUMN notification_history TEXT;
-- 存储格式: {"24h": 1708156800, "1h": 1708239200, "due": 1708243200}
```

##### 2. 新的查询逻辑

```rust
pub fn get_tasks_needing_notification(&self) -> Result<Vec<(Task, NotificationTiming)>> {
    let now = chrono::Utc::now().timestamp();

    // 查询需要24小时提醒的任务
    let tasks_24h = self.query(
        "WHERE deadline IS NOT NULL
         AND status != 'done'
         AND is_deleted = 0
         AND deadline > ?1
         AND deadline <= ?2
         AND (notification_24h_sent = 0 OR notification_24h_sent IS NULL)",
        [now, now + 86400]  // 未来24小时
    );

    // 查询需要1小时提醒的任务
    let tasks_1h = self.query(
        "WHERE deadline IS NOT NULL
         AND status != 'done'
         AND is_deleted = 0
         AND deadline > ?1
         AND deadline <= ?2
         AND (notification_1h_sent = 0 OR notification_1h_sent IS NULL)",
        [now, now + 3600]  // 未来1小时
    );

    // 查询需要到期提醒的任务（±5分钟容差）
    let tasks_due = self.query(
        "WHERE deadline IS NOT NULL
         AND status != 'done'
         AND is_deleted = 0
         AND deadline > ?1
         AND deadline <= ?2
         AND (notification_due_sent = 0 OR notification_due_sent IS NULL)",
        [now - 300, now + 300]  // ±5分钟
    );

    // 查询需要逾期提醒的任务
    let tasks_overdue = self.query(
        "WHERE deadline IS NOT NULL
         AND status != 'done'
         AND is_deleted = 0
         AND deadline < ?1
         AND deadline > ?2
         AND (notification_overdue_sent = 0 OR notification_overdue_sent IS NULL)",
        [now, now - 3600]  // 1小时前到期
    );
}
```

##### 3. 调度器改进

```rust
pub async fn add_deadline_reminder_job(&self) -> Result<()> {
    let db = self.database.clone();
    let app_handle = self.app_handle.clone();

    // 每5分钟检查一次（而不是15分钟）
    let job = Job::new_async("0 */5 * * * *", move |_uuid, _l| {
        let db = db.clone();
        let app_handle = app_handle.clone();

        Box::pin(async move {
            println!("Running intelligent deadline reminder check...");

            match db.get_tasks_needing_notification() {
                Ok(tasks) => {
                    for (task, timing) in tasks {
                        let (title, body, urgency) = match timing {
                            NotificationTiming::TwentyFourHours => (
                                format!("提前提醒: {}", task.title),
                                format!("任务将在24小时后到期\n截止时间: {}", format_deadline(task.deadline)),
                                "normal"
                            ),
                            NotificationTiming::OneHour => (
                                format!("即将到期: {}", task.title),
                                format!("任务将在1小时后到期！\n截止时间: {}", format_deadline(task.deadline)),
                                "high"
                            ),
                            NotificationTiming::Due => (
                                format!("⏰ 任务到期: {}", task.title),
                                format!("任务现在到期了！\n优先级: {:?}", task.priority),
                                "critical"
                            ),
                            NotificationTiming::Overdue => (
                                format!("❗ 任务逾期: {}", task.title),
                                format!("任务已逾期1小时\n请尽快完成！"),
                                "critical"
                            ),
                        };

                        // 发送通知
                        if let Err(e) = send_notification_internal(
                            &app_handle,
                            &title,
                            &body,
                            urgency,
                        ) {
                            eprintln!("Failed to send notification: {}", e);
                        } else {
                            // 标记已通知
                            let _ = db.mark_notification_sent(task.id.unwrap(), timing);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to query tasks needing notification: {}", e);
                }
            }
        })
    })?;

    let scheduler = self.scheduler.lock().await;
    scheduler.add(job).await?;
    println!("Added intelligent deadline reminder job (runs every 5 minutes)");
    Ok(())
}
```

### 方案B：简化方案（快速实现）

如果不想修改数据库结构，可以使用内存缓存：

```rust
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct TaskScheduler {
    scheduler: Arc<Mutex<JobScheduler>>,
    app_handle: AppHandle,
    database: Database,
    notified_tasks: Arc<Mutex<HashSet<i64>>>,  // 缓存已通知的任务ID
}

// 在检查逻辑中
let notified = self.notified_tasks.lock().await;
if !notified.contains(&task_id) {
    send_notification(...);
    notified.insert(task_id);
}
```

### 方案C：使用reminder_time字段（最优雅）

利用现有的 `reminder_time` 字段：

```rust
// 用户创建任务时，自动设置reminder_time
if let Some(deadline) = task.deadline {
    task.reminder_time = Some(deadline - 3600);  // 提前1小时提醒
}

// 查询逻辑
pub fn get_tasks_to_remind(&self) -> Result<Vec<Task>> {
    let now = chrono::Utc::now().timestamp();

    self.query(
        "WHERE reminder_time IS NOT NULL
         AND status != 'done'
         AND is_deleted = 0
         AND reminder_time > ?1
         AND reminder_time <= ?2",
        [now - 300, now]  // 刚过提醒时间的任务（5分钟容差）
    )
}

// 发送后清除reminder_time，避免重复
db.update_task(task_id, Task {
    reminder_time: None,
    ..task
});
```

## 推荐实施方案

**第一阶段**（立即）：使用方案C - reminder_time字段
- ✅ 不需要修改数据库结构
- ✅ 逻辑简单清晰
- ✅ 避免重复通知
- ✅ 精确提醒

**第二阶段**（后续优化）：升级到方案A - 多级提醒
- 添加notification_history字段
- 支持多个提醒时间点
- 更丰富的通知策略

## 测试方案

### 测试用例1：正常提醒
```
创建任务: deadline = now + 2小时
预期: 1小时后收到提醒
```

### 测试用例2：创建后立即到期
```
创建任务: deadline = now + 3分钟
预期: 3分钟后收到提醒（不会错过）
```

### 测试用例3：避免重复
```
创建任务: deadline = now + 30分钟
预期: 只收到一次提醒（不重复）
```

### 测试用例4：跨检查周期
```
创建任务: deadline = 16:22 (当前 16:20)
下次检查: 16:25
预期: 16:25检查时能发现并提醒
```
