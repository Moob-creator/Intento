/// Test with real image - screenshot of a task reminder
///
/// Image content: "明天16号上午10点给安全科开会，准备材料"

#[cfg(test)]
mod real_image_tests {
    use serde_json::json;
    use std::fs;

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

    #[tokio::test]
    #[ignore]
    async fn test_parse_real_task_image() {
        load_env();

        let api_key = std::env::var("MOONSHOT_API_KEY")
            .expect("MOONSHOT_API_KEY must be set");

        // Read the real test image
        let image_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples/test.jpg");

        let image_data = fs::read(&image_path)
            .expect("Failed to read test image");

        // Convert to base64
        let image_base64 = base64::encode(&image_data);

        println!("Image loaded, size: {} bytes", image_data.len());
        println!("Base64 length: {}", image_base64.len());

        // Define the create_task tool
        let tool = json!({
            "type": "function",
            "function": {
                "name": "create_task",
                "description": "创建新任务。当图片中包含待办事项、任务列表、日程安排、提醒事项时调用此工具。",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "title": {
                            "type": "string",
                            "description": "任务标题，简洁明了地概括任务内容"
                        },
                        "description": {
                            "type": "string",
                            "description": "任务详细描述，包含从图片中提取的所有相关信息"
                        },
                        "priority": {
                            "type": "string",
                            "enum": ["low", "medium", "high"],
                            "description": "任务优先级：high=紧急重要，medium=一般，low=不紧急"
                        },
                        "deadline": {
                            "type": "string",
                            "description": "截止时间 ISO8601 格式，例如 2024-03-16T10:00:00+08:00"
                        },
                        "tags": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "任务标签列表，如：工作、会议等"
                        }
                    },
                    "required": ["title"]
                }
            }
        });

        // Get current time
        let current_time = chrono::Utc::now().to_rfc3339();
        let prompt = format!(
            "当前时间: {}。请仔细分析这张图片，识别其中包含的任务、待办事项或日程信息。\
             如果图片中有任务相关内容，请调用 create_task 工具来创建任务。\
             注意：图片中提到的\"明天16号\"，请根据当前时间计算准确的日期和时间。",
            current_time
        );

        let messages = vec![json!({
            "role": "user",
            "content": [
                {
                    "type": "image_url",
                    "image_url": {
                        "url": format!("data:image/jpeg;base64,{}", image_base64)
                    }
                },
                {
                    "type": "text",
                    "text": prompt
                }
            ]
        })];

        // Call Kimi Vision API with tool
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

        println!("\n========================================");
        println!("API Status: {}", status);
        println!("========================================");
        println!("Response:\n{}", body);
        println!("========================================\n");

        assert!(status.is_success(), "API call should succeed");

        let json: serde_json::Value = serde_json::from_str(&body)
            .expect("Response should be valid JSON");

        // Check if tool_calls exist
        if let Some(tool_calls) = json["choices"][0]["message"]["tool_calls"].as_array() {
            println!("✅ Vision model successfully called tool!");
            println!("\nNumber of tool calls: {}", tool_calls.len());

            for (i, tool_call) in tool_calls.iter().enumerate() {
                println!("\n--- Tool Call {} ---", i + 1);
                println!("Function name: {}", tool_call["function"]["name"]);

                let args_str = tool_call["function"]["arguments"]
                    .as_str()
                    .expect("Arguments should be string");

                println!("Arguments: {}", args_str);

                // Parse arguments as JSON for better display
                if let Ok(args_json) = serde_json::from_str::<serde_json::Value>(args_str) {
                    println!("\nParsed task:");
                    println!("  Title: {}", args_json["title"].as_str().unwrap_or("N/A"));
                    println!("  Description: {}", args_json["description"].as_str().unwrap_or("N/A"));
                    println!("  Priority: {}", args_json["priority"].as_str().unwrap_or("N/A"));
                    println!("  Deadline: {}", args_json["deadline"].as_str().unwrap_or("N/A"));
                    if let Some(tags) = args_json["tags"].as_array() {
                        println!("  Tags: {:?}", tags);
                    }
                }
            }

            assert!(!tool_calls.is_empty(), "Should have at least one tool call");
        } else {
            println!("⚠️  No tool calls - vision model did not call create_task");
            println!("Message content: {:?}", json["choices"][0]["message"]["content"]);
            panic!("Expected tool calls but got none");
        }
    }
}
