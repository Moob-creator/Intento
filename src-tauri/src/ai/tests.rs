use super::models::ParsedTask;
use super::client::AiClient;

// ========== Integration Tests (requires API key) ==========

#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored --nocapture
async fn test_kimi_api_hello_world() {
    // Load .env file
    dotenv::dotenv().ok();

    println!("\n🔍 Testing Kimi API with 'Hello World'...\n");

    // Debug: Print environment variables (partially)
    if let Ok(provider) = std::env::var("AI_PROVIDER") {
        println!("📋 AI_PROVIDER: {}", provider);
    }
    if let Ok(key) = std::env::var("MOONSHOT_API_KEY") {
        println!("📋 API_KEY: {}...{}", &key[..15], &key[key.len()-8..]);
    }
    if let Ok(model) = std::env::var("AI_MODEL") {
        println!("📋 AI_MODEL: {}", model);
    }
    println!();

    // Create AI client
    let client = match AiClient::new_default() {
        Ok(c) => {
            println!("✅ AI Client created successfully");
            c
        }
        Err(e) => {
            panic!("❌ Failed to create AI client: {}\n\nMake sure:\n  1. .env file exists with MOONSHOT_API_KEY\n  2. AI_PROVIDER=kimi", e);
        }
    };

    // Test with simple input
    println!("📤 Sending: 'Hello World'");

    match client.parse_text_input("Hello World").await {
        Ok(parsed) => {
            println!("\n✅ Kimi API Response Successful!");
            println!("📝 Parsed Result:");
            println!("   Title: {}", parsed.title);
            if let Some(desc) = &parsed.description {
                println!("   Description: {}", desc);
            }
            println!("\n🎉 Test PASSED! Kimi API is working correctly.");

            // Verify we got a valid response
            assert!(!parsed.title.is_empty(), "Title should not be empty");
        }
        Err(e) => {
            panic!("\n❌ Kimi API call failed: {}\n\nPossible causes:\n  1. Invalid API key\n  2. Network issue\n  3. API endpoint down", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_kimi_api_chinese_task() {
    dotenv::dotenv().ok();

    println!("\n🔍 Testing Kimi API with Chinese task input...\n");

    let client = AiClient::new_default()
        .expect("Failed to create AI client");

    let test_input = "明天下午3点开会讨论项目进度，高优先级";
    println!("📤 Sending: '{}'", test_input);

    match client.parse_text_input(test_input).await {
        Ok(parsed) => {
            println!("\n✅ Parse Successful!");
            println!("📝 Parsed Task:");
            println!("   Title: {}", parsed.title);

            if let Some(desc) = &parsed.description {
                println!("   Description: {}", desc);
            }

            if let Some(deadline) = &parsed.deadline {
                println!("   Deadline: {}", deadline);
            }

            if let Some(priority) = &parsed.priority {
                println!("   Priority: {}", priority);
            }

            if let Some(tags) = &parsed.tags {
                println!("   Tags: {:?}", tags);
            }

            println!("\n🎉 Chinese task parsing test PASSED!");

            // Basic assertions
            assert!(!parsed.title.is_empty());
            assert!(parsed.title.contains("开会") || parsed.title.contains("会议"));
        }
        Err(e) => {
            panic!("❌ Failed: {}", e);
        }
    }
}

// ========== Unit Tests (no API required) ==========

#[test]
fn test_parsed_task_serialization() {
    let task = ParsedTask {
        title: "Test Task".to_string(),
        description: Some("This is a test".to_string()),
        deadline: Some("2024-12-31T23:59:59Z".to_string()),
        priority: Some("high".to_string()),
        tags: Some(vec!["test".to_string(), "work".to_string()]),
    };

    let json = serde_json::to_string(&task).expect("Failed to serialize");
    let deserialized: ParsedTask =
        serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(task, deserialized);
}

#[test]
fn test_parsed_task_minimal() {
    let task = ParsedTask {
        title: "Minimal Task".to_string(),
        description: None,
        deadline: None,
        priority: None,
        tags: None,
    };

    assert!(task.validate_priority().is_ok());
    assert!(task.parse_deadline().unwrap().is_none());
}

#[test]
fn test_parsed_task_with_tags() {
    let task = ParsedTask {
        title: "Tagged Task".to_string(),
        description: None,
        deadline: None,
        priority: None,
        tags: Some(vec![
            "work".to_string(),
            "urgent".to_string(),
            "meeting".to_string(),
        ]),
    };

    assert_eq!(task.tags.as_ref().unwrap().len(), 3);
    assert!(task.tags.as_ref().unwrap().contains(&"work".to_string()));
}
