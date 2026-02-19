# Quick Start: Notification Settings

Get your notification settings up and running in 5 minutes!

## Already Done ✅

The React frontend is **completely implemented and working**:
- NotificationSettings component created
- Integrated into SettingsPanel
- TypeScript types defined
- Animations added
- Build verified successful

## What You Need To Do

### Step 1: Implement Backend Commands (15 minutes)

Add these three commands to your Rust backend (`src-tauri/src/main.rs` or similar):

```rust
// Add to your Tauri command handler
use tauri::Manager;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub enabled: bool,
    pub reminder_enabled: bool,
    pub reminder_minutes: i32,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            reminder_enabled: true,
            reminder_minutes: 60, // 1 hour before
        }
    }
}

#[tauri::command]
async fn get_notification_settings() -> Result<NotificationConfig, String> {
    // For now, return default settings
    // TODO: Load from persistent storage
    Ok(NotificationConfig::default())
}

#[tauri::command]
async fn update_notification_settings(settings: NotificationConfig) -> Result<(), String> {
    // For now, just log the settings
    println!("Saving notification settings: {:?}", settings);
    // TODO: Save to persistent storage
    Ok(())
}

#[tauri::command]
async fn test_notification(app_handle: tauri::AppHandle) -> Result<(), String> {
    use tauri::Notification;

    Notification::new(&app_handle.config().identifier)
        .title("Test Notification")
        .body("Notifications are working! You'll receive reminders for your task deadlines.")
        .show()
        .map_err(|e| e.to_string())?;

    Ok(())
}

// In your main() function, add to invoke_handler:
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // ... your existing commands ...
            get_notification_settings,
            update_notification_settings,
            test_notification,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Step 2: Test the Feature (2 minutes)

1. Build and run your app:
   ```bash
   npm run tauri dev
   ```

2. Open Settings (⌘, or click settings icon)

3. Find the Notifications section

4. Toggle notifications on/off - should see visual changes

5. Click "Test Notification" - should see a system notification

### Step 3: Add Persistent Storage (Optional, 10 minutes)

To save settings between app restarts, update your backend:

```rust
use std::path::PathBuf;
use tauri::api::path::app_data_dir;

fn get_config_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let mut path = app_data_dir(&app_handle.config())
        .expect("Failed to get app data dir");
    path.push("notification_config.json");
    path
}

#[tauri::command]
async fn get_notification_settings(app_handle: tauri::AppHandle) -> Result<NotificationConfig, String> {
    let config_path = get_config_path(&app_handle);

    if config_path.exists() {
        let json = std::fs::read_to_string(&config_path)
            .map_err(|e| e.to_string())?;
        serde_json::from_str(&json)
            .map_err(|e| e.to_string())
    } else {
        Ok(NotificationConfig::default())
    }
}

#[tauri::command]
async fn update_notification_settings(
    settings: NotificationConfig,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let config_path = get_config_path(&app_handle);

    // Create parent directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| e.to_string())?;
    }

    let json = serde_json::to_string_pretty(&settings)
        .map_err(|e| e.to_string())?;
    std::fs::write(&config_path, json)
        .map_err(|e| e.to_string())?;

    Ok(())
}
```

### Step 4: Handle Permissions (Platform-specific)

#### macOS
Notifications should work automatically. If not, check System Settings > Notifications > [Your App]

#### Windows
Windows 10+ supports notifications out of the box via WinRT.

#### Linux
Ensure a notification daemon is running:
```bash
# Check if running
pgrep -a notify-osd  # or dunst, mako, etc.

# Install if needed (Ubuntu/Debian)
sudo apt install notify-osd
```

## Testing Checklist

- [ ] Settings panel opens
- [ ] Notifications section is visible
- [ ] Master toggle works
- [ ] Reminder settings appear when enabled
- [ ] Dropdown changes are saved
- [ ] Test button sends notification
- [ ] Success message appears after test
- [ ] Settings persist after app restart (if implemented)

## Common Issues

### "Command not found" error
**Problem:** Frontend calls backend command but gets error.
**Solution:** Make sure command is in `generate_handler![]` macro.

### Test notification doesn't appear
**Problem:** System notifications are disabled.
**Solution:** Check OS notification settings for your app.

### Settings don't persist
**Problem:** Storage not implemented yet.
**Solution:** This is expected if you're using the basic implementation. Follow Step 3 to add persistence.

## Next Steps

1. **Basic** (what you have now): Test notifications work
2. **Persistent** (Step 3): Settings save between restarts
3. **Scheduler** (future): Automatic reminders for task deadlines

See `notification-backend-implementation.md` for full scheduler implementation.

## Need Help?

Check these files for more details:
- **Component docs**: `docs/notification-settings-readme.md`
- **Backend guide**: `docs/notification-backend-implementation.md`
- **Implementation summary**: `docs/notification-settings-implementation-summary.md`

## File Locations

```
Frontend (already done):
  src/components/NotificationSettings.tsx
  src/components/SettingsPanel.tsx (updated)
  src/types/notification.ts
  src/App.css (updated)

Backend (your work):
  src-tauri/src/main.rs (add commands here)
  src-tauri/src/notification.rs (optional separate module)
```

## Estimated Time

- ⚡ **Quick start** (basic): 15 minutes
- 🔧 **With persistence**: 25 minutes
- 🚀 **Full scheduler**: 2-3 hours (see full backend guide)

## Success Criteria

You'll know it's working when:
1. ✓ Settings panel shows notification section
2. ✓ Toggles change the UI visually
3. ✓ Test button sends a notification
4. ✓ Notification appears in system notification center
5. ✓ Settings save between app restarts (if implemented)

Happy coding! 🎉
