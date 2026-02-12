// Scheduled summary generation jobs
// This module contains functions for automatic summary generation

use crate::ai::AiClient;
use crate::db::Database;
use crate::db::models::SummaryType;
use crate::summary::period::PeriodCalculator;
use crate::summary::SummaryGenerator;

/// Generate daily summaries for all active tags
pub async fn generate_daily_summaries(db: &Database) -> Result<(), String> {
    println!("🔄 Starting daily summary generation...");

    generate_summaries_for_type(db, SummaryType::Daily).await
}

/// Generate weekly summaries
pub async fn generate_weekly_summaries(db: &Database) -> Result<(), String> {
    println!("🔄 Starting weekly summary generation...");

    generate_summaries_for_type(db, SummaryType::Weekly).await
}

/// Generate monthly summaries
pub async fn generate_monthly_summaries(db: &Database) -> Result<(), String> {
    println!("🔄 Starting monthly summary generation...");

    generate_summaries_for_type(db, SummaryType::Monthly).await
}

/// Generate semi-annual summaries
pub async fn generate_semi_annual_summaries(db: &Database) -> Result<(), String> {
    println!("🔄 Starting semi-annual summary generation...");

    generate_summaries_for_type(db, SummaryType::SemiAnnual).await
}

/// Generate yearly summaries
pub async fn generate_yearly_summaries(db: &Database) -> Result<(), String> {
    println!("🔄 Starting yearly summary generation...");

    generate_summaries_for_type(db, SummaryType::Yearly).await
}

/// Generic function to generate summaries for a specific type
async fn generate_summaries_for_type(
    db: &Database,
    summary_type: SummaryType,
) -> Result<(), String> {
    // Get the current period for this summary type
    let (period_start, period_end) = PeriodCalculator::current_period(&summary_type);

    println!(
        "📅 Period: {} - {} (type: {:?})",
        format_timestamp(period_start),
        format_timestamp(period_end),
        summary_type
    );

    // Check if summary already exists for this period
    let existing = db
        .find_summary_by_period(&summary_type, period_start, period_end, None)
        .map_err(|e| format!("Failed to check existing summary: {}", e))?;

    if existing.is_some() {
        println!("✅ Summary already exists for this period, skipping generation");
        return Ok(());
    }

    // Initialize AI client
    let ai_client = AiClient::new_default()
        .map_err(|e| format!("Failed to initialize AI client: {}", e))?;

    // Create generator
    let generator = SummaryGenerator::new(db.clone(), ai_client);

    // Generate global summary (no tag filter)
    match generator
        .generate_summary(None, summary_type.clone(), period_start, period_end)
        .await
    {
        Ok(summary) => {
            println!(
                "✅ Generated global {:?} summary (ID: {})",
                summary_type,
                summary.id.unwrap_or(0)
            );
        }
        Err(e) => {
            eprintln!(
                "❌ Failed to generate global {:?} summary: {}",
                summary_type, e
            );
            return Err(format!("Failed to generate summary: {}", e));
        }
    }

    // Get all unique tags from tasks
    let all_tasks = db
        .list_tasks(None)
        .map_err(|e| format!("Failed to list tasks: {}", e))?;

    let mut all_tags = std::collections::HashSet::new();
    for task in &all_tasks {
        if let Some(tags) = &task.tags {
            for tag in tags {
                all_tags.insert(tag.clone());
            }
        }
    }

    println!("📋 Found {} unique tags", all_tags.len());

    // Generate summaries for each tag
    let mut success_count = 0;
    let mut error_count = 0;

    for tag in all_tags {
        // Check if tag-specific summary already exists
        let existing_tag = db
            .find_summary_by_period(&summary_type, period_start, period_end, Some(&tag))
            .map_err(|e| format!("Failed to check existing tag summary: {}", e))?;

        if existing_tag.is_some() {
            println!("✅ Summary for tag '{}' already exists, skipping", tag);
            continue;
        }

        // Generate summary for this tag
        match generator
            .generate_summary(Some(tag.clone()), summary_type.clone(), period_start, period_end)
            .await
        {
            Ok(summary) => {
                println!(
                    "✅ Generated {:?} summary for tag '{}' (ID: {})",
                    summary_type,
                    tag,
                    summary.id.unwrap_or(0)
                );
                success_count += 1;
            }
            Err(e) => {
                eprintln!(
                    "❌ Failed to generate {:?} summary for tag '{}': {}",
                    summary_type, tag, e
                );
                error_count += 1;
            }
        }
    }

    println!(
        "🎉 Summary generation complete: {} successful, {} errors",
        success_count + 1, // +1 for global summary
        error_count
    );

    if error_count > 0 {
        Err(format!(
            "Summary generation completed with {} errors",
            error_count
        ))
    } else {
        Ok(())
    }
}

/// Helper function to format timestamp for logging
fn format_timestamp(timestamp: i64) -> String {
    use chrono::{DateTime, Utc, TimeZone};
    let dt: DateTime<Utc> = Utc.timestamp_opt(timestamp, 0).unwrap();
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

