# Phase 5: Tauri Commands API 设计

## API 概览

Phase 5 新增的 Tauri Commands，用于前端调用后端总结功能。

---

## Commands 列表

### 1. generate_summary - 生成总结

**功能**: 为指定 tag 和时间范围生成新总结

```rust
#[tauri::command]
pub async fn generate_summary(
    tag: Option<String>,
    summary_type: String,
    period_start: i64,
    period_end: i64,
    db: State<'_, Database>,
    ai_state: State<'_, AiClientState>,
) -> Result<Summary, String> {
    let summary_type = SummaryType::from_str(&summary_type)
        .map_err(|e| format!("Invalid summary type: {}", e))?;

    let ai_client = ai_state.get_client()
        .map_err(|e| format!("Failed to get AI client: {}", e))?;

    let generator = SummaryGenerator::new(db.inner().clone(), ai_client);

    generator.generate_summary(tag, summary_type, period_start, period_end)
        .await
        .map_err(|e| format!("Failed to generate summary: {}", e))
}
```

**前端调用**:
```typescript
const summary = await invoke<Summary>('generate_summary', {
  tag: 'work',
  summaryType: 'weekly',
  periodStart: startTimestamp,
  periodEnd: endTimestamp,
});
```

---

### 2. get_or_generate_summary - 获取或生成总结

**功能**: 如果总结已存在则返回，否则生成新的

```rust
#[tauri::command]
pub async fn get_or_generate_summary(
    tag: Option<String>,
    summary_type: String,
    period_start: i64,
    period_end: i64,
    db: State<'_, Database>,
    ai_state: State<'_, AiClientState>,
) -> Result<Summary, String> {
    let summary_type = SummaryType::from_str(&summary_type)
        .map_err(|e| format!("Invalid summary type: {}", e))?;

    let ai_client = ai_state.get_client()
        .map_err(|e| format!("Failed to get AI client: {}", e))?;

    let generator = SummaryGenerator::new(db.inner().clone(), ai_client);

    generator.get_or_generate_summary(tag, summary_type, period_start, period_end)
        .await
        .map_err(|e| format!("Failed to get or generate summary: {}", e))
}
```

**前端调用**:
```typescript
const summary = await invoke<Summary>('get_or_generate_summary', {
  tag: null, // 全局总结
  summaryType: 'daily',
  periodStart: todayStart,
  periodEnd: todayEnd,
});
```

---

### 3. list_summaries - 列出总结

**功能**: 查询总结列表，支持筛选和分页

```rust
#[tauri::command]
pub async fn list_summaries(
    tag: Option<String>,
    summary_type: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    db: State<'_, Database>,
) -> Result<Vec<Summary>, String> {
    let summary_type = summary_type
        .map(|s| SummaryType::from_str(&s))
        .transpose()
        .map_err(|e| format!("Invalid summary type: {}", e))?;

    db.list_summaries(tag.as_deref(), summary_type.as_ref(), limit, offset)
        .map_err(|e| format!("Failed to list summaries: {}", e))
}
```

**前端调用**:
```typescript
// 获取 work tag 的最近 20 条总结
const summaries = await invoke<Summary[]>('list_summaries', {
  tag: 'work',
  summaryType: null,
  limit: 20,
  offset: 0,
});

// 获取所有周总结
const weeklySummaries = await invoke<Summary[]>('list_summaries', {
  tag: null,
  summaryType: 'weekly',
  limit: 10,
  offset: 0,
});
```

---

### 4. get_summary - 获取单个总结

**功能**: 根据 ID 获取总结详情

```rust
#[tauri::command]
pub async fn get_summary(
    id: i64,
    db: State<'_, Database>,
) -> Result<Option<Summary>, String> {
    db.get_summary(id)
        .map_err(|e| format!("Failed to get summary: {}", e))
}
```

**前端调用**:
```typescript
const summary = await invoke<Summary | null>('get_summary', { id: 123 });
if (summary) {
  console.log(summary.content);
}
```

---

### 5. delete_summary - 删除总结

**功能**: 软删除总结（设置 is_deleted 标志）

```rust
#[tauri::command]
pub async fn delete_summary(
    id: i64,
    db: State<'_, Database>,
) -> Result<(), String> {
    db.delete_summary(id)
        .map_err(|e| format!("Failed to delete summary: {}", e))
}
```

**前端调用**:
```typescript
await invoke('delete_summary', { id: 123 });
toast.success('Summary deleted');
```

---

### 6. export_summary - 导出总结

**功能**: 将总结导出为 Markdown 或纯文本

```rust
#[tauri::command]
pub async fn export_summary(
    id: i64,
    format: String, // "markdown" | "text"
    db: State<'_, Database>,
) -> Result<String, String> {
    let summary = db.get_summary(id)
        .map_err(|e| format!("Failed to get summary: {}", e))?
        .ok_or_else(|| "Summary not found".to_string())?;

    let content = match format.as_str() {
        "markdown" => format_as_markdown(&summary),
        "text" => format_as_text(&summary),
        _ => return Err("Invalid format. Use 'markdown' or 'text'".to_string()),
    };

    Ok(content)
}

fn format_as_markdown(summary: &Summary) -> String {
    let period_range = format_period_range(summary.period_start, summary.period_end);
    let tag_info = summary.tag.as_ref()
        .map(|t| format!("**Tag:** {}\n\n", t))
        .unwrap_or_default();

    format!(
        "# {} Summary\n\n{}\
         **Period:** {}\n\n\
         ---\n\n\
         {}\n\n\
         ---\n\n\
         *Generated on {}*\n",
        summary.summary_type.as_str().to_uppercase(),
        tag_info,
        period_range,
        summary.content,
        format_timestamp(summary.created_at)
    )
}

fn format_as_text(summary: &Summary) -> String {
    // 移除 Markdown 标记，转换为纯文本
    let content = summary.content
        .replace("# ", "")
        .replace("## ", "")
        .replace("### ", "")
        .replace("**", "")
        .replace("*", "");

    format!(
        "{} SUMMARY\n\
         ================\n\n\
         Period: {}\n\
         Tag: {}\n\n\
         {}\n\n\
         Generated on: {}\n",
        summary.summary_type.as_str().to_uppercase(),
        format_period_range(summary.period_start, summary.period_end),
        summary.tag.as_deref().unwrap_or("All"),
        content,
        format_timestamp(summary.created_at)
    )
}
```

**前端调用**:
```typescript
// 导出为 Markdown
const content = await invoke<string>('export_summary', {
  id: 123,
  format: 'markdown',
});

// 保存文件
import { save, writeTextFile } from '@tauri-apps/api/dialog';

const path = await save({
  defaultPath: `summary-${id}.md`,
  filters: [{ name: 'Markdown', extensions: ['md'] }],
});

if (path) {
  await writeTextFile(path, content);
  toast.success(`Exported to ${path}`);
}
```

---

### 7. get_summary_statistics - 获取总结统计概览

**功能**: 获取总结系统的统计信息（可选，用于仪表板）

```rust
#[tauri::command]
pub async fn get_summary_statistics(
    db: State<'_, Database>,
) -> Result<SummaryStatisticsOverview, String> {
    let all_summaries = db.list_summaries(None, None, None, None)
        .map_err(|e| format!("Failed to list summaries: {}", e))?;

    let total_count = all_summaries.len();
    let by_type = count_by_type(&all_summaries);
    let by_tag = count_by_tag(&all_summaries);
    let latest = all_summaries.first().map(|s| s.created_at);

    Ok(SummaryStatisticsOverview {
        total_count,
        by_type,
        by_tag,
        latest_generated_at: latest,
    })
}

#[derive(serde::Serialize)]
pub struct SummaryStatisticsOverview {
    total_count: usize,
    by_type: std::collections::HashMap<String, usize>,
    by_tag: std::collections::HashMap<String, usize>,
    latest_generated_at: Option<i64>,
}
```

**前端调用**:
```typescript
const stats = await invoke<SummaryStatisticsOverview>('get_summary_statistics');
console.log(`Total summaries: ${stats.total_count}`);
console.log(`Daily summaries: ${stats.by_type.daily}`);
```

---

## 注册 Commands

在 `main.rs` 中注册所有新增的 commands：

```rust
tauri::Builder::default()
    .manage(db)
    .manage(ai_client_state)
    .invoke_handler(tauri::generate_handler![
        // Phase 1-4 existing commands...
        create_task,
        update_task,
        delete_task,
        list_tasks,

        // ✨ Phase 5 new commands
        generate_summary,
        get_or_generate_summary,
        list_summaries,
        get_summary,
        delete_summary,
        export_summary,
        get_summary_statistics,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
```

---

## 错误处理

所有 commands 统一返回 `Result<T, String>`，前端需要捕获错误：

```typescript
try {
  const summary = await invoke<Summary>('generate_summary', params);
  // 处理成功
} catch (error) {
  toast.error(`Failed to generate summary: ${error}`);
}
```

---

## API 使用示例

### 完整的前端集成示例

```typescript
// src/hooks/useSummary.ts

import { invoke } from '@tauri-apps/api/tauri';
import { Summary, SummaryType } from '../types/summary';

export function useSummary() {
  const generateSummary = async (
    tag: string | null,
    summaryType: SummaryType,
    periodStart: number,
    periodEnd: number
  ): Promise<Summary> => {
    return await invoke<Summary>('generate_summary', {
      tag,
      summaryType,
      periodStart,
      periodEnd,
    });
  };

  const getSummary = async (id: number): Promise<Summary | null> => {
    return await invoke<Summary | null>('get_summary', { id });
  };

  const listSummaries = async (
    tag?: string | null,
    summaryType?: SummaryType | null,
    limit: number = 20,
    offset: number = 0
  ): Promise<Summary[]> => {
    return await invoke<Summary[]>('list_summaries', {
      tag,
      summaryType,
      limit,
      offset,
    });
  };

  const deleteSummary = async (id: number): Promise<void> => {
    await invoke('delete_summary', { id });
  };

  const exportSummary = async (
    id: number,
    format: 'markdown' | 'text'
  ): Promise<string> => {
    return await invoke<string>('export_summary', { id, format });
  };

  return {
    generateSummary,
    getSummary,
    listSummaries,
    deleteSummary,
    exportSummary,
  };
}
```
