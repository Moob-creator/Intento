use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::Database;

/// API Keys settings structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeysSettings {
    pub base_url: Option<String>,
    pub api_key: Option<String>,
}

impl Default for ApiKeysSettings {
    fn default() -> Self {
        Self {
            base_url: Some("https://api.openai.com/v1".to_string()),
            api_key: None,
        }
    }
}

/// Get API keys from database or environment
#[tauri::command]
pub fn get_api_keys(db: State<'_, Database>) -> Result<ApiKeysSettings, String> {
    let mut result = ApiKeysSettings::default();

    // Try to get from database first
    if let Ok(Some(url)) = db.inner().get_setting("api_base_url") {
        result.base_url = Some(url);
    } else if let Ok(url) = std::env::var("API_BASE_URL") {
        result.base_url = Some(url);
    }

    if let Ok(Some(key)) = db.inner().get_setting("api_key") {
        result.api_key = Some(key);
    } else if let Ok(key) = std::env::var("API_KEY") {
        result.api_key = Some(key);
    }

    Ok(result)
}

/// Update API keys in database
#[tauri::command]
pub fn update_api_keys(
    db: State<'_, Database>,
    base_url: Option<String>,
    api_key: Option<String>,
) -> Result<(), String> {
    // Update base URL
    let updated_url = if let Some(url) = base_url {
        if url.is_empty() {
            db.inner().delete_setting("api_base_url")
                .map_err(|e| format!("Failed to delete base URL: {}", e))?;
            None
        } else {
            db.inner().set_setting("api_base_url", &url)
                .map_err(|e| format!("Failed to save base URL: {}", e))?;
            Some(url.clone())
        }
    } else {
        db.inner().get_setting("api_base_url").ok().flatten()
    };

    // Update API key
    let updated_key = if let Some(key) = api_key {
        if key.is_empty() {
            db.inner().delete_setting("api_key")
                .map_err(|e| format!("Failed to delete API key: {}", e))?;
            None
        } else {
            db.inner().set_setting("api_key", &key)
                .map_err(|e| format!("Failed to save API key: {}", e))?;
            Some(key.clone())
        }
    } else {
        db.inner().get_setting("api_key").ok().flatten()
    };

    // ✨ Re-apply all environment variables after any update
    // Get current values from database for complete configuration
    let current_url = updated_url.or_else(|| db.inner().get_setting("api_base_url").ok().flatten());
    let current_key = updated_key.or_else(|| db.inner().get_setting("api_key").ok().flatten());

    if let (Some(url), Some(key)) = (current_url, current_key) {
        std::env::set_var("API_BASE_URL", &url);
        std::env::set_var("API_KEY", &key);

        // Detect provider and set specific env vars
        let url_lower = url.to_lowercase();
        if url_lower.contains("openai.com") || url_lower.contains("azure.com") {
            std::env::set_var("OPENAI_BASE_URL", &url);
            std::env::set_var("OPENAI_API_KEY", &key);
            std::env::set_var("AI_PROVIDER", "openai");
            println!("✅ Updated API configuration: OpenAI");
        } else if url_lower.contains("anthropic.com") {
            std::env::set_var("ANTHROPIC_BASE_URL", &url);
            std::env::set_var("ANTHROPIC_API_KEY", &key);
            std::env::set_var("AI_PROVIDER", "anthropic");
            println!("✅ Updated API configuration: Anthropic");
        } else if url_lower.contains("moonshot.cn") {
            std::env::set_var("MOONSHOT_BASE_URL", &url);
            std::env::set_var("MOONSHOT_API_KEY", &key);
            std::env::set_var("AI_PROVIDER", "kimi");
            println!("✅ Updated API configuration: Moonshot/Kimi");
        } else {
            // Default to OpenAI-compatible for custom URLs
            std::env::set_var("OPENAI_BASE_URL", &url);
            std::env::set_var("OPENAI_API_KEY", &key);
            std::env::set_var("AI_PROVIDER", "openai");
            println!("✅ Updated API configuration: OpenAI-compatible");
        }
    }

    Ok(())
}

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

// ========================================
// Notification Settings
// ========================================

/// Notification preferences structure
/// Controls all aspects of notification behavior in the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    /// Global notification toggle
    pub enabled: bool,

    /// Deadline reminder settings
    pub deadline_enabled: bool,
    /// Hours before deadline to send notification (e.g., 24 for 1 day before)
    pub deadline_advance_hours: i32,

    /// Daily review reminder settings
    pub daily_review_enabled: bool,
    /// Time for daily review in HH:MM format (24-hour)
    pub daily_review_time: String,

    /// Task completion notification settings
    pub task_completion_enabled: bool,

    /// Sound settings
    pub sound_enabled: bool,

    /// Do Not Disturb settings
    pub dnd_enabled: bool,
    /// DND start time in HH:MM format (24-hour)
    pub dnd_start_time: String,
    /// DND end time in HH:MM format (24-hour)
    pub dnd_end_time: String,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            deadline_enabled: true,
            deadline_advance_hours: 24,
            daily_review_enabled: true,
            daily_review_time: "09:00".to_string(),
            task_completion_enabled: true,
            sound_enabled: true,
            dnd_enabled: false,
            dnd_start_time: "22:00".to_string(),
            dnd_end_time: "08:00".to_string(),
        }
    }
}

impl NotificationSettings {
    /// Validate notification settings
    /// Returns Ok(()) if valid, Err with description if invalid
    pub fn validate(&self) -> Result<(), String> {
        // Validate deadline_advance_hours range (1-168 hours = 1 week max)
        if self.deadline_advance_hours < 1 || self.deadline_advance_hours > 168 {
            return Err(format!(
                "Invalid deadline_advance_hours: {}. Must be between 1 and 168.",
                self.deadline_advance_hours
            ));
        }

        // Validate time format for daily_review_time
        if !Self::is_valid_time_format(&self.daily_review_time) {
            return Err(format!(
                "Invalid daily_review_time format: {}. Expected HH:MM (24-hour format).",
                self.daily_review_time
            ));
        }

        // Validate time format for DND times
        if !Self::is_valid_time_format(&self.dnd_start_time) {
            return Err(format!(
                "Invalid dnd_start_time format: {}. Expected HH:MM (24-hour format).",
                self.dnd_start_time
            ));
        }

        if !Self::is_valid_time_format(&self.dnd_end_time) {
            return Err(format!(
                "Invalid dnd_end_time format: {}. Expected HH:MM (24-hour format).",
                self.dnd_end_time
            ));
        }

        Ok(())
    }

    /// Check if a time string is in valid HH:MM format (24-hour)
    fn is_valid_time_format(time: &str) -> bool {
        let parts: Vec<&str> = time.split(':').collect();
        if parts.len() != 2 {
            return false;
        }

        // Parse hours and minutes
        let hours = parts[0].parse::<u32>().ok();
        let minutes = parts[1].parse::<u32>().ok();

        match (hours, minutes) {
            (Some(h), Some(m)) => h < 24 && m < 60,
            _ => false,
        }
    }

    /// Check if notifications should be silenced based on Do Not Disturb settings
    /// Returns true if currently in DND period
    pub fn is_dnd_active(&self) -> bool {
        if !self.dnd_enabled {
            return false;
        }

        let now = chrono::Local::now();
        let current_time = now.format("%H:%M").to_string();

        // Compare times as strings (works for HH:MM format)
        // Handle overnight DND periods (e.g., 22:00 to 08:00)
        if self.dnd_start_time <= self.dnd_end_time {
            // Same day period (e.g., 10:00 to 18:00)
            current_time >= self.dnd_start_time && current_time < self.dnd_end_time
        } else {
            // Overnight period (e.g., 22:00 to 08:00)
            current_time >= self.dnd_start_time || current_time < self.dnd_end_time
        }
    }

    /// Check if notifications are currently allowed based on all settings
    pub fn should_notify(&self) -> bool {
        self.enabled && !self.is_dnd_active()
    }
}

/// Get notification settings from database
///
/// Retrieves all notification preferences from the database.
/// Returns default settings if none are configured.
#[tauri::command]
pub fn get_notification_settings(db: State<'_, Database>) -> Result<NotificationSettings, String> {
    let settings = db.inner().get_settings_by_prefix("notification_")
        .map_err(|e| format!("Failed to get notification settings: {}", e))?;

    let mut result = NotificationSettings::default();

    for (key, value) in settings {
        match key.as_str() {
            "notification_enabled" => result.enabled = value == "true",
            "notification_deadline_enabled" => result.deadline_enabled = value == "true",
            "notification_deadline_advance_hours" => {
                result.deadline_advance_hours = value.parse().unwrap_or(24);
            }
            "notification_daily_review_enabled" => result.daily_review_enabled = value == "true",
            "notification_daily_review_time" => result.daily_review_time = value,
            "notification_task_completion_enabled" => {
                result.task_completion_enabled = value == "true"
            }
            "notification_sound_enabled" => result.sound_enabled = value == "true",
            "notification_dnd_enabled" => result.dnd_enabled = value == "true",
            "notification_dnd_start_time" => result.dnd_start_time = value,
            "notification_dnd_end_time" => result.dnd_end_time = value,
            _ => {}
        }
    }

    Ok(result)
}

/// Update notification settings in database
///
/// Validates and stores notification preferences.
/// All settings are persisted to the database.
#[tauri::command]
pub fn update_notification_settings(
    db: State<'_, Database>,
    settings: NotificationSettings,
) -> Result<(), String> {
    // Validate settings before saving
    settings.validate()?;

    // Save each setting to database
    db.inner()
        .set_setting("notification_enabled", &settings.enabled.to_string())
        .map_err(|e| format!("Failed to set enabled: {}", e))?;

    db.inner()
        .set_setting("notification_deadline_enabled", &settings.deadline_enabled.to_string())
        .map_err(|e| format!("Failed to set deadline_enabled: {}", e))?;

    db.inner()
        .set_setting(
            "notification_deadline_advance_hours",
            &settings.deadline_advance_hours.to_string(),
        )
        .map_err(|e| format!("Failed to set deadline_advance_hours: {}", e))?;

    db.inner()
        .set_setting(
            "notification_daily_review_enabled",
            &settings.daily_review_enabled.to_string(),
        )
        .map_err(|e| format!("Failed to set daily_review_enabled: {}", e))?;

    db.inner()
        .set_setting("notification_daily_review_time", &settings.daily_review_time)
        .map_err(|e| format!("Failed to set daily_review_time: {}", e))?;

    db.inner()
        .set_setting(
            "notification_task_completion_enabled",
            &settings.task_completion_enabled.to_string(),
        )
        .map_err(|e| format!("Failed to set task_completion_enabled: {}", e))?;

    db.inner()
        .set_setting("notification_sound_enabled", &settings.sound_enabled.to_string())
        .map_err(|e| format!("Failed to set sound_enabled: {}", e))?;

    db.inner()
        .set_setting("notification_dnd_enabled", &settings.dnd_enabled.to_string())
        .map_err(|e| format!("Failed to set dnd_enabled: {}", e))?;

    db.inner()
        .set_setting("notification_dnd_start_time", &settings.dnd_start_time)
        .map_err(|e| format!("Failed to set dnd_start_time: {}", e))?;

    db.inner()
        .set_setting("notification_dnd_end_time", &settings.dnd_end_time)
        .map_err(|e| format!("Failed to set dnd_end_time: {}", e))?;

    println!("✓ Notification settings updated successfully");
    Ok(())
}

// ========================================
// Unit Tests
// ========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_settings_default() {
        let settings = NotificationSettings::default();
        assert!(settings.enabled);
        assert!(settings.deadline_enabled);
        assert_eq!(settings.deadline_advance_hours, 24);
        assert_eq!(settings.daily_review_time, "09:00");
        assert!(settings.daily_review_enabled);
        assert!(settings.task_completion_enabled);
        assert!(settings.sound_enabled);
        assert!(!settings.dnd_enabled);
    }

    #[test]
    fn test_notification_settings_validation_valid() {
        let settings = NotificationSettings {
            enabled: true,
            deadline_enabled: true,
            deadline_advance_hours: 48,
            daily_review_enabled: true,
            daily_review_time: "14:30".to_string(),
            task_completion_enabled: true,
            sound_enabled: true,
            dnd_enabled: true,
            dnd_start_time: "22:00".to_string(),
            dnd_end_time: "08:00".to_string(),
        };

        assert!(settings.validate().is_ok());
    }

    #[test]
    fn test_notification_settings_validation_invalid_advance_hours() {
        let mut settings = NotificationSettings::default();

        // Too small
        settings.deadline_advance_hours = 0;
        assert!(settings.validate().is_err());

        // Too large
        settings.deadline_advance_hours = 200;
        assert!(settings.validate().is_err());

        // Negative
        settings.deadline_advance_hours = -5;
        assert!(settings.validate().is_err());
    }

    #[test]
    fn test_notification_settings_validation_invalid_time_format() {
        let mut settings = NotificationSettings::default();

        // Invalid daily review time
        settings.daily_review_time = "25:00".to_string();
        assert!(settings.validate().is_err());

        settings.daily_review_time = "12:60".to_string();
        assert!(settings.validate().is_err());

        settings.daily_review_time = "12-30".to_string();
        assert!(settings.validate().is_err());

        settings.daily_review_time = "12".to_string();
        assert!(settings.validate().is_err());

        // Invalid DND times
        settings.daily_review_time = "09:00".to_string(); // Reset to valid
        settings.dnd_start_time = "invalid".to_string();
        assert!(settings.validate().is_err());

        settings.dnd_start_time = "22:00".to_string(); // Reset to valid
        settings.dnd_end_time = "24:00".to_string();
        assert!(settings.validate().is_err());
    }

    #[test]
    fn test_is_valid_time_format() {
        // Valid formats
        assert!(NotificationSettings::is_valid_time_format("00:00"));
        assert!(NotificationSettings::is_valid_time_format("09:30"));
        assert!(NotificationSettings::is_valid_time_format("12:00"));
        assert!(NotificationSettings::is_valid_time_format("23:59"));

        // Invalid formats
        assert!(!NotificationSettings::is_valid_time_format("24:00"));
        assert!(!NotificationSettings::is_valid_time_format("12:60"));
        assert!(!NotificationSettings::is_valid_time_format("25:30"));
        assert!(!NotificationSettings::is_valid_time_format("12"));
        assert!(!NotificationSettings::is_valid_time_format("12:30:45"));
        assert!(!NotificationSettings::is_valid_time_format("invalid"));
        assert!(!NotificationSettings::is_valid_time_format(""));
    }

    #[test]
    fn test_should_notify_when_enabled() {
        let settings = NotificationSettings {
            enabled: true,
            dnd_enabled: false,
            ..Default::default()
        };

        assert!(settings.should_notify());
    }

    #[test]
    fn test_should_notify_when_disabled() {
        let settings = NotificationSettings {
            enabled: false,
            dnd_enabled: false,
            ..Default::default()
        };

        assert!(!settings.should_notify());
    }

    #[test]
    fn test_serialization() {
        let settings = NotificationSettings::default();
        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: NotificationSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(settings.enabled, deserialized.enabled);
        assert_eq!(settings.deadline_advance_hours, deserialized.deadline_advance_hours);
        assert_eq!(settings.daily_review_time, deserialized.daily_review_time);
    }

    // Integration tests with database
    #[test]
    fn test_get_notification_settings_default() {
        let temp_dir = std::env::temp_dir();
        let db_path = temp_dir.join("intento_test_notification_settings_get.db");

        let db = Database::new(db_path.clone()).unwrap();

        // Test retrieving default settings
        let settings = db.get_settings_by_prefix("notification_").unwrap();
        assert!(!settings.is_empty());

        // Verify some default values exist
        let enabled = db.get_setting("notification_enabled").unwrap();
        assert_eq!(enabled, Some("true".to_string()));

        let deadline_hours = db.get_setting("notification_deadline_advance_hours").unwrap();
        assert_eq!(deadline_hours, Some("24".to_string()));

        // Clean up
        std::fs::remove_file(db_path).ok();
    }

    #[test]
    fn test_update_and_get_notification_settings() {
        let temp_dir = std::env::temp_dir();
        let db_path = temp_dir.join("intento_test_notification_settings_update.db");

        let db = Database::new(db_path.clone()).unwrap();

        // Create custom settings
        let custom_settings = NotificationSettings {
            enabled: false,
            deadline_enabled: false,
            deadline_advance_hours: 48,
            daily_review_enabled: false,
            daily_review_time: "14:30".to_string(),
            task_completion_enabled: false,
            sound_enabled: false,
            dnd_enabled: true,
            dnd_start_time: "21:00".to_string(),
            dnd_end_time: "07:00".to_string(),
        };

        // Save settings
        db.set_setting("notification_enabled", &custom_settings.enabled.to_string()).unwrap();
        db.set_setting("notification_deadline_enabled", &custom_settings.deadline_enabled.to_string()).unwrap();
        db.set_setting("notification_deadline_advance_hours", &custom_settings.deadline_advance_hours.to_string()).unwrap();
        db.set_setting("notification_daily_review_enabled", &custom_settings.daily_review_enabled.to_string()).unwrap();
        db.set_setting("notification_daily_review_time", &custom_settings.daily_review_time).unwrap();
        db.set_setting("notification_task_completion_enabled", &custom_settings.task_completion_enabled.to_string()).unwrap();
        db.set_setting("notification_sound_enabled", &custom_settings.sound_enabled.to_string()).unwrap();
        db.set_setting("notification_dnd_enabled", &custom_settings.dnd_enabled.to_string()).unwrap();
        db.set_setting("notification_dnd_start_time", &custom_settings.dnd_start_time).unwrap();
        db.set_setting("notification_dnd_end_time", &custom_settings.dnd_end_time).unwrap();

        // Retrieve and verify
        let settings_map = db.get_settings_by_prefix("notification_").unwrap();

        // Build NotificationSettings from settings_map
        let mut retrieved = NotificationSettings::default();
        for (key, value) in settings_map {
            match key.as_str() {
                "notification_enabled" => retrieved.enabled = value == "true",
                "notification_deadline_enabled" => retrieved.deadline_enabled = value == "true",
                "notification_deadline_advance_hours" => {
                    retrieved.deadline_advance_hours = value.parse().unwrap_or(24);
                }
                "notification_daily_review_enabled" => retrieved.daily_review_enabled = value == "true",
                "notification_daily_review_time" => retrieved.daily_review_time = value,
                "notification_task_completion_enabled" => {
                    retrieved.task_completion_enabled = value == "true"
                }
                "notification_sound_enabled" => retrieved.sound_enabled = value == "true",
                "notification_dnd_enabled" => retrieved.dnd_enabled = value == "true",
                "notification_dnd_start_time" => retrieved.dnd_start_time = value,
                "notification_dnd_end_time" => retrieved.dnd_end_time = value,
                _ => {}
            }
        }

        assert_eq!(retrieved.enabled, custom_settings.enabled);
        assert_eq!(retrieved.deadline_enabled, custom_settings.deadline_enabled);
        assert_eq!(retrieved.deadline_advance_hours, custom_settings.deadline_advance_hours);
        assert_eq!(retrieved.daily_review_time, custom_settings.daily_review_time);
        assert_eq!(retrieved.dnd_enabled, custom_settings.dnd_enabled);
        assert_eq!(retrieved.dnd_start_time, custom_settings.dnd_start_time);
        assert_eq!(retrieved.dnd_end_time, custom_settings.dnd_end_time);

        // Clean up
        std::fs::remove_file(db_path).ok();
    }

    #[test]
    fn test_settings_persistence() {
        let temp_dir = std::env::temp_dir();
        let db_path = temp_dir.join("intento_test_notification_persistence.db");

        {
            let db = Database::new(db_path.clone()).unwrap();
            db.set_setting("notification_enabled", "false").unwrap();
            db.set_setting("notification_deadline_advance_hours", "72").unwrap();
        }

        // Reopen database and verify persistence
        {
            let db = Database::new(db_path.clone()).unwrap();
            let enabled = db.get_setting("notification_enabled").unwrap();
            assert_eq!(enabled, Some("false".to_string()));

            let hours = db.get_setting("notification_deadline_advance_hours").unwrap();
            assert_eq!(hours, Some("72".to_string()));
        }

        // Clean up
        std::fs::remove_file(db_path).ok();
    }
}

