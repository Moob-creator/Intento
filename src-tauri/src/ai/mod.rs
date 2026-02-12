//! AI module for natural language processing and task parsing
//!
//! This module provides AI-powered functionality including:
//! - Text input parsing to structured task information
//! - Image input parsing with vision models
//! - Tool-based task operation extraction
//! - Integration with OpenAI and Claude APIs via adk-rust

mod client;
mod models;
mod prompts;
mod task_operations;

pub use client::AiClient;
pub use models::ParsedTask;

// Re-export ModelProvider for external use
pub use client::ModelProvider;

// Re-export task operations types
pub use task_operations::{
    TaskOperation, ImageParseResult, ToolSet,
    TaskToolRegistry, ToolCallParser,
};

#[cfg(test)]
mod tests;
