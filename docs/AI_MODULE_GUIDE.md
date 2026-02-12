# AI Module Configuration Guide

## Overview
The AI module provides natural language processing capabilities for parsing task input. It supports both OpenAI and Anthropic Claude APIs through the ADK-Rust framework.

## Setup

### 1. Install Dependencies
Dependencies are already configured in `Cargo.toml`:
```toml
adk-core = "0.3"
adk-agent = "0.3"
adk-model = { version = "0.3", features = ["openai", "anthropic"] }
chrono = { version = "0.4", features = ["serde"] }
```

### 2. Configure API Keys

#### Option A: Environment Variables (Recommended for Development)
```bash
# For OpenAI (default)
export OPENAI_API_KEY="sk-your-openai-api-key-here"

# OR for Anthropic
export ANTHROPIC_API_KEY="sk-ant-your-anthropic-api-key-here"

# Optional: Specify provider
export AI_PROVIDER="openai"  # or "anthropic"

# Optional: Override model
export AI_MODEL="gpt-4o-mini"  # or "claude-3-5-sonnet-20241022"
```

#### Option B: .env File
```bash
# Copy the example file
cp .env.example .env

# Edit .env and add your API key
# The file is gitignored for security
```

### 3. Test the Setup
```bash
# Run tests (requires API key)
cd src-tauri
cargo test --package intento --lib ai::client::tests::test_parse_simple_task -- --ignored --exact

# Or run all tests including unit tests (no API key needed)
cargo test
```

## Usage

### From Rust Code
```rust
use crate::ai::{AiClient, ParsedTask};
use adk_model::ModelProvider;

// Create client
let client = AiClient::new(ModelProvider::OpenAI)?;
// Or use default from environment
let client = AiClient::new_default()?;

// Parse text input
let parsed = client.parse_text_input("Finish report by tomorrow 5pm, urgent").await?;

println!("Title: {}", parsed.title);
println!("Priority: {:?}", parsed.priority);
println!("Deadline: {:?}", parsed.deadline);
```

### From Frontend (Tauri Command)
```typescript
import { invoke } from '@tauri-apps/api/core';

// Parse text input
const result = await invoke<ParsedTask>('parse_text_input', {
  text: 'Buy groceries tomorrow, low priority'
});

console.log(result.title);        // "Buy groceries"
console.log(result.deadline);     // "2024-12-01T00:00:00Z"
console.log(result.priority);     // "low"

// Check AI service health
const isHealthy = await invoke<boolean>('ai_health_check');

// Get configured provider
const provider = await invoke<string>('get_ai_provider');
console.log(`Using: ${provider}`); // "openai" or "anthropic"
```

## API Reference

### AiClient

#### Methods

- `new(provider: ModelProvider) -> Result<Self>`
  - Creates client with specific provider
  - Requires corresponding API key in environment

- `new_default() -> Result<Self>`
  - Creates client using `AI_PROVIDER` environment variable
  - Falls back to OpenAI if not specified

- `parse_text_input(&self, text: &str) -> Result<ParsedTask>`
  - Parses natural language into structured task
  - Automatically infers deadline, priority, tags

- `health_check(&self) -> bool`
  - Tests if API is accessible
  - Returns true if test call succeeds

### ParsedTask

```rust
pub struct ParsedTask {
    pub title: String,                    // Required
    pub description: Option<String>,      // Optional
    pub deadline: Option<String>,         // ISO8601 format
    pub priority: Option<String>,         // "low", "medium", "high"
    pub tags: Option<Vec<String>>,        // Optional tags
}
```

#### Methods

- `validate_priority(&self) -> Result<(), String>`
  - Validates priority is one of: low, medium, high

- `parse_deadline(&self) -> Result<Option<DateTime<Utc>>, String>`
  - Parses deadline string to DateTime

- `normalize_priority(&mut self)`
  - Converts priority to lowercase

## Tauri Commands

### parse_text_input
Parses natural language text into structured task information.

**Parameters:**
- `text: String` - Natural language task description

**Returns:**
- `Result<ParsedTask, String>` - Parsed task or error message

**Example:**
```typescript
const task = await invoke('parse_text_input', {
  text: 'Meeting with team next Monday at 10am, high priority'
});
```

### ai_health_check
Tests AI client connectivity.

**Returns:**
- `Result<bool, String>` - Health status

**Example:**
```typescript
const healthy = await invoke('ai_health_check');
```

### get_ai_provider
Gets current AI provider configuration.

**Returns:**
- `Result<String, String>` - Provider name ("openai" or "anthropic")

**Example:**
```typescript
const provider = await invoke('get_ai_provider');
```

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `OPENAI_API_KEY` | Conditional* | - | OpenAI API key |
| `ANTHROPIC_API_KEY` | Conditional* | - | Anthropic API key |
| `AI_PROVIDER` | No | `openai` | Provider selection |
| `AI_MODEL` | No | Provider default | Model override |

*At least one API key is required

## Examples

### Example 1: Simple Task
```bash
Input: "Buy milk"
Output: {
  "title": "Buy milk",
  "description": null,
  "deadline": null,
  "priority": null,
  "tags": null
}
```

### Example 2: Task with Deadline
```bash
Input: "Submit report by tomorrow 5pm"
Output: {
  "title": "Submit report",
  "description": null,
  "deadline": "2024-12-01T17:00:00Z",
  "priority": null,
  "tags": ["report"]
}
```

### Example 3: Urgent Task
```bash
Input: "Fix production bug ASAP, critical priority"
Output: {
  "title": "Fix production bug",
  "description": null,
  "deadline": null,
  "priority": "high",
  "tags": ["production", "bug"]
}
```

### Example 4: Complex Task
```bash
Input: "Schedule team meeting next Monday 2pm to discuss Q4 roadmap, include product and engineering"
Output: {
  "title": "Schedule team meeting",
  "description": "Discuss Q4 roadmap with product and engineering teams",
  "deadline": "2024-12-04T14:00:00Z",
  "priority": "medium",
  "tags": ["meeting", "team", "roadmap", "Q4"]
}
```

## Troubleshooting

### Error: "OPENAI_API_KEY environment variable not set"
- Set the API key: `export OPENAI_API_KEY="sk-..."`
- Or create a .env file with the key

### Error: "Failed to parse AI response as JSON"
- Check API key is valid and has credits
- Try running health_check to diagnose
- Increase timeout in ModelConfig if needed

### Tests Failing
- Integration tests require valid API key
- Use `cargo test` to run only unit tests (no API key needed)
- Use `cargo test -- --ignored` to run integration tests

### Client Not Initializing
- Verify API key is in environment before starting app
- Check logs for specific error messages
- Use `get_ai_provider` command to verify configuration

## Security Notes

1. Never commit API keys to version control
2. Use environment variables or secure secret management
3. The .env file is gitignored by default
4. In production, use your platform's secret management (e.g., AWS Secrets Manager)
5. Rotate API keys regularly
6. Monitor API usage and set billing alerts

## Performance Considerations

- AI client is lazy-initialized on first use
- Client instance is cached in Tauri state
- Each API call has ~500ms-2s latency depending on provider
- Consider implementing request caching for repeated inputs
- Low temperature (0.1) ensures consistent parsing results

## Future Enhancements

- [ ] Response caching for repeated queries
- [ ] Batch processing for multiple tasks
- [ ] Custom model fine-tuning
- [ ] Offline mode with local models
- [ ] Multi-language support
- [ ] Conversation context for follow-up refinements
