use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::db::Database;

/// Notification type for different reminder scenarios
#[derive(Debug, Clone)]
pub enum NotificationType {
    /// Task deadline reminder
    Deadline,
    /// Daily review reminder
    DailyReview,
    /// Custom notification
    Custom,
}

/// Task scheduler for managing periodic jobs and reminders
pub struct TaskScheduler {
    scheduler: Arc<Mutex<JobScheduler>>,
    app_handle: AppHandle,
    database: Database,
}

impl TaskScheduler {
    /// Create a new task scheduler instance
    pub async fn new(app_handle: AppHandle, database: Database) -> Result<Self> {
        let scheduler = JobScheduler::new()
            .await
            .context("Failed to create job scheduler")?;

        Ok(Self {
            scheduler: Arc::new(Mutex::new(scheduler)),
            app_handle,
            database,
        })
    }

    /// Start the scheduler and all registered jobs
    pub async fn start(&self) -> Result<()> {
        let scheduler = self.scheduler.lock().await;
        scheduler
            .start()
            .await
            .context("Failed to start scheduler")?;

        println!("Task scheduler started successfully");
        Ok(())
    }

    /// Stop the scheduler
    pub async fn stop(&self) -> Result<()> {
        let mut scheduler = self.scheduler.lock().await;
        scheduler
            .shutdown()
            .await
            .context("Failed to stop scheduler")?;

        println!("Task scheduler stopped");
        Ok(())
    }

    /// Add a job to check for expiring tasks every 15 minutes
    /// This job will query tasks that will expire within 24 hours
    pub async fn add_deadline_reminder_job(&self) -> Result<()> {
        let db = self.database.clone();
        let app_handle = self.app_handle.clone();

        // Run every 15 minutes
        let job = Job::new_async("0 */15 * * * *", move |_uuid, _l| {
            let db = db.clone();
            let app_handle = app_handle.clone();

            Box::pin(async move {
                println!("Running deadline reminder check...");

                // Check for tasks expiring in the next 24 hours
                match db.get_expiring_tasks(24 * 60 * 60) {
                    Ok(tasks) => {
                        if tasks.is_empty() {
                            println!("No tasks expiring in the next 24 hours");
                            return;
                        }

                        println!("Found {} task(s) expiring within 24 hours", tasks.len());

                        for task in tasks {
                            let deadline_str = if let Some(deadline) = task.deadline {
                                let dt = DateTime::<Utc>::from_timestamp(deadline, 0)
                                    .unwrap_or_else(|| Utc::now());
                                dt.format("%Y-%m-%d %H:%M").to_string()
                            } else {
                                "Unknown".to_string()
                            };

                            let title = format!("Task Deadline Reminder: {}", task.title);
                            let body = format!(
                                "Deadline: {}\nPriority: {:?}",
                                deadline_str, task.priority
                            );

                            // Send notification
                            if let Err(e) = send_notification_internal(
                                &app_handle,
                                &title,
                                &body,
                                NotificationType::Deadline,
                            ) {
                                eprintln!("Failed to send notification for task {}: {}",
                                    task.id.unwrap_or(0), e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to query expiring tasks: {}", e);
                    }
                }
            })
        })
        .context("Failed to create deadline reminder job")?;

        let scheduler = self.scheduler.lock().await;
        scheduler
            .add(job)
            .await
            .context("Failed to add deadline reminder job")?;

        println!("Added deadline reminder job (runs every 15 minutes)");
        Ok(())
    }

    /// Add a daily summary job that runs at 6 PM every day
    pub async fn add_daily_summary_job(&self) -> Result<()> {
        let app_handle = self.app_handle.clone();

        // Run at 18:00 every day
        let job = Job::new_async("0 0 18 * * *", move |_uuid, _l| {
            let app_handle = app_handle.clone();

            Box::pin(async move {
                println!("Generating daily summary...");

                let title = "Daily Review Reminder".to_string();
                let body = "Time to review your tasks for today!".to_string();

                // Send notification
                if let Err(e) = send_notification_internal(
                    &app_handle,
                    &title,
                    &body,
                    NotificationType::DailyReview,
                ) {
                    eprintln!("Failed to send daily summary notification: {}", e);
                }
            })
        })
        .context("Failed to create daily summary job")?;

        let scheduler = self.scheduler.lock().await;
        scheduler
            .add(job)
            .await
            .context("Failed to add daily summary job")?;

        println!("Added daily summary job (runs at 18:00 daily)");
        Ok(())
    }

    /// Add automatic summary generation jobs
    /// These jobs will generate summaries at scheduled times
    pub async fn add_auto_summary_jobs(&self) -> Result<()> {
        // Daily summaries - run at 1:00 AM every day
        self.add_daily_auto_summary_job().await?;

        // Weekly summaries - run at 2:00 AM every Monday
        self.add_weekly_auto_summary_job().await?;

        // Monthly summaries - run at 3:00 AM on the 1st of each month
        self.add_monthly_auto_summary_job().await?;

        // Semi-annual summaries - run at 4:00 AM on Jan 1 and Jul 1
        self.add_semi_annual_auto_summary_job().await?;

        // Yearly summaries - run at 5:00 AM on Jan 1
        self.add_yearly_auto_summary_job().await?;

        println!("✅ Added all automatic summary generation jobs");
        Ok(())
    }

    /// Add daily auto summary job
    async fn add_daily_auto_summary_job(&self) -> Result<()> {
        let db = self.database.clone();

        let job = Job::new_async("0 0 1 * * *", move |_uuid, _l| {
            let db = db.clone();
            Box::pin(async move {
                println!("🔄 Running daily summary generation job...");
                match crate::summary::scheduler_jobs::generate_daily_summaries(&db).await {
                    Ok(()) => println!("✅ Daily summary generation completed successfully"),
                    Err(e) => eprintln!("❌ Daily summary generation failed: {}", e),
                }
            })
        })
        .context("Failed to create daily summary job")?;

        let scheduler = self.scheduler.lock().await;
        scheduler.add(job).await.context("Failed to add daily summary job")?;
        println!("Added daily auto summary job (runs at 1:00 AM daily)");
        Ok(())
    }

    /// Add weekly auto summary job
    async fn add_weekly_auto_summary_job(&self) -> Result<()> {
        let db = self.database.clone();

        let job = Job::new_async("0 0 2 * * MON", move |_uuid, _l| {
            let db = db.clone();
            Box::pin(async move {
                println!("🔄 Running weekly summary generation job...");
                match crate::summary::scheduler_jobs::generate_weekly_summaries(&db).await {
                    Ok(()) => println!("✅ Weekly summary generation completed successfully"),
                    Err(e) => eprintln!("❌ Weekly summary generation failed: {}", e),
                }
            })
        })
        .context("Failed to create weekly summary job")?;

        let scheduler = self.scheduler.lock().await;
        scheduler.add(job).await.context("Failed to add weekly summary job")?;
        println!("Added weekly auto summary job (runs at 2:00 AM every Monday)");
        Ok(())
    }

    /// Add monthly auto summary job
    async fn add_monthly_auto_summary_job(&self) -> Result<()> {
        let db = self.database.clone();

        let job = Job::new_async("0 0 3 1 * *", move |_uuid, _l| {
            let db = db.clone();
            Box::pin(async move {
                println!("🔄 Running monthly summary generation job...");
                match crate::summary::scheduler_jobs::generate_monthly_summaries(&db).await {
                    Ok(()) => println!("✅ Monthly summary generation completed successfully"),
                    Err(e) => eprintln!("❌ Monthly summary generation failed: {}", e),
                }
            })
        })
        .context("Failed to create monthly summary job")?;

        let scheduler = self.scheduler.lock().await;
        scheduler.add(job).await.context("Failed to add monthly summary job")?;
        println!("Added monthly auto summary job (runs at 3:00 AM on 1st of month)");
        Ok(())
    }

    /// Add semi-annual auto summary job
    async fn add_semi_annual_auto_summary_job(&self) -> Result<()> {
        let db = self.database.clone();

        let job = Job::new_async("0 0 4 1 1,7 *", move |_uuid, _l| {
            let db = db.clone();
            Box::pin(async move {
                println!("🔄 Running semi-annual summary generation job...");
                match crate::summary::scheduler_jobs::generate_semi_annual_summaries(&db).await {
                    Ok(()) => println!("✅ Semi-annual summary generation completed successfully"),
                    Err(e) => eprintln!("❌ Semi-annual summary generation failed: {}", e),
                }
            })
        })
        .context("Failed to create semi-annual summary job")?;

        let scheduler = self.scheduler.lock().await;
        scheduler.add(job).await.context("Failed to add semi-annual summary job")?;
        println!("Added semi-annual auto summary job (runs at 4:00 AM on Jan 1 and Jul 1)");
        Ok(())
    }

    /// Add yearly auto summary job
    async fn add_yearly_auto_summary_job(&self) -> Result<()> {
        let db = self.database.clone();

        let job = Job::new_async("0 0 5 1 1 *", move |_uuid, _l| {
            let db = db.clone();
            Box::pin(async move {
                println!("🔄 Running yearly summary generation job...");
                match crate::summary::scheduler_jobs::generate_yearly_summaries(&db).await {
                    Ok(()) => println!("✅ Yearly summary generation completed successfully"),
                    Err(e) => eprintln!("❌ Yearly summary generation failed: {}", e),
                }
            })
        })
        .context("Failed to create yearly summary job")?;

        let scheduler = self.scheduler.lock().await;
        scheduler.add(job).await.context("Failed to add yearly summary job")?;
        println!("Added yearly auto summary job (runs at 5:00 AM on Jan 1)");
        Ok(())
    }

    /// Add a custom cron job
    pub async fn add_custom_job<F>(&self, cron_expression: &str, job_fn: F) -> Result<()>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let job = Job::new(cron_expression, move |_uuid, _l| {
            job_fn();
        })
        .context("Failed to create custom job")?;

        let scheduler = self.scheduler.lock().await;
        scheduler
            .add(job)
            .await
            .context("Failed to add custom job")?;

        Ok(())
    }
}

/// Internal function to send a notification
fn send_notification_internal(
    app_handle: &AppHandle,
    title: &str,
    body: &str,
    notification_type: NotificationType,
) -> Result<()> {
    let _ = notification_type; // Mark as used

    tauri_plugin_notification::NotificationExt::notification(app_handle)
        .builder()
        .title(title)
        .body(body)
        .show()
        .context("Failed to show notification")?;

    println!("Notification sent: {} - {}", title, body);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::models::{Priority, Task, TaskStatus};

    #[tokio::test]
    async fn test_scheduler_creation() {
        // Note: This test requires a valid Tauri app handle
        // In a real test environment, you would mock or create a test app

        // Create a temporary database
        let temp_dir = std::env::temp_dir();
        let db_path = temp_dir.join("intento_scheduler_test.db");
        let _db = Database::new(db_path.clone()).unwrap();

        // Clean up
        std::fs::remove_file(db_path).ok();
    }

    #[tokio::test]
    async fn test_job_scheduler_lifecycle() {
        // Test that we can create and start a basic scheduler
        let mut scheduler = JobScheduler::new().await.unwrap();

        // Start the scheduler
        scheduler.start().await.unwrap();

        // Shutdown the scheduler
        scheduler.shutdown().await.unwrap();
    }

    #[test]
    fn test_expiring_tasks_query() {
        use chrono::Utc;

        let temp_dir = std::env::temp_dir();
        let db_path = temp_dir.join("intento_expiring_test.db");
        let db = Database::new(db_path.clone()).unwrap();

        // Create a task that expires in 12 hours
        let now = Utc::now().timestamp();
        let deadline_12h = now + (12 * 60 * 60);

        let mut task = Task::new("Test Task Expiring Soon".to_string());
        task.deadline = Some(deadline_12h);
        task.priority = Priority::High;

        let task_id = db.create_task(&task).unwrap();
        assert!(task_id > 0);

        // Create a task that expires in 48 hours (should not be included)
        let deadline_48h = now + (48 * 60 * 60);
        let mut task2 = Task::new("Test Task Expiring Later".to_string());
        task2.deadline = Some(deadline_48h);

        let task_id2 = db.create_task(&task2).unwrap();
        assert!(task_id2 > 0);

        // Query tasks expiring within 24 hours
        let expiring_tasks = db.get_expiring_tasks(24 * 60 * 60).unwrap();

        // Should only include the first task
        assert_eq!(expiring_tasks.len(), 1);
        assert_eq!(expiring_tasks[0].title, "Test Task Expiring Soon");
        assert_eq!(expiring_tasks[0].priority, Priority::High);

        // Query tasks expiring within 72 hours
        let expiring_tasks_72h = db.get_expiring_tasks(72 * 60 * 60).unwrap();

        // Should include both tasks
        assert_eq!(expiring_tasks_72h.len(), 2);

        // Clean up
        std::fs::remove_file(db_path).ok();
    }

    #[test]
    fn test_expiring_tasks_excludes_completed() {
        use chrono::Utc;

        let temp_dir = std::env::temp_dir();
        let db_path = temp_dir.join("intento_completed_test.db");
        let db = Database::new(db_path.clone()).unwrap();

        let now = Utc::now().timestamp();
        let deadline = now + (12 * 60 * 60);

        // Create a completed task with a deadline
        let mut task = Task::new("Completed Task".to_string());
        task.deadline = Some(deadline);
        task.status = TaskStatus::Done;
        task.completed_at = Some(now);

        let task_id = db.create_task(&task).unwrap();
        assert!(task_id > 0);

        // Query tasks expiring within 24 hours
        let expiring_tasks = db.get_expiring_tasks(24 * 60 * 60).unwrap();

        // Should not include completed tasks
        assert_eq!(expiring_tasks.len(), 0);

        // Clean up
        std::fs::remove_file(db_path).ok();
    }
}
