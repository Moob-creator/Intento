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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SummaryType {
    Daily,
    Monthly,
    Quarterly,
    Yearly,
}

impl SummaryType {
    pub fn as_str(&self) -> &str {
        match self {
            SummaryType::Daily => "daily",
            SummaryType::Monthly => "monthly",
            SummaryType::Quarterly => "quarterly",
            SummaryType::Yearly => "yearly",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "daily" => Ok(SummaryType::Daily),
            "monthly" => Ok(SummaryType::Monthly),
            "quarterly" => Ok(SummaryType::Quarterly),
            "yearly" => Ok(SummaryType::Yearly),
            _ => Err(format!("Invalid summary type: {}", s)),
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
    pub content: String,
    pub statistics: Option<String>,
    pub task_ids: Option<Vec<i64>>,
    pub created_at: i64,
    pub is_deleted: bool,
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
