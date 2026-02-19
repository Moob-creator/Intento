# Notification Settings API Documentation

This document describes the Rust backend commands for managing notification preferences in Intento.

## Overview

The notification settings system provides comprehensive control over all notification behaviors in the application, including deadline reminders, daily reviews, task completions, and Do Not Disturb periods.

## Rust Backend Implementation

### Files Structure

```
src-tauri/
├── src/
│   ├── commands/
│   │   └── settings.rs          # Notification settings commands
│   ├── db/
│   │   └── mod.rs               # Database operations
│   └── main.rs                  # Command registration
├── migrations/
│   └── v5_add_notification_settings.sql  # Database migration
```

### Database Schema

Settings are stored in the `settings` table as key-value pairs:

```sql
CREATE TABLE settings (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);
```

**Notification Setting Keys:**
- `notification_enabled` - Global toggle
- `notification_deadline_enabled` - Deadline reminders toggle
- `notification_deadline_advance_hours` - Hours before deadline (1-168)
- `notification_daily_review_enabled` - Daily review toggle
- `notification_daily_review_time` - Daily review time (HH:MM)
- `notification_task_completion_enabled` - Task completion notifications
- `notification_sound_enabled` - Sound toggle
- `notification_dnd_enabled` - Do Not Disturb toggle
- `notification_dnd_start_time` - DND start time (HH:MM)
- `notification_dnd_end_time` - DND end time (HH:MM)

## Tauri Commands

### 1. Get Notification Settings

Retrieves current notification preferences from the database.

**Command:** `get_notification_settings`

**Rust Signature:**
```rust
#[tauri::command]
pub fn get_notification_settings(db: State<'_, Database>)
    -> Result<NotificationSettings, String>
```

**TypeScript Usage:**
```typescript
import { invoke } from '@tauri-apps/api/core';
import type { NotificationSettings } from './types/notification-settings';

async function getSettings(): Promise<NotificationSettings> {
  try {
    const settings = await invoke<NotificationSettings>('get_notification_settings');
    console.log('Current settings:', settings);
    return settings;
  } catch (error) {
    console.error('Failed to get notification settings:', error);
    throw error;
  }
}
```

**Response Example:**
```json
{
  "enabled": true,
  "deadline_enabled": true,
  "deadline_advance_hours": 24,
  "daily_review_enabled": true,
  "daily_review_time": "09:00",
  "task_completion_enabled": true,
  "sound_enabled": true,
  "dnd_enabled": false,
  "dnd_start_time": "22:00",
  "dnd_end_time": "08:00"
}
```

### 2. Update Notification Settings

Updates notification preferences in the database with validation.

**Command:** `update_notification_settings`

**Rust Signature:**
```rust
#[tauri::command]
pub fn update_notification_settings(
    db: State<'_, Database>,
    settings: NotificationSettings,
) -> Result<(), String>
```

**TypeScript Usage:**
```typescript
import { invoke } from '@tauri-apps/api/core';
import type { NotificationSettings } from './types/notification-settings';

async function updateSettings(settings: NotificationSettings): Promise<void> {
  try {
    await invoke('update_notification_settings', { settings });
    console.log('Settings updated successfully');
  } catch (error) {
    console.error('Failed to update notification settings:', error);
    throw error;
  }
}

// Example: Disable all notifications
await updateSettings({
  enabled: false,
  deadline_enabled: true,
  deadline_advance_hours: 24,
  daily_review_enabled: true,
  daily_review_time: "09:00",
  task_completion_enabled: true,
  sound_enabled: true,
  dnd_enabled: false,
  dnd_start_time: "22:00",
  dnd_end_time: "08:00"
});

// Example: Configure Do Not Disturb
await updateSettings({
  enabled: true,
  deadline_enabled: true,
  deadline_advance_hours: 48,  // 2 days before
  daily_review_enabled: true,
  daily_review_time: "14:30",
  task_completion_enabled: true,
  sound_enabled: true,
  dnd_enabled: true,
  dnd_start_time: "22:00",
  dnd_end_time: "08:00"
});
```

## Data Structures

### NotificationSettings

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub enabled: bool,
    pub deadline_enabled: bool,
    pub deadline_advance_hours: i32,
    pub daily_review_enabled: bool,
    pub daily_review_time: String,
    pub task_completion_enabled: bool,
    pub sound_enabled: bool,
    pub dnd_enabled: bool,
    pub dnd_start_time: String,
    pub dnd_end_time: String,
}
```

### Validation Rules

The `NotificationSettings::validate()` method enforces:

1. **deadline_advance_hours**: Must be between 1 and 168 (1 hour to 1 week)
2. **daily_review_time**: Must be in HH:MM format (24-hour), valid hours (0-23) and minutes (0-59)
3. **dnd_start_time**: Must be in HH:MM format (24-hour)
4. **dnd_end_time**: Must be in HH:MM format (24-hour)

**Example Validation Errors:**
```rust
// Invalid advance hours
settings.deadline_advance_hours = 200;
// Error: "Invalid deadline_advance_hours: 200. Must be between 1 and 168."

// Invalid time format
settings.daily_review_time = "25:00";
// Error: "Invalid daily_review_time format: 25:00. Expected HH:MM (24-hour format)."
```

## Helper Methods

### should_notify()

Determines if notifications should be sent based on current settings.

```rust
impl NotificationSettings {
    pub fn should_notify(&self) -> bool {
        self.enabled && !self.is_dnd_active()
    }
}
```

### is_dnd_active()

Checks if current time falls within Do Not Disturb period.

```rust
impl NotificationSettings {
    pub fn is_dnd_active(&self) -> bool {
        if !self.dnd_enabled {
            return false;
        }

        let now = chrono::Local::now();
        let current_time = now.format("%H:%M").to_string();

        // Handles both same-day and overnight DND periods
        if self.dnd_start_time <= self.dnd_end_time {
            current_time >= self.dnd_start_time && current_time < self.dnd_end_time
        } else {
            current_time >= self.dnd_start_time || current_time < self.dnd_end_time
        }
    }
}
```

## Unit Tests

The implementation includes comprehensive unit tests covering:

### Basic Functionality Tests
- `test_notification_settings_default()` - Verifies default values
- `test_notification_settings_validation_valid()` - Tests valid settings
- `test_serialization()` - Tests JSON serialization/deserialization

### Validation Tests
- `test_notification_settings_validation_invalid_advance_hours()` - Tests boundary conditions
- `test_notification_settings_validation_invalid_time_format()` - Tests time format validation
- `test_is_valid_time_format()` - Tests time format parsing

### Logic Tests
- `test_should_notify_when_enabled()` - Tests notification permission logic
- `test_should_notify_when_disabled()` - Tests disabled state

### Database Integration Tests
- `test_get_notification_settings_default()` - Tests retrieving default settings from DB
- `test_update_and_get_notification_settings()` - Tests full CRUD cycle
- `test_settings_persistence()` - Tests database persistence across connections

### Running Tests

```bash
# Run all notification settings tests
cargo test settings::tests --lib -- --nocapture

# Run specific test
cargo test settings::tests::test_notification_settings_validation_valid --lib

# Run all tests
cargo test --lib
```

## Usage Examples

### Example 1: Settings Panel Component (React)

```typescript
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { NotificationSettings } from './types/notification-settings';

function NotificationSettingsPanel() {
  const [settings, setSettings] = useState<NotificationSettings | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadSettings();
  }, []);

  async function loadSettings() {
    try {
      const data = await invoke<NotificationSettings>('get_notification_settings');
      setSettings(data);
    } catch (error) {
      console.error('Failed to load settings:', error);
    }
  }

  async function saveSettings() {
    if (!settings) return;

    setLoading(true);
    try {
      await invoke('update_notification_settings', { settings });
      alert('Settings saved successfully!');
    } catch (error) {
      alert(`Failed to save settings: ${error}`);
    } finally {
      setLoading(false);
    }
  }

  if (!settings) return <div>Loading...</div>;

  return (
    <div className="settings-panel">
      <h2>Notification Settings</h2>

      <label>
        <input
          type="checkbox"
          checked={settings.enabled}
          onChange={(e) => setSettings({ ...settings, enabled: e.target.checked })}
        />
        Enable Notifications
      </label>

      <h3>Deadline Reminders</h3>
      <label>
        <input
          type="checkbox"
          checked={settings.deadline_enabled}
          onChange={(e) => setSettings({ ...settings, deadline_enabled: e.target.checked })}
        />
        Enable Deadline Reminders
      </label>

      <label>
        Notify
        <input
          type="number"
          min="1"
          max="168"
          value={settings.deadline_advance_hours}
          onChange={(e) => setSettings({
            ...settings,
            deadline_advance_hours: parseInt(e.target.value)
          })}
        />
        hours before deadline
      </label>

      <h3>Daily Review</h3>
      <label>
        <input
          type="checkbox"
          checked={settings.daily_review_enabled}
          onChange={(e) => setSettings({
            ...settings,
            daily_review_enabled: e.target.checked
          })}
        />
        Enable Daily Review Reminder
      </label>

      <label>
        Review Time:
        <input
          type="time"
          value={settings.daily_review_time}
          onChange={(e) => setSettings({
            ...settings,
            daily_review_time: e.target.value
          })}
        />
      </label>

      <h3>Do Not Disturb</h3>
      <label>
        <input
          type="checkbox"
          checked={settings.dnd_enabled}
          onChange={(e) => setSettings({ ...settings, dnd_enabled: e.target.checked })}
        />
        Enable Do Not Disturb
      </label>

      <label>
        From:
        <input
          type="time"
          value={settings.dnd_start_time}
          onChange={(e) => setSettings({
            ...settings,
            dnd_start_time: e.target.value
          })}
        />
      </label>

      <label>
        To:
        <input
          type="time"
          value={settings.dnd_end_time}
          onChange={(e) => setSettings({
            ...settings,
            dnd_end_time: e.target.value
          })}
        />
      </label>

      <button onClick={saveSettings} disabled={loading}>
        {loading ? 'Saving...' : 'Save Settings'}
      </button>
    </div>
  );
}
```

### Example 2: Checking Before Sending Notifications

```rust
use crate::commands::settings::NotificationSettings;

async fn send_task_reminder(
    app: AppHandle,
    db: State<'_, Database>,
    task_id: i64,
) -> Result<(), String> {
    // Load notification settings
    let settings = get_notification_settings(db.clone())?;

    // Check if notifications are allowed
    if !settings.should_notify() {
        println!("Notifications disabled or in DND period");
        return Ok(());
    }

    // Check if deadline notifications are enabled
    if !settings.deadline_enabled {
        println!("Deadline notifications disabled");
        return Ok(());
    }

    // Send the notification
    send_notification(
        app,
        "Task Deadline".to_string(),
        "Your task is due soon!".to_string(),
        Some(NotificationType::Deadline),
    ).await
}
```

### Example 3: Testing Notification Settings

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_notification_workflow() {
        let temp_dir = std::env::temp_dir();
        let db_path = temp_dir.join("intento_test_workflow.db");
        let db = Database::new(db_path.clone()).unwrap();

        // Get default settings
        let mut settings = NotificationSettings::default();
        assert!(settings.enabled);

        // Modify settings
        settings.deadline_advance_hours = 48;
        settings.dnd_enabled = true;

        // Validate
        assert!(settings.validate().is_ok());

        // Save to database
        db.set_setting("notification_deadline_advance_hours", "48").unwrap();
        db.set_setting("notification_dnd_enabled", "true").unwrap();

        // Retrieve and verify
        let saved_hours = db.get_setting("notification_deadline_advance_hours")
            .unwrap()
            .unwrap();
        assert_eq!(saved_hours, "48");

        // Clean up
        std::fs::remove_file(db_path).ok();
    }
}
```

## Error Handling

All commands return `Result<T, String>` for proper error propagation:

```typescript
try {
  await invoke('update_notification_settings', { settings });
} catch (error) {
  // Error messages are descriptive:
  // - "Invalid deadline_advance_hours: 200. Must be between 1 and 168."
  // - "Invalid daily_review_time format: 25:00. Expected HH:MM (24-hour format)."
  // - "Failed to set enabled: database error..."
  console.error('Settings update failed:', error);
}
```

## Performance Considerations

1. **Database Operations**: All settings are stored as individual key-value pairs for efficient updates
2. **Validation**: Settings are validated before database writes to prevent invalid states
3. **Caching**: Settings should be cached in frontend to minimize database calls
4. **Indexing**: The settings table uses an index on the `key` column for fast lookups

## Migration Strategy

The v5 migration creates default settings on first run. Existing installations will automatically receive default values when upgrading.

```sql
INSERT OR IGNORE INTO settings (key, value) VALUES
    ('notification_enabled', 'true'),
    -- ... other defaults
```

The `OR IGNORE` clause ensures existing custom settings are preserved.

## Dependencies

**Cargo.toml additions:**
```toml
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rusqlite = { version = "0.38", features = ["bundled"] }
chrono = { version = "0.4", features = ["serde"] }
```

## Summary

This implementation provides:
- Type-safe notification settings management
- Comprehensive validation
- Database persistence
- Do Not Disturb functionality
- 11 comprehensive unit tests
- Full integration with Tauri IPC
- Production-ready error handling

All tests pass successfully, and the code follows Rust best practices with proper error handling, documentation, and type safety.
