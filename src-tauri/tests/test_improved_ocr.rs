/// Improved test with better OCR prompt

#[cfg(test)]
mod improved_ocr_tests {
    use serde_json::json;
    use std::fs;

    fn load_env() {
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
    async fn test_with_explicit_ocr_prompt() {
        load_env();

        let api_key = std::env::var("MOONSHOT_API_KEY")
            .expect("MOONSHOT_API_KEY must be set");

        // Read the test image
        let image_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples/test.jpg");

        let image_data = fs::read(&image_path)
            .expect("Failed to read test image");

        use base64::{Engine as _, engine::general_purpose};
        let image_base64 = general_purpose::STANDARD.encode(&image_data);

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
                            "description": "任务标题，从图片中**准确提取**的任务内容"
                        },
                        "description": {
                            "type": "string",
                            "description": "任务详细描述，包含从图片中**准确识别**的所有文字信息"
                        },
                        "priority": {
                            "type": "string",
                            "enum": ["low", "medium", "high"],
                            "description": "优先级：如果图片中提到\"重要\"\"紧急\"等词汇则为high"
                        },
                        "deadline": {
                            "type": "string",
                            "description": "截止时间 ISO8601 格式，从图片中提取的具体时间"
                        },
                        "tags": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "标签，如：会议、工作等"
                        }
                    },
                    "required": ["title"]
                }
            }
        });

        let current_time = chrono::Utc::now().to_rfc3339();
        let prompt = format!(
            "当前时间: {}。\n\n\
             **重要指示：请仔细识别图片中的所有文字内容。**\n\n\
             1. 首先，使用OCR准确识别图片中的所有文字\n\
             2. 然后，分析这些文字内容，判断是否包含任务信息\n\
             3. 如果包含任务信息，提取以下内容：\n\
                - 任务具体内容（不要用\"xx\"替代，要准确提取原文）\n\
                - 时间信息（如\"明天16号上午10点\"）\n\
                - 地点或对象（如\"给安全科开会\"）\n\
                - 具体要做的事（如\"准备材料\"）\n\
             4. 最后，调用 create_task 工具创建任务\n\n\
             **请确保：**\n\
             - title 字段必须包含图片中识别的完整、准确的文字内容\n\
             - 不要使用占位符如\"xx\"，要准确提取原文",
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

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.moonshot.cn/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "moonshot-v1-8k-vision-preview",
                "messages": messages,
                "tools": [tool],
                "temperature": 0.1  // 降低温度以提高准确性
            }))
            .send()
            .await
            .expect("API call failed");

        let status = response.status();
        let body = response.text().await.expect("Failed to read response");

        println!("\n========================================");
        println!("IMPROVED OCR TEST");
        println!("========================================");
        println!("Response:\n{}", body);
        println!("========================================\n");

        assert!(status.is_success());

        let json: serde_json::Value = serde_json::from_str(&body)
            .expect("Response should be valid JSON");

        if let Some(tool_calls) = json["choices"][0]["message"]["tool_calls"].as_array() {
            for tool_call in tool_calls {
                let args_str = tool_call["function"]["arguments"]
                    .as_str()
                    .expect("Arguments should be string");

                if let Ok(args_json) = serde_json::from_str::<serde_json::Value>(args_str) {
                    println!("\n✅ Extracted Task:");
                    println!("  Title: {}", args_json["title"].as_str().unwrap_or("N/A"));
                    println!("  Description: {}", args_json["description"].as_str().unwrap_or("N/A"));
                    println!("  Priority: {}", args_json["priority"].as_str().unwrap_or("N/A"));
                    println!("  Deadline: {}", args_json["deadline"].as_str().unwrap_or("N/A"));
                    if let Some(tags) = args_json["tags"].as_array() {
                        println!("  Tags: {:?}", tags);
                    }

                    // Check if OCR was accurate
                    let title = args_json["title"].as_str().unwrap_or("");
                    let has_placeholder = title.contains("xx") || title.contains("XX");

                    if has_placeholder {
                        println!("\n⚠️  WARNING: Title contains placeholder 'xx'");
                        println!("  This suggests OCR accuracy issues");
                    } else {
                        println!("\n✅ Title appears to be accurately extracted (no placeholders)");
                    }
                }
            }
        }
    }
}
