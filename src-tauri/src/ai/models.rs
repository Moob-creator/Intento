use serde::{Deserialize, Serialize};

/// Parsed task information from natural language input
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParsedTask {
    /// Task title (required)
    pub title: String,

    /// Optional task description
    pub description: Option<String>,

    /// Optional deadline in ISO8601 format (e.g., "2024-12-31T23:59:59Z")
    pub deadline: Option<String>,

    /// Optional priority level: "low", "medium", or "high"
    pub priority: Option<String>,

    /// Optional tags for categorization
    pub tags: Option<Vec<String>>,
}

impl ParsedTask {
    /// Validates the priority field if present
    pub fn validate_priority(&self) -> Result<(), String> {
        if let Some(ref priority) = self.priority {
            match priority.to_lowercase().as_str() {
                "low" | "medium" | "high" => Ok(()),
                _ => Err(format!("Invalid priority: {}. Must be 'low', 'medium', or 'high'", priority)),
            }
        } else {
            Ok(())
        }
    }

    /// Parses and validates the deadline field if present
    pub fn parse_deadline(&self) -> Result<Option<chrono::DateTime<chrono::Utc>>, String> {
        if let Some(ref deadline_str) = self.deadline {
            chrono::DateTime::parse_from_rfc3339(deadline_str)
                .map(|dt| Some(dt.with_timezone(&chrono::Utc)))
                .map_err(|e| format!("Invalid deadline format: {}", e))
        } else {
            Ok(None)
        }
    }

    /// Normalizes priority to lowercase
    pub fn normalize_priority(&mut self) {
        if let Some(ref priority) = self.priority {
            self.priority = Some(priority.to_lowercase());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_priority_valid() {
        let task = ParsedTask {
            title: "Test".to_string(),
            description: None,
            deadline: None,
            priority: Some("high".to_string()),
            tags: None,
        };
        assert!(task.validate_priority().is_ok());
    }

    #[test]
    fn test_validate_priority_invalid() {
        let task = ParsedTask {
            title: "Test".to_string(),
            description: None,
            deadline: None,
            priority: Some("urgent".to_string()),
            tags: None,
        };
        assert!(task.validate_priority().is_err());
    }

    #[test]
    fn test_parse_deadline_valid() {
        let task = ParsedTask {
            title: "Test".to_string(),
            description: None,
            deadline: Some("2024-12-31T23:59:59Z".to_string()),
            priority: None,
            tags: None,
        };
        assert!(task.parse_deadline().is_ok());
    }

    #[test]
    fn test_parse_deadline_invalid() {
        let task = ParsedTask {
            title: "Test".to_string(),
            description: None,
            deadline: Some("invalid-date".to_string()),
            priority: None,
            tags: None,
        };
        assert!(task.parse_deadline().is_err());
    }

    #[test]
    fn test_normalize_priority() {
        let mut task = ParsedTask {
            title: "Test".to_string(),
            description: None,
            deadline: None,
            priority: Some("HIGH".to_string()),
            tags: None,
        };
        task.normalize_priority();
        assert_eq!(task.priority, Some("high".to_string()));
    }
}
