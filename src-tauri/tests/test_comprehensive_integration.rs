/// Comprehensive Integration Tests for Intento
///
/// This module contains end-to-end integration tests that verify:
/// - Task CRUD operations with database
/// - AI parsing (text and image) integration
/// - Summary generation workflow
/// - Notification system
/// - Settings management
/// - Scheduler functionality
/// - Cross-module interactions

use intento::ai::{AiClient, ParsedTask, TaskOperation, ToolSet};
use intento::db::{
    models::{Priority, Summary, SummaryType, Task, TaskStatus},
    Database,
};
use intento::summary::SummaryGenerator;
use tempfile::TempDir;

/// Helper to create a test database with unique temp file
fn create_test_db() -> (Database, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test_intento.db");
    let db = Database::new(db_path).expect("Failed to create test database");
    (db, temp_dir)
}

/// Helper to load environment variables for AI tests
fn load_test_env() {
    let env_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".env");

    if env_path.exists() {
        dotenv::from_path(&env_path).ok();
    }
}

// ============================================================================
// Database Integration Tests
// ============================================================================

#[tokio::test]
async fn test_task_lifecycle_integration() {
    let (db, _temp_dir) = create_test_db();

    // Create a task
    let mut task = Task::new("Complete project documentation".to_string());
    task.description = Some("Write comprehensive API docs".to_string());
    task.priority = Priority::High;
    task.tags = Some(vec!["work".to_string(), "documentation".to_string()]);
    task.deadline = Some(chrono::Utc::now().timestamp() + 86400); // 1 day from now

    let task_id = db.create_task(&task).expect("Failed to create task");
    assert!(task_id > 0);

    // Read the task
    let retrieved = db
        .get_task(task_id)
        .expect("Failed to retrieve task")
        .expect("Task not found");
    assert_eq!(retrieved.title, "Complete project documentation");
    assert_eq!(retrieved.priority, Priority::High);
    assert_eq!(
        retrieved.tags,
        Some(vec!["work".to_string(), "documentation".to_string()])
    );

    // Update task status
    let mut updated_task = retrieved.clone();
    updated_task.status = TaskStatus::Doing;
    updated_task.updated_at = chrono::Utc::now().timestamp();
    db.update_task(&updated_task)
        .expect("Failed to update task");

    let retrieved = db
        .get_task(task_id)
        .expect("Failed to retrieve updated task")
        .expect("Task not found");
    assert_eq!(retrieved.status, TaskStatus::Doing);

    // Complete task
    let mut completed_task = retrieved.clone();
    completed_task.status = TaskStatus::Done;
    completed_task.completed_at = Some(chrono::Utc::now().timestamp());
    completed_task.updated_at = chrono::Utc::now().timestamp();
    db.update_task(&completed_task)
        .expect("Failed to complete task");

    let retrieved = db
        .get_task(task_id)
        .expect("Failed to retrieve completed task")
        .expect("Task not found");
    assert_eq!(retrieved.status, TaskStatus::Done);
    assert!(retrieved.completed_at.is_some());

    // List tasks with filter
    let all_tasks = db.list_tasks(None).expect("Failed to list all tasks");
    assert_eq!(all_tasks.len(), 1);

    let done_tasks = db
        .list_tasks(Some(TaskStatus::Done))
        .expect("Failed to list done tasks");
    assert_eq!(done_tasks.len(), 1);

    // Soft delete
    db.delete_task(task_id).expect("Failed to delete task");
    let deleted = db.get_task(task_id).expect("Failed to query deleted task");
    assert!(deleted.is_none());
}

#[tokio::test]
async fn test_multiple_tasks_with_filters() {
    let (db, _temp_dir) = create_test_db();

    // Create multiple tasks with different properties
    let tasks_data = vec![
        ("High priority task", Priority::High, Some(vec!["urgent".to_string()])),
        ("Medium priority task", Priority::Medium, Some(vec!["work".to_string()])),
        ("Low priority task", Priority::Low, Some(vec!["personal".to_string()])),
        ("Another high priority", Priority::High, Some(vec!["urgent".to_string(), "work".to_string()])),
    ];

    let mut task_ids = Vec::new();
    for (title, priority, tags) in tasks_data {
        let mut task = Task::new(title.to_string());
        task.priority = priority;
        task.tags = tags;
        let id = db.create_task(&task).expect("Failed to create task");
        task_ids.push(id);
    }

    // Verify all tasks are created
    let all_tasks = db.list_tasks(None).expect("Failed to list tasks");
    assert_eq!(all_tasks.len(), 4);

    // Update some tasks to different statuses
    for (i, &task_id) in task_ids.iter().enumerate() {
        let mut task = db
            .get_task(task_id)
            .expect("Failed to get task")
            .expect("Task not found");

        task.status = match i % 3 {
            0 => TaskStatus::Todo,
            1 => TaskStatus::Doing,
            _ => TaskStatus::Done,
        };

        if task.status == TaskStatus::Done {
            task.completed_at = Some(chrono::Utc::now().timestamp());
        }

        task.updated_at = chrono::Utc::now().timestamp();
        db.update_task(&task).expect("Failed to update task");
    }

    // Filter by status
    let todo_tasks = db
        .list_tasks(Some(TaskStatus::Todo))
        .expect("Failed to list todo tasks");
    assert!(todo_tasks.len() >= 1);

    let doing_tasks = db
        .list_tasks(Some(TaskStatus::Doing))
        .expect("Failed to list doing tasks");
    assert!(doing_tasks.len() >= 1);

    let done_tasks = db
        .list_tasks(Some(TaskStatus::Done))
        .expect("Failed to list done tasks");
    assert!(done_tasks.len() >= 1);
}

#[tokio::test]
async fn test_task_reminder_functionality() {
    let (db, _temp_dir) = create_test_db();

    let now = chrono::Utc::now().timestamp();
    let future_deadline = now + 7200; // 2 hours from now

    // Create task with reminder
    let mut task = Task::new("Important meeting".to_string());
    task.deadline = Some(future_deadline);
    task.reminder_time = Some(future_deadline - 3600); // 1 hour before deadline

    let task_id = db.create_task(&task).expect("Failed to create task");

    // Test get_tasks_needing_reminder (should not return this task yet)
    let tasks_needing_reminder = db
        .get_tasks_needing_reminder()
        .expect("Failed to get tasks needing reminder");
    assert!(tasks_needing_reminder.is_empty());

    // Create a task with past reminder time (should be returned)
    let mut urgent_task = Task::new("Urgent task".to_string());
    urgent_task.deadline = Some(now + 300); // 5 minutes from now
    urgent_task.reminder_time = Some(now - 60); // 1 minute ago

    let urgent_id = db
        .create_task(&urgent_task)
        .expect("Failed to create urgent task");

    let tasks_needing_reminder = db
        .get_tasks_needing_reminder()
        .expect("Failed to get tasks needing reminder");
    assert_eq!(tasks_needing_reminder.len(), 1);
    assert_eq!(tasks_needing_reminder[0].id, Some(urgent_id));

    // Clear reminder
    db.clear_reminder(urgent_id)
        .expect("Failed to clear reminder");

    let tasks_needing_reminder = db
        .get_tasks_needing_reminder()
        .expect("Failed to get tasks needing reminder after clear");
    assert!(tasks_needing_reminder.is_empty());
}

#[tokio::test]
async fn test_expiring_tasks() {
    let (db, _temp_dir) = create_test_db();

    let now = chrono::Utc::now().timestamp();

    // Create tasks with different deadlines
    let task_data = vec![
        ("Expires in 10 minutes", now + 600),
        ("Expires in 1 hour", now + 3600),
        ("Expires in 1 day", now + 86400),
        ("Expired task", now - 3600),
    ];

    for (title, deadline) in task_data {
        let mut task = Task::new(title.to_string());
        task.deadline = Some(deadline);
        db.create_task(&task).expect("Failed to create task");
    }

    // Get tasks expiring within 30 minutes
    let expiring_soon = db
        .get_expiring_tasks(1800)
        .expect("Failed to get expiring tasks");
    assert_eq!(expiring_soon.len(), 1);
    assert!(expiring_soon[0].title.contains("10 minutes"));

    // Get tasks expiring within 2 hours
    let expiring_2h = db
        .get_expiring_tasks(7200)
        .expect("Failed to get expiring tasks");
    assert_eq!(expiring_2h.len(), 2); // 10 minutes and 1 hour
}

// ============================================================================
// Summary Integration Tests
// ============================================================================

#[tokio::test]
async fn test_summary_crud_integration() {
    let (db, _temp_dir) = create_test_db();

    // Create test tasks
    let task_ids = vec![
        db.create_task(&Task::new("Task 1".to_string()))
            .expect("Failed to create task"),
        db.create_task(&Task::new("Task 2".to_string()))
            .expect("Failed to create task"),
    ];

    // Create summary
    let start = chrono::Utc::now()
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();
    let end = start + 86399; // End of day

    let mut summary = Summary::new(
        SummaryType::Daily,
        start,
        end,
        Some("work".to_string()),
        "# Daily Summary\n\nCompleted 2 tasks today.".to_string(),
        Some("{\"completed\": 2, \"created\": 2}".to_string()),
        task_ids.clone(),
    );
    summary.tag_filter = Some(vec!["work".to_string()]);

    let summary_id = db
        .create_summary(&summary)
        .expect("Failed to create summary");
    assert!(summary_id > 0);

    // Retrieve summary
    let retrieved = db
        .get_summary(summary_id)
        .expect("Failed to get summary")
        .expect("Summary not found");
    assert_eq!(retrieved.summary_type, SummaryType::Daily);
    assert_eq!(retrieved.tag, Some("work".to_string()));
    assert_eq!(retrieved.task_ids, Some(task_ids));

    // List summaries
    let all_summaries = db
        .list_summaries(None, None, None, None)
        .expect("Failed to list summaries");
    assert_eq!(all_summaries.len(), 1);

    // List with tag filter
    let work_summaries = db
        .list_summaries(Some("work"), None, None, None)
        .expect("Failed to list work summaries");
    assert_eq!(work_summaries.len(), 1);

    let other_summaries = db
        .list_summaries(Some("personal"), None, None, None)
        .expect("Failed to list personal summaries");
    assert_eq!(other_summaries.len(), 0);

    // List with type filter
    let daily_summaries = db
        .list_summaries(None, Some(&SummaryType::Daily), None, None)
        .expect("Failed to list daily summaries");
    assert_eq!(daily_summaries.len(), 1);

    // Test find by period
    let found = db
        .find_summary_by_period(&SummaryType::Daily, start, end, Some("work"))
        .expect("Failed to find summary")
        .expect("Summary not found by period");
    assert_eq!(found.id, Some(summary_id));

    // Delete summary
    db.delete_summary(summary_id)
        .expect("Failed to delete summary");
    let deleted = db
        .get_summary(summary_id)
        .expect("Failed to query deleted summary");
    assert!(deleted.is_none());
}

#[tokio::test]
async fn test_summary_pagination() {
    let (db, _temp_dir) = create_test_db();

    let base_time = chrono::Utc::now().timestamp();

    // Create multiple summaries
    for i in 0..10 {
        let start = base_time + (i * 86400);
        let end = start + 86399;
        let summary = Summary::new(
            SummaryType::Daily,
            start,
            end,
            None,
            format!("Summary #{}", i),
            None,
            vec![],
        );
        db.create_summary(&summary)
            .expect("Failed to create summary");
    }

    // Test limit
    let limited = db
        .list_summaries(None, None, Some(5), None)
        .expect("Failed to list with limit");
    assert_eq!(limited.len(), 5);

    // Test offset
    let offset = db
        .list_summaries(None, None, Some(5), Some(5))
        .expect("Failed to list with offset");
    assert_eq!(offset.len(), 5);

    // Verify pagination works correctly
    let all = db
        .list_summaries(None, None, None, None)
        .expect("Failed to list all");
    assert_eq!(all.len(), 10);
}

// ============================================================================
// Settings Integration Tests
// ============================================================================

#[tokio::test]
async fn test_settings_management() {
    let (db, _temp_dir) = create_test_db();

    // Set settings
    db.set_setting("auto_summary.enabled", "true")
        .expect("Failed to set setting");
    db.set_setting("auto_summary.daily", "true")
        .expect("Failed to set setting");
    db.set_setting("notification.enabled", "true")
        .expect("Failed to set setting");
    db.set_setting("notification.sound", "default")
        .expect("Failed to set setting");

    // Get individual setting
    let enabled = db
        .get_setting("auto_summary.enabled")
        .expect("Failed to get setting");
    assert_eq!(enabled, Some("true".to_string()));

    // Get settings by prefix
    let auto_summary_settings = db
        .get_settings_by_prefix("auto_summary")
        .expect("Failed to get settings by prefix");
    assert_eq!(auto_summary_settings.len(), 2);

    let notification_settings = db
        .get_settings_by_prefix("notification")
        .expect("Failed to get notification settings");
    assert_eq!(notification_settings.len(), 2);

    // Update existing setting
    db.set_setting("auto_summary.enabled", "false")
        .expect("Failed to update setting");
    let updated = db
        .get_setting("auto_summary.enabled")
        .expect("Failed to get updated setting");
    assert_eq!(updated, Some("false".to_string()));
}

// ============================================================================
// AI Integration Tests (require API keys)
// ============================================================================

#[tokio::test]
#[ignore] // Ignore by default as it requires API keys
async fn test_ai_text_parsing_integration() {
    load_test_env();

    let client = match AiClient::new_default() {
        Ok(client) => client,
        Err(_) => {
            println!("⚠️  AI client not available, skipping test");
            return;
        }
    };

    // Test basic text parsing
    let input = "Complete the quarterly report by next Friday at 5pm, high priority, tag: work";
    let result = client
        .parse_text_input(input)
        .await
        .expect("Failed to parse text");

    assert!(!result.title.is_empty());
    assert!(result.deadline.is_some());
    assert_eq!(result.priority, Some("high".to_string()));
    assert!(result.tags.is_some());

    println!("Parsed task: {:?}", result);
}

#[tokio::test]
#[ignore] // Ignore by default as it requires API keys
async fn test_ai_text_parsing_with_existing_tags() {
    load_test_env();

    let client = match AiClient::new_default() {
        Ok(client) => client,
        Err(_) => {
            println!("⚠️  AI client not available, skipping test");
            return;
        }
    };

    let existing_tags = vec![
        "work".to_string(),
        "personal".to_string(),
        "urgent".to_string(),
    ];

    let input = "Buy groceries tomorrow, urgent";
    let result = client
        .parse_text_input_with_tags(input, Some(&existing_tags))
        .await
        .expect("Failed to parse text with tags");

    assert!(!result.title.is_empty());
    println!("Parsed with existing tags: {:?}", result);
}

#[tokio::test]
#[ignore] // Ignore by default as it requires API keys and test image
async fn test_ai_image_parsing_integration() {
    load_test_env();

    let client = match AiClient::new_default() {
        Ok(client) => client,
        Err(_) => {
            println!("⚠️  AI client not available, skipping test");
            return;
        }
    };

    // Load test image
    let image_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/test.jpg");

    if !image_path.exists() {
        println!("⚠️  Test image not found, skipping test");
        return;
    }

    let image_data = std::fs::read(&image_path).expect("Failed to read test image");
    let image_base64 = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        &image_data,
    );

    // Test with Basic tools (Create only)
    let result = client
        .parse_image_for_operations(&image_base64, "image/jpeg", ToolSet::Basic)
        .await
        .expect("Failed to parse image");

    println!("Image parsing result:");
    println!("  Confidence: {:.2}%", result.confidence * 100.0);
    println!("  Operations: {}", result.operations.len());

    if let Some(desc) = &result.image_description {
        println!("  Description: {}", desc);
    }

    for (i, op) in result.operations.iter().enumerate() {
        println!("  {}. [{}] {}", i + 1, op.operation_name(), op.description());
    }
}

#[tokio::test]
#[ignore] // Ignore by default as it requires API keys
async fn test_summary_generation_integration() {
    load_test_env();

    let (db, _temp_dir) = create_test_db();

    // Create test tasks
    let now = chrono::Utc::now().timestamp();
    let tasks_data = vec![
        ("Complete documentation", TaskStatus::Done, Priority::High),
        ("Fix bugs", TaskStatus::Done, Priority::Medium),
        ("Code review", TaskStatus::Doing, Priority::Medium),
        ("Write tests", TaskStatus::Todo, Priority::Low),
    ];

    for (title, status, priority) in tasks_data {
        let mut task = Task::new(title.to_string());
        task.status = status.clone();
        task.priority = priority;
        task.tags = Some(vec!["work".to_string()]);
        task.created_at = now - 3600; // Created 1 hour ago

        if status == TaskStatus::Done {
            task.completed_at = Some(now - 1800); // Completed 30 minutes ago
        }

        db.create_task(&task).expect("Failed to create task");
    }

    // Create AI client and generator
    let client = match AiClient::new_default() {
        Ok(client) => client,
        Err(_) => {
            println!("⚠️  AI client not available, skipping test");
            return;
        }
    };

    let generator = SummaryGenerator::new(db.clone(), client);

    // Generate daily summary
    let start = chrono::Utc::now()
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();
    let end = start + 86399;

    let summary = generator
        .generate_summary(Some("work".to_string()), SummaryType::Daily, start, end)
        .await
        .expect("Failed to generate summary");

    println!("Generated summary:");
    println!("  Type: {:?}", summary.summary_type);
    println!("  Tag: {:?}", summary.tag);
    println!("  Content length: {} chars", summary.content.len());
    println!("  Statistics: {:?}", summary.statistics);

    assert!(!summary.content.is_empty());
    assert_eq!(summary.summary_type, SummaryType::Daily);
    assert_eq!(summary.tag, Some("work".to_string()));
}

// ============================================================================
// End-to-End Workflow Tests
// ============================================================================

#[tokio::test]
async fn test_complete_task_workflow() {
    let (db, _temp_dir) = create_test_db();

    // Step 1: Create a task
    let mut task = Task::new("Implement new feature".to_string());
    task.description = Some("Add user authentication".to_string());
    task.priority = Priority::High;
    task.tags = Some(vec!["development".to_string(), "backend".to_string()]);
    task.deadline = Some(chrono::Utc::now().timestamp() + 172800); // 2 days

    let task_id = db.create_task(&task).expect("Failed to create task");

    // Step 2: Start working on task
    let mut task = db
        .get_task(task_id)
        .expect("Failed to get task")
        .expect("Task not found");
    task.status = TaskStatus::Doing;
    task.updated_at = chrono::Utc::now().timestamp();
    db.update_task(&task).expect("Failed to update task");

    // Step 3: Update progress
    task.description = Some("Add user authentication - JWT tokens implemented".to_string());
    task.updated_at = chrono::Utc::now().timestamp();
    db.update_task(&task).expect("Failed to update progress");

    // Step 4: Complete task
    task.status = TaskStatus::Done;
    task.completed_at = Some(chrono::Utc::now().timestamp());
    task.updated_at = chrono::Utc::now().timestamp();
    db.update_task(&task).expect("Failed to complete task");

    // Step 5: Verify final state
    let final_task = db
        .get_task(task_id)
        .expect("Failed to get final task")
        .expect("Task not found");

    assert_eq!(final_task.status, TaskStatus::Done);
    assert!(final_task.completed_at.is_some());
    assert!(final_task.description.is_some());
    assert!(final_task
        .description
        .unwrap()
        .contains("JWT tokens implemented"));

    // Step 6: Verify in lists
    let done_tasks = db
        .list_tasks(Some(TaskStatus::Done))
        .expect("Failed to list done tasks");
    assert_eq!(done_tasks.len(), 1);
}

#[tokio::test]
async fn test_multi_tag_task_organization() {
    let (db, _temp_dir) = create_test_db();

    // Create tasks with different tag combinations
    let tasks_with_tags = vec![
        ("Backend API", vec!["work", "backend", "api"]),
        ("Frontend UI", vec!["work", "frontend", "ui"]),
        ("Database migration", vec!["work", "backend", "database"]),
        ("Personal blog post", vec!["personal", "writing"]),
        ("Team meeting", vec!["work", "meeting"]),
    ];

    for (title, tags) in tasks_with_tags {
        let mut task = Task::new(title.to_string());
        task.tags = Some(tags.iter().map(|s| s.to_string()).collect());
        db.create_task(&task).expect("Failed to create task");
    }

    let all_tasks = db.list_tasks(None).expect("Failed to list tasks");
    assert_eq!(all_tasks.len(), 5);

    // Verify tag distribution
    let mut backend_count = 0;
    let mut frontend_count = 0;
    let mut work_count = 0;
    let mut personal_count = 0;

    for task in &all_tasks {
        if let Some(tags) = &task.tags {
            if tags.contains(&"backend".to_string()) {
                backend_count += 1;
            }
            if tags.contains(&"frontend".to_string()) {
                frontend_count += 1;
            }
            if tags.contains(&"work".to_string()) {
                work_count += 1;
            }
            if tags.contains(&"personal".to_string()) {
                personal_count += 1;
            }
        }
    }

    assert_eq!(backend_count, 2);
    assert_eq!(frontend_count, 1);
    assert_eq!(work_count, 4);
    assert_eq!(personal_count, 1);
}

#[tokio::test]
async fn test_deadline_priority_workflow() {
    let (db, _temp_dir) = create_test_db();

    let now = chrono::Utc::now().timestamp();

    // Create tasks with various deadlines and priorities
    let tasks = vec![
        ("Urgent bug fix", Priority::High, now + 3600),      // 1 hour
        ("Important feature", Priority::High, now + 86400),  // 1 day
        ("Minor update", Priority::Low, now + 172800),       // 2 days
        ("Review PR", Priority::Medium, now + 7200),         // 2 hours
    ];

    for (title, priority, deadline) in tasks {
        let mut task = Task::new(title.to_string());
        task.priority = priority;
        task.deadline = Some(deadline);
        db.create_task(&task).expect("Failed to create task");
    }

    // Check expiring tasks (within 90 minutes)
    let urgent = db
        .get_expiring_tasks(5400)
        .expect("Failed to get urgent tasks");
    assert_eq!(urgent.len(), 2); // Bug fix and PR review

    // Verify they're sorted by deadline (earliest first)
    assert!(urgent[0].deadline.unwrap() < urgent[1].deadline.unwrap());
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[tokio::test]
async fn test_database_error_handling() {
    let (db, _temp_dir) = create_test_db();

    // Try to get non-existent task
    let result = db.get_task(99999).expect("Failed to query task");
    assert!(result.is_none());

    // Try to update non-existent task
    let mut task = Task::new("Non-existent".to_string());
    task.id = Some(99999);
    let result = db.update_task(&task);
    // This should succeed but affect 0 rows (soft failure)
    assert!(result.is_ok());

    // Try to delete non-existent task
    let result = db.delete_task(99999);
    assert!(result.is_ok()); // Soft failure
}

#[tokio::test]
async fn test_invalid_enum_values() {
    // Test TaskStatus parsing
    let result = TaskStatus::from_str("invalid");
    assert!(result.is_err());

    let result = TaskStatus::from_str("todo");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), TaskStatus::Todo);

    // Test Priority parsing
    let result = Priority::from_str("invalid");
    assert!(result.is_err());

    let result = Priority::from_str("high");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Priority::High);

    // Test SummaryType parsing
    let result = SummaryType::from_str("invalid");
    assert!(result.is_err());

    let result = SummaryType::from_str("daily");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), SummaryType::Daily);
}

// ============================================================================
// Performance Tests
// ============================================================================

#[tokio::test]
async fn test_bulk_operations_performance() {
    let (db, _temp_dir) = create_test_db();

    let start_time = std::time::Instant::now();

    // Create 100 tasks
    for i in 0..100 {
        let mut task = Task::new(format!("Task #{}", i));
        task.priority = match i % 3 {
            0 => Priority::Low,
            1 => Priority::Medium,
            _ => Priority::High,
        };
        task.tags = Some(vec![format!("tag{}", i % 10)]);
        db.create_task(&task).expect("Failed to create task");
    }

    let create_duration = start_time.elapsed();
    println!("Created 100 tasks in {:?}", create_duration);

    // List all tasks
    let list_start = std::time::Instant::now();
    let tasks = db.list_tasks(None).expect("Failed to list tasks");
    let list_duration = list_start.elapsed();
    println!("Listed {} tasks in {:?}", tasks.len(), list_duration);

    assert_eq!(tasks.len(), 100);

    // Update all tasks
    let update_start = std::time::Instant::now();
    for task in &tasks {
        let mut updated = task.clone();
        updated.status = TaskStatus::Done;
        updated.completed_at = Some(chrono::Utc::now().timestamp());
        updated.updated_at = chrono::Utc::now().timestamp();
        db.update_task(&updated).expect("Failed to update task");
    }
    let update_duration = update_start.elapsed();
    println!("Updated 100 tasks in {:?}", update_duration);

    // Verify performance is reasonable (should complete in under 2 seconds)
    let total_duration = start_time.elapsed();
    assert!(
        total_duration.as_secs() < 2,
        "Bulk operations took too long: {:?}",
        total_duration
    );
}

#[tokio::test]
async fn test_concurrent_database_access() {
    let (db, _temp_dir) = create_test_db();

    let db_clone = db.clone();

    // Spawn multiple concurrent tasks
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let db = db_clone.clone();
            tokio::spawn(async move {
                let mut task = Task::new(format!("Concurrent task {}", i));
                task.priority = Priority::Medium;
                db.create_task(&task).expect("Failed to create task");
            })
        })
        .collect();

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.expect("Task panicked");
    }

    // Verify all tasks were created
    let tasks = db.list_tasks(None).expect("Failed to list tasks");
    assert_eq!(tasks.len(), 10);
}

// ============================================================================
// Migration Tests
// ============================================================================

#[tokio::test]
async fn test_database_version() {
    let (db, _temp_dir) = create_test_db();

    let version = db.get_version().expect("Failed to get version");
    assert_eq!(version, 5, "Expected database version 5");
}

#[tokio::test]
async fn test_database_schema() {
    let (db, _temp_dir) = create_test_db();

    // Verify we can create all types of records
    let task = Task::new("Test task".to_string());
    let task_id = db.create_task(&task).expect("Failed to create task");
    assert!(task_id > 0);

    let summary = Summary::new(
        SummaryType::Daily,
        0,
        0,
        None,
        "Test summary".to_string(),
        None,
        vec![],
    );
    let summary_id = db
        .create_summary(&summary)
        .expect("Failed to create summary");
    assert!(summary_id > 0);

    // Verify settings work
    db.set_setting("test.key", "test_value")
        .expect("Failed to set setting");
    let value = db
        .get_setting("test.key")
        .expect("Failed to get setting");
    assert_eq!(value, Some("test_value".to_string()));
}

#[cfg(test)]
mod ai_model_tests {
    use super::*;

    #[test]
    fn test_parsed_task_validation() {
        let mut task = ParsedTask {
            title: "Test task".to_string(),
            description: None,
            deadline: Some("2024-12-31T23:59:59Z".to_string()),
            priority: Some("HIGH".to_string()),
            tags: None,
        };

        // Normalize priority
        task.normalize_priority();
        assert_eq!(task.priority, Some("high".to_string()));

        // Validate priority
        assert!(task.validate_priority().is_ok());

        // Parse deadline
        let deadline = task.parse_deadline().expect("Failed to parse deadline");
        assert!(deadline.is_some());
    }

    #[test]
    fn test_task_operation_descriptions() {
        let create_op = TaskOperation::Create {
            title: "New task".to_string(),
            description: None,
            priority: Some("high".to_string()),
            deadline: None,
            tags: None,
        };
        assert_eq!(create_op.operation_name(), "create_task");
        assert!(create_op.description().contains("New task"));

        let update_op = TaskOperation::Update {
            task_identifier: "Task 1".to_string(),
            title: Some("Updated title".to_string()),
            description: None,
            priority: None,
            deadline: None,
            tags: None,
        };
        assert_eq!(update_op.operation_name(), "update_task");

        let complete_op = TaskOperation::Complete {
            task_identifier: "Task 1".to_string(),
        };
        assert_eq!(complete_op.operation_name(), "complete_task");
    }
}
