/// Test Kimi Vision + Tool Calling capabilities
///
/// This test verifies if moonshot-v1-8k-vision-preview can effectively
/// combine vision recognition with tool calling in a single request.

#[cfg(test)]
mod kimi_vision_tool_tests {
    use serde_json::json;

    fn load_env() {
        // Load .env file from project root
        let env_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(".env");

        if env_path.exists() {
            dotenv::from_path(&env_path).ok();
        }
    }

    /// Test 1: Vision + Tool Calling in one request
    #[tokio::test]
    #[ignore] // Run with: cargo test --test test_kimi_vision_tools -- --ignored
    async fn test_vision_with_tool_calling() {
        load_env();

        let api_key = std::env::var("MOONSHOT_API_KEY")
            .expect("MOONSHOT_API_KEY must be set");

        // Create a simple test image (red square with text "买牛奶!!!")
        // For actual test, you would use a real image
        let test_image_base64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";

        let tool = json!({
            "type": "function",
            "function": {
                "name": "create_task",
                "description": "创建任务。当图片中包含待办事项、任务列表时调用。",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "title": {
                            "type": "string",
                            "description": "任务标题"
                        },
                        "priority": {
                            "type": "string",
                            "enum": ["low", "medium", "high"]
                        }
                    },
                    "required": ["title"]
                }
            }
        });

        let messages = vec![json!({
            "role": "user",
            "content": [
                {
                    "type": "image_url",
                    "image_url": {
                        "url": format!("data:image/png;base64,{}", test_image_base64)
                    }
                },
                {
                    "type": "text",
                    "text": "分析这张图片，如果包含任务信息就调用 create_task 工具"
                }
            ]
        })];

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.moonshot.cn/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "moonshot-v1-8k-vision-preview",
                "messages": messages,
                "tools": [tool],
                "temperature": 0.3
            }))
            .send()
            .await
            .expect("API call failed");

        let status = response.status();
        let body = response.text().await.expect("Failed to read response");

        println!("Status: {}", status);
        println!("Response: {}", body);

        assert!(status.is_success(), "API call should succeed");

        let json: serde_json::Value = serde_json::from_str(&body)
            .expect("Response should be valid JSON");

        // Check if tool_calls exist
        if let Some(tool_calls) = json["choices"][0]["message"]["tool_calls"].as_array() {
            println!("✅ Vision model successfully called tool!");
            println!("Tool calls: {:?}", tool_calls);
            assert!(!tool_calls.is_empty(), "Should have tool calls");
        } else {
            println!("⚠️  No tool calls - vision model may not support tool calling well");
            println!("Message content: {:?}", json["choices"][0]["message"]["content"]);
        }
    }

    /// Test 2: Text-only with Tool Calling (baseline comparison)
    #[tokio::test]
    #[ignore]
    async fn test_text_only_with_tool_calling() {
        load_env();

        let api_key = std::env::var("MOONSHOT_API_KEY")
            .expect("MOONSHOT_API_KEY must be set");

        let tool = json!({
            "type": "function",
            "function": {
                "name": "create_task",
                "description": "创建任务",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "title": {"type": "string"}
                    },
                    "required": ["title"]
                }
            }
        });

        let messages = vec![json!({
            "role": "user",
            "content": "请帮我创建一个任务：买牛奶"
        })];

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.moonshot.cn/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "kimi-k2-turbo-preview",  // Text-only model
                "messages": messages,
                "tools": [tool],
                "temperature": 0.3
            }))
            .send()
            .await
            .expect("API call failed");

        let status = response.status();
        let body = response.text().await.expect("Failed to read response");

        println!("Status: {}", status);
        println!("Response: {}", body);

        let json: serde_json::Value = serde_json::from_str(&body)
            .expect("Response should be valid JSON");

        if let Some(tool_calls) = json["choices"][0]["message"]["tool_calls"].as_array() {
            println!("✅ Text model successfully called tool!");
            println!("Tool calls: {:?}", tool_calls);
        }
    }
}
