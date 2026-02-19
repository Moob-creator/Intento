/// Tauri Command Integration Tests
///
/// Tests Tauri command handlers to ensure proper integration with
/// database and AI services. These tests simulate frontend invoke calls.

use intento::commands::ai::AiClientState;
use intento::db::{
    models::{Priority, Task, TaskStatus},
    Database,
};
use tempfile::TempDir;

/// Helper to create test database
fn create_test_db() -> (Database, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test_commands.db");
    let db = Database::new(db_path).expect("Failed to create test database");
    (db, temp_dir)
}

// ============================================================================
// Task Command Tests
// ============================================================================

#[tokio::test]
async fn test_create_task_command() {
    use intento::commands::task::create_task;
    use tauri::State;

    let (db, _temp_dir) = create_test_db();
    let state = State::from(db);

    // Test basic task creation
    let result = create_task(
        state.clone(),
        "Write documentation".to_string(),
        Some("API documentation for v2".to_string()),
        Some("high".to_string()),
        None,
        Some(vec!["docs".to_string(), "work".to_string()]),
    )
    .await;

    assert!(result.is_ok());
    let task_id = result.unwrap();
    assert!(task_id > 0);

    // Verify task was created correctly
    let task = state.get_task(task_id).expect("Failed to get task");
    assert!(task.is_some());
    let task = task.unwrap();
    assert_eq!(task.title, "Write documentation");
    assert_eq!(task.priority, Priority::High);
    assert_eq!(task.tags, Some(vec!["docs".to_string(), "work".to_string()]));
}

#[tokio::test]
async fn test_create_task_with_deadline() {
    use intento::commands::task::create_task;
    use tauri::State;

    let (db, _temp_dir) = create_test_db();
    let state = State::from(db);

    let deadline = chrono::Utc::now().timestamp() + 86400; // Tomorrow

    let result = create_task(
        state.clone(),
        "Complete report".to_string(),
        None,
        Some("high".to_string()),
        Some(deadline),
        None,
    )
    .await;

    assert!(result.is_ok());
    let task_id = result.unwrap();

    // Verify reminder time was automatically set
    let task = state.get_task(task_id).expect("Failed to get task").unwrap();
    assert!(task.reminder_time.is_some());
    assert_eq!(task.deadline, Some(deadline));
}

#[tokio::test]
async fn test_get_task_command() {
    use intento::commands::task::{create_task, get_task};
    use tauri::State;

    let (db, _temp_dir) = create_test_db();
    let state = State::from(db);

    // Create a task
    let task_id = create_task(
        state.clone(),
        "Test task".to_string(),
        Some("Description".to_string()),
        None,
        None,
        None,
    )
    .await
    .unwrap();

    // Retrieve the task
    let result = get_task(state.clone(), task_id).await;
    assert!(result.is_ok());

    let task = result.unwrap();
    assert!(task.is_some());
    let task = task.unwrap();
    assert_eq!(task.title, "Test task");
    assert_eq!(task.description, Some("Description".to_string()));
}

#[tokio::test]
async fn test_update_task_command() {
    use intento::commands::task::{create_task, get_task, update_task};
    use tauri::State;

    let (db, _temp_dir) = create_test_db();
    let state = State::from(db);

    // Create a task
    let task_id = create_task(
        state.clone(),
        "Original title".to_string(),
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();

    // Update the task
    let result = update_task(
        state.clone(),
        task_id,
        Some("Updated title".to_string()),
        Some("New description".to_string()),
        Some("doing".to_string()),
        Some("high".to_string()),
        None,
        Some(vec!["updated".to_string()]),
        None,
    )
    .await;

    assert!(result.is_ok());

    // Verify updates
    let task = get_task(state.clone(), task_id).await.unwrap().unwrap();
    assert_eq!(task.title, "Updated title");
    assert_eq!(task.description, Some("New description".to_string()));
    assert_eq!(task.status, TaskStatus::Doing);
    assert_eq!(task.priority, Priority::High);
    assert_eq!(task.tags, Some(vec!["updated".to_string()]));
}

#[tokio::test]
async fn test_update_task_to_done() {
    use intento::commands::task::{create_task, get_task, update_task};
    use tauri::State;

    let (db, _temp_dir) = create_test_db();
    let state = State::from(db);

    // Create a task
    let task_id = create_task(state.clone(), "Task to complete".to_string(), None, None, None, None)
        .await
        .unwrap();

    // Update to done
    let result = update_task(
        state.clone(),
        task_id,
        None,
        None,
        Some("done".to_string()),
        None,
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok());

    // Verify completed_at is set
    let task = get_task(state.clone(), task_id).await.unwrap().unwrap();
    assert_eq!(task.status, TaskStatus::Done);
    assert!(task.completed_at.is_some());
}

#[tokio::test]
async fn test_delete_task_command() {
    use intento::commands::task::{create_task, delete_task, get_task};
    use tauri::State;

    let (db, _temp_dir) = create_test_db();
    let state = State::from(db);

    // Create a task
    let task_id = create_task(state.clone(), "Task to delete".to_string(), None, None, None, None)
        .await
        .unwrap();

    // Delete the task
    let result = delete_task(state.clone(), task_id).await;
    assert!(result.is_ok());

    // Verify task is soft deleted
    let task = get_task(state.clone(), task_id).await.unwrap();
    assert!(task.is_none());
}

#[tokio::test]
async fn test_list_tasks_command() {
    use intento::commands::task::{create_task, list_tasks};
    use tauri::State;

    let (db, _temp_dir) = create_test_db();
    let state = State::from(db);

    // Create tasks with different statuses
    create_task(state.clone(), "Todo task 1".to_string(), None, None, None, None)
        .await
        .unwrap();
    create_task(state.clone(), "Todo task 2".to_string(), None, None, None, None)
        .await
        .unwrap();

    let doing_id = create_task(state.clone(), "Doing task".to_string(), None, None, None, None)
        .await
        .unwrap();

    // Update one task to doing
    use intento::commands::task::update_task;
    update_task(
        state.clone(),
        doing_id,
        None,
        None,
        Some("doing".to_string()),
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();

    // List all tasks
    let all_tasks = list_tasks(state.clone(), None).await.unwrap();
    assert_eq!(all_tasks.len(), 3);

    // List only todo tasks
    let todo_tasks = list_tasks(state.clone(), Some("todo".to_string()))
        .await
        .unwrap();
    assert_eq!(todo_tasks.len(), 2);

    // List only doing tasks
    let doing_tasks = list_tasks(state.clone(), Some("doing".to_string()))
        .await
        .unwrap();
    assert_eq!(doing_tasks.len(), 1);
}

#[tokio::test]
async fn test_get_db_version_command() {
    use intento::commands::task::get_db_version;
    use tauri::State;

    let (db, _temp_dir) = create_test_db();
    let state = State::from(db);

    let result = get_db_version(state).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 5);
}

// ============================================================================
// AI Command Tests (require API keys - marked as ignore)
// ============================================================================

#[tokio::test]
#[ignore]
async fn test_parse_text_input_command() {
    use intento::commands::ai::parse_text_input;
    use tauri::State;

    let env_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".env");
    if env_path.exists() {
        dotenv::from_path(&env_path).ok();
    }

    let ai_state = AiClientState::new();
    let state = State::from(ai_state);

    let result = parse_text_input(
        "Complete the project proposal by Friday 3pm, high priority".to_string(),
        None,
        state,
    )
    .await;

    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert!(!parsed.title.is_empty());
    println!("Parsed task: {:?}", parsed);
}

#[tokio::test]
#[ignore]
async fn test_ai_health_check_command() {
    use intento::commands::ai::ai_health_check;
    use tauri::State;

    let env_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".env");
    if env_path.exists() {
        dotenv::from_path(&env_path).ok();
    }

    let ai_state = AiClientState::new();
    let state = State::from(ai_state);

    let result = ai_health_check(state).await;
    assert!(result.is_ok());
    println!("AI Health: {}", result.unwrap());
}

// ============================================================================
// Notification Command Tests
// ============================================================================

#[tokio::test]
async fn test_check_expiring_tasks_command() {
    use intento::commands::notification::check_expiring_tasks;
    use intento::commands::task::create_task;
    use tauri::State;

    let (db, _temp_dir) = create_test_db();
    let state = State::from(db);

    let now = chrono::Utc::now().timestamp();

    // Create task expiring soon
    create_task(
        state.clone(),
        "Expiring task".to_string(),
        None,
        None,
        Some(now + 1800), // 30 minutes from now
        None,
    )
    .await
    .unwrap();

    // Create task expiring later
    create_task(
        state.clone(),
        "Later task".to_string(),
        None,
        None,
        Some(now + 7200), // 2 hours from now
        None,
    )
    .await
    .unwrap();

    // Check for tasks expiring within 1 hour
    let result = check_expiring_tasks(state, 3600).await;
    assert!(result.is_ok());
    let expiring = result.unwrap();
    assert_eq!(expiring.len(), 1);
    assert!(expiring[0].title.contains("Expiring"));
}

// ============================================================================
// Settings Command Tests
// ============================================================================

#[tokio::test]
async fn test_settings_commands() {
    use intento::commands::settings::{
        get_auto_summary_settings, get_notification_settings, update_auto_summary_settings,
        update_notification_settings,
    };
    use tauri::State;

    let (db, _temp_dir) = create_test_db();
    let state = State::from(db);

    // Test auto summary settings
    let initial_summary = get_auto_summary_settings(state.clone()).await.unwrap();
    assert!(!initial_summary.enabled); // Default should be false

    // Update settings
    let update_result = update_auto_summary_settings(
        state.clone(),
        true,
        true,
        true,
        false,
        false,
        false,
    )
    .await;
    assert!(update_result.is_ok());

    // Verify update
    let updated_summary = get_auto_summary_settings(state.clone()).await.unwrap();
    assert!(updated_summary.enabled);
    assert!(updated_summary.daily);
    assert!(updated_summary.weekly);

    // Test notification settings
    let initial_notif = get_notification_settings(state.clone()).await.unwrap();
    assert!(initial_notif.enabled); // Default should be true

    // Update notification settings
    let notif_update = update_notification_settings(state.clone(), false, "custom".to_string(), 2)
        .await;
    assert!(notif_update.is_ok());

    // Verify notification update
    let updated_notif = get_notification_settings(state.clone()).await.unwrap();
    assert!(!updated_notif.enabled);
    assert_eq!(updated_notif.sound, "custom");
    assert_eq!(updated_notif.deadline_warning_hours, 2);
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[tokio::test]
async fn test_command_error_handling() {
    use intento::commands::task::{get_task, update_task};
    use tauri::State;

    let (db, _temp_dir) = create_test_db();
    let state = State::from(db);

    // Test getting non-existent task
    let result = get_task(state.clone(), 99999).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());

    // Test updating non-existent task
    let result = update_task(
        state.clone(),
        99999,
        Some("Updated".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await;
    // Should fail with error message
    assert!(result.is_err());
}

#[tokio::test]
async fn test_invalid_status_priority() {
    use intento::commands::task::{create_task, update_task};
    use tauri::State;

    let (db, _temp_dir) = create_test_db();
    let state = State::from(db);

    // Test invalid priority
    let result = create_task(
        state.clone(),
        "Test".to_string(),
        None,
        Some("invalid".to_string()),
        None,
        None,
    )
    .await;
    assert!(result.is_err());

    // Create valid task
    let task_id = create_task(state.clone(), "Test".to_string(), None, None, None, None)
        .await
        .unwrap();

    // Test invalid status
    let result = update_task(
        state.clone(),
        task_id,
        None,
        None,
        Some("invalid".to_string()),
        None,
        None,
        None,
        None,
    )
    .await;
    assert!(result.is_err());
}

// ============================================================================
// Concurrent Command Tests
// ============================================================================

#[tokio::test]
async fn test_concurrent_task_creation() {
    use intento::commands::task::create_task;
    use tauri::State;

    let (db, _temp_dir) = create_test_db();
    let state = State::from(db);

    // Create multiple tasks concurrently
    let mut handles = vec![];

    for i in 0..10 {
        let state_clone = state.clone();
        let handle = tokio::spawn(async move {
            create_task(
                state_clone,
                format!("Concurrent task {}", i),
                None,
                None,
                None,
                None,
            )
            .await
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    let mut success_count = 0;
    for handle in handles {
        if let Ok(result) = handle.await {
            if result.is_ok() {
                success_count += 1;
            }
        }
    }

    assert_eq!(success_count, 10);

    // Verify all tasks were created
    use intento::commands::task::list_tasks;
    let tasks = list_tasks(state, None).await.unwrap();
    assert_eq!(tasks.len(), 10);
}

#[tokio::test]
async fn test_concurrent_task_updates() {
    use intento::commands::task::{create_task, list_tasks, update_task};
    use tauri::State;

    let (db, _temp_dir) = create_test_db();
    let state = State::from(db);

    // Create tasks
    let mut task_ids = vec![];
    for i in 0..5 {
        let id = create_task(
            state.clone(),
            format!("Task {}", i),
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();
        task_ids.push(id);
    }

    // Update all tasks concurrently
    let mut handles = vec![];
    for task_id in task_ids {
        let state_clone = state.clone();
        let handle = tokio::spawn(async move {
            update_task(
                state_clone,
                task_id,
                None,
                None,
                Some("done".to_string()),
                None,
                None,
                None,
                None,
            )
            .await
        });
        handles.push(handle);
    }

    // Wait for all updates
    for handle in handles {
        handle.await.unwrap().unwrap();
    }

    // Verify all tasks are done
    let done_tasks = list_tasks(state, Some("done".to_string()))
        .await
        .unwrap();
    assert_eq!(done_tasks.len(), 5);
}
