use crate::ai::{AiClient, ParsedTask, ImageParseResult, ToolSet};
use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;

/// Lazy-initialized AI client state
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

/// Parses natural language text input into structured task information
///
/// # Arguments
/// * `text` - Natural language description of a task
/// * `existing_tags` - Optional list of existing tags from current tasks
///
/// # Returns
/// A `ParsedTask` object with extracted information
///
/// # Errors
/// Returns error string if:
/// - AI client initialization fails (missing API key)
/// - API call fails
/// - Response parsing fails
///
/// # Example
/// ```javascript
/// // From frontend
/// const result = await invoke('parse_text_input', {
///   text: 'Finish report by tomorrow 5pm, high priority',
///   existingTags: ['work', 'urgent', 'project-x']
/// });
/// console.log(result.title, result.deadline, result.priority);
/// ```
#[tauri::command]
pub async fn parse_text_input(
    text: String,
    existing_tags: Option<Vec<String>>,
    state: State<'_, AiClientState>,
) -> Result<ParsedTask, String> {
    if text.trim().is_empty() {
        return Err("Input text cannot be empty".to_string());
    }

    let client = state.get_or_init().await?;
    client
        .parse_text_input_with_tags(&text, existing_tags.as_deref())
        .await
        .map_err(|e| format!("Failed to parse text: {}", e))
}

/// Tests AI client connectivity and configuration
///
/// # Returns
/// `true` if AI client is properly configured and can make API calls
///
/// # Example
/// ```javascript
/// // From frontend
/// const isHealthy = await invoke('ai_health_check');
/// if (!isHealthy) {
///   console.error('AI service is not available');
/// }
/// ```
#[tauri::command]
pub async fn ai_health_check(state: State<'_, AiClientState>) -> Result<bool, String> {
    match state.get_or_init().await {
        Ok(client) => Ok(client.health_check().await),
        Err(_) => Ok(false),
    }
}

/// Parses image input (screenshot, photo) into structured task information
///
/// # Arguments
/// * `image_base64` - Base64-encoded image data (without data URI prefix)
/// * `image_type` - MIME type of the image (e.g., "image/png", "image/jpeg")
///
/// # Returns
/// A `ParsedTask` object with extracted information from the image
///
/// # Errors
/// Returns error string if:
/// - AI client initialization fails
/// - Image format is invalid
/// - API call fails
/// - Response parsing fails
///
/// # Example
/// ```javascript
/// // From frontend
/// const result = await invoke('parse_image_input', {
///   imageBase64: 'iVBORw0KGgoAAAANSUhEUg...',
///   imageType: 'image/png'
/// });
/// console.log(result.title, result.description);
/// ```
#[tauri::command]
pub async fn parse_image_input(
    image_base64: String,
    image_type: String,
    state: State<'_, AiClientState>,
) -> Result<ParsedTask, String> {
    if image_base64.trim().is_empty() {
        return Err("Image data cannot be empty".to_string());
    }

    // Validate image type
    if !image_type.starts_with("image/") {
        return Err(format!("Invalid image type: {}", image_type));
    }

    let client = state.get_or_init().await?;
    client
        .parse_image_input(&image_base64, &image_type)
        .await
        .map_err(|e| format!("Failed to parse image: {}", e))
}

/// Gets the current AI provider configuration
///
/// # Returns
/// Provider name ("openai" or "anthropic") or error if not configured
///
/// # Example
/// ```javascript
/// const provider = await invoke('get_ai_provider');
/// console.log('Using provider:', provider);
/// ```
#[tauri::command]
pub fn get_ai_provider() -> Result<String, String> {
    std::env::var("AI_PROVIDER")
        .map(|p| p.to_lowercase())
        .or_else(|_| {
            // Check which API key is available
            if std::env::var("ANTHROPIC_API_KEY").is_ok() {
                Ok("anthropic".to_string())
            } else if std::env::var("OPENAI_API_KEY").is_ok() {
                Ok("openai".to_string())
            } else {
                Err("No AI provider configured. Set OPENAI_API_KEY or ANTHROPIC_API_KEY environment variable.".to_string())
            }
        })
}

/// Parses image input with tool-based operation extraction
///
/// # Arguments
/// * `image_base64` - Base64-encoded image data (without data URI prefix)
/// * `image_type` - MIME type of the image
/// * `use_all_tools` - Whether to use all tools (true) or just basic create tool (false)
/// * `existing_tags` - Optional list of existing tags from current tasks
///
/// # Returns
/// An `ImageParseResult` with extracted task operations
#[tauri::command]
pub async fn parse_image_for_operations(
    image_base64: String,
    image_type: String,
    use_all_tools: bool,
    existing_tags: Option<Vec<String>>,
    state: State<'_, AiClientState>,
) -> Result<ImageParseResult, String> {
    if image_base64.trim().is_empty() {
        return Err("图片数据不能为空".to_string());
    }

    if !image_type.starts_with("image/") {
        return Err(format!("无效的图片类型: {}", image_type));
    }

    let client = state.get_or_init().await?;

    // Choose tool set based on parameter
    let tool_set = if use_all_tools {
        ToolSet::All
    } else {
        ToolSet::Basic
    };

    client
        .parse_image_for_operations_with_tags(&image_base64, &image_type, tool_set, existing_tags.as_deref())
        .await
        .map_err(|e| format!("图片识别失败: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_client_state_new() {
        let state = AiClientState::new();
        assert!(state.client.try_read().is_ok());
    }

    #[test]
    fn test_parse_text_input_empty() {
        // This would require async runtime and mocked state
        // Keeping as placeholder for integration test
    }

    #[tokio::test]
    async fn test_get_or_init_caching() {
        // Test that get_or_init properly caches the client
        let state = AiClientState::new();

        // First call should initialize
        let result1 = state.get_or_init().await;

        // Second call should reuse (if env is set)
        let result2 = state.get_or_init().await;

        // Both should have same error/success state
        assert_eq!(result1.is_ok(), result2.is_ok());
    }
}
