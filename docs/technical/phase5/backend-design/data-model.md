# Phase 5: 数据模型设计

## 数据库架构

### 1. 扩展 Summary 表

#### 当前表结构 (Phase 1)

```sql
CREATE TABLE IF NOT EXISTS summaries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    summary_type TEXT NOT NULL,
    period_start INTEGER NOT NULL,
    period_end INTEGER NOT NULL,
    content TEXT NOT NULL,
    statistics TEXT,              -- JSON format
    task_ids TEXT,                -- JSON array
    created_at INTEGER NOT NULL,
    is_deleted INTEGER DEFAULT 0
);
```

#### 新增字段

```sql
-- Migration V2: Add tag support to summaries
ALTER TABLE summaries ADD COLUMN tag TEXT;
ALTER TABLE summaries ADD COLUMN tag_filter TEXT;  -- JSON array for multi-tag support

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_summaries_tag ON summaries(tag);
CREATE INDEX IF NOT EXISTS idx_summaries_type ON summaries(summary_type);
CREATE INDEX IF NOT EXISTS idx_summaries_period ON summaries(period_start, period_end);
CREATE INDEX IF NOT EXISTS idx_summaries_composite ON summaries(tag, summary_type, period_start, period_end);
```

#### 完整表结构 (Phase 5)

```sql
CREATE TABLE IF NOT EXISTS summaries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    summary_type TEXT NOT NULL,        -- 'daily' | 'weekly' | 'monthly' | 'semi_annual' | 'yearly'
    period_start INTEGER NOT NULL,      -- Unix timestamp
    period_end INTEGER NOT NULL,        -- Unix timestamp
    tag TEXT,                           -- Single tag for this summary (NULL = all tags)
    tag_filter TEXT,                    -- JSON array: ["work", "personal"] for multi-tag filtering
    content TEXT NOT NULL,              -- AI generated summary (Markdown format)
    statistics TEXT,                    -- JSON: completion rate, priority distribution, etc.
    task_ids TEXT,                      -- JSON array: [1, 2, 3, ...]
    created_at INTEGER NOT NULL,        -- Unix timestamp
    is_deleted INTEGER DEFAULT 0        -- Soft delete flag
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_summaries_tag ON summaries(tag);
CREATE INDEX IF NOT EXISTS idx_summaries_type ON summaries(summary_type);
CREATE INDEX IF NOT EXISTS idx_summaries_period ON summaries(period_start, period_end);
CREATE INDEX IF NOT EXISTS idx_summaries_composite ON summaries(tag, summary_type, period_start, period_end);
```

---

## Rust 数据模型

### 1. Summary 结构体

#### 扩展后的 Summary

```rust
// src-tauri/src/db/models.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Summary {
    pub id: Option<i64>,
    pub summary_type: SummaryType,
    pub period_start: i64,
    pub period_end: i64,

    // ✨ Phase 5 新增字段
    pub tag: Option<String>,              // 单个 tag 过滤
    pub tag_filter: Option<Vec<String>>,  // 多 tag 过滤（预留）

    pub content: String,                  // AI 生成的 Markdown 总结
    pub statistics: Option<String>,       // JSON 格式统计数据
    pub task_ids: Option<Vec<i64>>,      // 包含的任务 IDs
    pub created_at: i64,                 // Unix timestamp
    pub is_deleted: bool,
}

impl Summary {
    /// 创建新的总结
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

    /// 获取解析后的统计数据
    pub fn parsed_statistics(&self) -> Option<SummaryStatistics> {
        self.statistics.as_ref().and_then(|s| {
            serde_json::from_str(s).ok()
        })
    }
}
```

### 2. SummaryType 枚举

#### 扩展后的 SummaryType

```rust
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SummaryType {
    Daily,
    Weekly,      // ✨ 新增
    Monthly,
    SemiAnnual,  // ✨ 新增 (替代 Quarterly)
    Yearly,
}

impl SummaryType {
    /// 转换为字符串（用于数据库存储）
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Daily => "daily",
            Self::Weekly => "weekly",
            Self::Monthly => "monthly",
            Self::SemiAnnual => "semi_annual",
            Self::Yearly => "yearly",
        }
    }

    /// 从字符串解析
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "daily" => Ok(Self::Daily),
            "weekly" => Ok(Self::Weekly),
            "monthly" => Ok(Self::Monthly),
            "semi_annual" => Ok(Self::SemiAnnual),
            "yearly" => Ok(Self::Yearly),
            _ => Err(format!("Invalid summary type: {}", s)),
        }
    }

    /// 获取 cron 表达式（用于定时任务）
    pub fn cron_expression(&self) -> &'static str {
        match self {
            Self::Daily => "0 0 1 * * *",           // 每天凌晨 1 点
            Self::Weekly => "0 0 2 * * MON",        // 每周一凌晨 2 点
            Self::Monthly => "0 0 3 1 * *",         // 每月 1 号凌晨 3 点
            Self::SemiAnnual => "0 0 4 1 1,7 *",   // 每年 1 月和 7 月 1 号凌晨 4 点
            Self::Yearly => "0 0 5 1 1 *",          // 每年 1 月 1 号凌晨 5 点
        }
    }
}
```

### 3. SummaryStatistics 结构体

```rust
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
    /// 从任务列表计算统计数据
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
            high: tasks.iter().filter(|t| t.priority == TaskPriority::High).count(),
            medium: tasks.iter().filter(|t| t.priority == TaskPriority::Medium).count(),
            low: tasks.iter().filter(|t| t.priority == TaskPriority::Low).count(),
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

        // 计算平均完成时间（从创建到完成）
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

        // 计算逾期任务数量
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
```

---

## TypeScript 类型定义

### 前端类型映射

```typescript
// src/types/summary.ts

export type SummaryType = 'daily' | 'weekly' | 'monthly' | 'semi_annual' | 'yearly';

export interface Summary {
  id: number;
  summary_type: SummaryType;
  period_start: number;        // Unix timestamp
  period_end: number;          // Unix timestamp
  tag: string | null;
  tag_filter: string[] | null;
  content: string;             // Markdown format
  statistics: string;          // JSON string
  task_ids: number[];
  created_at: number;          // Unix timestamp
  is_deleted: boolean;
}

export interface SummaryStatistics {
  total_tasks: number;
  completed: number;
  in_progress: number;
  todo: number;
  completion_rate: number;
  priority_distribution: PriorityDistribution;
  time_stats?: TimeStats;
}

export interface PriorityDistribution {
  high: number;
  medium: number;
  low: number;
}

export interface TimeStats {
  avg_completion_time_hours?: number;
  overdue_count: number;
}

// 解析 statistics 字段
export function parseSummaryStatistics(summary: Summary): SummaryStatistics | null {
  try {
    return JSON.parse(summary.statistics);
  } catch {
    return null;
  }
}

// 格式化时间范围
export function formatPeriodRange(start: number, end: number): string {
  const startDate = new Date(start * 1000);
  const endDate = new Date(end * 1000);

  const options: Intl.DateTimeFormatOptions = {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  };

  return `${startDate.toLocaleDateString('en-US', options)} - ${endDate.toLocaleDateString('en-US', options)}`;
}

// 格式化相对时间
export function formatRelativeTime(timestamp: number): string {
  const now = Date.now();
  const diff = now - timestamp * 1000;

  const minutes = Math.floor(diff / 60000);
  const hours = Math.floor(diff / 3600000);
  const days = Math.floor(diff / 86400000);

  if (minutes < 1) return 'just now';
  if (minutes < 60) return `${minutes} minute${minutes > 1 ? 's' : ''} ago`;
  if (hours < 24) return `${hours} hour${hours > 1 ? 's' : ''} ago`;
  if (days < 7) return `${days} day${days > 1 ? 's' : ''} ago`;
  if (days < 30) return `${Math.floor(days / 7)} week${Math.floor(days / 7) > 1 ? 's' : ''} ago`;
  if (days < 365) return `${Math.floor(days / 30)} month${Math.floor(days / 30) > 1 ? 's' : ''} ago`;
  return `${Math.floor(days / 365)} year${Math.floor(days / 365) > 1 ? 's' : ''} ago`;
}
```

---

## 数据查询接口

### Database 扩展方法

```rust
// src-tauri/src/db/mod.rs

impl Database {
    /// 创建总结
    pub fn create_summary(&self, summary: &Summary) -> Result<i64, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "INSERT INTO summaries (
                summary_type, period_start, period_end, tag, tag_filter,
                content, statistics, task_ids, created_at, is_deleted
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                summary.summary_type.as_str(),
                summary.period_start,
                summary.period_end,
                summary.tag,
                summary.tag_filter.as_ref().map(|t| serde_json::to_string(t).unwrap()),
                summary.content,
                summary.statistics,
                summary.task_ids.as_ref().map(|t| serde_json::to_string(t).unwrap()),
                summary.created_at,
                summary.is_deleted as i64,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// 根据条件查询总结（用于避免重复生成）
    pub fn get_summary_by_criteria(
        &self,
        summary_type: &SummaryType,
        tag: Option<&str>,
        period_start: i64,
        period_end: i64,
    ) -> Result<Option<Summary>, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, summary_type, period_start, period_end, tag, tag_filter,
                    content, statistics, task_ids, created_at, is_deleted
             FROM summaries
             WHERE summary_type = ?1
               AND (tag IS ?2 OR (?2 IS NULL AND tag IS NULL))
               AND period_start = ?3
               AND period_end = ?4
               AND is_deleted = 0
             ORDER BY created_at DESC
             LIMIT 1"
        )?;

        stmt.query_row(
            params![summary_type.as_str(), tag, period_start, period_end],
            |row| self.row_to_summary(row)
        )
        .optional()
    }

    /// 列出总结（支持分页和筛选）
    pub fn list_summaries(
        &self,
        tag: Option<&str>,
        summary_type: Option<&SummaryType>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Summary>, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();

        let mut query = String::from(
            "SELECT id, summary_type, period_start, period_end, tag, tag_filter,
                    content, statistics, task_ids, created_at, is_deleted
             FROM summaries
             WHERE is_deleted = 0"
        );

        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(t) = tag {
            query.push_str(" AND tag = ?");
            params.push(Box::new(t.to_string()));
        }

        if let Some(st) = summary_type {
            query.push_str(" AND summary_type = ?");
            params.push(Box::new(st.as_str().to_string()));
        }

        query.push_str(" ORDER BY created_at DESC");

        if let Some(l) = limit {
            query.push_str(&format!(" LIMIT {}", l));
        }

        if let Some(o) = offset {
            query.push_str(&format!(" OFFSET {}", o));
        }

        let mut stmt = conn.prepare(&query)?;
        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter()
            .map(|p| p.as_ref() as &dyn rusqlite::ToSql)
            .collect();

        let summaries = stmt.query_map(&params_refs[..], |row| {
            self.row_to_summary(row)
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(summaries)
    }

    /// 获取单个总结
    pub fn get_summary(&self, id: i64) -> Result<Option<Summary>, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();

        conn.query_row(
            "SELECT id, summary_type, period_start, period_end, tag, tag_filter,
                    content, statistics, task_ids, created_at, is_deleted
             FROM summaries
             WHERE id = ?1 AND is_deleted = 0",
            params![id],
            |row| self.row_to_summary(row)
        )
        .optional()
    }

    /// 删除总结（软删除）
    pub fn delete_summary(&self, id: i64) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "UPDATE summaries SET is_deleted = 1 WHERE id = ?1",
            params![id],
        )?;

        Ok(())
    }

    /// Row to Summary 转换
    fn row_to_summary(&self, row: &rusqlite::Row) -> Result<Summary, rusqlite::Error> {
        let task_ids_json: Option<String> = row.get(8)?;
        let tag_filter_json: Option<String> = row.get(5)?;

        Ok(Summary {
            id: Some(row.get(0)?),
            summary_type: SummaryType::from_str(&row.get::<_, String>(1)?)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e))))?,
            period_start: row.get(2)?,
            period_end: row.get(3)?,
            tag: row.get(4)?,
            tag_filter: tag_filter_json.and_then(|s| serde_json::from_str(&s).ok()),
            content: row.get(6)?,
            statistics: row.get(7)?,
            task_ids: task_ids_json.and_then(|s| serde_json::from_str(&s).ok()),
            created_at: row.get(9)?,
            is_deleted: row.get::<_, i64>(10)? != 0,
        })
    }
}
```

---

## 数据验证和约束

### 业务逻辑约束

```rust
impl Summary {
    /// 验证总结数据
    pub fn validate(&self) -> Result<(), String> {
        // 验证时间范围
        if self.period_start >= self.period_end {
            return Err("period_start must be less than period_end".to_string());
        }

        // 验证内容不为空
        if self.content.trim().is_empty() {
            return Err("content cannot be empty".to_string());
        }

        // 验证任务 IDs
        if let Some(ref task_ids) = self.task_ids {
            if task_ids.is_empty() {
                return Err("task_ids cannot be empty if present".to_string());
            }
        }

        Ok(())
    }
}
```

---

## 数据迁移脚本

```rust
// src-tauri/src/db/migrations.rs

pub fn migrate_to_v2(conn: &Connection) -> Result<(), rusqlite::Error> {
    // 添加 tag 和 tag_filter 字段
    conn.execute_batch(
        "ALTER TABLE summaries ADD COLUMN tag TEXT;
         ALTER TABLE summaries ADD COLUMN tag_filter TEXT;"
    )?;

    // 创建索引
    conn.execute_batch(
        "CREATE INDEX IF NOT EXISTS idx_summaries_tag ON summaries(tag);
         CREATE INDEX IF NOT EXISTS idx_summaries_type ON summaries(summary_type);
         CREATE INDEX IF NOT EXISTS idx_summaries_period ON summaries(period_start, period_end);
         CREATE INDEX IF NOT EXISTS idx_summaries_composite ON summaries(tag, summary_type, period_start, period_end);"
    )?;

    Ok(())
}
```
