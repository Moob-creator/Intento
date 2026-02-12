use crate::ai::AiClient;
use crate::db::{Database, models::{Summary, SummaryType, SummaryStatistics, Task, TaskStatus, Priority}};
use anyhow::Result;

/// Summary generator - generates AI-powered summaries for tasks
pub struct SummaryGenerator {
    db: Database,
    ai_client: AiClient,
}

impl SummaryGenerator {
    /// Create a new summary generator
    pub fn new(db: Database, ai_client: AiClient) -> Self {
        Self { db, ai_client }
    }

    /// Generate a new summary for specified period and tag
    pub async fn generate_summary(
        &self,
        tag: Option<String>,
        summary_type: SummaryType,
        period_start: i64,
        period_end: i64,
    ) -> Result<Summary> {
        // 1. Query tasks in period
        let tasks = self.query_tasks_by_period(tag.as_deref(), period_start, period_end)?;

        // 2. If no tasks, return empty summary
        if tasks.is_empty() {
            return Ok(self.create_empty_summary(tag, summary_type, period_start, period_end));
        }

        // 3. Calculate statistics
        let statistics = SummaryStatistics::from_tasks(&tasks);

        // 4. Generate AI summary content
        let content = self.generate_summary_content(&tasks, &statistics, &summary_type).await?;

        // 5. Create and save summary
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

        // Save to database
        let summary_id = self.db.create_summary(&summary)
            .map_err(|e| anyhow::anyhow!("Failed to save summary: {}", e))?;

        Ok(Summary {
            id: Some(summary_id),
            ..summary
        })
    }

    /// Get or generate summary (returns existing if found, otherwise generates new one)
    pub async fn get_or_generate_summary(
        &self,
        tag: Option<String>,
        summary_type: SummaryType,
        period_start: i64,
        period_end: i64,
    ) -> Result<Summary> {
        // Check if summary already exists in database
        let existing = self.db.find_summary_by_period(
            &summary_type,
            period_start,
            period_end,
            tag.as_deref(),
        )?;

        if let Some(summary) = existing {
            // Return cached summary
            return Ok(summary);
        }

        // Generate new summary if not found
        self.generate_summary(tag, summary_type, period_start, period_end).await
    }

    /// Query tasks within the specified period
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
                // Time period filter
                let in_period = task.completed_at
                    .map(|t| t >= period_start && t <= period_end)
                    .unwrap_or_else(|| {
                        // For uncompleted tasks, check if created or updated in period
                        (task.created_at >= period_start && task.created_at <= period_end) ||
                        (task.updated_at >= period_start && task.updated_at <= period_end)
                    });

                // Tag filter
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

    /// Generate summary content using AI
    async fn generate_summary_content(
        &self,
        tasks: &[Task],
        statistics: &SummaryStatistics,
        summary_type: &SummaryType,
    ) -> Result<String> {
        let prompt = self.build_summary_prompt(tasks, statistics, summary_type);

        // Use the new chat method for AI interaction
        let response = self.ai_client
            .chat(&prompt)
            .await
            .map_err(|e| anyhow::anyhow!("AI generation failed: {}", e))?;

        Ok(response)
    }

    /// Build prompt for AI summary generation
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

        // Classify tasks
        let completed: Vec<&Task> = tasks.iter().filter(|t| t.status == TaskStatus::Done).collect();
        let in_progress: Vec<&Task> = tasks.iter().filter(|t| t.status == TaskStatus::Doing).collect();
        let todo: Vec<&Task> = tasks.iter().filter(|t| t.status == TaskStatus::Todo).collect();

        let completed_list = self.format_task_list(&completed);
        let in_progress_list = self.format_task_list(&in_progress);
        let todo_list = self.format_task_list(&todo);

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
            completed_count = completed.len(),
            in_progress_count = in_progress.len(),
            todo_count = todo.len(),
            completed_list = completed_list,
            in_progress_list = in_progress_list,
            todo_list = todo_list,
        )
    }

    /// Format task list for prompt
    fn format_task_list(&self, tasks: &[&Task]) -> String {
        if tasks.is_empty() {
            return "无".to_string();
        }

        tasks
            .iter()
            .take(10) // Max 10 tasks
            .map(|task| {
                let priority_emoji = match task.priority {
                    Priority::High => "🔥",
                    Priority::Medium => "⭐",
                    Priority::Low => "📝",
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

    /// Create empty summary when no tasks found
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
