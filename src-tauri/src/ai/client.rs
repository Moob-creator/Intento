use adk_model::anthropic::{AnthropicClient, AnthropicConfig};
use adk_model::openai::{OpenAIClient, OpenAIConfig};
use adk_core::{Llm, LlmRequest, Content, Part};
use anyhow::{Context, Result};
use futures::StreamExt;
use serde_json::{self, json};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use super::models::ParsedTask;
use super::prompts;

/// Supported AI model providers
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModelProvider {
    OpenAI,
    Anthropic,
    Kimi,
}

/// AI client for natural language processing tasks
pub struct AiClient {
    model: Arc<dyn Llm + Send + Sync>,
    provider: ModelProvider,
}

impl AiClient {
    /// Creates a new AI client with the specified model provider
    ///
    /// # Arguments
    /// * `provider` - The model provider to use (OpenAI or Anthropic)
    ///
    /// # Environment Variables
    /// * `OPENAI_API_KEY` - Required if using OpenAI provider
    /// * `ANTHROPIC_API_KEY` - Required if using Anthropic provider
    /// * `MOONSHOT_API_KEY` - Required if using Kimi provider
    /// * `AI_MODEL` - Optional model name override (defaults: gpt-4o-mini, claude-3-5-sonnet-20241022, moonshot-v1-8k)
    ///
    /// # Errors
    /// Returns error if API key environment variable is not set
    pub fn new(provider: ModelProvider) -> Result<Self> {
        let (api_key, default_model, base_url) = match provider {
            ModelProvider::OpenAI => {
                let key = env::var("OPENAI_API_KEY")
                    .context("OPENAI_API_KEY environment variable not set")?;
                (key, "gpt-4o-mini", None)
            }
            ModelProvider::Anthropic => {
                let key = env::var("ANTHROPIC_API_KEY")
                    .context("ANTHROPIC_API_KEY environment variable not set")?;
                (key, "claude-3-5-sonnet-20241022", None)
            }
            ModelProvider::Kimi => {
                let key = env::var("MOONSHOT_API_KEY")
                    .context("MOONSHOT_API_KEY environment variable not set")?;
                (key, "kimi-k2-turbo-preview", Some("https://api.moonshot.cn/v1".to_string()))
            }
        };

        // Allow model override via environment variable
        let model_name = env::var("AI_MODEL").unwrap_or_else(|_| default_model.to_string());

        // Create model client based on provider
        let model: Arc<dyn Llm + Send + Sync> = match provider {
            ModelProvider::OpenAI => {
                let config = OpenAIConfig::new(api_key, &model_name);
                Arc::new(OpenAIClient::new(config)?)
            }
            ModelProvider::Anthropic => {
                let config = AnthropicConfig::new(api_key, &model_name);
                Arc::new(AnthropicClient::new(config)?)
            }
            ModelProvider::Kimi => {
                // Kimi uses OpenAI-compatible API
                let config = if let Some(url) = base_url {
                    OpenAIConfig::compatible(api_key, url, &model_name)
                } else {
                    OpenAIConfig::compatible(api_key, "https://api.moonshot.cn/v1", &model_name)
                };
                Arc::new(OpenAIClient::new(config)?)
            }
        };

        Ok(Self { model, provider })
    }

    /// Creates a new AI client using the default provider from environment
    ///
    /// # Environment Variables
    /// * `AI_PROVIDER` - Provider to use ("openai", "anthropic", or "kimi", defaults to "openai")
    /// * Plus provider-specific API key (see `new()`)
    pub fn new_default() -> Result<Self> {
        let provider_str = env::var("AI_PROVIDER").unwrap_or_else(|_| "openai".to_string());
        let provider = match provider_str.to_lowercase().as_str() {
            "anthropic" | "claude" => ModelProvider::Anthropic,
            "kimi" | "moonshot" => ModelProvider::Kimi,
            _ => ModelProvider::OpenAI,
        };

        Self::new(provider)
    }

    /// Parses natural language text input into structured task information
    ///
    /// # Arguments
    /// * `text` - The natural language input describing a task
    ///
    /// # Returns
    /// A `ParsedTask` struct containing extracted task information
    ///
    /// # Errors
    /// Returns error if:
    /// - API call fails
    /// - Response cannot be parsed as valid JSON
    /// - Response doesn't match expected structure
    ///
    /// # Example
    /// ```rust,no_run
    /// # use intento::ai::{AiClient, ModelProvider};
    /// # fn main() -> anyhow::Result<()> {
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let client = AiClient::new(ModelProvider::OpenAI)?;
    /// let parsed = client.parse_text_input("Finish report by tomorrow 5pm, urgent").await?;
    /// assert_eq!(parsed.priority, Some("high".to_string()));
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn parse_text_input(&self, text: &str) -> Result<ParsedTask> {
        // Get current time in ISO8601 format
        let current_time = chrono::Utc::now().to_rfc3339();

        // Build the prompt
        let prompt = prompts::build_parse_task_prompt(text, &current_time);

        // Create LLM request
        let content = Content::new("user").with_text(prompt);
        let request = LlmRequest {
            model: String::new(), // Model is already configured in the client
            contents: vec![content],
            config: None,
            tools: HashMap::new(),
        };

        // Call AI model and collect streamed response
        let mut stream = self
            .model
            .generate_content(request, false)
            .await
            .context("Failed to call AI model")?;

        let mut response_text = String::new();
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    if let Some(content) = &response.content {
                        for part in &content.parts {
                            if let Part::Text { text } = part {
                                response_text.push_str(text);
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Error in response stream: {}. Response so far: '{}'",
                        e,
                        response_text
                    ));
                }
            }
        }

        // Parse JSON response
        let mut parsed: ParsedTask = serde_json::from_str(response_text.trim())
            .context("Failed to parse AI response as JSON")?;

        // Normalize and validate
        parsed.normalize_priority();
        if let Err(e) = parsed.validate_priority() {
            return Err(anyhow::anyhow!("AI returned invalid priority: {}", e));
        }

        if let Some(ref deadline) = parsed.deadline {
            parsed.parse_deadline().map_err(|e| {
                anyhow::anyhow!("AI returned invalid deadline format '{}': {}", deadline, e)
            })?;
        }

        Ok(parsed)
    }

    /// Parses image input (screenshot, photo) into structured task information
    ///
    /// # Arguments
    /// * `image_base64` - The base64-encoded image data (without data URI prefix)
    /// * `image_type` - The image MIME type (e.g., "image/png", "image/jpeg")
    ///
    /// # Returns
    /// A `ParsedTask` struct containing extracted task information from the image
    ///
    /// # Errors
    /// Returns error if:
    /// - API call fails
    /// - Response cannot be parsed as valid JSON
    /// - Image format is not supported
    /// - Provider doesn't support vision capabilities
    ///
    /// # Note
    /// Supports vision via OpenAI's gpt-4o or Anthropic's Claude 3.5 Sonnet
    pub async fn parse_image_input(
        &self,
        image_base64: &str,
        image_type: &str,
    ) -> Result<ParsedTask> {
        // Check which vision-capable provider is available
        let (vision_api_key, vision_provider) = if let Ok(key) = env::var("OPENAI_API_KEY") {
            (key, "openai")
        } else if let Ok(key) = env::var("ANTHROPIC_API_KEY") {
            (key, "anthropic")
        } else {
            return Err(anyhow::anyhow!(
                "图片识别需要 OpenAI 或 Anthropic API 密钥。\n\
                 当前配置的 AI 提供商 (Kimi) 不支持图片识别功能。\n\
                 请设置 OPENAI_API_KEY 或 ANTHROPIC_API_KEY 环境变量以启用图片识别。"
            ));
        };

        // Get current time
        let current_time = chrono::Utc::now().to_rfc3339();

        // Build the vision prompt
        let text_prompt = prompts::build_parse_image_prompt(&current_time);

        println!("Using vision provider: {}", vision_provider);

        // Call the appropriate vision API
        let response = if vision_provider == "openai" {
            // Construct image data URL for OpenAI
            let image_data_url = format!("data:{};base64,{}", image_type, image_base64);

            let messages = vec![json!({
                "role": "user",
                "content": [
                    {"type": "text", "text": text_prompt},
                    {"type": "image_url", "image_url": {"url": image_data_url}}
                ]
            })];

            self.call_openai_vision_api(&vision_api_key, "gpt-4o", &messages).await?
        } else {
            // Anthropic Claude vision
            self.call_anthropic_vision_api(&vision_api_key, "claude-3-5-sonnet-20241022", &text_prompt, image_base64, image_type).await?
        };

        // Parse JSON response
        let mut parsed: ParsedTask = serde_json::from_str(response.trim())
            .context("Failed to parse vision AI response as JSON")?;

        // Normalize and validate
        parsed.normalize_priority();
        if let Err(e) = parsed.validate_priority() {
            return Err(anyhow::anyhow!("AI returned invalid priority: {}", e));
        }

        if let Some(ref deadline) = parsed.deadline {
            parsed.parse_deadline().map_err(|e| {
                anyhow::anyhow!("AI returned invalid deadline format '{}': {}", deadline, e)
            })?;
        }

        Ok(parsed)
    }

    /// Direct OpenAI vision API call helper
    async fn call_openai_vision_api(
        &self,
        api_key: &str,
        model: &str,
        messages: &[serde_json::Value],
    ) -> Result<String> {
        use serde_json::json;

        println!("Calling OpenAI Vision API with model: {}", model);

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": model,
                "messages": messages,
                "max_tokens": 1000
            }))
            .send()
            .await
            .context("Failed to call OpenAI vision API")?;

        let status = response.status();
        let response_text = response.text().await.context("Failed to read response body")?;

        println!("OpenAI API response status: {}", status);
        println!("OpenAI API response body: {}", response_text);

        if !status.is_success() {
            return Err(anyhow::anyhow!(
                "OpenAI API returned error {}: {}",
                status,
                response_text
            ));
        }

        // Parse the response JSON
        let response_json: serde_json::Value = serde_json::from_str(&response_text)
            .context("Failed to parse OpenAI response as JSON")?;

        // Extract the content from response
        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .context("Invalid response format from OpenAI")?;

        Ok(content.to_string())
    }

    /// Direct Anthropic vision API call helper
    async fn call_anthropic_vision_api(
        &self,
        api_key: &str,
        model: &str,
        text_prompt: &str,
        image_base64: &str,
        image_type: &str,
    ) -> Result<String> {
        use serde_json::json;

        println!("Calling Anthropic Vision API with model: {}", model);

        // Anthropic expects media_type without "image/" prefix for some formats
        let media_type = if image_type == "image/jpg" {
            "image/jpeg"
        } else {
            image_type
        };

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": model,
                "max_tokens": 1000,
                "messages": [{
                    "role": "user",
                    "content": [
                        {
                            "type": "image",
                            "source": {
                                "type": "base64",
                                "media_type": media_type,
                                "data": image_base64
                            }
                        },
                        {
                            "type": "text",
                            "text": text_prompt
                        }
                    ]
                }]
            }))
            .send()
            .await
            .context("Failed to call Anthropic vision API")?;

        let status = response.status();
        let response_text = response.text().await.context("Failed to read response body")?;

        println!("Anthropic API response status: {}", status);
        println!("Anthropic API response body: {}", response_text);

        if !status.is_success() {
            return Err(anyhow::anyhow!(
                "Anthropic API returned error {}: {}",
                status,
                response_text
            ));
        }

        // Parse the response JSON
        let response_json: serde_json::Value = serde_json::from_str(&response_text)
            .context("Failed to parse Anthropic response as JSON")?;

        // Extract the content from response
        let content = response_json["content"][0]["text"]
            .as_str()
            .context("Invalid response format from Anthropic")?;

        Ok(content.to_string())
    }

    /// Checks if the client is properly configured and can make API calls
    ///
    /// # Returns
    /// `true` if a test call succeeds, `false` otherwise
    pub async fn health_check(&self) -> bool {
        let content = Content::new("user").with_text("Say 'OK' if you can hear me.");
        let request = LlmRequest {
            model: String::new(),
            contents: vec![content],
            config: None,
            tools: HashMap::new(),
        };

        match self.model.generate_content(request, false).await {
            Ok(mut stream) => {
                // Just check if we can get at least one response
                stream.next().await.is_some()
            }
            Err(_) => false,
        }
    }

    /// Gets the provider being used by this client
    pub fn provider(&self) -> ModelProvider {
        self.provider
    }

    /// Parses image input using vision + tool use to extract task operations
    ///
    /// # Arguments
    /// * `image_base64` - The base64-encoded image data (without data URI prefix)
    /// * `image_type` - The image MIME type (e.g., "image/png", "image/jpeg")
    /// * `tool_set` - Which set of tools to provide to the model
    ///
    /// # Returns
    /// An `ImageParseResult` containing extracted task operations
    ///
    /// # Errors
    /// Returns error if API call fails or response cannot be parsed
    pub async fn parse_image_for_operations(
        &self,
        image_base64: &str,
        image_type: &str,
        tool_set: super::task_operations::ToolSet,
    ) -> Result<super::task_operations::ImageParseResult> {
        use super::task_operations::{TaskToolRegistry, ToolCallParser, ImageParseResult};

        // Get current time for deadline inference
        let current_time = chrono::Utc::now().to_rfc3339();

        // Get tools based on tool set
        let tools = TaskToolRegistry::get_tools(tool_set);

        let prompt = format!(
            "当前时间: {}。请仔细分析这张图片，识别其中包含的任务、待办事项或日程信息。\
             如果图片中有任务相关内容，请调用相应的工具来创建或管理任务。\
             如果图片不包含任务相关信息，请不要调用任何工具。",
            current_time
        );

        // Construct image data URL
        let image_data_url = format!("data:{};base64,{}", image_type, image_base64);

        // Build messages with image and text
        let messages = vec![json!({
            "role": "user",
            "content": [
                {
                    "type": "image_url",
                    "image_url": {
                        "url": image_data_url
                    }
                },
                {
                    "type": "text",
                    "text": prompt
                }
            ]
        })];

        // Get API key based on provider
        let api_key = match self.provider {
            ModelProvider::Kimi => {
                env::var("MOONSHOT_API_KEY")
                    .context("MOONSHOT_API_KEY required for Kimi vision")?
            }
            ModelProvider::OpenAI => {
                env::var("OPENAI_API_KEY")
                    .context("OPENAI_API_KEY required for OpenAI vision")?
            }
            ModelProvider::Anthropic => {
                env::var("ANTHROPIC_API_KEY")
                    .context("ANTHROPIC_API_KEY required for Anthropic vision")?
            }
        };

        // Call appropriate vision API
        let response_json: serde_json::Value = match self.provider {
            ModelProvider::Kimi => {
                self.call_kimi_vision_with_tools(&api_key, "moonshot-v1-8k-vision-preview", &messages, &tools).await?
            }
            ModelProvider::OpenAI => {
                self.call_openai_vision_with_tools(&api_key, "gpt-4o", &messages, &tools).await?
            }
            ModelProvider::Anthropic => {
                // For now, Anthropic doesn't support this method, return empty response
                return Err(anyhow::anyhow!("Anthropic vision + tool calling not yet implemented"));
            }
        };

        // Parse response based on provider
        let (operations, image_description): (Vec<super::task_operations::TaskOperation>, Option<String>) =
            if self.provider == ModelProvider::Kimi || self.provider == ModelProvider::OpenAI {
            // OpenAI-compatible response format
            if let Some(tool_calls) = response_json["choices"][0]["message"]["tool_calls"].as_array() {
                let ops = ToolCallParser::parse_tool_calls(tool_calls)?;
                let desc: Option<String> = response_json["choices"][0]["message"]["content"]
                    .as_str()
                    .map(|s: &str| s.to_string());
                (ops, desc)
            } else {
                let desc: Option<String> = response_json["choices"][0]["message"]["content"]
                    .as_str()
                    .map(|s: &str| s.to_string());
                (Vec::new(), desc)
            }
        } else {
            // Anthropic format would be different, but similar logic
            (Vec::new(), None)
        };

        let warnings: Vec<String> = if operations.is_empty() {
            vec!["图片中未识别到任务操作".to_string()]
        } else {
            Vec::new()
        };

        let confidence = if operations.is_empty() { 0.0 } else { 0.9 };

        Ok(ImageParseResult {
            operations,
            confidence,
            image_description,
            warnings,
        })
    }

    /// Calls Kimi Vision API with tools
    async fn call_kimi_vision_with_tools(
        &self,
        api_key: &str,
        model: &str,
        messages: &[serde_json::Value],
        tools: &[serde_json::Value],
    ) -> Result<serde_json::Value> {
        println!("Calling Kimi Vision API with tools: model={}", model);

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.moonshot.cn/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": model,
                "messages": messages,
                "tools": tools,
                "temperature": 0.3,
            }))
            .send()
            .await
            .context("Failed to call Kimi vision API")?;

        let status = response.status();
        let response_text = response.text().await.context("Failed to read response")?;

        println!("Kimi API response: status={}", status);

        if !status.is_success() {
            return Err(anyhow::anyhow!(
                "Kimi API error {}: {}",
                status,
                response_text
            ));
        }

        let response_json: serde_json::Value = serde_json::from_str(&response_text)
            .context("Failed to parse Kimi response as JSON")?;

        Ok(response_json)
    }

    /// Calls OpenAI Vision API with tools
    async fn call_openai_vision_with_tools(
        &self,
        api_key: &str,
        model: &str,
        messages: &[serde_json::Value],
        tools: &[serde_json::Value],
    ) -> Result<serde_json::Value> {
        println!("Calling OpenAI Vision API with tools: model={}", model);

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": model,
                "messages": messages,
                "tools": tools,
                "max_tokens": 1000,
                "temperature": 0.3,
            }))
            .send()
            .await
            .context("Failed to call OpenAI vision API")?;

        let status = response.status();
        let response_text = response.text().await.context("Failed to read response")?;

        println!("OpenAI API response: status={}", status);

        if !status.is_success() {
            return Err(anyhow::anyhow!(
                "OpenAI API error {}: {}",
                status,
                response_text
            ));
        }

        let response_json: serde_json::Value = serde_json::from_str(&response_text)
            .context("Failed to parse OpenAI response as JSON")?;

        Ok(response_json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_parse_simple_task() {
        let client = AiClient::new_default().expect("Failed to create client");
        let result = client
            .parse_text_input("Buy groceries")
            .await
            .expect("Failed to parse");

        assert!(!result.title.is_empty());
        assert!(result.title.to_lowercase().contains("groceries")
            || result.title.to_lowercase().contains("buy"));
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_parse_task_with_priority() {
        let client = AiClient::new_default().expect("Failed to create client");
        let result = client
            .parse_text_input("Urgent: Fix production bug ASAP")
            .await
            .expect("Failed to parse");

        assert_eq!(result.priority, Some("high".to_string()));
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_parse_task_with_deadline() {
        let client = AiClient::new_default().expect("Failed to create client");
        let result = client
            .parse_text_input("Submit report by tomorrow")
            .await
            .expect("Failed to parse");

        assert!(result.deadline.is_some());
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_health_check() {
        let client = AiClient::new_default().expect("Failed to create client");
        let healthy = client.health_check().await;
        assert!(healthy);
    }
}
