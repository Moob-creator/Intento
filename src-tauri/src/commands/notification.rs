use tauri::AppHandle;

/// Notification type for different reminder scenarios
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationType {
    /// Task deadline reminder
    Deadline,
    /// Daily review reminder
    DailyReview,
    /// Custom notification
    Custom,
}

/// Send a desktop notification
///
/// # Arguments
/// * `app` - The Tauri app handle
/// * `title` - Notification title
/// * `body` - Notification body/message
/// * `notification_type` - Type of notification (deadline, daily_review, custom)
///
/// # Returns
/// Result indicating success or error message
#[tauri::command]
pub async fn send_notification(
    app: AppHandle,
    title: String,
    body: String,
    notification_type: Option<NotificationType>,
) -> Result<(), String> {
    let _type = notification_type.unwrap_or(NotificationType::Custom);

    println!("Attempting to send notification: {} - {}", title, body);

    // Get app name from config
    let app_name = app.config().product_name.clone().unwrap_or_else(|| "Intento".to_string());

    // Send the notification with icon
    match tauri_plugin_notification::NotificationExt::notification(&app)
        .builder()
        .title(&title)
        .body(&body)
        .icon("icon") // Use the app icon
        .show()
    {
        Ok(_) => {
            println!("✓ Notification sent successfully: {}", app_name);
            Ok(())
        }
        Err(e) => {
            let error_msg = format!("Failed to send notification: {}", e);
            eprintln!("✗ {}", error_msg);
            Err(error_msg)
        }
    }
}

/// Trigger a manual check for expiring tasks
///
/// This command can be called from the frontend to manually check
/// for tasks that are expiring soon and send notifications
#[tauri::command]
pub async fn check_expiring_tasks(
    app: AppHandle,
    database: tauri::State<'_, crate::db::Database>,
) -> Result<usize, String> {
    use chrono::{DateTime, Utc};

    // Check for tasks expiring in the next 24 hours
    let tasks = database
        .get_expiring_tasks(24 * 60 * 60)
        .map_err(|e| format!("Failed to query expiring tasks: {}", e))?;

    let count = tasks.len();

    for task in tasks {
        let deadline_str = if let Some(deadline) = task.deadline {
            let dt =
                DateTime::<Utc>::from_timestamp(deadline, 0).unwrap_or_else(|| Utc::now());
            dt.format("%Y-%m-%d %H:%M").to_string()
        } else {
            "Unknown".to_string()
        };

        let title = format!("Task Deadline Reminder: {}", task.title);
        let body = format!("Deadline: {}\nPriority: {:?}", deadline_str, task.priority);

        send_notification(app.clone(), title, body, Some(NotificationType::Deadline))
            .await
            .map_err(|e| format!("Failed to send notification: {}", e))?;
    }

    Ok(count)
}

/// Test notification functionality
///
/// Sends a test notification to verify the notification system is working
#[tauri::command]
pub async fn test_notification(app: AppHandle) -> Result<(), String> {
    send_notification(
        app,
        "Intento".to_string(),
        "✨ 通知系统正常工作！\n这是来自 Intento 的测试通知。".to_string(),
        Some(NotificationType::Custom),
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_type_serialization() {
        use serde_json;

        let deadline = NotificationType::Deadline;
        let json = serde_json::to_string(&deadline).unwrap();
        assert_eq!(json, "\"deadline\"");

        let daily_review = NotificationType::DailyReview;
        let json = serde_json::to_string(&daily_review).unwrap();
        assert_eq!(json, "\"dailyreview\"");

        let custom = NotificationType::Custom;
        let json = serde_json::to_string(&custom).unwrap();
        assert_eq!(json, "\"custom\"");
    }

    #[test]
    fn test_notification_type_deserialization() {
        use serde_json;

        let deadline: NotificationType = serde_json::from_str("\"deadline\"").unwrap();
        assert!(matches!(deadline, NotificationType::Deadline));

        let daily_review: NotificationType = serde_json::from_str("\"dailyreview\"").unwrap();
        assert!(matches!(daily_review, NotificationType::DailyReview));

        let custom: NotificationType = serde_json::from_str("\"custom\"").unwrap();
        assert!(matches!(custom, NotificationType::Custom));
    }
}
