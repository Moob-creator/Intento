use anyhow::{Context, Result};
use rusqlite::{Connection, OptionalExtension};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub mod models;

use models::*;

/// Database connection wrapper
/// Thread-safe database handle that can be cloned and shared across threads
#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    /// Create a new database connection
    pub fn new(db_path: PathBuf) -> Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create database directory")?;
        }

        let conn = Connection::open(&db_path)
            .context(format!("Failed to open database at {:?}", db_path))?;

        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", [])
            .context("Failed to enable foreign keys")?;

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };

        // Initialize tables
        db.init_tables()?;

        Ok(db)
    }

    /// Initialize database tables
    fn init_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        // Read and execute v1 migration script
        let migration_sql = include_str!("../../migrations/v1_initial.sql");
        conn.execute_batch(migration_sql)
            .context("Failed to execute v1 migration script")?;

        drop(conn); // Release lock before calling run_migrations

        // Run additional migrations
        self.run_migrations()?;

        Ok(())
    }

    /// Run database migrations based on current version
    fn run_migrations(&self) -> Result<()> {
        let version = self.get_version()?;
        let conn = self.conn.lock().unwrap();

        // Migration v2: Add tag support to summaries
        if version < 2 {
            // Try to add columns - ignore error if they already exist
            let _ = conn.execute("ALTER TABLE summaries ADD COLUMN tag TEXT", []);
            let _ = conn.execute("ALTER TABLE summaries ADD COLUMN tag_filter TEXT", []);

            // Execute the rest of the migration (indexes and version update)
            let migration_v2 = include_str!("../../migrations/v2_add_tag_support.sql");
            conn.execute_batch(migration_v2)
                .context("Failed to execute v2 migration")?;
            println!("✓ Applied migration v2: Add tag support to summaries");
        }

        // Migration v3: Fix summary types CHECK constraint
        if version < 3 {
            let migration_v3 = include_str!("../../migrations/v3_fix_summary_types.sql");
            conn.execute_batch(migration_v3)
                .context("Failed to execute v3 migration")?;
            println!("✓ Applied migration v3: Fix summary types to support weekly and semi_annual");
        }

        // Migration v4: Add settings table
        if version < 4 {
            let migration_v4 = include_str!("../../migrations/v4_add_settings_table.sql");
            conn.execute_batch(migration_v4)
                .context("Failed to execute v4 migration")?;
            println!("✓ Applied migration v4: Add settings table");
        }

        Ok(())
    }

    /// Get current database version
    pub fn get_version(&self) -> Result<i32> {
        let conn = self.conn.lock().unwrap();
        let version: i32 = conn
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .context("Failed to get database version")?;
        Ok(version)
    }

    // ========================================
    // Task operations
    // ========================================

    /// Create a new task
    pub fn create_task(&self, task: &Task) -> Result<i64> {
        let conn = self.conn.lock().unwrap();

        let tags_json = task.tags.as_ref().map(|t| serde_json::to_string(t).unwrap());
        let attachments_json = task
            .attachments
            .as_ref()
            .map(|a| serde_json::to_string(a).unwrap());

        conn.execute(
            "INSERT INTO tasks (title, description, status, priority, deadline, created_at,
             updated_at, completed_at, context, tags, attachments, reminder_time, is_deleted)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            (
                &task.title,
                &task.description,
                task.status.as_str(),
                task.priority.as_str(),
                &task.deadline,
                &task.created_at,
                &task.updated_at,
                &task.completed_at,
                &task.context,
                &tags_json,
                &attachments_json,
                &task.reminder_time,
                if task.is_deleted { 1 } else { 0 },
            ),
        )
        .context("Failed to insert task")?;

        Ok(conn.last_insert_rowid())
    }

    /// Get task by ID
    pub fn get_task(&self, id: i64) -> Result<Option<Task>> {
        let conn = self.conn.lock().unwrap();

        let task = conn
            .query_row(
                "SELECT id, title, description, status, priority, deadline, created_at,
                 updated_at, completed_at, context, tags, attachments, reminder_time, is_deleted
                 FROM tasks WHERE id = ?1 AND is_deleted = 0",
                [id],
                |row| {
                    let tags_json: Option<String> = row.get(10)?;
                    let tags = tags_json.and_then(|s| serde_json::from_str(&s).ok());

                    let attachments_json: Option<String> = row.get(11)?;
                    let attachments = attachments_json.and_then(|s| serde_json::from_str(&s).ok());

                    let status_str: String = row.get(3)?;
                    let priority_str: String = row.get(4)?;

                    Ok(Task {
                        id: Some(row.get(0)?),
                        title: row.get(1)?,
                        description: row.get(2)?,
                        status: TaskStatus::from_str(&status_str).unwrap(),
                        priority: Priority::from_str(&priority_str).unwrap(),
                        deadline: row.get(5)?,
                        created_at: row.get(6)?,
                        updated_at: row.get(7)?,
                        completed_at: row.get(8)?,
                        context: row.get(9)?,
                        tags,
                        attachments,
                        reminder_time: row.get(12)?,
                        is_deleted: row.get::<_, i32>(13)? != 0,
                    })
                },
            )
            .optional()
            .context("Failed to query task")?;

        Ok(task)
    }

    /// Update task
    pub fn update_task(&self, task: &Task) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        let tags_json = task.tags.as_ref().map(|t| serde_json::to_string(t).unwrap());
        let attachments_json = task
            .attachments
            .as_ref()
            .map(|a| serde_json::to_string(a).unwrap());

        conn.execute(
            "UPDATE tasks SET title = ?1, description = ?2, status = ?3, priority = ?4,
             deadline = ?5, updated_at = ?6, completed_at = ?7, context = ?8, tags = ?9,
             attachments = ?10, reminder_time = ?11 WHERE id = ?12",
            (
                &task.title,
                &task.description,
                task.status.as_str(),
                task.priority.as_str(),
                &task.deadline,
                &task.updated_at,
                &task.completed_at,
                &task.context,
                &tags_json,
                &attachments_json,
                &task.reminder_time,
                &task.id,
            ),
        )
        .context("Failed to update task")?;

        Ok(())
    }

    /// Delete task (soft delete)
    pub fn delete_task(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "UPDATE tasks SET is_deleted = 1, updated_at = ?1 WHERE id = ?2",
            (chrono::Utc::now().timestamp(), id),
        )
        .context("Failed to delete task")?;

        Ok(())
    }

    /// List all tasks
    pub fn list_tasks(&self, status: Option<TaskStatus>) -> Result<Vec<Task>> {
        let conn = self.conn.lock().unwrap();

        let mut sql = String::from(
            "SELECT id, title, description, status, priority, deadline, created_at,
             updated_at, completed_at, context, tags, attachments, reminder_time, is_deleted
             FROM tasks WHERE is_deleted = 0",
        );

        if let Some(s) = status {
            sql.push_str(&format!(" AND status = '{}'", s.as_str()));
        }

        sql.push_str(" ORDER BY created_at DESC");

        let mut stmt = conn.prepare(&sql).context("Failed to prepare statement")?;

        let tasks = stmt
            .query_map([], |row| {
                let tags_json: Option<String> = row.get(10)?;
                let tags = tags_json.and_then(|s| serde_json::from_str(&s).ok());

                let attachments_json: Option<String> = row.get(11)?;
                let attachments = attachments_json.and_then(|s| serde_json::from_str(&s).ok());

                let status_str: String = row.get(3)?;
                let priority_str: String = row.get(4)?;

                Ok(Task {
                    id: Some(row.get(0)?),
                    title: row.get(1)?,
                    description: row.get(2)?,
                    status: TaskStatus::from_str(&status_str).unwrap(),
                    priority: Priority::from_str(&priority_str).unwrap(),
                    deadline: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                    completed_at: row.get(8)?,
                    context: row.get(9)?,
                    tags,
                    attachments,
                    reminder_time: row.get(12)?,
                    is_deleted: row.get::<_, i32>(13)? != 0,
                })
            })
            .context("Failed to query tasks")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect tasks")?;

        Ok(tasks)
    }

    /// Get tasks expiring within a time window (in seconds)
    /// Returns tasks that have a deadline between now and now + window_seconds
    pub fn get_expiring_tasks(&self, window_seconds: i64) -> Result<Vec<Task>> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().timestamp();
        let deadline_threshold = now + window_seconds;

        let mut stmt = conn
            .prepare(
                "SELECT id, title, description, status, priority, deadline, created_at,
                 updated_at, completed_at, context, tags, attachments, reminder_time, is_deleted
                 FROM tasks
                 WHERE is_deleted = 0
                 AND status != 'done'
                 AND deadline IS NOT NULL
                 AND deadline > ?1
                 AND deadline <= ?2
                 ORDER BY deadline ASC",
            )
            .context("Failed to prepare expiring tasks query")?;

        let tasks = stmt
            .query_map([now, deadline_threshold], |row| {
                let tags_json: Option<String> = row.get(10)?;
                let tags = tags_json.and_then(|s| serde_json::from_str(&s).ok());

                let attachments_json: Option<String> = row.get(11)?;
                let attachments = attachments_json.and_then(|s| serde_json::from_str(&s).ok());

                let status_str: String = row.get(3)?;
                let priority_str: String = row.get(4)?;

                Ok(Task {
                    id: Some(row.get(0)?),
                    title: row.get(1)?,
                    description: row.get(2)?,
                    status: TaskStatus::from_str(&status_str).unwrap(),
                    priority: Priority::from_str(&priority_str).unwrap(),
                    deadline: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                    completed_at: row.get(8)?,
                    context: row.get(9)?,
                    tags,
                    attachments,
                    reminder_time: row.get(12)?,
                    is_deleted: row.get::<_, i32>(13)? != 0,
                })
            })
            .context("Failed to query expiring tasks")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect expiring tasks")?;

        Ok(tasks)
    }

    /// Get tasks that need reminders now (based on reminder_time)
    /// This method provides more accurate notifications by checking reminder_time
    /// instead of just checking if deadline is within a window
    pub fn get_tasks_needing_reminder(&self) -> Result<Vec<Task>> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().timestamp();

        // Query tasks where:
        // 1. reminder_time is set and has passed (within last 5 minutes to avoid missing)
        // 2. Task is not completed or deleted
        let mut stmt = conn
            .prepare(
                "SELECT id, title, description, status, priority, deadline, created_at,
                 updated_at, completed_at, context, tags, attachments, reminder_time, is_deleted
                 FROM tasks
                 WHERE is_deleted = 0
                 AND status != 'done'
                 AND reminder_time IS NOT NULL
                 AND reminder_time <= ?1
                 AND reminder_time > ?2
                 ORDER BY reminder_time ASC",
            )
            .context("Failed to prepare reminder tasks query")?;

        let tasks = stmt
            .query_map([now, now - 300], |row| {
                // 5 minutes tolerance window
                let tags_json: Option<String> = row.get(10)?;
                let tags = tags_json.and_then(|s| serde_json::from_str(&s).ok());

                let attachments_json: Option<String> = row.get(11)?;
                let attachments = attachments_json.and_then(|s| serde_json::from_str(&s).ok());

                let status_str: String = row.get(3)?;
                let priority_str: String = row.get(4)?;

                Ok(Task {
                    id: Some(row.get(0)?),
                    title: row.get(1)?,
                    description: row.get(2)?,
                    status: TaskStatus::from_str(&status_str).unwrap(),
                    priority: Priority::from_str(&priority_str).unwrap(),
                    deadline: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                    completed_at: row.get(8)?,
                    context: row.get(9)?,
                    tags,
                    attachments,
                    reminder_time: row.get(12)?,
                    is_deleted: row.get(13)?,
                })
            })
            .context("Failed to execute reminder tasks query")?
            .collect::<std::result::Result<Vec<Task>, _>>()
            .context("Failed to collect reminder tasks")?;

        Ok(tasks)
    }

    /// Clear reminder_time after notification is sent to avoid duplicate notifications
    pub fn clear_reminder(&self, task_id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE tasks SET reminder_time = NULL WHERE id = ?1",
            [task_id],
        )
        .context("Failed to clear reminder time")?;
        Ok(())
    }

    // ========================================
    // Summary operations
    // ========================================

    /// Create a new summary
    pub fn create_summary(&self, summary: &Summary) -> Result<i64> {
        let conn = self.conn.lock().unwrap();

        let task_ids_json = summary
            .task_ids
            .as_ref()
            .map(|t| serde_json::to_string(t).unwrap());

        let tag_filter_json = summary
            .tag_filter
            .as_ref()
            .map(|t| serde_json::to_string(t).unwrap());

        conn.execute(
            "INSERT INTO summaries (summary_type, period_start, period_end, tag, tag_filter,
             content, statistics, task_ids, created_at, is_deleted)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            (
                summary.summary_type.as_str(),
                &summary.period_start,
                &summary.period_end,
                &summary.tag,
                &tag_filter_json,
                &summary.content,
                &summary.statistics,
                &task_ids_json,
                &summary.created_at,
                if summary.is_deleted { 1 } else { 0 },
            ),
        )
        .context("Failed to insert summary")?;

        Ok(conn.last_insert_rowid())
    }

    /// Get summary by ID
    pub fn get_summary(&self, id: i64) -> Result<Option<Summary>> {
        let conn = self.conn.lock().unwrap();

        let summary = conn
            .query_row(
                "SELECT id, summary_type, period_start, period_end, tag, tag_filter,
                 content, statistics, task_ids, created_at, is_deleted
                 FROM summaries WHERE id = ?1 AND is_deleted = 0",
                [id],
                |row| {
                    let task_ids_json: Option<String> = row.get(8)?;
                    let task_ids = task_ids_json.and_then(|s| serde_json::from_str(&s).ok());

                    let tag_filter_json: Option<String> = row.get(5)?;
                    let tag_filter = tag_filter_json.and_then(|s| serde_json::from_str(&s).ok());

                    let summary_type_str: String = row.get(1)?;

                    Ok(Summary {
                        id: Some(row.get(0)?),
                        summary_type: SummaryType::from_str(&summary_type_str).unwrap(),
                        period_start: row.get(2)?,
                        period_end: row.get(3)?,
                        tag: row.get(4)?,
                        tag_filter,
                        content: row.get(6)?,
                        statistics: row.get(7)?,
                        task_ids,
                        created_at: row.get(9)?,
                        is_deleted: row.get::<_, i32>(10)? != 0,
                    })
                },
            )
            .optional()
            .context("Failed to query summary")?;

        Ok(summary)
    }

    /// Find existing summary by period and tag
    /// Returns the most recent summary matching the criteria
    pub fn find_summary_by_period(
        &self,
        summary_type: &SummaryType,
        period_start: i64,
        period_end: i64,
        tag: Option<&str>,
    ) -> Result<Option<Summary>> {
        let conn = self.conn.lock().unwrap();

        let summary = conn
            .query_row(
                "SELECT id, summary_type, period_start, period_end, tag, tag_filter,
                 content, statistics, task_ids, created_at, is_deleted
                 FROM summaries
                 WHERE summary_type = ?1
                 AND period_start = ?2
                 AND period_end = ?3
                 AND ((?4 IS NULL AND tag IS NULL) OR tag = ?4)
                 AND is_deleted = 0
                 ORDER BY created_at DESC
                 LIMIT 1",
                (summary_type.as_str(), period_start, period_end, tag),
                |row| {
                    let task_ids_json: Option<String> = row.get(8)?;
                    let task_ids = task_ids_json.and_then(|s| serde_json::from_str(&s).ok());

                    let tag_filter_json: Option<String> = row.get(5)?;
                    let tag_filter = tag_filter_json.and_then(|s| serde_json::from_str(&s).ok());

                    let summary_type_str: String = row.get(1)?;

                    Ok(Summary {
                        id: Some(row.get(0)?),
                        summary_type: SummaryType::from_str(&summary_type_str).unwrap(),
                        period_start: row.get(2)?,
                        period_end: row.get(3)?,
                        tag: row.get(4)?,
                        tag_filter,
                        content: row.get(6)?,
                        statistics: row.get(7)?,
                        task_ids,
                        created_at: row.get(9)?,
                        is_deleted: row.get::<_, i32>(10)? != 0,
                    })
                },
            )
            .optional()
            .context("Failed to query summary by period")?;

        Ok(summary)
    }

    /// List summaries with optional filters
    /// Returns summaries sorted by created_at DESC
    pub fn list_summaries(
        &self,
        tag: Option<&str>,
        summary_type: Option<&SummaryType>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Summary>> {
        let conn = self.conn.lock().unwrap();

        // Build dynamic query
        let mut sql = String::from(
            "SELECT id, summary_type, period_start, period_end, tag, tag_filter,
             content, statistics, task_ids, created_at, is_deleted
             FROM summaries WHERE is_deleted = 0"
        );

        let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![];

        // Add tag filter
        if let Some(t) = tag {
            sql.push_str(" AND tag = ?");
            params.push(Box::new(t.to_string()));
        }

        // Add summary_type filter
        if let Some(st) = summary_type {
            sql.push_str(" AND summary_type = ?");
            params.push(Box::new(st.as_str().to_string()));
        }

        // Add ordering
        sql.push_str(" ORDER BY created_at DESC");

        // Add limit and offset
        if let Some(lim) = limit {
            sql.push_str(" LIMIT ?");
            params.push(Box::new(lim as i64));
        }
        if let Some(off) = offset {
            sql.push_str(" OFFSET ?");
            params.push(Box::new(off as i64));
        }

        let mut stmt = conn.prepare(&sql).context("Failed to prepare list_summaries statement")?;

        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let summaries = stmt
            .query_map(param_refs.as_slice(), |row| {
                let task_ids_json: Option<String> = row.get(8)?;
                let task_ids = task_ids_json.and_then(|s| serde_json::from_str(&s).ok());

                let tag_filter_json: Option<String> = row.get(5)?;
                let tag_filter = tag_filter_json.and_then(|s| serde_json::from_str(&s).ok());

                let summary_type_str: String = row.get(1)?;

                Ok(Summary {
                    id: Some(row.get(0)?),
                    summary_type: SummaryType::from_str(&summary_type_str).unwrap(),
                    period_start: row.get(2)?,
                    period_end: row.get(3)?,
                    tag: row.get(4)?,
                    tag_filter,
                    content: row.get(6)?,
                    statistics: row.get(7)?,
                    task_ids,
                    created_at: row.get(9)?,
                    is_deleted: row.get::<_, i32>(10)? != 0,
                })
            })
            .context("Failed to query summaries list")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect summaries")?;

        Ok(summaries)
    }

    /// Delete summary (soft delete)
    pub fn delete_summary(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "UPDATE summaries SET is_deleted = 1 WHERE id = ?1",
            [id],
        )
        .context("Failed to soft delete summary")?;

        Ok(())
    }

    // ========================================
    // Context cache operations
    // ========================================

    /// Set cache
    pub fn set_cache(&self, cache: &ContextCache) -> Result<i64> {
        let conn = self.conn.lock().unwrap();

        // Try to update first
        let updated = conn
            .execute(
                "UPDATE context_cache SET content = ?1, last_accessed_at = ?2,
                 access_count = access_count + 1, expires_at = ?3
                 WHERE cache_key = ?4",
                (
                    &cache.content,
                    &cache.last_accessed_at,
                    &cache.expires_at,
                    &cache.cache_key,
                ),
            )
            .context("Failed to update cache")?;

        if updated > 0 {
            // Return existing ID
            let id: i64 = conn
                .query_row(
                    "SELECT id FROM context_cache WHERE cache_key = ?1",
                    [&cache.cache_key],
                    |row| row.get(0),
                )
                .context("Failed to get cache ID")?;
            return Ok(id);
        }

        // Insert new
        conn.execute(
            "INSERT INTO context_cache (cache_key, cache_type, content, source_data,
             created_at, last_accessed_at, access_count, expires_at, is_deleted)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            (
                &cache.cache_key,
                cache.cache_type.as_str(),
                &cache.content,
                &cache.source_data,
                &cache.created_at,
                &cache.last_accessed_at,
                &cache.access_count,
                &cache.expires_at,
                if cache.is_deleted { 1 } else { 0 },
            ),
        )
        .context("Failed to insert cache")?;

        Ok(conn.last_insert_rowid())
    }

    /// Get cache by key
    pub fn get_cache(&self, cache_key: &str) -> Result<Option<ContextCache>> {
        let conn = self.conn.lock().unwrap();

        let cache = conn
            .query_row(
                "SELECT id, cache_key, cache_type, content, source_data, created_at,
                 last_accessed_at, access_count, expires_at, is_deleted
                 FROM context_cache WHERE cache_key = ?1 AND is_deleted = 0",
                [cache_key],
                |row| {
                    let cache_type_str: String = row.get(2)?;

                    Ok(ContextCache {
                        id: Some(row.get(0)?),
                        cache_key: row.get(1)?,
                        cache_type: CacheType::from_str(&cache_type_str).unwrap(),
                        content: row.get(3)?,
                        source_data: row.get(4)?,
                        created_at: row.get(5)?,
                        last_accessed_at: row.get(6)?,
                        access_count: row.get(7)?,
                        expires_at: row.get(8)?,
                        is_deleted: row.get::<_, i32>(9)? != 0,
                    })
                },
            )
            .optional()
            .context("Failed to query cache")?;

        Ok(cache)
    }

    /// Clean expired cache
    pub fn clean_expired_cache(&self) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().timestamp();

        let deleted = conn
            .execute(
                "UPDATE context_cache SET is_deleted = 1
                 WHERE expires_at IS NOT NULL AND expires_at < ?1",
                [now],
            )
            .context("Failed to clean expired cache")?;

        Ok(deleted)
    }

    // ========================================
    // Settings operations
    // ========================================

    /// Get a setting value by key
    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let value: Option<String> = conn
            .query_row(
                "SELECT value FROM settings WHERE key = ?1",
                [key],
                |row| row.get(0),
            )
            .optional()
            .context("Failed to get setting")?;
        Ok(value)
    }

    /// Set a setting value (insert or update)
    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().timestamp();

        conn.execute(
            "INSERT INTO settings (key, value, updated_at) VALUES (?1, ?2, ?3)
             ON CONFLICT(key) DO UPDATE SET value = ?2, updated_at = ?3",
            rusqlite::params![key, value, now],
        )
        .context("Failed to set setting")?;

        Ok(())
    }

    /// Get multiple settings by prefix
    pub fn get_settings_by_prefix(&self, prefix: &str) -> Result<Vec<(String, String)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT key, value FROM settings WHERE key LIKE ?1 || '%'")
            .context("Failed to prepare settings query")?;

        let settings = stmt
            .query_map([prefix], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .context("Failed to query settings")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect settings")?;

        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        let temp_dir = std::env::temp_dir();
        let db_path = temp_dir.join("intento_test.db");

        let db = Database::new(db_path.clone()).unwrap();
        let version = db.get_version().unwrap();
        assert_eq!(version, 3);

        // Clean up
        std::fs::remove_file(db_path).ok();
    }

    #[test]
    fn test_task_crud() {
        let temp_dir = std::env::temp_dir();
        let db_path = temp_dir.join("intento_test_task.db");

        let db = Database::new(db_path.clone()).unwrap();

        // Create
        let mut task = Task::new("Test Task".to_string());
        task.description = Some("Test description".to_string());
        let task_id = db.create_task(&task).unwrap();
        assert!(task_id > 0);

        // Read
        let retrieved = db.get_task(task_id).unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.title, "Test Task");

        // Update
        let mut updated_task = retrieved.clone();
        updated_task.status = TaskStatus::Done;
        updated_task.updated_at = chrono::Utc::now().timestamp();
        db.update_task(&updated_task).unwrap();

        let retrieved = db.get_task(task_id).unwrap().unwrap();
        assert_eq!(retrieved.status, TaskStatus::Done);

        // Delete
        db.delete_task(task_id).unwrap();
        let retrieved = db.get_task(task_id).unwrap();
        assert!(retrieved.is_none());

        // Clean up
        std::fs::remove_file(db_path).ok();
    }

    #[test]
    fn test_summary_crud() {
        let temp_dir = std::env::temp_dir();
        let db_path = temp_dir.join("intento_test_summary.db");

        let db = Database::new(db_path.clone()).unwrap();

        // Create
        let summary = Summary::new(
            SummaryType::Daily,
            1707667200, // 2024-02-12 00:00:00
            1707753599, // 2024-02-12 23:59:59
            Some("test_tag".to_string()),
            "# Daily Summary\n\nTest content".to_string(),
            None,
            vec![],
        );
        let summary_id = db.create_summary(&summary).unwrap();
        assert!(summary_id > 0);

        // Read
        let retrieved = db.get_summary(summary_id).unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.summary_type, SummaryType::Daily);
        assert_eq!(retrieved.tag, Some("test_tag".to_string()));

        // List - should find the summary
        let list = db.list_summaries(None, None, None, None).unwrap();
        assert_eq!(list.len(), 1);

        // List with tag filter
        let list = db.list_summaries(Some("test_tag"), None, None, None).unwrap();
        assert_eq!(list.len(), 1);

        // List with non-matching tag
        let list = db.list_summaries(Some("other_tag"), None, None, None).unwrap();
        assert_eq!(list.len(), 0);

        // List with summary_type filter
        let list = db.list_summaries(None, Some(&SummaryType::Daily), None, None).unwrap();
        assert_eq!(list.len(), 1);

        // List with limit
        let list = db.list_summaries(None, None, Some(1), None).unwrap();
        assert_eq!(list.len(), 1);

        // Delete
        db.delete_summary(summary_id).unwrap();
        let retrieved = db.get_summary(summary_id).unwrap();
        assert!(retrieved.is_none());

        // Clean up
        std::fs::remove_file(db_path).ok();
    }
}
