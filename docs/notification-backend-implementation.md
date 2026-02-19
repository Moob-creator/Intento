# Notification Settings - Rust Backend Implementation Guide

This document outlines the Tauri commands that need to be implemented in the Rust backend to support the notification settings feature.

## Required Tauri Commands

### 1. Get Notification Settings

**Command:** `get_notification_settings`

**Description:** Retrieves the current notification configuration from persistent storage.

**Parameters:** None

**Returns:** `NotificationConfig`

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationConfig {
    pub enabled: bool,
    pub reminder_enabled: bool,
    pub reminder_minutes: i32, // Minutes before deadline
}
```

**Example Implementation:**

```rust
#[tauri::command]
async fn get_notification_settings(state: State<'_, AppState>) -> Result<NotificationConfig, String> {
    // Load from persistent storage (e.g., config file, database)
    // Return default if not found
    let config = NotificationConfig {
        enabled: true,
        reminder_enabled: true,
        reminder_minutes: 60, // Default: 1 hour before
    };

    Ok(config)
}
```

### 2. Update Notification Settings

**Command:** `update_notification_settings`

**Description:** Saves the notification configuration to persistent storage.

**Parameters:**
- `settings: NotificationConfig` - The new notification configuration

**Returns:** `Result<(), String>`

**Example Implementation:**

```rust
#[tauri::command]
async fn update_notification_settings(
    settings: NotificationConfig,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // Save to persistent storage
    // Update any active notification schedulers if needed

    Ok(())
}
```

### 3. Test Notification (Already Exists)

**Command:** `test_notification`

**Description:** Sends a test notification to verify the system is working correctly.

**Parameters:** None

**Returns:** `Result<(), String>`

**Example Implementation:**

```rust
use tauri::api::notification::Notification;

#[tauri::command]
async fn test_notification(app_handle: tauri::AppHandle) -> Result<(), String> {
    Notification::new(&app_handle.config().tauri.bundle.identifier)
        .title("Test Notification")
        .body("Notifications are working! You'll receive reminders for your task deadlines.")
        .icon("icon")
        .show()
        .map_err(|e| e.to_string())?;

    Ok(())
}
```

## Notification Scheduler

You'll need to implement a background scheduler that:

1. Monitors tasks with upcoming deadlines
2. Checks the user's notification settings
3. Sends notifications at the configured time before deadlines

**Example Architecture:**

```rust
use tokio::time::{interval, Duration};

pub struct NotificationScheduler {
    config: Arc<RwLock<NotificationConfig>>,
    task_store: Arc<TaskStore>,
}

impl NotificationScheduler {
    pub async fn start(&self) {
        let mut interval = interval(Duration::from_secs(60)); // Check every minute

        loop {
            interval.tick().await;

            let config = self.config.read().await;
            if !config.enabled || !config.reminder_enabled {
                continue;
            }

            // Get tasks that need reminders
            let upcoming_tasks = self.get_upcoming_tasks(config.reminder_minutes).await;

            for task in upcoming_tasks {
                self.send_reminder(&task).await;
            }
        }
    }

    async fn get_upcoming_tasks(&self, reminder_minutes: i32) -> Vec<Task> {
        // Calculate the time window for reminders
        let now = chrono::Utc::now().timestamp();
        let reminder_time = now + (reminder_minutes as i64 * 60);

        // Query tasks with deadlines in the reminder window
        // Return tasks that haven't been completed and haven't been reminded yet
        vec![]
    }

    async fn send_reminder(&self, task: &Task) {
        // Send notification for the task
        // Mark the task as reminded to avoid duplicate notifications
    }
}
```

## Integration with Main App

In your `main.rs`:

```rust
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // ... existing commands
            get_notification_settings,
            update_notification_settings,
            test_notification,
        ])
        .setup(|app| {
            // Initialize notification scheduler
            let scheduler = NotificationScheduler::new(app.handle());
            tokio::spawn(async move {
                scheduler.start().await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Persistent Storage

Store the notification configuration in your app's data directory:

```rust
use std::path::PathBuf;
use tauri::api::path::app_data_dir;

fn get_config_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let mut path = app_data_dir(&app_handle.config())
        .expect("Failed to get app data dir");
    path.push("notification_config.json");
    path
}

fn save_config(config: &NotificationConfig, path: &PathBuf) -> Result<(), String> {
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| e.to_string())?;
    std::fs::write(path, json)
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn load_config(path: &PathBuf) -> Result<NotificationConfig, String> {
    if !path.exists() {
        return Ok(NotificationConfig::default());
    }

    let json = std::fs::read_to_string(path)
        .map_err(|e| e.to_string())?;
    serde_json::from_str(&json)
        .map_err(|e| e.to_string())
}
```

## Platform-Specific Considerations

### macOS
- Request notification permissions on first launch
- Use `tauri::api::notification` for system notifications

### Windows
- Windows 10+ has native notification support via WinRT
- Use `tauri::api::notification` which handles this automatically

### Linux
- Uses libnotify through the notification daemon
- Ensure the daemon is running for notifications to work

## Error Handling

Always provide user-friendly error messages:

```rust
.map_err(|e| format!("Failed to send notification: {}. Please check your system notification settings.", e))
```

## Testing

Test cases to implement:

1. Load default settings on first run
2. Save and load custom settings
3. Toggle notifications on/off
4. Change reminder time
5. Send test notification
6. Handle notification permission errors gracefully
7. Test scheduler with different reminder times

## Next Steps

1. Implement the Tauri commands above
2. Set up persistent storage for notification config
3. Implement the notification scheduler
4. Test on each platform (macOS, Windows, Linux)
5. Handle edge cases (permissions denied, no notification daemon, etc.)
