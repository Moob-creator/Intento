/// Notification System Integration Tests
///
/// Tests the notification system including:
/// - Task reminder notifications
/// - Expiring task notifications
/// - Notification scheduling
/// - Notification settings integration

use intento::db::{
    models::{Priority, Task, TaskStatus},
    Database,
};
use tempfile::TempDir;

/// Helper to create a test database
fn create_test_db() -> (Database, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test_notification.db");
    let db = Database::new(db_path).expect("Failed to create test database");
    (db, temp_dir)
}

#[tokio::test]
async fn test_reminder_notification_workflow() {
    let (db, _temp_dir) = create_test_db();

    let now = chrono::Utc::now().timestamp();

    // Create task with upcoming deadline
    let mut task = Task::new("Important presentation".to_string());
    task.deadline = Some(now + 3600); // 1 hour from now
    task.reminder_time = Some(now - 300); // Reminder was 5 minutes ago (should trigger)
    task.priority = Priority::High;

    let task_id = db.create_task(&task).expect("Failed to create task");

    // Check for tasks needing reminders
    let tasks_to_remind = db
        .get_tasks_needing_reminder()
        .expect("Failed to get tasks needing reminder");

    assert_eq!(tasks_to_remind.len(), 1);
    assert_eq!(tasks_to_remind[0].id, Some(task_id));
    assert_eq!(tasks_to_remind[0].title, "Important presentation");

    // Simulate sending notification and clearing reminder
    db.clear_reminder(task_id)
        .expect("Failed to clear reminder");

    // Verify reminder is cleared
    let tasks_to_remind = db
        .get_tasks_needing_reminder()
        .expect("Failed to get tasks after clearing");
    assert!(tasks_to_remind.is_empty());
}

#[tokio::test]
async fn test_multiple_reminders() {
    let (db, _temp_dir) = create_test_db();

    let now = chrono::Utc::now().timestamp();

    // Create multiple tasks with different reminder times
    let tasks_data = vec![
        ("Task 1 - past reminder", now - 600, Priority::High),
        ("Task 2 - recent reminder", now - 60, Priority::Medium),
        ("Task 3 - future reminder", now + 600, Priority::Low),
        ("Task 4 - no reminder", 0, Priority::Medium),
    ];

    let mut expected_reminders = vec![];

    for (title, reminder_time, priority) in tasks_data {
        let mut task = Task::new(title.to_string());
        task.priority = priority.clone();
        task.deadline = Some(now + 3600);

        if reminder_time > 0 {
            task.reminder_time = Some(reminder_time);
        }

        let task_id = db.create_task(&task).expect("Failed to create task");

        // Tasks with past reminders should be included
        if reminder_time > 0 && reminder_time < now && reminder_time > now - 300 {
            expected_reminders.push(task_id);
        }
    }

    // Get tasks needing reminders
    let tasks_to_remind = db
        .get_tasks_needing_reminder()
        .expect("Failed to get tasks needing reminder");

    // Should get tasks with reminders in the past 5 minutes
    assert_eq!(tasks_to_remind.len(), 2); // Task 1 and Task 2
}

#[tokio::test]
async fn test_expiring_tasks_notification() {
    let (db, _temp_dir) = create_test_db();

    let now = chrono::Utc::now().timestamp();

    // Create tasks with various deadline times
    let tasks = vec![
        ("Expires in 5 minutes", now + 300, Priority::High),
        ("Expires in 30 minutes", now + 1800, Priority::Medium),
        ("Expires in 2 hours", now + 7200, Priority::Low),
        ("Expired task", now - 600, Priority::Medium),
    ];

    for (title, deadline, priority) in tasks {
        let mut task = Task::new(title.to_string());
        task.deadline = Some(deadline);
        task.priority = priority;
        task.status = TaskStatus::Todo;
        db.create_task(&task).expect("Failed to create task");
    }

    // Get tasks expiring within 1 hour
    let expiring_1h = db
        .get_expiring_tasks(3600)
        .expect("Failed to get expiring tasks");

    // Should get 2 tasks (5 minutes and 30 minutes, not expired)
    assert_eq!(expiring_1h.len(), 2);

    // Verify ordering (earliest deadline first)
    assert!(expiring_1h[0].deadline.unwrap() < expiring_1h[1].deadline.unwrap());
    assert!(expiring_1h[0].title.contains("5 minutes"));
}

#[tokio::test]
async fn test_completed_tasks_no_reminder() {
    let (db, _temp_dir) = create_test_db();

    let now = chrono::Utc::now().timestamp();

    // Create completed task with past reminder time
    let mut completed_task = Task::new("Completed task".to_string());
    completed_task.status = TaskStatus::Done;
    completed_task.completed_at = Some(now - 3600);
    completed_task.deadline = Some(now + 3600);
    completed_task.reminder_time = Some(now - 300); // Past reminder

    db.create_task(&completed_task)
        .expect("Failed to create completed task");

    // Create pending task with past reminder time
    let mut pending_task = Task::new("Pending task".to_string());
    pending_task.status = TaskStatus::Todo;
    pending_task.deadline = Some(now + 3600);
    pending_task.reminder_time = Some(now - 300); // Past reminder

    let pending_id = db
        .create_task(&pending_task)
        .expect("Failed to create pending task");

    // Get tasks needing reminders
    let tasks_to_remind = db
        .get_tasks_needing_reminder()
        .expect("Failed to get tasks needing reminder");

    // Should only get the pending task, not the completed one
    assert_eq!(tasks_to_remind.len(), 1);
    assert_eq!(tasks_to_remind[0].id, Some(pending_id));
    assert_eq!(tasks_to_remind[0].title, "Pending task");
}

#[tokio::test]
async fn test_deleted_tasks_no_reminder() {
    let (db, _temp_dir) = create_test_db();

    let now = chrono::Utc::now().timestamp();

    // Create task with past reminder
    let mut task = Task::new("Task to be deleted".to_string());
    task.deadline = Some(now + 3600);
    task.reminder_time = Some(now - 300);

    let task_id = db.create_task(&task).expect("Failed to create task");

    // Verify task needs reminder
    let tasks = db
        .get_tasks_needing_reminder()
        .expect("Failed to get tasks");
    assert_eq!(tasks.len(), 1);

    // Delete the task
    db.delete_task(task_id).expect("Failed to delete task");

    // Verify deleted task doesn't appear in reminders
    let tasks = db
        .get_tasks_needing_reminder()
        .expect("Failed to get tasks after delete");
    assert!(tasks.is_empty());
}

#[tokio::test]
async fn test_reminder_time_update_on_deadline_change() {
    let (db, _temp_dir) = create_test_db();

    let now = chrono::Utc::now().timestamp();

    // Create task with deadline
    let mut task = Task::new("Task with changing deadline".to_string());
    let original_deadline = now + 7200; // 2 hours from now
    task.deadline = Some(original_deadline);
    task.reminder_time = Some(original_deadline - 3600); // 1 hour before

    let task_id = db.create_task(&task).expect("Failed to create task");

    // Retrieve and verify initial reminder time
    let task = db
        .get_task(task_id)
        .expect("Failed to get task")
        .expect("Task not found");
    assert_eq!(task.reminder_time, Some(original_deadline - 3600));

    // Update deadline to sooner time
    let mut updated_task = task.clone();
    let new_deadline = now + 1800; // 30 minutes from now
    updated_task.deadline = Some(new_deadline);
    updated_task.reminder_time = Some(new_deadline - 300); // 5 minutes before
    updated_task.updated_at = chrono::Utc::now().timestamp();

    db.update_task(&updated_task)
        .expect("Failed to update task");

    // Verify reminder time is updated
    let task = db
        .get_task(task_id)
        .expect("Failed to get task")
        .expect("Task not found");
    assert_eq!(task.reminder_time, Some(new_deadline - 300));
}

#[tokio::test]
async fn test_notification_settings_integration() {
    let (db, _temp_dir) = create_test_db();

    // Set notification settings
    db.set_setting("notification.enabled", "true")
        .expect("Failed to set enabled");
    db.set_setting("notification.sound", "default")
        .expect("Failed to set sound");
    db.set_setting("notification.deadline_warning_hours", "1")
        .expect("Failed to set warning hours");

    // Get settings
    let settings = db
        .get_settings_by_prefix("notification")
        .expect("Failed to get settings");

    assert_eq!(settings.len(), 3);

    let enabled = settings
        .iter()
        .find(|(k, _)| k == "notification.enabled")
        .map(|(_, v)| v);
    assert_eq!(enabled, Some(&"true".to_string()));

    let sound = settings
        .iter()
        .find(|(k, _)| k == "notification.sound")
        .map(|(_, v)| v);
    assert_eq!(sound, Some(&"default".to_string()));

    let warning_hours = settings
        .iter()
        .find(|(k, _)| k == "notification.deadline_warning_hours")
        .map(|(_, v)| v);
    assert_eq!(warning_hours, Some(&"1".to_string()));
}

#[tokio::test]
async fn test_high_priority_task_notifications() {
    let (db, _temp_dir) = create_test_db();

    let now = chrono::Utc::now().timestamp();

    // Create tasks with different priorities, all expiring soon
    let tasks = vec![
        ("High priority task 1", Priority::High, now + 600),
        ("High priority task 2", Priority::High, now + 900),
        ("Medium priority task", Priority::Medium, now + 800),
        ("Low priority task", Priority::Low, now + 700),
    ];

    for (title, priority, deadline) in tasks {
        let mut task = Task::new(title.to_string());
        task.priority = priority;
        task.deadline = Some(deadline);
        task.status = TaskStatus::Todo;
        db.create_task(&task).expect("Failed to create task");
    }

    // Get all expiring tasks
    let expiring = db
        .get_expiring_tasks(3600)
        .expect("Failed to get expiring tasks");

    // Should get all 4 tasks
    assert_eq!(expiring.len(), 4);

    // Count high priority tasks
    let high_priority_count = expiring
        .iter()
        .filter(|t| t.priority == Priority::High)
        .count();

    assert_eq!(high_priority_count, 2);
}

#[tokio::test]
async fn test_reminder_edge_cases() {
    let (db, _temp_dir) = create_test_db();

    let now = chrono::Utc::now().timestamp();

    // Test edge case: reminder exactly at current time
    let mut task1 = Task::new("Reminder exactly now".to_string());
    task1.reminder_time = Some(now);
    task1.deadline = Some(now + 3600);
    db.create_task(&task1).expect("Failed to create task1");

    // Test edge case: reminder just outside tolerance window (6 minutes ago)
    let mut task2 = Task::new("Reminder too old".to_string());
    task2.reminder_time = Some(now - 360);
    task2.deadline = Some(now + 3600);
    db.create_task(&task2).expect("Failed to create task2");

    // Test edge case: reminder 1 second ago
    let mut task3 = Task::new("Reminder 1 second ago".to_string());
    task3.reminder_time = Some(now - 1);
    task3.deadline = Some(now + 3600);
    db.create_task(&task3).expect("Failed to create task3");

    let reminders = db
        .get_tasks_needing_reminder()
        .expect("Failed to get reminders");

    // Should get task1 and task3 (within 5 minute tolerance)
    assert!(reminders.len() >= 2);
}

#[tokio::test]
async fn test_notification_deduplication() {
    let (db, _temp_dir) = create_test_db();

    let now = chrono::Utc::now().timestamp();

    // Create task with reminder
    let mut task = Task::new("Task with reminder".to_string());
    task.reminder_time = Some(now - 60);
    task.deadline = Some(now + 3600);

    let task_id = db.create_task(&task).expect("Failed to create task");

    // First check - should get the task
    let reminders1 = db
        .get_tasks_needing_reminder()
        .expect("Failed to get reminders");
    assert_eq!(reminders1.len(), 1);

    // Clear reminder (simulating notification sent)
    db.clear_reminder(task_id)
        .expect("Failed to clear reminder");

    // Second check - should not get the task again
    let reminders2 = db
        .get_tasks_needing_reminder()
        .expect("Failed to get reminders");
    assert!(reminders2.is_empty());
}

#[tokio::test]
async fn test_bulk_notification_clearing() {
    let (db, _temp_dir) = create_test_db();

    let now = chrono::Utc::now().timestamp();

    // Create multiple tasks with past reminders
    let mut task_ids = vec![];
    for i in 0..5 {
        let mut task = Task::new(format!("Task {}", i));
        task.reminder_time = Some(now - 120);
        task.deadline = Some(now + 3600);
        let id = db.create_task(&task).expect("Failed to create task");
        task_ids.push(id);
    }

    // Verify all tasks need reminders
    let reminders = db
        .get_tasks_needing_reminder()
        .expect("Failed to get reminders");
    assert_eq!(reminders.len(), 5);

    // Clear all reminders
    for task_id in &task_ids {
        db.clear_reminder(*task_id)
            .expect("Failed to clear reminder");
    }

    // Verify no tasks need reminders
    let reminders = db
        .get_tasks_needing_reminder()
        .expect("Failed to get reminders");
    assert!(reminders.is_empty());
}
