use serde::{Deserialize, Serialize};

/// Task status enum
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Todo,
    Doing,
    Done,
}

impl TaskStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TaskStatus::Todo => "todo",
            TaskStatus::Doing => "doing",
            TaskStatus::Done => "done",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "todo" => Ok(TaskStatus::Todo),
            "doing" => Ok(TaskStatus::Doing),
            "done" => Ok(TaskStatus::Done),
            _ => Err(format!("Invalid task status: {}", s)),
        }
    }
}

/// Task priority enum
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    pub fn as_str(&self) -> &str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            _ => Err(format!("Invalid priority: {}", s)),
        }
    }
}

/// Task model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: Priority,
    pub deadline: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
    pub completed_at: Option<i64>,
    pub context: Option<String>,
    pub tags: Option<Vec<String>>,
    pub attachments: Option<Vec<String>>,
    pub reminder_time: Option<i64>,
    pub is_deleted: bool,
}

impl Task {
    pub fn new(title: String) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: None,
            title,
            description: None,
            status: TaskStatus::Todo,
            priority: Priority::Medium,
            deadline: None,
            created_at: now,
            updated_at: now,
            completed_at: None,
            context: None,
            tags: None,
            attachments: None,
            reminder_time: None,
            is_deleted: false,
        }
    }
}

/// Summary type enum
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SummaryType {
    Daily,
    Weekly,      // ✨ Phase 5: New
    Monthly,
    SemiAnnual,  // ✨ Phase 5: New (replaces Quarterly)
    Yearly,
}

impl SummaryType {
    pub fn as_str(&self) -> &str {
        match self {
            SummaryType::Daily => "daily",
            SummaryType::Weekly => "weekly",
            SummaryType::Monthly => "monthly",
            SummaryType::SemiAnnual => "semi_annual",
            SummaryType::Yearly => "yearly",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "daily" => Ok(SummaryType::Daily),
            "weekly" => Ok(SummaryType::Weekly),
            "monthly" => Ok(SummaryType::Monthly),
            "semi_annual" => Ok(SummaryType::SemiAnnual),
            "yearly" => Ok(SummaryType::Yearly),
            // Backwards compatibility
            "quarterly" => Ok(SummaryType::SemiAnnual),
            _ => Err(format!("Invalid summary type: {}", s)),
        }
    }

    /// Get cron expression for scheduled generation
    pub fn cron_expression(&self) -> &'static str {
        match self {
            SummaryType::Daily => "0 0 1 * * *",           // Every day at 01:00
            SummaryType::Weekly => "0 0 2 * * MON",        // Every Monday at 02:00
            SummaryType::Monthly => "0 0 3 1 * *",         // 1st of month at 03:00
            SummaryType::SemiAnnual => "0 0 4 1 1,7 *",   // Jan 1 and Jul 1 at 04:00
            SummaryType::Yearly => "0 0 5 1 1 *",          // Jan 1 at 05:00
        }
    }
}

/// Summary model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Summary {
    pub id: Option<i64>,
    pub summary_type: SummaryType,
    pub period_start: i64,
    pub period_end: i64,

    // ✨ Phase 5: New fields
    pub tag: Option<String>,              // Single tag filter
    pub tag_filter: Option<Vec<String>>,  // Multi-tag filter (future use)

    pub content: String,                  // AI generated markdown summary
    pub statistics: Option<String>,       // JSON format statistics
    pub task_ids: Option<Vec<i64>>,      // Included task IDs
    pub created_at: i64,
    pub is_deleted: bool,
}

impl Summary {
    /// Create a new summary
    pub fn new(
        summary_type: SummaryType,
        period_start: i64,
        period_end: i64,
        tag: Option<String>,
        content: String,
        statistics: Option<SummaryStatistics>,
        task_ids: Vec<i64>,
    ) -> Self {
        Self {
            id: None,
            summary_type,
            period_start,
            period_end,
            tag: tag.clone(),
            tag_filter: tag.map(|t| vec![t]),
            content,
            statistics: statistics.map(|s| serde_json::to_string(&s).unwrap()),
            task_ids: Some(task_ids),
            created_at: chrono::Utc::now().timestamp(),
            is_deleted: false,
        }
    }

    /// Get parsed statistics
    pub fn parsed_statistics(&self) -> Option<SummaryStatistics> {
        self.statistics.as_ref().and_then(|s| {
            serde_json::from_str(s).ok()
        })
    }
}

/// Summary statistics structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SummaryStatistics {
    pub total_tasks: usize,
    pub completed: usize,
    pub in_progress: usize,
    pub todo: usize,
    pub completion_rate: f64,
    pub priority_distribution: PriorityDistribution,
    pub time_stats: Option<TimeStats>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriorityDistribution {
    pub high: usize,
    pub medium: usize,
    pub low: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeStats {
    pub avg_completion_time_hours: Option<f64>,
    pub overdue_count: usize,
}

impl SummaryStatistics {
    /// Calculate statistics from task list
    pub fn from_tasks(tasks: &[Task]) -> Self {
        let total = tasks.len();
        let completed = tasks.iter().filter(|t| t.status == TaskStatus::Done).count();
        let in_progress = tasks.iter().filter(|t| t.status == TaskStatus::Doing).count();
        let todo = tasks.iter().filter(|t| t.status == TaskStatus::Todo).count();

        let completion_rate = if total > 0 {
            completed as f64 / total as f64
        } else {
            0.0
        };

        let priority_distribution = PriorityDistribution {
            high: tasks.iter().filter(|t| t.priority == Priority::High).count(),
            medium: tasks.iter().filter(|t| t.priority == Priority::Medium).count(),
            low: tasks.iter().filter(|t| t.priority == Priority::Low).count(),
        };

        let time_stats = Self::calculate_time_stats(tasks);

        Self {
            total_tasks: total,
            completed,
            in_progress,
            todo,
            completion_rate,
            priority_distribution,
            time_stats,
        }
    }

    fn calculate_time_stats(tasks: &[Task]) -> Option<TimeStats> {
        let completed_tasks: Vec<_> = tasks.iter()
            .filter(|t| t.status == TaskStatus::Done && t.completed_at.is_some())
            .collect();

        if completed_tasks.is_empty() {
            return None;
        }

        // Calculate average completion time
        let total_time: i64 = completed_tasks.iter()
            .filter_map(|t| {
                let completed = t.completed_at?;
                Some(completed - t.created_at)
            })
            .sum();

        let avg_completion_time_hours = if !completed_tasks.is_empty() {
            Some(total_time as f64 / completed_tasks.len() as f64 / 3600.0)
        } else {
            None
        };

        // Calculate overdue count
        let now = chrono::Utc::now().timestamp();
        let overdue_count = tasks.iter()
            .filter(|t| {
                t.status != TaskStatus::Done &&
                t.deadline.map(|d| d < now).unwrap_or(false)
            })
            .count();

        Some(TimeStats {
            avg_completion_time_hours,
            overdue_count,
        })
    }
}

/// Cache type enum
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CacheType {
    TaskContext,
    ImageAnalysis,
    TextParse,
}

impl CacheType {
    pub fn as_str(&self) -> &str {
        match self {
            CacheType::TaskContext => "task_context",
            CacheType::ImageAnalysis => "image_analysis",
            CacheType::TextParse => "text_parse",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "task_context" => Ok(CacheType::TaskContext),
            "image_analysis" => Ok(CacheType::ImageAnalysis),
            "text_parse" => Ok(CacheType::TextParse),
            _ => Err(format!("Invalid cache type: {}", s)),
        }
    }
}

/// Context cache model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContextCache {
    pub id: Option<i64>,
    pub cache_key: String,
    pub cache_type: CacheType,
    pub content: String,
    pub source_data: Option<String>,
    pub created_at: i64,
    pub last_accessed_at: i64,
    pub access_count: i64,
    pub expires_at: Option<i64>,
    pub is_deleted: bool,
}
