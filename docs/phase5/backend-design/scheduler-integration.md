# Phase 5: 定时任务集成设计

## 定时任务架构

### 现有 TaskScheduler 扩展

Intento 已经在 Phase 1 中实现了基于 `tokio-cron-scheduler` 的定时任务系统。Phase 5 需要在此基础上添加总结生成的定时任务。

---

## 1. Scheduler 集成

### 修改 scheduler/mod.rs

```rust
// src-tauri/src/scheduler/mod.rs

use tokio_cron_scheduler::{Job, JobScheduler};
use crate::db::Database;
use crate::summary::scheduler_jobs::{
    generate_daily_summaries,
    generate_weekly_summaries,
    generate_monthly_summaries,
    generate_semi_annual_summaries,
    generate_yearly_summaries,
};

pub struct TaskScheduler {
    scheduler: JobScheduler,
}

impl TaskScheduler {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let scheduler = JobScheduler::new().await?;
        Ok(Self { scheduler })
    }

    /// 启动调度器并添加所有任务
    pub async fn start(&self, db: Database) -> Result<(), String> {
        let scheduler_clone = self.scheduler.clone();

        // ==================== Phase 1: 通知任务 ====================
        let db_clone = db.clone();
        let notification_job = Job::new_async("0 */5 * * * *", move |_uuid, _l| {
            let db = db_clone.clone();
            Box::pin(async move {
                if let Err(e) = check_deadline_notifications(&db).await {
                    eprintln!("Notification job error: {}", e);
                }
            })
        }).map_err(|e| format!("Failed to create notification job: {}", e))?;

        scheduler_clone.add(notification_job).await
            .map_err(|e| format!("Failed to add notification job: {}", e))?;

        // ==================== Phase 5: 总结生成任务 ====================

        // ✨ 每日总结（每天凌晨 1 点）
        let db_daily = db.clone();
        let daily_summary_job = Job::new_async("0 0 1 * * *", move |_uuid, _l| {
            let db = db_daily.clone();
            Box::pin(async move {
                println!("[Scheduler] Starting daily summary generation...");
                match generate_daily_summaries(&db).await {
                    Ok(_) => println!("[Scheduler] Daily summaries generated successfully"),
                    Err(e) => eprintln!("[Scheduler] Failed to generate daily summaries: {}", e),
                }
            })
        }).map_err(|e| format!("Failed to create daily summary job: {}", e))?;

        scheduler_clone.add(daily_summary_job).await
            .map_err(|e| format!("Failed to add daily summary job: {}", e))?;

        // ✨ 每周总结（每周一凌晨 2 点）
        let db_weekly = db.clone();
        let weekly_summary_job = Job::new_async("0 0 2 * * MON", move |_uuid, _l| {
            let db = db_weekly.clone();
            Box::pin(async move {
                println!("[Scheduler] Starting weekly summary generation...");
                match generate_weekly_summaries(&db).await {
                    Ok(_) => println!("[Scheduler] Weekly summaries generated successfully"),
                    Err(e) => eprintln!("[Scheduler] Failed to generate weekly summaries: {}", e),
                }
            })
        }).map_err(|e| format!("Failed to create weekly summary job: {}", e))?;

        scheduler_clone.add(weekly_summary_job).await
            .map_err(|e| format!("Failed to add weekly summary job: {}", e))?;

        // ✨ 每月总结（每月 1 号凌晨 3 点）
        let db_monthly = db.clone();
        let monthly_summary_job = Job::new_async("0 0 3 1 * *", move |_uuid, _l| {
            let db = db_monthly.clone();
            Box::pin(async move {
                println!("[Scheduler] Starting monthly summary generation...");
                match generate_monthly_summaries(&db).await {
                    Ok(_) => println!("[Scheduler] Monthly summaries generated successfully"),
                    Err(e) => eprintln!("[Scheduler] Failed to generate monthly summaries: {}", e),
                }
            })
        }).map_err(|e| format!("Failed to create monthly summary job: {}", e))?;

        scheduler_clone.add(monthly_summary_job).await
            .map_err(|e| format!("Failed to add monthly summary job: {}", e))?;

        // ✨ 每半年总结（每年 1 月和 7 月 1 号凌晨 4 点）
        let db_semi_annual = db.clone();
        let semi_annual_summary_job = Job::new_async("0 0 4 1 1,7 *", move |_uuid, _l| {
            let db = db_semi_annual.clone();
            Box::pin(async move {
                println!("[Scheduler] Starting semi-annual summary generation...");
                match generate_semi_annual_summaries(&db).await {
                    Ok(_) => println!("[Scheduler] Semi-annual summaries generated successfully"),
                    Err(e) => eprintln!("[Scheduler] Failed to generate semi-annual summaries: {}", e),
                }
            })
        }).map_err(|e| format!("Failed to create semi-annual summary job: {}", e))?;

        scheduler_clone.add(semi_annual_summary_job).await
            .map_err(|e| format!("Failed to add semi-annual summary job: {}", e))?;

        // ✨ 每年总结（每年 1 月 1 号凌晨 5 点）
        let db_yearly = db.clone();
        let yearly_summary_job = Job::new_async("0 0 5 1 1 *", move |_uuid, _l| {
            let db = db_yearly.clone();
            Box::pin(async move {
                println!("[Scheduler] Starting yearly summary generation...");
                match generate_yearly_summaries(&db).await {
                    Ok(_) => println!("[Scheduler] Yearly summaries generated successfully"),
                    Err(e) => eprintln!("[Scheduler] Failed to generate yearly summaries: {}", e),
                }
            })
        }).map_err(|e| format!("Failed to create yearly summary job: {}", e))?;

        scheduler_clone.add(yearly_summary_job).await
            .map_err(|e| format!("Failed to add yearly summary job: {}", e))?;

        // ✨ 清理旧总结（每天凌晨 6 点）
        let db_cleanup = db.clone();
        let cleanup_job = Job::new_async("0 0 6 * * *", move |_uuid, _l| {
            let db = db_cleanup.clone();
            Box::pin(async move {
                println!("[Scheduler] Starting old summaries cleanup...");
                if let Err(e) = cleanup_old_summaries(&db, 6).await {
                    eprintln!("[Scheduler] Failed to cleanup old summaries: {}", e);
                } else {
                    println!("[Scheduler] Old summaries cleaned up successfully");
                }
            })
        }).map_err(|e| format!("Failed to create cleanup job: {}", e))?;

        scheduler_clone.add(cleanup_job).await
            .map_err(|e| format!("Failed to add cleanup job: {}", e))?;

        // 启动调度器
        self.scheduler.start().await
            .map_err(|e| format!("Failed to start scheduler: {}", e))?;

        println!("[Scheduler] Task scheduler started successfully");
        Ok(())
    }

    /// 停止调度器
    pub async fn shutdown(&self) -> Result<(), String> {
        self.scheduler.shutdown().await
            .map_err(|e| format!("Failed to shutdown scheduler: {}", e))
    }
}

// Phase 1 已有的通知检查函数
async fn check_deadline_notifications(db: &Database) -> Result<(), String> {
    // ... 现有代码 ...
}

// ✨ Phase 5: 清理旧总结
async fn cleanup_old_summaries(db: &Database, retention_months: i64) -> Result<(), String> {
    use chrono::{Utc, Duration};

    let cutoff_date = Utc::now() - Duration::days(retention_months * 30);
    let cutoff_timestamp = cutoff_date.timestamp();

    let conn = db.conn.lock().unwrap();
    let deleted = conn.execute(
        "UPDATE summaries SET is_deleted = 1 WHERE created_at < ?1 AND is_deleted = 0",
        [cutoff_timestamp],
    ).map_err(|e| format!("Failed to cleanup summaries: {}", e))?;

    println!("[Scheduler] Cleaned up {} old summaries", deleted);
    Ok(())
}
```

---

## 2. Cron 表达式详解

### 时间配置

| 任务类型 | Cron 表达式 | 执行时间 | 说明 |
|---------|------------|---------|------|
| 每日总结 | `0 0 1 * * *` | 每天 01:00 | 总结昨天的任务 |
| 每周总结 | `0 0 2 * * MON` | 每周一 02:00 | 总结上周的任务 |
| 每月总结 | `0 0 3 1 * *` | 每月 1 号 03:00 | 总结上月的任务 |
| 每半年总结 | `0 0 4 1 1,7 *` | 1/1 和 7/1 04:00 | 总结上半年的任务 |
| 每年总结 | `0 0 5 1 1 *` | 每年 1/1 05:00 | 总结去年的任务 |
| 清理旧数据 | `0 0 6 * * *` | 每天 06:00 | 清理 6 个月前的总结 |

### Cron 格式说明

```
秒 分 时 日 月 星期
│ │ │ │ │ │
│ │ │ │ │ └─ 星期几 (0-7, 0和7都表示周日, MON-SUN)
│ │ │ │ └─── 月份 (1-12)
│ │ │ └───── 日期 (1-31)
│ │ └─────── 小时 (0-23)
│ └───────── 分钟 (0-59)
└─────────── 秒 (0-59)
```

---

## 3. 配置化定时任务

### 支持用户自定义时间

为了让用户可以在设置中自定义总结生成时间，需要实现动态调度：

```rust
// src-tauri/src/scheduler/mod.rs

impl TaskScheduler {
    /// 添加或更新总结生成任务
    pub async fn update_summary_job(
        &self,
        summary_type: SummaryType,
        enabled: bool,
        cron_expr: &str,
        db: Database,
    ) -> Result<(), String> {
        // 先移除旧任务
        self.remove_summary_job(&summary_type).await?;

        // 如果启用，添加新任务
        if enabled {
            let db_clone = db.clone();
            let type_clone = summary_type.clone();

            let job = Job::new_async(cron_expr, move |_uuid, _l| {
                let db = db_clone.clone();
                let summary_type = type_clone.clone();

                Box::pin(async move {
                    println!("[Scheduler] Generating {} summary...", summary_type.as_str());

                    let result = match summary_type {
                        SummaryType::Daily => generate_daily_summaries(&db).await,
                        SummaryType::Weekly => generate_weekly_summaries(&db).await,
                        SummaryType::Monthly => generate_monthly_summaries(&db).await,
                        SummaryType::SemiAnnual => generate_semi_annual_summaries(&db).await,
                        SummaryType::Yearly => generate_yearly_summaries(&db).await,
                    };

                    match result {
                        Ok(_) => println!("[Scheduler] {} summary generated successfully", summary_type.as_str()),
                        Err(e) => eprintln!("[Scheduler] Failed to generate {} summary: {}", summary_type.as_str(), e),
                    }
                })
            }).map_err(|e| format!("Failed to create job: {}", e))?;

            self.scheduler.add(job).await
                .map_err(|e| format!("Failed to add job: {}", e))?;

            println!("[Scheduler] Added {} summary job with schedule: {}", summary_type.as_str(), cron_expr);
        }

        Ok(())
    }

    /// 移除总结任务
    async fn remove_summary_job(&self, _summary_type: &SummaryType) -> Result<(), String> {
        // tokio-cron-scheduler 目前不支持直接按类型移除任务
        // 需要在添加任务时保存 UUID，然后根据 UUID 移除
        // 这里简化处理，在实际实现中需要维护一个任务 UUID 映射
        Ok(())
    }
}
```

---

## 4. 应用启动时初始化

### 在 main.rs 中启动调度器

```rust
// src-tauri/src/main.rs

use crate::scheduler::TaskScheduler;

#[tokio::main]
async fn main() {
    // ... 其他初始化代码 ...

    // 初始化数据库
    let db = Database::new("./data/intento.db")
        .expect("Failed to initialize database");

    // ✨ 启动任务调度器
    let scheduler = TaskScheduler::new().await
        .expect("Failed to create task scheduler");

    let db_clone = db.clone();
    tokio::spawn(async move {
        if let Err(e) = scheduler.start(db_clone).await {
            eprintln!("Scheduler error: {}", e);
        }
    });

    println!("✓ Task scheduler started");

    // 启动 Tauri 应用
    tauri::Builder::default()
        .manage(db)
        .manage(scheduler) // 将 scheduler 添加到 Tauri 状态管理
        .invoke_handler(tauri::generate_handler![
            // ... 现有 commands ...
            generate_summary,
            list_summaries,
            get_summary,
            delete_summary,
            export_summary,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## 5. 错误处理和日志

### 任务失败重试机制

```rust
// src-tauri/src/summary/scheduler_jobs.rs

use std::time::Duration;
use tokio::time::sleep;

/// 带重试的总结生成
async fn generate_summary_with_retry(
    generator: &SummaryGenerator,
    tag: Option<String>,
    summary_type: SummaryType,
    period_start: i64,
    period_end: i64,
    max_retries: u32,
) -> Result<Summary, String> {
    let mut retries = 0;

    loop {
        match generator.generate_summary(
            tag.clone(),
            summary_type.clone(),
            period_start,
            period_end,
        ).await {
            Ok(summary) => return Ok(summary),
            Err(e) => {
                retries += 1;
                if retries >= max_retries {
                    return Err(format!("Failed after {} retries: {}", max_retries, e));
                }

                eprintln!("Retry {}/{}: {}", retries, max_retries, e);
                sleep(Duration::from_secs(5 * retries as u64)).await;
            }
        }
    }
}
```

### 结构化日志

```rust
use log::{info, warn, error};

pub async fn generate_daily_summaries(db: &Database) -> Result<(), String> {
    info!("[Summary] Starting daily summary generation");

    let ai_client = AiClient::new_default()
        .map_err(|e| {
            error!("[Summary] Failed to create AI client: {}", e);
            format!("Failed to create AI client: {}", e)
        })?;

    let generator = SummaryGenerator::new(db.clone(), ai_client);
    let (period_start, period_end) = PeriodCalculator::yesterday();

    let active_tags = get_active_tags(db)?;
    info!("[Summary] Found {} active tags", active_tags.len());

    let mut success_count = 0;
    let mut failure_count = 0;

    for tag in active_tags {
        match generate_summary_with_retry(
            &generator,
            Some(tag.clone()),
            SummaryType::Daily,
            period_start,
            period_end,
            3, // 最多重试 3 次
        ).await {
            Ok(_) => {
                info!("[Summary] ✓ Generated daily summary for tag: {}", tag);
                success_count += 1;
            }
            Err(e) => {
                error!("[Summary] ✗ Failed to generate daily summary for tag {}: {}", tag, e);
                failure_count += 1;
            }
        }
    }

    // 全局总结
    match generate_summary_with_retry(
        &generator,
        None,
        SummaryType::Daily,
        period_start,
        period_end,
        3,
    ).await {
        Ok(_) => {
            info!("[Summary] ✓ Generated daily summary for all tasks");
            success_count += 1;
        }
        Err(e) => {
            error!("[Summary] ✗ Failed to generate daily summary for all tasks: {}", e);
            failure_count += 1;
        }
    }

    info!(
        "[Summary] Daily summary generation completed: {} succeeded, {} failed",
        success_count, failure_count
    );

    Ok(())
}
```

---

## 6. 测试定时任务

### 手动触发测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_daily_summary_generation() {
        let db = Database::new(":memory:").unwrap();
        // 插入测试数据...

        let result = generate_daily_summaries(&db).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_scheduler_start() {
        let db = Database::new(":memory:").unwrap();
        let scheduler = TaskScheduler::new().await.unwrap();

        let result = scheduler.start(db).await;
        assert!(result.is_ok());

        scheduler.shutdown().await.unwrap();
    }
}
```

### 开发环境快速测试

```rust
// 在开发环境中，可以使用更短的 cron 表达式测试
// 例如：每分钟执行一次
let test_job = Job::new_async("0 * * * * *", move |_uuid, _l| {
    // 测试逻辑
});
```

---

## 7. 监控和告警

### 添加健康检查端点

```rust
#[tauri::command]
pub async fn get_scheduler_status(
    db: State<'_, Database>,
) -> Result<SchedulerStatus, String> {
    let last_daily = db.get_latest_summary(&SummaryType::Daily)?;
    let last_weekly = db.get_latest_summary(&SummaryType::Weekly)?;

    Ok(SchedulerStatus {
        is_running: true,
        last_daily_run: last_daily.map(|s| s.created_at),
        last_weekly_run: last_weekly.map(|s| s.created_at),
        next_daily_run: calculate_next_run("0 0 1 * * *"),
        next_weekly_run: calculate_next_run("0 0 2 * * MON"),
    })
}

#[derive(serde::Serialize)]
pub struct SchedulerStatus {
    is_running: bool,
    last_daily_run: Option<i64>,
    last_weekly_run: Option<i64>,
    next_daily_run: i64,
    next_weekly_run: i64,
}
```
