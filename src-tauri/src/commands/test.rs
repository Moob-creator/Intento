use crate::ai::AiClient;
use tauri::State;

/// Test Kimi API connection with a simple "Hello World" request
#[tauri::command]
pub async fn test_kimi_connection() -> Result<String, String> {
    // Create AI client
    let client = AiClient::new_default().map_err(|e| {
        format!("Failed to create AI client: {}", e)
    })?;

    // Send a simple test message
    let test_result = client
        .parse_text_input("Hello World")
        .await
        .map_err(|e| format!("Kimi API call failed: {}", e))?;

    // Return success message with parsed result
    Ok(format!(
        "✅ Kimi API connected successfully!\n\nTest Input: 'Hello World'\nParsed Title: {}",
        test_result.title
    ))
}
