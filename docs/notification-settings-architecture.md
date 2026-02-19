# Notification Settings Architecture

## System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         Frontend (TypeScript)                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌────────────────────────────────────────────────────────┐    │
│  │         Settings UI Component (React)                   │    │
│  │  - Toggle switches for enable/disable                   │    │
│  │  - Number input for advance hours                       │    │
│  │  - Time pickers for DND and daily review               │    │
│  └────────────────────────────────────────────────────────┘    │
│                            │                                     │
│                            │ invoke()                            │
│                            ▼                                     │
│  ┌────────────────────────────────────────────────────────┐    │
│  │         notification-settings.ts                        │    │
│  │  - TypeScript types                                     │    │
│  │  - Validation helpers                                   │    │
│  │  - Default constants                                    │    │
│  └────────────────────────────────────────────────────────┘    │
│                            │                                     │
└────────────────────────────┼─────────────────────────────────────┘
                             │ Tauri IPC
                             │
┌────────────────────────────┼─────────────────────────────────────┐
│                            ▼                  Rust Backend        │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌────────────────────────────────────────────────────────┐    │
│  │              main.rs (Command Registration)             │    │
│  │  .invoke_handler(tauri::generate_handler![              │    │
│  │      get_notification_settings,                         │    │
│  │      update_notification_settings,                      │    │
│  │  ])                                                      │    │
│  └────────────────────────────────────────────────────────┘    │
│                            │                                     │
│                            ▼                                     │
│  ┌────────────────────────────────────────────────────────┐    │
│  │           commands/settings.rs                          │    │
│  │                                                          │    │
│  │  #[tauri::command]                                      │    │
│  │  get_notification_settings(db: State) -> Result<>      │    │
│  │    ├─ Load from database                                │    │
│  │    ├─ Parse values                                      │    │
│  │    └─ Return NotificationSettings                       │    │
│  │                                                          │    │
│  │  #[tauri::command]                                      │    │
│  │  update_notification_settings(db, settings) -> Result  │    │
│  │    ├─ Validate settings                                 │    │
│  │    ├─ Save to database                                  │    │
│  │    └─ Return success/error                              │    │
│  │                                                          │    │
│  │  NotificationSettings struct:                           │    │
│  │    ├─ enabled: bool                                     │    │
│  │    ├─ deadline_enabled: bool                            │    │
│  │    ├─ deadline_advance_hours: i32                       │    │
│  │    ├─ daily_review_enabled: bool                        │    │
│  │    ├─ daily_review_time: String                         │    │
│  │    ├─ task_completion_enabled: bool                     │    │
│  │    ├─ sound_enabled: bool                               │    │
│  │    ├─ dnd_enabled: bool                                 │    │
│  │    ├─ dnd_start_time: String                            │    │
│  │    └─ dnd_end_time: String                              │    │
│  │                                                          │    │
│  │  Helper Methods:                                        │    │
│  │    ├─ validate() -> Result<()>                          │    │
│  │    ├─ is_valid_time_format() -> bool                    │    │
│  │    ├─ is_dnd_active() -> bool                           │    │
│  │    └─ should_notify() -> bool                           │    │
│  └────────────────────────────────────────────────────────┘    │
│                            │                                     │
│                            ▼                                     │
│  ┌────────────────────────────────────────────────────────┐    │
│  │                  db/mod.rs                              │    │
│  │                                                          │    │
│  │  get_setting(key: &str) -> Result<Option<String>>      │    │
│  │  set_setting(key: &str, value: &str) -> Result<()>     │    │
│  │  get_settings_by_prefix(prefix: &str) -> Result<Vec>   │    │
│  │                                                          │    │
│  │  run_migrations():                                      │    │
│  │    └─ Apply v5_add_notification_settings.sql           │    │
│  └────────────────────────────────────────────────────────┘    │
│                            │                                     │
│                            ▼                                     │
│  ┌────────────────────────────────────────────────────────┐    │
│  │              SQLite Database (intento.db)               │    │
│  │                                                          │    │
│  │  settings table:                                        │    │
│  │  ┌──────────────────────┬───────────┬─────────────┐   │    │
│  │  │ key                  │ value     │ updated_at  │   │    │
│  │  ├──────────────────────┼───────────┼─────────────┤   │    │
│  │  │ notification_enabled │ "true"    │ 1708369200  │   │    │
│  │  │ notification_deadl..│ "true"    │ 1708369200  │   │    │
│  │  │ notification_deadl..│ "24"      │ 1708369200  │   │    │
│  │  │ notification_daily..│ "true"    │ 1708369200  │   │    │
│  │  │ notification_daily..│ "09:00"   │ 1708369200  │   │    │
│  │  │ notification_task_..│ "true"    │ 1708369200  │   │    │
│  │  │ notification_sound..│ "true"    │ 1708369200  │   │    │
│  │  │ notification_dnd_e..│ "false"   │ 1708369200  │   │    │
│  │  │ notification_dnd_s..│ "22:00"   │ 1708369200  │   │    │
│  │  │ notification_dnd_e..│ "08:00"   │ 1708369200  │   │    │
│  │  └──────────────────────┴───────────┴─────────────┘   │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                   │
└───────────────────────────────────────────────────────────────────┘
```

## Data Flow Diagrams

### Get Notification Settings Flow

```
┌─────────┐         ┌──────────┐         ┌──────────┐         ┌──────────┐
│ Frontend│         │ Tauri    │         │ Settings │         │ Database │
│   UI    │         │   IPC    │         │  Module  │         │          │
└────┬────┘         └────┬─────┘         └────┬─────┘         └────┬─────┘
     │                   │                     │                     │
     │ invoke('get_n..') │                     │                     │
     ├──────────────────>│                     │                     │
     │                   │  get_notification_  │                     │
     │                   │      settings()     │                     │
     │                   ├────────────────────>│                     │
     │                   │                     │ get_settings_by_    │
     │                   │                     │    prefix("notif..")│
     │                   │                     ├────────────────────>│
     │                   │                     │                     │
     │                   │                     │  Vec<(key, value)>  │
     │                   │                     │<────────────────────┤
     │                   │                     │                     │
     │                   │  Parse into         │                     │
     │                   │  NotificationSettings│                    │
     │                   │                     │                     │
     │                   │   Result<Settings>  │                     │
     │                   │<────────────────────┤                     │
     │                   │                     │                     │
     │  JSON response    │                     │                     │
     │<──────────────────┤                     │                     │
     │                   │                     │                     │
     │  Render UI        │                     │                     │
     │                   │                     │                     │
```

### Update Notification Settings Flow

```
┌─────────┐         ┌──────────┐         ┌──────────┐         ┌──────────┐
│ Frontend│         │ Tauri    │         │ Settings │         │ Database │
│   UI    │         │   IPC    │         │  Module  │         │          │
└────┬────┘         └────┬─────┘         └────┬─────┘         └────┬─────┘
     │                   │                     │                     │
     │ invoke('update_', │                     │                     │
     │    { settings })  │                     │                     │
     ├──────────────────>│                     │                     │
     │                   │  update_notification│                     │
     │                   │      _settings()    │                     │
     │                   ├────────────────────>│                     │
     │                   │                     │                     │
     │                   │                     │ validate()          │
     │                   │                     ├────┐                │
     │                   │                     │<───┘                │
     │                   │                     │                     │
     │                   │                     │ set_setting() x 10  │
     │                   │                     ├────────────────────>│
     │                   │                     │                     │
     │                   │                     │   Result<()>        │
     │                   │                     │<────────────────────┤
     │                   │                     │                     │
     │                   │     Result<()>      │                     │
     │                   │<────────────────────┤                     │
     │                   │                     │                     │
     │   Success/Error   │                     │                     │
     │<──────────────────┤                     │                     │
     │                   │                     │                     │
     │  Show toast/alert │                     │                     │
     │                   │                     │                     │
```

## Validation Flow

```
┌─────────────────────────────────────────────────────────────┐
│              NotificationSettings::validate()                │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  1. Check deadline_advance_hours                             │
│     ├─ Must be >= 1                                          │
│     ├─ Must be <= 168 (1 week)                               │
│     └─ Error if out of range                                 │
│                                                               │
│  2. Validate daily_review_time                               │
│     ├─ Must match HH:MM format                               │
│     ├─ Hours: 0-23                                           │
│     ├─ Minutes: 0-59                                         │
│     └─ Error if invalid                                      │
│                                                               │
│  3. Validate dnd_start_time                                  │
│     ├─ Must match HH:MM format                               │
│     ├─ Hours: 0-23                                           │
│     ├─ Minutes: 0-59                                         │
│     └─ Error if invalid                                      │
│                                                               │
│  4. Validate dnd_end_time                                    │
│     ├─ Must match HH:MM format                               │
│     ├─ Hours: 0-23                                           │
│     ├─ Minutes: 0-59                                         │
│     └─ Error if invalid                                      │
│                                                               │
│  Return: Ok(()) or Err(String)                               │
└─────────────────────────────────────────────────────────────┘
```

## DND (Do Not Disturb) Logic

```
┌─────────────────────────────────────────────────────────────┐
│           NotificationSettings::is_dnd_active()              │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  if !dnd_enabled:                                            │
│      return false                                            │
│                                                               │
│  current_time = now.format("%H:%M")                          │
│                                                               │
│  if dnd_start_time <= dnd_end_time:                          │
│      // Same-day period (e.g., 10:00 to 18:00)              │
│      return current_time >= start && current_time < end      │
│  else:                                                        │
│      // Overnight period (e.g., 22:00 to 08:00)             │
│      return current_time >= start || current_time < end      │
└─────────────────────────────────────────────────────────────┘

Examples:
  Same-day DND:  10:00 - 18:00
    11:00 → IN DND
    20:00 → NOT in DND

  Overnight DND: 22:00 - 08:00
    23:30 → IN DND
    07:00 → IN DND
    10:00 → NOT in DND
```

## Integration with Notification System

```
┌─────────────────────────────────────────────────────────────┐
│         Notification Sending Decision Tree                   │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  1. Load NotificationSettings from DB                        │
│     └─ get_notification_settings()                           │
│                                                               │
│  2. Check if should_notify()                                 │
│     ├─ Is globally enabled?                                  │
│     └─ Is NOT in DND period?                                 │
│                                                               │
│  3. Check notification type                                  │
│     ├─ Deadline: deadline_enabled?                           │
│     ├─ Daily Review: daily_review_enabled?                   │
│     └─ Task Completion: task_completion_enabled?             │
│                                                               │
│  4. Check timing                                             │
│     ├─ Deadline: now + advance_hours >= deadline?            │
│     └─ Daily Review: now == daily_review_time?               │
│                                                               │
│  5. Send notification if all checks pass                     │
│     └─ Use sound_enabled for audio                           │
└─────────────────────────────────────────────────────────────┘
```

## Testing Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Test Structure                           │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  Unit Tests (11 tests)                                       │
│  ├─ Basic Functionality                                      │
│  │  ├─ test_notification_settings_default                    │
│  │  ├─ test_notification_settings_validation_valid           │
│  │  └─ test_serialization                                    │
│  │                                                            │
│  ├─ Validation Logic                                         │
│  │  ├─ test_notification_settings_validation_invalid_advance │
│  │  ├─ test_notification_settings_validation_invalid_time    │
│  │  └─ test_is_valid_time_format                             │
│  │                                                            │
│  ├─ Notification Logic                                       │
│  │  ├─ test_should_notify_when_enabled                       │
│  │  └─ test_should_notify_when_disabled                      │
│  │                                                            │
│  └─ Database Integration                                     │
│     ├─ test_get_notification_settings_default                │
│     ├─ test_update_and_get_notification_settings             │
│     └─ test_settings_persistence                             │
│                                                               │
│  Test Database: /tmp/intento_test_*.db                       │
│  Cleanup: Automatic on test completion                       │
└─────────────────────────────────────────────────────────────┘
```

## File Structure

```
intento/
├── src-tauri/
│   ├── src/
│   │   ├── commands/
│   │   │   └── settings.rs          ← NotificationSettings implementation
│   │   ├── db/
│   │   │   └── mod.rs                ← Database operations
│   │   └── main.rs                   ← Command registration
│   │
│   └── migrations/
│       └── v5_add_notification_settings.sql  ← Schema migration
│
├── src/
│   └── types/
│       └── notification-settings.ts  ← TypeScript definitions
│
└── docs/
    ├── notification-settings-api.md         ← API documentation
    └── notification-settings-implementation.md  ← Implementation guide
```

## State Management

```
┌─────────────────────────────────────────────────────────────┐
│                Application State Flow                        │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  App Startup:                                                │
│  1. Load database (intento.db)                               │
│  2. Run migrations (including v5)                            │
│  3. Default settings inserted if not exist                   │
│  4. Database managed in Tauri State                          │
│                                                               │
│  Settings Access:                                            │
│  - Commands receive State<'_, Database>                      │
│  - State provides thread-safe access via Arc<Mutex<>>        │
│  - Multiple commands can access simultaneously               │
│                                                               │
│  Settings Persistence:                                       │
│  - Every update writes to database immediately               │
│  - No in-memory caching (database is source of truth)        │
│  - Settings survive app restart                              │
└─────────────────────────────────────────────────────────────┘
```

This architecture provides a clean separation of concerns, type safety across the stack, comprehensive validation, and robust error handling.
