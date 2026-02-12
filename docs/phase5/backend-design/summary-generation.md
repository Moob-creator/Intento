# Phase 5: 总结生成逻辑设计

## 核心架构

### 模块结构

```
src-tauri/src/summary/
├── mod.rs                 # 模块导出
├── generator.rs           # SummaryGenerator 核心逻辑
├── period.rs              # 时间周期计算
└── scheduler_jobs.rs      # 定时任务函数
```

---

## 1. SummaryGenerator 核心类

### 结构定义

```rust
// src-tauri/src/summary/generator.rs

use crate::ai::AiClient;
use crate::db::{Database, models::{Summary, SummaryType, SummaryStatistics, Task, TaskStatus}};
use anyhow::Result;
use chrono::{Utc, Datelike, Timelike};

pub struct SummaryGenerator {
    db: Database,
    ai_client: AiClient,
}

impl SummaryGenerator {
    /// 创建新的 Generator 实例
    pub fn new(db: Database, ai_client: AiClient) -> Self {
        Self { db, ai_client }
    }

    /// 生成或获取总结（如果已存在则返回，否则生成新的）
    pub async fn get_or_generate_summary(
        &self,
        tag: Option<String>,
        summary_type: SummaryType,
        period_start: i64,
        period_end: i64,
    ) -> Result<Summary> {
        // 1. 检查是否已存在
        if let Some(existing) = self.db.get_summary_by_criteria(
            &summary_type,
            tag.as_deref(),
            period_start,
            period_end,
        )? {
            return Ok(existing);
        }

        // 2. 不存在则生成新总结
        self.generate_summary(tag, summary_type, period_start, period_end).await
    }

    /// 生成新总结
    pub async fn generate_summary(
        &self,
        tag: Option<String>,
        summary_type: SummaryType,
        period_start: i64,
        period_end: i64,
    ) -> Result<Summary> {
        // 1. 查询时间范围内的任务
        let tasks = self.query_tasks_by_period(tag.as_deref(), period_start, period_end)?;

        // 如果没有任务，返回空总结
        if tasks.is_empty() {
            return Ok(self.create_empty_summary(tag, summary_type, period_start, period_end));
        }

        // 2. 计算统计数据
        let statistics = SummaryStatistics::from_tasks(&tasks);

        // 3. 使用 AI 生成总结文本
        let content = self.generate_summary_content(&tasks, &statistics, &summary_type).await?;

        // 4. 创建 Summary 对象
        let task_ids: Vec<i64> = tasks.iter().filter_map(|t| t.id).collect();
        let summary = Summary::new(
            summary_type,
            period_start,
            period_end,
            tag,
            content,
            Some(statistics),
            task_ids,
        );

        // 5. 保存到数据库
        let summary_id = self.db.create_summary(&summary)?;

        Ok(Summary {
            id: Some(summary_id),
            ..summary
        })
    }

    /// 查询指定时间范围内的任务
    fn query_tasks_by_period(
        &self,
        tag: Option<&str>,
        period_start: i64,
        period_end: i64,
    ) -> Result<Vec<Task>> {
        let all_tasks = self.db.list_tasks(None)?;

        let filtered: Vec<Task> = all_tasks
            .into_iter()
            .filter(|task| {
                // 时间范围过滤（包括已完成和进行中的任务）
                let in_period = task.completed_at
                    .map(|t| t >= period_start && t <= period_end)
                    .unwrap_or_else(|| {
                        // 对于未完成的任务，检查是否在此期间创建或更新
                        (task.created_at >= period_start && task.created_at <= period_end) ||
                        (task.updated_at >= period_start && task.updated_at <= period_end)
                    });

                // Tag 过滤
                let matches_tag = tag.map(|t| {
                    task.tags
                        .as_ref()
                        .map(|tags| tags.contains(&t.to_string()))
                        .unwrap_or(false)
                }).unwrap_or(true);

                in_period && matches_tag && !task.is_deleted
            })
            .collect();

        Ok(filtered)
    }

    /// 使用 AI 生成总结内容
    async fn generate_summary_content(
        &self,
        tasks: &[Task],
        statistics: &SummaryStatistics,
        summary_type: &SummaryType,
    ) -> Result<String> {
        // 构建 Prompt
        let prompt = self.build_summary_prompt(tasks, statistics, summary_type);

        // 调用 AI
        let response = self.ai_client
            .chat(&prompt)
            .await
            .map_err(|e| anyhow::anyhow!("AI generation failed: {}", e))?;

        Ok(response)
    }

    /// 构建总结生成的 Prompt
    fn build_summary_prompt(
        &self,
        tasks: &[Task],
        statistics: &SummaryStatistics,
        summary_type: &SummaryType,
    ) -> String {
        let period_name = match summary_type {
            SummaryType::Daily => "今日",
            SummaryType::Weekly => "本周",
            SummaryType::Monthly => "本月",
            SummaryType::SemiAnnual => "本半年",
            SummaryType::Yearly => "本年",
        };

        // 分类任务
        let completed_tasks: Vec<&Task> = tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Done)
            .collect();

        let in_progress_tasks: Vec<&Task> = tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Doing)
            .collect();

        let todo_tasks: Vec<&Task> = tasks
            .iter()
            .filter(|t| t.status == TaskStatus::Todo)
            .collect();

        // 构建任务列表
        let completed_list = self.format_task_list(&completed_tasks);
        let in_progress_list = self.format_task_list(&in_progress_tasks);
        let todo_list = self.format_task_list(&todo_tasks);

        format!(
            r#"你是一个温暖、友好的任务总结助手。请根据以下{period_name}的任务数据生成一份简洁、鼓励性的总结报告。

## 统计数据
- 总任务数: {total}
- 已完成: {completed} ({completion_rate:.1}%)
- 进行中: {in_progress}
- 待办: {todo}
- 优先级分布:
  - 高优先级: {high_priority}
  - 中优先级: {medium_priority}
  - 低优先级: {low_priority}

## 已完成任务 ({completed_count} 个)
{completed_list}

## 进行中任务 ({in_progress_count} 个)
{in_progress_list}

## 待办任务 ({todo_count} 个)
{todo_list}

## 要求
请生成一份 Markdown 格式的总结报告，包含以下部分：

### 1. 一句话总结 (🎯)
用一句话概括{period_name}的工作成果，突出最重要的成就。

### 2. 亮点成就 (✨)
列举 2-3 个值得表扬的地方：
- 完成的高优先级或重要任务
- 效率提升或进步表现
- 任何值得庆祝的里程碑

### 3. 进行中的工作 (🔄)
如果有进行中的任务，简要说明进展情况。

### 4. 待办提醒 (📋)
温和地提醒未完成的重要任务，但要保持鼓励性语气。

### 5. 下一步建议 (💡)
基于数据趋势，给出 1-2 条建设性建议。

## 风格要求
- ✅ 温暖、友好、鼓励性
- ✅ 突出成就感和进步
- ✅ 避免批评性或负面语言
- ✅ 适当使用表情符号增加亲和力
- ✅ 控制在 200-400 字以内
- ✅ 使用中文输出
- ✅ 使用 Markdown 格式

请直接输出总结报告，不要有任何前言或解释。"#,
            period_name = period_name,
            total = statistics.total_tasks,
            completed = statistics.completed,
            completion_rate = statistics.completion_rate * 100.0,
            in_progress = statistics.in_progress,
            todo = statistics.todo,
            high_priority = statistics.priority_distribution.high,
            medium_priority = statistics.priority_distribution.medium,
            low_priority = statistics.priority_distribution.low,
            completed_count = completed_tasks.len(),
            in_progress_count = in_progress_tasks.len(),
            todo_count = todo_tasks.len(),
            completed_list = completed_list,
            in_progress_list = in_progress_list,
            todo_list = todo_list,
        )
    }

    /// 格式化任务列表
    fn format_task_list(&self, tasks: &[&Task]) -> String {
        if tasks.is_empty() {
            return "无".to_string();
        }

        tasks
            .iter()
            .take(10) // 最多列出 10 个任务
            .map(|task| {
                let priority_emoji = match task.priority {
                    crate::db::models::TaskPriority::High => "🔥",
                    crate::db::models::TaskPriority::Medium => "⭐",
                    crate::db::models::TaskPriority::Low => "📝",
                };

                let tags_str = task.tags
                    .as_ref()
                    .map(|tags| {
                        if tags.is_empty() {
                            String::new()
                        } else {
                            format!(" #{}", tags.join(" #"))
                        }
                    })
                    .unwrap_or_default();

                format!("- {} {} {}", priority_emoji, task.title, tags_str)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// 创建空总结（当没有任务时）
    fn create_empty_summary(
        &self,
        tag: Option<String>,
        summary_type: SummaryType,
        period_start: i64,
        period_end: i64,
    ) -> Summary {
        let period_name = match summary_type {
            SummaryType::Daily => "今日",
            SummaryType::Weekly => "本周",
            SummaryType::Monthly => "本月",
            SummaryType::SemiAnnual => "本半年",
            SummaryType::Yearly => "本年",
        };

        let content = format!(
            r#"# 📝 {period_name}总结

{period_name}暂时没有任务记录。

## 💡 建议

这是一个全新的开始！考虑添加一些任务来管理你的工作和生活吧。

---

*{period_name}没有任务数据*"#,
            period_name = period_name
        );

        Summary::new(
            summary_type,
            period_start,
            period_end,
            tag,
            content,
            Some(SummaryStatistics {
                total_tasks: 0,
                completed: 0,
                in_progress: 0,
                todo: 0,
                completion_rate: 0.0,
                priority_distribution: crate::db::models::PriorityDistribution {
                    high: 0,
                    medium: 0,
                    low: 0,
                },
                time_stats: None,
            }),
            vec![],
        )
    }
}
```

---

## 2. 时间周期计算

### 周期计算工具

```rust
// src-tauri/src/summary/period.rs

use chrono::{DateTime, Utc, Datelike, Duration, NaiveDate, TimeZone};

/// 计算指定类型的时间周期
pub struct PeriodCalculator;

impl PeriodCalculator {
    /// 获取今日的开始和结束时间
    pub fn today() -> (i64, i64) {
        let now = Utc::now();
        let start = now
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp();
        let end = start + 86400 - 1; // 23:59:59
        (start, end)
    }

    /// 获取本周的开始和结束时间（周一到周日）
    pub fn this_week() -> (i64, i64) {
        let now = Utc::now();
        let weekday = now.weekday().num_days_from_monday();

        let start = (now - Duration::days(weekday as i64))
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp();

        let end = start + 7 * 86400 - 1;
        (start, end)
    }

    /// 获取本月的开始和结束时间
    pub fn this_month() -> (i64, i64) {
        let now = Utc::now();

        let start = NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp();

        let next_month = if now.month() == 12 {
            NaiveDate::from_ymd_opt(now.year() + 1, 1, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(now.year(), now.month() + 1, 1).unwrap()
        };

        let end = next_month
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp() - 1;

        (start, end)
    }

    /// 获取本半年的开始和结束时间
    pub fn this_semi_annual() -> (i64, i64) {
        let now = Utc::now();
        let year = now.year();

        let (start_month, end_month) = if now.month() <= 6 {
            (1, 6) // 1-6 月
        } else {
            (7, 12) // 7-12 月
        };

        let start = NaiveDate::from_ymd_opt(year, start_month, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp();

        let end_date = if end_month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(year, end_month + 1, 1).unwrap()
        };

        let end = end_date
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp() - 1;

        (start, end)
    }

    /// 获取本年的开始和结束时间
    pub fn this_year() -> (i64, i64) {
        let now = Utc::now();
        let year = now.year();

        let start = NaiveDate::from_ymd_opt(year, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp();

        let end = NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp() - 1;

        (start, end)
    }

    /// 根据 SummaryType 获取当前周期
    pub fn current_period(summary_type: &crate::db::models::SummaryType) -> (i64, i64) {
        match summary_type {
            crate::db::models::SummaryType::Daily => Self::today(),
            crate::db::models::SummaryType::Weekly => Self::this_week(),
            crate::db::models::SummaryType::Monthly => Self::this_month(),
            crate::db::models::SummaryType::SemiAnnual => Self::this_semi_annual(),
            crate::db::models::SummaryType::Yearly => Self::this_year(),
        }
    }

    /// 获取昨日的开始和结束时间
    pub fn yesterday() -> (i64, i64) {
        let now = Utc::now();
        let yesterday = now - Duration::days(1);
        let start = yesterday
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp();
        let end = start + 86400 - 1;
        (start, end)
    }

    /// 获取上周的开始和结束时间
    pub fn last_week() -> (i64, i64) {
        let (this_week_start, _) = Self::this_week();
        let last_week_start = this_week_start - 7 * 86400;
        let last_week_end = this_week_start - 1;
        (last_week_start, last_week_end)
    }

    /// 获取上月的开始和结束时间
    pub fn last_month() -> (i64, i64) {
        let now = Utc::now();

        let (year, month) = if now.month() == 1 {
            (now.year() - 1, 12)
        } else {
            (now.year(), now.month() - 1)
        };

        let start = NaiveDate::from_ymd_opt(year, month, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp();

        let this_month_start = NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp();

        let end = this_month_start - 1;

        (start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_today() {
        let (start, end) = PeriodCalculator::today();
        assert!(start < end);
        assert_eq!(end - start, 86399); // 一天的秒数减 1
    }

    #[test]
    fn test_this_week() {
        let (start, end) = PeriodCalculator::this_week();
        assert!(start < end);
        assert_eq!(end - start, 7 * 86400 - 1);
    }

    #[test]
    fn test_this_month() {
        let (start, end) = PeriodCalculator::this_month();
        assert!(start < end);
        // 月份天数不固定，只检查范围合理性
        let days = (end - start + 1) / 86400;
        assert!(days >= 28 && days <= 31);
    }
}
```

---

## 3. 定时任务函数

```rust
// src-tauri/src/summary/scheduler_jobs.rs

use crate::db::Database;
use crate::ai::AiClient;
use crate::db::models::SummaryType;
use super::{SummaryGenerator, PeriodCalculator};
use std::collections::HashSet;

/// 生成每日总结（为所有活跃 tags）
pub async fn generate_daily_summaries(db: &Database) -> Result<(), String> {
    let ai_client = AiClient::new_default()
        .map_err(|e| format!("Failed to create AI client: {}", e))?;

    let generator = SummaryGenerator::new(db.clone(), ai_client);

    // 获取昨日的时间范围（因为是凌晨 1 点执行）
    let (period_start, period_end) = PeriodCalculator::yesterday();

    // 获取所有活跃的 tags
    let active_tags = get_active_tags(db)?;

    // 为每个 tag 生成总结
    for tag in active_tags {
        match generator.generate_summary(
            Some(tag.clone()),
            SummaryType::Daily,
            period_start,
            period_end,
        ).await {
            Ok(_) => println!("✓ Generated daily summary for tag: {}", tag),
            Err(e) => eprintln!("✗ Failed to generate daily summary for tag {}: {}", tag, e),
        }
    }

    // 生成全局总结（无 tag 过滤）
    match generator.generate_summary(
        None,
        SummaryType::Daily,
        period_start,
        period_end,
    ).await {
        Ok(_) => println!("✓ Generated daily summary for all tasks"),
        Err(e) => eprintln!("✗ Failed to generate daily summary for all tasks: {}", e),
    }

    Ok(())
}

/// 生成每周总结
pub async fn generate_weekly_summaries(db: &Database) -> Result<(), String> {
    let ai_client = AiClient::new_default()
        .map_err(|e| format!("Failed to create AI client: {}", e))?;

    let generator = SummaryGenerator::new(db.clone(), ai_client);

    // 获取上周的时间范围
    let (period_start, period_end) = PeriodCalculator::last_week();

    let active_tags = get_active_tags(db)?;

    for tag in active_tags {
        match generator.generate_summary(
            Some(tag.clone()),
            SummaryType::Weekly,
            period_start,
            period_end,
        ).await {
            Ok(_) => println!("✓ Generated weekly summary for tag: {}", tag),
            Err(e) => eprintln!("✗ Failed to generate weekly summary for tag {}: {}", tag, e),
        }
    }

    // 全局周总结
    match generator.generate_summary(
        None,
        SummaryType::Weekly,
        period_start,
        period_end,
    ).await {
        Ok(_) => println!("✓ Generated weekly summary for all tasks"),
        Err(e) => eprintln!("✗ Failed to generate weekly summary for all tasks: {}", e),
    }

    Ok(())
}

/// 生成每月总结
pub async fn generate_monthly_summaries(db: &Database) -> Result<(), String> {
    let ai_client = AiClient::new_default()
        .map_err(|e| format!("Failed to create AI client: {}", e))?;

    let generator = SummaryGenerator::new(db.clone(), ai_client);

    // 获取上月的时间范围
    let (period_start, period_end) = PeriodCalculator::last_month();

    let active_tags = get_active_tags(db)?;

    for tag in active_tags {
        match generator.generate_summary(
            Some(tag.clone()),
            SummaryType::Monthly,
            period_start,
            period_end,
        ).await {
            Ok(_) => println!("✓ Generated monthly summary for tag: {}", tag),
            Err(e) => eprintln!("✗ Failed to generate monthly summary for tag {}: {}", tag, e),
        }
    }

    // 全局月总结
    match generator.generate_summary(
        None,
        SummaryType::Monthly,
        period_start,
        period_end,
    ).await {
        Ok(_) => println!("✓ Generated monthly summary for all tasks"),
        Err(e) => eprintln!("✗ Failed to generate monthly summary for all tasks: {}", e),
    }

    Ok(())
}

/// 获取所有活跃的 tags
fn get_active_tags(db: &Database) -> Result<Vec<String>, String> {
    let tasks = db.list_tasks(None)
        .map_err(|e| format!("Failed to list tasks: {}", e))?;

    let tags: HashSet<String> = tasks
        .iter()
        .filter(|t| !t.is_deleted)
        .flat_map(|t| t.tags.as_ref())
        .flatten()
        .cloned()
        .collect();

    Ok(tags.into_iter().collect())
}
```

---

## 4. 错误处理

### 自定义错误类型

```rust
// src-tauri/src/summary/error.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SummaryError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("AI generation error: {0}")]
    AiGeneration(String),

    #[error("Invalid time period: start={0}, end={1}")]
    InvalidPeriod(i64, i64),

    #[error("No tasks found in the specified period")]
    NoTasksFound,

    #[error("Summary already exists: id={0}")]
    AlreadyExists(i64),
}
```

---

## 5. 使用示例

### 手动生成总结

```rust
use crate::summary::{SummaryGenerator, PeriodCalculator};

// 创建 Generator
let generator = SummaryGenerator::new(db, ai_client);

// 生成今日的 work tag 总结
let (start, end) = PeriodCalculator::today();
let summary = generator.generate_summary(
    Some("work".to_string()),
    SummaryType::Daily,
    start,
    end,
).await?;

println!("Generated summary: {}", summary.content);
```

### 检查并获取总结

```rust
// 如果存在则返回，否则生成新的
let summary = generator.get_or_generate_summary(
    Some("work".to_string()),
    SummaryType::Weekly,
    start,
    end,
).await?;
```
