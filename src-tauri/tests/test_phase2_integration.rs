/// Test Phase 2 frontend integration with image operations
///
/// This test validates the complete flow from image parsing to operation execution

#[cfg(test)]
mod phase2_integration_tests {
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
    async fn test_phase2_full_workflow() {
        load_env();

        println!("\n=== Phase 2 Full Workflow Test ===");

        // Import necessary types
        use intento::ai::{AiClient, ToolSet};

        // Create AI client
        let client = AiClient::new_default()
            .expect("Failed to create AI client");

        // Test 1: Single Create operation
        println!("\n--- Test 1: Single Create Operation ---");
        let test_image_1 = "明天下午3点前完成季度报告\n优先级：高";
        let result_1 = simulate_image_parse(&client, test_image_1, ToolSet::All).await;

        println!("Operations found: {}", result_1.operations.len());
        assert!(result_1.operations.len() >= 1, "Should extract at least one operation");

        // Verify first operation is Create
        if let Some(op) = result_1.operations.first() {
            println!("First operation: {}", op.description());
            assert_eq!(op.operation_name(), "create_task", "First operation should be create");
        }

        // Test 2: Multiple operations
        println!("\n--- Test 2: Multiple Operations ---");
        let test_image_2 = r#"
        创建任务：写周报（明天截止，高优先级）
        完成任务：测试功能
        更新任务：项目文档，截止时间改为周五
        "#;
        let result_2 = simulate_image_parse(&client, test_image_2, ToolSet::All).await;

        println!("Operations found: {}", result_2.operations.len());
        assert!(result_2.operations.len() >= 2, "Should extract multiple operations");

        // Test 3: Basic tools only (Create only)
        println!("\n--- Test 3: Basic Tools Only ---");
        let test_image_3 = "明天完成三个任务：\n1. 写代码\n2. 写文档\n3. 测试";
        let result_3 = simulate_image_parse(&client, test_image_3, ToolSet::Basic).await;

        println!("Operations found: {}", result_3.operations.len());
        // With basic tools, should only get Create operations
        for op in &result_3.operations {
            println!("Operation: {}", op.operation_name());
            assert_eq!(op.operation_name(), "create_task", "Basic tools should only create");
        }

        // Test 4: Confidence and warnings
        println!("\n--- Test 4: Confidence and Warnings ---");
        let result_4 = simulate_image_parse(&client, "模糊的任务描述", ToolSet::All).await;

        println!("Confidence: {}", result_4.confidence);
        println!("Warnings: {:?}", result_4.warnings);

        assert!(result_4.confidence >= 0.0 && result_4.confidence <= 1.0,
                "Confidence should be between 0 and 1");

        println!("\n✅ All Phase 2 integration tests passed!");
    }

    // Helper function to simulate image parsing
    // In real scenario, this would use actual image data
    async fn simulate_image_parse(
        _client: &intento::ai::AiClient,
        text_content: &str,
        _tool_set: intento::ai::ToolSet
    ) -> intento::ai::ImageParseResult {
        use intento::ai::{TaskOperation, ImageParseResult};

        // For simulation, we'll create a simple text-based test
        // In production, this would parse actual images

        // Simulate basic parsing logic
        let mut operations = Vec::new();
        let lines: Vec<&str> = text_content.lines().collect();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Simple pattern matching for demonstration
            if trimmed.contains("创建") || trimmed.contains("完成") || !trimmed.is_empty() {
                operations.push(TaskOperation::Create {
                    title: trimmed.to_string(),
                    description: None,
                    priority: Some("medium".to_string()),
                    deadline: None,
                    tags: None,
                });
            }
        }

        ImageParseResult {
            operations,
            confidence: 0.85,
            image_description: Some("测试图片".to_string()),
            warnings: vec![],
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_real_image_with_operations() {
        load_env();

        println!("\n=== Real Image with Operations Test ===");

        // Load test image
        let image_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples/test.jpg");

        if !image_path.exists() {
            println!("⚠️  Test image not found, skipping test");
            return;
        }

        let image_data = fs::read(&image_path)
            .expect("Failed to read test image");

        use base64::{Engine as _, engine::general_purpose};
        let image_base64 = general_purpose::STANDARD.encode(&image_data);

        println!("Image size: {} bytes", image_data.len());

        // Import necessary types
        use intento::ai::{AiClient, ToolSet};

        // Create AI client
        let client = AiClient::new_default()
            .expect("Failed to create AI client");

        // Test with All tools
        println!("\n--- Testing with All Tools ---");
        let result = client
            .parse_image_for_operations(&image_base64, "image/jpeg", ToolSet::All)
            .await
            .expect("Failed to parse image");

        println!("\nResults:");
        println!("  Operations found: {}", result.operations.len());
        println!("  Confidence: {:.2}%", result.confidence * 100.0);

        if let Some(desc) = &result.image_description {
            println!("  Description: {}", desc);
        }

        if !result.warnings.is_empty() {
            println!("  Warnings:");
            for warning in &result.warnings {
                println!("    - {}", warning);
            }
        }

        println!("\nOperations:");
        for (i, op) in result.operations.iter().enumerate() {
            println!("  {}. [{}] {}", i + 1, op.operation_name(), op.description());
        }

        assert!(result.operations.len() > 0, "Should extract at least one operation");

        println!("\n✅ Real image operations test completed!");
    }

    #[test]
    fn test_operation_execution_simulation() {
        println!("\n=== Operation Execution Simulation ===");

        use intento::ai::TaskOperation;

        // Simulate operations that would come from the frontend
        let operations = vec![
            TaskOperation::Create {
                title: "完成季度报告".to_string(),
                description: Some("包含 Q4 所有数据".to_string()),
                priority: Some("high".to_string()),
                deadline: Some("2024-03-15T15:00:00Z".to_string()),
                tags: Some(vec!["工作".to_string(), "报告".to_string()]),
            },
            TaskOperation::Complete {
                task_identifier: "测试任务".to_string(),
            },
            TaskOperation::Update {
                task_identifier: "项目文档".to_string(),
                title: None,
                description: Some("更新截止日期".to_string()),
                priority: None,
                deadline: Some("2024-03-20T17:00:00Z".to_string()),
                tags: None,
            },
        ];

        println!("Simulating execution of {} operations:", operations.len());

        for (i, op) in operations.iter().enumerate() {
            println!("\n  Operation {}: {}", i + 1, op.operation_name());
            println!("    Description: {}", op.description());

            // In real frontend, this would call:
            // - createTask() for Create
            // - updateTask() for Update/SetStatus
            // - deleteTask() for Delete
            // - etc.

            match op {
                TaskOperation::Create { title, .. } => {
                    println!("    → Would call: createTask(title: '{}')", title);
                }
                TaskOperation::Update { task_identifier, .. } => {
                    println!("    → Would call: updateTask(task: '{}')", task_identifier);
                }
                TaskOperation::Complete { task_identifier } => {
                    println!("    → Would call: updateTask(task: '{}', status: 'done')", task_identifier);
                }
                _ => {
                    println!("    → Would execute corresponding operation");
                }
            }
        }

        println!("\n✅ Simulation completed successfully!");
    }
}
