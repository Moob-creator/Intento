/// Integration test for the new task operations architecture

#[cfg(test)]
mod integration_tests {
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
    async fn test_full_workflow_with_real_image() {
        load_env();

        // Load test image
        let image_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples/test.jpg");

        let image_data = fs::read(&image_path)
            .expect("Failed to read test image");

        use base64::{Engine as _, engine::general_purpose};
        let image_base64 = general_purpose::STANDARD.encode(&image_data);

        println!("\n=== Full Workflow Test ===");
        println!("Image size: {} bytes", image_data.len());

        // Import necessary types
        use intento::ai::{AiClient, ToolSet};

        // Create AI client
        let client = AiClient::new_default()
            .expect("Failed to create AI client");

        // Parse image with basic tools (create only)
        println!("\n--- Testing with Basic Tools (create only) ---");
        let result = client
            .parse_image_for_operations(&image_base64, "image/jpeg", ToolSet::Basic)
            .await
            .expect("Failed to parse image");

        println!("Operations found: {}", result.operation_count());
        println!("Confidence: {}", result.confidence);
        println!("Warnings: {:?}", result.warnings);

        if let Some(desc) = &result.image_description {
            println!("Image description: {}", desc);
        }

        for (i, op) in result.operations.iter().enumerate() {
            println!("\nOperation {}: {}", i + 1, op.description());
            println!("Type: {}", op.operation_name());
        }

        // Verify we got at least one operation
        assert!(result.has_operations(), "Should have extracted at least one operation");

        println!("\n✅ Full workflow test completed successfully!");
    }
}
