// Test Kimi API connection
// Run with: cargo run --example test_kimi

mod ai {
    include!("../src/ai/mod.rs");
}

use ai::AiClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv::dotenv().ok();

    println!("🔍 Testing Kimi API connection...\n");

    // Check environment variables
    let provider = env::var("AI_PROVIDER").unwrap_or_else(|_| "not set".to_string());
    let api_key = env::var("MOONSHOT_API_KEY").unwrap_or_else(|_| "not set".to_string());
    let model = env::var("AI_MODEL").unwrap_or_else(|_| "default".to_string());

    println!("📋 Configuration:");
    println!("  Provider: {}", provider);
    println!("  API Key: {}...{}",
        &api_key.chars().take(15).collect::<String>(),
        &api_key.chars().rev().take(8).collect::<String>().chars().rev().collect::<String>()
    );
    println!("  Model: {}\n", model);

    // Create AI client
    println!("🚀 Creating AI client...");
    let client = AiClient::new_default()?;
    println!("✅ Client created successfully!\n");

    // Test parse_text_input
    println!("🧪 Testing parse_text_input...");
    let test_input = "明天下午3点开会讨论项目进度，高优先级";
    println!("  Input: {}\n", test_input);

    match client.parse_text_input(test_input).await {
        Ok(parsed) => {
            println!("✅ Parse successful!");
            println!("\n📝 Parsed Task:");
            println!("  Title: {}", parsed.title);
            if let Some(desc) = &parsed.description {
                println!("  Description: {}", desc);
            }
            if let Some(deadline) = &parsed.deadline {
                println!("  Deadline: {}", deadline);
            }
            if let Some(priority) = &parsed.priority {
                println!("  Priority: {}", priority);
            }
            if let Some(tags) = &parsed.tags {
                println!("  Tags: {:?}", tags);
            }
            println!("\n🎉 Kimi API connection test passed!");
        }
        Err(e) => {
            println!("❌ Parse failed!");
            println!("  Error: {}", e);
            println!("\n💡 Possible reasons:");
            println!("  1. API key is invalid or expired");
            println!("  2. Network connection issue");
            println!("  3. API rate limit exceeded");
            println!("  4. Kimi service is unavailable");
            return Err(e);
        }
    }

    Ok(())
}
