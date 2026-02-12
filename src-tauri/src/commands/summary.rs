use crate::ai::AiClient;
use crate::db::{Database, models::{Summary, SummaryType}};
use crate::summary::SummaryGenerator;
use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Lazy-initialized AI client state for summary commands
pub struct AiClientState {
    client: Arc<RwLock<Option<AiClient>>>,
}

impl AiClientState {
    pub fn new() -> Self {
        Self {
            client: Arc::new(RwLock::new(None)),
        }
    }

    /// Gets or initializes the AI client
    async fn get_or_init(&self) -> Result<AiClient, String> {
        let read_lock = self.client.read().await;
        if read_lock.is_some() {
            // Client already initialized, create a new one (cheap operation)
            drop(read_lock);
            return AiClient::new_default().map_err(|e| e.to_string());
        }
        drop(read_lock);

        let mut write_lock = self.client.write().await;
        if write_lock.is_none() {
            let client = AiClient::new_default().map_err(|e| e.to_string())?;
            *write_lock = Some(client);
        }

        AiClient::new_default().map_err(|e| e.to_string())
    }
}

/// Generate a new summary for specified tag and period
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

    let ai_client = ai_state.get_or_init().await?;
    let generator = SummaryGenerator::new(db.inner().clone(), ai_client);

    generator.generate_summary(tag, summary_type, period_start, period_end)
        .await
        .map_err(|e| format!("Failed to generate summary: {}", e))
}

/// Get existing summary or generate new one if not exists
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

    // get_or_generate_summary checks cache first before generating
    let ai_client = ai_state.get_or_init().await?;
    let generator = SummaryGenerator::new(db.inner().clone(), ai_client);

    generator.get_or_generate_summary(tag, summary_type, period_start, period_end)
        .await
        .map_err(|e| format!("Failed to get or generate summary: {}", e))
}

/// List summaries with optional filters
#[tauri::command]
pub async fn list_summaries(
    tag: Option<String>,
    summary_type: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    db: State<'_, Database>,
) -> Result<Vec<Summary>, String> {
    let summary_type_parsed = summary_type
        .map(|s| SummaryType::from_str(&s))
        .transpose()
        .map_err(|e| format!("Invalid summary type: {}", e))?;

    db.list_summaries(
        tag.as_deref(),
        summary_type_parsed.as_ref(),
        limit,
        offset,
    )
    .map_err(|e| format!("Failed to list summaries: {}", e))
}

/// Get a single summary by ID
#[tauri::command]
pub async fn get_summary(
    id: i64,
    db: State<'_, Database>,
) -> Result<Option<Summary>, String> {
    db.get_summary(id)
        .map_err(|e| format!("Failed to get summary: {}", e))
}

/// Delete a summary (soft delete)
#[tauri::command]
pub async fn delete_summary(
    id: i64,
    db: State<'_, Database>,
) -> Result<(), String> {
    db.delete_summary(id)
        .map_err(|e| format!("Failed to delete summary: {}", e))
}

/// Export summary to markdown or text format
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

/// Format summary as markdown
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

/// Format summary as plain text
fn format_as_text(summary: &Summary) -> String {
    // Remove markdown formatting
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

/// Format timestamp to readable string
fn format_timestamp(timestamp: i64) -> String {
    use chrono::{DateTime, Utc, TimeZone};
    let dt: DateTime<Utc> = Utc.timestamp_opt(timestamp, 0).unwrap();
    dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

/// Format period range to readable string
fn format_period_range(start: i64, end: i64) -> String {
    use chrono::{DateTime, Utc, TimeZone};
    let start_dt: DateTime<Utc> = Utc.timestamp_opt(start, 0).unwrap();
    let end_dt: DateTime<Utc> = Utc.timestamp_opt(end, 0).unwrap();

    format!(
        "{} - {}",
        start_dt.format("%Y-%m-%d"),
        end_dt.format("%Y-%m-%d")
    )
}
