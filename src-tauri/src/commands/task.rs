use crate::db::{models::*, Database};
use tauri::State;

/// Create a new task
#[tauri::command]
pub async fn create_task(
    db: State<'_, Database>,
    title: String,
    description: Option<String>,
    priority: Option<String>,
    deadline: Option<i64>,
    tags: Option<Vec<String>>,
) -> Result<i64, String> {
    let mut task = Task::new(title);
    task.description = description;
    task.deadline = deadline;
    task.tags = tags;

    if let Some(p) = priority {
        task.priority = Priority::from_str(&p).map_err(|e| e.to_string())?;
    }

    db.create_task(&task).map_err(|e| e.to_string())
}

/// Get a task by ID
#[tauri::command]
pub async fn get_task(db: State<'_, Database>, id: i64) -> Result<Option<Task>, String> {
    db.get_task(id).map_err(|e| e.to_string())
}

/// Update an existing task
#[tauri::command]
pub async fn update_task(
    db: State<'_, Database>,
    id: i64,
    title: Option<String>,
    description: Option<String>,
    status: Option<String>,
    priority: Option<String>,
    deadline: Option<i64>,
    tags: Option<Vec<String>>,
    completed_at: Option<i64>,
) -> Result<(), String> {
    // Get existing task
    let mut task = db
        .get_task(id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Task with id {} not found", id))?;

    // Update fields if provided
    if let Some(t) = title {
        task.title = t;
    }
    if let Some(d) = description {
        task.description = Some(d);
    }
    if let Some(s) = status {
        task.status = TaskStatus::from_str(&s).map_err(|e| e.to_string())?;
        // If status is done and completed_at is not set, set it to now
        if task.status == TaskStatus::Done && task.completed_at.is_none() {
            task.completed_at = Some(chrono::Utc::now().timestamp());
        }
    }
    if let Some(p) = priority {
        task.priority = Priority::from_str(&p).map_err(|e| e.to_string())?;
    }
    if let Some(d) = deadline {
        task.deadline = Some(d);
    }
    if let Some(t) = tags {
        task.tags = Some(t);
    }
    if let Some(c) = completed_at {
        task.completed_at = Some(c);
    }

    // Update timestamp
    task.updated_at = chrono::Utc::now().timestamp();

    db.update_task(&task).map_err(|e| e.to_string())
}

/// Delete a task (soft delete)
#[tauri::command]
pub async fn delete_task(db: State<'_, Database>, id: i64) -> Result<(), String> {
    db.delete_task(id).map_err(|e| e.to_string())
}

/// List all tasks with optional status filter
#[tauri::command]
pub async fn list_tasks(
    db: State<'_, Database>,
    status: Option<String>,
) -> Result<Vec<Task>, String> {
    let task_status = if let Some(s) = status {
        Some(TaskStatus::from_str(&s).map_err(|e| e.to_string())?)
    } else {
        None
    };

    db.list_tasks(task_status).map_err(|e| e.to_string())
}

/// Get database version (for debugging)
#[tauri::command]
pub async fn get_db_version(db: State<'_, Database>) -> Result<i32, String> {
    db.get_version().map_err(|e| e.to_string())
}
