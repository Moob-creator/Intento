use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::Database;

/// Auto summary settings structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoSummarySettings {
    pub enabled: bool,
    pub daily_enabled: bool,
    pub weekly_enabled: bool,
    pub monthly_enabled: bool,
    pub semi_annual_enabled: bool,
    pub yearly_enabled: bool,
    pub retention_days: i32,
}

impl Default for AutoSummarySettings {
    fn default() -> Self {
        Self {
            enabled: true,
            daily_enabled: true,
            weekly_enabled: true,
            monthly_enabled: true,
            semi_annual_enabled: true,
            yearly_enabled: true,
            retention_days: 365,
        }
    }
}

/// Get auto summary settings from database
#[tauri::command]
pub fn get_auto_summary_settings(db: State<'_, Database>) -> Result<AutoSummarySettings, String> {
    let settings = db.inner().get_settings_by_prefix("auto_summary_")
        .map_err(|e| format!("Failed to get settings: {}", e))?;

    let mut result = AutoSummarySettings::default();

    for (key, value) in settings {
        match key.as_str() {
            "auto_summary_enabled" => result.enabled = value == "true",
            "auto_summary_daily_enabled" => result.daily_enabled = value == "true",
            "auto_summary_weekly_enabled" => result.weekly_enabled = value == "true",
            "auto_summary_monthly_enabled" => result.monthly_enabled = value == "true",
            "auto_summary_semi_annual_enabled" => result.semi_annual_enabled = value == "true",
            "auto_summary_yearly_enabled" => result.yearly_enabled = value == "true",
            "auto_summary_retention_days" => {
                result.retention_days = value.parse().unwrap_or(365);
            }
            _ => {}
        }
    }

    Ok(result)
}

/// Update auto summary settings in database
#[tauri::command]
pub fn update_auto_summary_settings(
    db: State<'_, Database>,
    settings: AutoSummarySettings,
) -> Result<(), String> {
    db.inner().set_setting("auto_summary_enabled", &settings.enabled.to_string())
        .map_err(|e| format!("Failed to set enabled: {}", e))?;

    db.inner().set_setting("auto_summary_daily_enabled", &settings.daily_enabled.to_string())
        .map_err(|e| format!("Failed to set daily_enabled: {}", e))?;

    db.inner().set_setting("auto_summary_weekly_enabled", &settings.weekly_enabled.to_string())
        .map_err(|e| format!("Failed to set weekly_enabled: {}", e))?;

    db.inner().set_setting("auto_summary_monthly_enabled", &settings.monthly_enabled.to_string())
        .map_err(|e| format!("Failed to set monthly_enabled: {}", e))?;

    db.inner().set_setting("auto_summary_semi_annual_enabled", &settings.semi_annual_enabled.to_string())
        .map_err(|e| format!("Failed to set semi_annual_enabled: {}", e))?;

    db.inner().set_setting("auto_summary_yearly_enabled", &settings.yearly_enabled.to_string())
        .map_err(|e| format!("Failed to set yearly_enabled: {}", e))?;

    db.inner().set_setting("auto_summary_retention_days", &settings.retention_days.to_string())
        .map_err(|e| format!("Failed to set retention_days: {}", e))?;

    println!("✓ Auto summary settings updated successfully");
    Ok(())
}
