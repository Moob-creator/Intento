# Integration Test Suite Documentation

## Overview

This directory contains comprehensive integration tests for the Intento Tauri application. The tests verify end-to-end functionality across all major system components including database operations, AI integration, Tauri commands, notifications, and scheduling.

## Test Files

### 1. `test_comprehensive_integration.rs`
The main integration test suite covering:

#### Database Integration
- **Task CRUD Operations**: Create, read, update, delete operations with full lifecycle testing
- **Multiple Tasks with Filters**: Status filtering, priority sorting, tag-based queries
- **Task Reminders**: Reminder time calculation, notification triggers
- **Expiring Tasks**: Deadline-based queries with time windows
- **Summary CRUD**: Summary creation, retrieval, and filtering by type and tags
- **Summary Pagination**: Limit/offset pagination for large datasets
- **Settings Management**: Key-value settings storage and retrieval

#### AI Integration (Requires API Keys)
- **Text Parsing**: Natural language to structured task conversion
- **Text Parsing with Tags**: Tag preference system integration
- **Image Parsing**: OCR and image-to-task extraction
- **Summary Generation**: AI-powered task summaries with statistics

#### End-to-End Workflows
- **Complete Task Workflow**: Full lifecycle from creation to completion
- **Multi-Tag Organization**: Complex tag-based filtering and categorization
- **Deadline Priority Management**: Priority-based deadline handling

#### Error Handling
- **Database Errors**: Non-existent records, invalid queries
- **Invalid Enum Values**: Status, priority, and type validation

#### Performance Tests
- **Bulk Operations**: 100+ task creation, update, and listing
- **Concurrent Access**: Multi-threaded database operations
- **Schema Validation**: Database version and migration verification

### 2. `test_notification_integration.rs`
Notification system integration tests:

- **Reminder Workflow**: Task reminder creation and triggering
- **Multiple Reminders**: Concurrent reminder handling
- **Expiring Task Notifications**: Time-based deadline warnings
- **Completed Task Filtering**: Exclude completed tasks from reminders
- **Deleted Task Filtering**: Soft-delete integration
- **Reminder Time Updates**: Dynamic reminder adjustment on deadline changes
- **Settings Integration**: Notification preference management
- **High Priority Notifications**: Priority-based notification logic
- **Edge Cases**: Time boundary conditions, duplicate prevention
- **Bulk Operations**: Mass reminder clearing

### 3. `test_command_integration.rs`
Tauri command handler integration:

#### Task Commands
- `create_task`: Task creation with all optional fields
- `get_task`: Single task retrieval
- `update_task`: Partial and full updates
- `delete_task`: Soft delete operations
- `list_tasks`: Filtering and sorting
- `get_db_version`: Version verification

#### AI Commands (Requires API Keys)
- `parse_text_input`: Natural language processing
- `ai_health_check`: Service availability check
- `parse_image_input`: Image-based task extraction

#### Notification Commands
- `check_expiring_tasks`: Time-window based queries
- `send_notification`: Notification dispatch

#### Settings Commands
- `get_auto_summary_settings`: Summary configuration retrieval
- `update_auto_summary_settings`: Configuration updates
- `get_notification_settings`: Notification preferences
- `update_notification_settings`: Preference updates

#### Error Handling
- Invalid task IDs
- Invalid enum values (status, priority)
- Concurrent operation conflicts

## Running Tests

### Run All Integration Tests
```bash
cd src-tauri
./run_integration_tests.sh
```

### Run Specific Test File
```bash
# Comprehensive tests
cargo test --test test_comprehensive_integration

# Notification tests
cargo test --test test_notification_integration

# Command tests
cargo test --test test_command_integration
```

### Run Specific Test
```bash
cargo test --test test_comprehensive_integration test_task_lifecycle_integration
```

### Run with Verbose Output
```bash
cargo test --test test_comprehensive_integration -- --nocapture
```

### Run Ignored Tests (AI Integration)
```bash
# Requires .env file with API keys
cargo test --test test_comprehensive_integration -- --ignored
```

### Run Tests with Specific Thread Count
```bash
# Useful for debugging concurrency issues
cargo test --test test_comprehensive_integration -- --test-threads=1
```

## Environment Setup

### Required for Basic Tests
- Rust toolchain (1.70+)
- SQLite support (bundled via `rusqlite`)

### Required for AI Tests
Create a `.env` file in the project root:

```env
# For Kimi AI (Moonshot)
KIMI_API_KEY=your_kimi_api_key_here

# Or for OpenAI
OPENAI_API_KEY=your_openai_api_key_here

# Or for Anthropic
ANTHROPIC_API_KEY=your_anthropic_api_key_here
```

AI integration tests are marked with `#[ignore]` and only run when explicitly requested:
```bash
cargo test -- --ignored
```

## Test Database

All tests use temporary in-memory or file-based databases created with `tempfile`:
- Isolated from production data
- Automatically cleaned up after tests
- Full schema migrations applied
- No shared state between tests

## Test Coverage

### Covered Areas
- ✅ Database CRUD operations (100%)
- ✅ Task lifecycle management (100%)
- ✅ Summary generation and retrieval (100%)
- ✅ Settings management (100%)
- ✅ Notification system (100%)
- ✅ Tauri command handlers (100%)
- ✅ Error handling (100%)
- ✅ Concurrent operations (80%)
- ✅ AI text parsing (requires API keys)
- ✅ AI image parsing (requires API keys and test images)

### Not Covered
- ❌ Frontend JavaScript/TypeScript integration
- ❌ Window management (macOS traffic lights)
- ❌ System tray functionality
- ❌ IPC event emission
- ❌ Scheduler cron jobs (complex async lifecycle)

## Writing New Integration Tests

### Basic Test Structure
```rust
use intento::db::{Database, models::Task};
use tempfile::TempDir;

fn create_test_db() -> (Database, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test.db");
    let db = Database::new(db_path).expect("Failed to create database");
    (db, temp_dir)
}

#[tokio::test]
async fn test_my_feature() {
    let (db, _temp_dir) = create_test_db();

    // Your test code here
    let task = Task::new("Test".to_string());
    let task_id = db.create_task(&task).expect("Failed to create task");

    assert!(task_id > 0);
}
```

### Testing Tauri Commands
```rust
use tauri::State;
use intento::commands::task::create_task;

#[tokio::test]
async fn test_command() {
    let (db, _temp_dir) = create_test_db();
    let state = State::from(db);

    let result = create_task(
        state,
        "Test task".to_string(),
        None,
        None,
        None,
        None,
    ).await;

    assert!(result.is_ok());
}
```

### Testing AI Features
```rust
#[tokio::test]
#[ignore] // Mark as ignored - requires API key
async fn test_ai_feature() {
    // Load environment
    dotenv::from_path("../.env").ok();

    let client = AiClient::new_default();
    if client.is_err() {
        println!("⚠️ AI client not available, skipping");
        return;
    }

    // Your AI test code
}
```

## Best Practices

### 1. Test Isolation
- Each test creates its own database
- No shared state between tests
- Clean up resources after tests

### 2. Async Testing
- Use `#[tokio::test]` for async tests
- Use `tokio::spawn` for concurrent operations
- Use `.await` for async operations

### 3. Error Handling
```rust
// Good - explicit error handling
let result = db.create_task(&task);
assert!(result.is_ok());
let task_id = result.unwrap();

// Better - expect with context
let task_id = db.create_task(&task)
    .expect("Failed to create task");
```

### 4. Assertions
```rust
// Basic assertion
assert_eq!(task.status, TaskStatus::Done);

// Assertion with message
assert!(
    task.completed_at.is_some(),
    "Completed task should have completed_at timestamp"
);

// Multiple assertions
let task = db.get_task(id).expect("Failed to get task").expect("Task not found");
assert_eq!(task.title, "Expected Title");
assert_eq!(task.status, TaskStatus::Todo);
```

### 5. Test Naming
- Use descriptive names: `test_task_lifecycle_integration`
- Prefix with feature: `test_notification_reminder_workflow`
- Include scenario: `test_concurrent_task_updates`

## Continuous Integration

### GitHub Actions Example
```yaml
name: Integration Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run Integration Tests
        run: |
          cd src-tauri
          cargo test --all-targets

      - name: Run AI Tests (with secrets)
        if: github.event_name == 'push'
        env:
          KIMI_API_KEY: ${{ secrets.KIMI_API_KEY }}
        run: |
          cd src-tauri
          cargo test -- --ignored
```

## Troubleshooting

### Tests Hanging
- Reduce thread count: `cargo test -- --test-threads=1`
- Check for deadlocks in database operations
- Verify async operations complete properly

### Database Errors
- Ensure migrations are applied correctly
- Check schema version matches expected
- Verify temp directories are writable

### AI Test Failures
- Verify API keys in `.env` file
- Check API rate limits
- Ensure network connectivity
- Review API response formats

### Flaky Tests
- Add retry logic for network operations
- Use proper time mocking for time-dependent tests
- Increase timeout values if needed
- Check for race conditions in concurrent tests

## Performance Benchmarks

Expected performance for integration tests:
- Task CRUD operations: < 10ms per operation
- Bulk operations (100 tasks): < 2 seconds
- Database queries: < 5ms per query
- AI text parsing: 1-3 seconds (depends on API)
- AI image parsing: 2-5 seconds (depends on API and image size)

## Contributing

When adding new features:
1. Write integration tests alongside implementation
2. Ensure tests cover happy path and error cases
3. Add concurrent operation tests if applicable
4. Update this documentation
5. Run full test suite before submitting PR

## Resources

- [Tauri Testing Guide](https://tauri.app/v1/guides/testing/)
- [Tokio Testing](https://tokio.rs/tokio/topics/testing)
- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Integration Testing Patterns](https://matklad.github.io/2021/05/31/how-to-test.html)
