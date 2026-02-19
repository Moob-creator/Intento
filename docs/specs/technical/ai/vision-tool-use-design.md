# 图片识别 + Tool Use 技术方案设计

## 1. 背景与目标

### 1.1 当前问题
- 使用 `moonshot-v1-8k-vision-preview` 可以识别图片内容
- 但是模型直接返回的是自然语言描述，不是结构化的 Task 数据
- 需要将识别结果转化为符合应用需求的 Task 对象

### 1.2 解决方案
利用 Kimi API 的 **Tool Use (Function Calling)** 功能，让模型在识别图片后，主动调用我们提供的 `create_task` 工具来创建任务。

## 2. Tool Use 核心概念

### 2.1 工作流程

```
用户上传图片
    ↓
前端发送图片到后端
    ↓
后端调用 Kimi Vision API（带 tools 参数）
    ↓
Kimi 分析图片内容
    ↓
Kimi 决定是否需要调用 create_task 工具
    ↓
返回 tool_calls（包含任务参数）
    ↓
后端解析 tool_calls，提取任务信息
    ↓
返回结构化的 ParsedTask 给前端
    ↓
前端展示确认对话框
    ↓
用户确认后创建任务
```

### 2.2 Tool Use 示例

#### 请求格式
```json
{
  "model": "moonshot-v1-8k-vision-preview",
  "messages": [
    {
      "role": "user",
      "content": [
        {"type": "image_url", "image_url": {"url": "data:image/png;base64,..."}},
        {"type": "text", "text": "请识别图片中的任务信息"}
      ]
    }
  ],
  "tools": [
    {
      "type": "function",
      "function": {
        "name": "create_task",
        "description": "根据识别的内容创建任务",
        "parameters": {
          "type": "object",
          "properties": {
            "title": {
              "type": "string",
              "description": "任务标题"
            },
            "description": {
              "type": "string",
              "description": "任务详细描述"
            },
            "priority": {
              "type": "string",
              "enum": ["low", "medium", "high"],
              "description": "任务优先级"
            },
            "deadline": {
              "type": "string",
              "description": "截止时间（ISO8601 格式）"
            },
            "tags": {
              "type": "array",
              "items": {"type": "string"},
              "description": "任务标签列表"
            }
          },
          "required": ["title"]
        }
      }
    }
  ]
}
```

#### 返回格式（模型决定调用工具）
```json
{
  "id": "chatcmpl-xxx",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": null,
        "tool_calls": [
          {
            "id": "call_abc123",
            "type": "function",
            "function": {
              "name": "create_task",
              "arguments": "{\"title\":\"完成季度报告\",\"description\":\"根据图片显示，需要在周五前完成Q4季度报告\",\"priority\":\"high\",\"deadline\":\"2024-03-15T17:00:00+08:00\",\"tags\":[\"工作\",\"报告\"]}"
            }
          }
        ]
      },
      "finish_reason": "tool_calls"
    }
  ]
}
```

## 3. 技术实现方案

### 3.1 架构设计

```
┌─────────────────────────────────────────────────────────────┐
│  Frontend (React)                                           │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  用户粘贴/拖拽图片                                      │  │
│  │  ↓                                                     │  │
│  │  invoke('parse_image_input', {imageBase64, imageType}) │  │
│  │  ↓                                                     │  │
│  │  显示 ParsedTask 确认对话框                             │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ Tauri IPC
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  Backend (Rust)                                             │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  commands::ai::parse_image_input                      │  │
│  │  ↓                                                     │  │
│  │  AiClient::parse_image_input_with_tools               │  │
│  │  ↓                                                     │  │
│  │  构建 Tool Definition (create_task)                    │  │
│  │  ↓                                                     │  │
│  │  调用 Kimi Vision API with tools                       │  │
│  │  ↓                                                     │  │
│  │  解析 tool_calls → ParsedTask                          │  │
│  │  ↓                                                     │  │
│  │  返回给前端                                              │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ HTTPS
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  Kimi API (api.moonshot.cn)                                 │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Vision Model (moonshot-v1-8k-vision-preview)         │  │
│  │  ↓                                                     │  │
│  │  图片识别 + Tool Selection                             │  │
│  │  ↓                                                     │  │
│  │  返回 tool_calls                                       │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 代码实现

#### 3.2.1 Rust 后端修改

**src/ai/client.rs** - 添加 Tool Use 支持

```rust
/// Parses image input using Kimi Vision + Tool Use
pub async fn parse_image_input_with_tools(
    &self,
    image_base64: &str,
    image_type: &str,
) -> Result<ParsedTask> {
    // 获取当前时间用于 deadline 推断
    let current_time = chrono::Utc::now().to_rfc3339();

    // 构建 tool definition
    let tool = json!({
        "type": "function",
        "function": {
            "name": "create_task",
            "description": "根据图片中识别的任务相关信息创建任务。当图片中包含待办事项、任务列表、日程安排、提醒事项时调用此工具。",
            "parameters": {
                "type": "object",
                "properties": {
                    "title": {
                        "type": "string",
                        "description": "任务标题，简洁明了地概括任务内容"
                    },
                    "description": {
                        "type": "string",
                        "description": "任务的详细描述，包含从图片中提取的所有相关信息"
                    },
                    "priority": {
                        "type": "string",
                        "enum": ["low", "medium", "high"],
                        "description": "任务优先级。high=紧急重要，medium=一般，low=不紧急。根据图片中的关键词判断：'urgent'/'紧急'/'ASAP'→high，'重要'/'important'→high，其他默认medium"
                    },
                    "deadline": {
                        "type": "string",
                        "description": format!("任务截止时间，ISO8601 格式。当前时间: {}。根据图片中的时间信息转换：'明天'→当前时间+1天，'下周五'→计算具体日期，'3月15日'→2024-03-15T23:59:59+08:00", current_time)
                    },
                    "tags": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "任务标签列表，从图片中提取的分类信息，如：工作、学习、生活、购物等"
                    }
                },
                "required": ["title"]
            }
        }
    });

    // 构建消息
    let image_data_url = format!("data:{};base64,{}", image_type, image_base64);
    let messages = vec![json!({
        "role": "user",
        "content": [
            {
                "type": "image_url",
                "image_url": {"url": image_data_url}
            },
            {
                "type": "text",
                "text": "请仔细分析这张图片，识别其中包含的任务、待办事项或日程信息。如果图片中有任务相关内容，请调用 create_task 工具来创建任务。"
            }
        ]
    })];

    // 调用 Kimi API
    let response = self.call_kimi_vision_with_tools(
        &self.api_key,
        "moonshot-v1-8k-vision-preview",
        &messages,
        &[tool],
    ).await?;

    // 解析 tool_calls
    if let Some(tool_calls) = response["choices"][0]["message"]["tool_calls"].as_array() {
        if let Some(tool_call) = tool_calls.first() {
            let arguments_str = tool_call["function"]["arguments"]
                .as_str()
                .context("Invalid tool call arguments")?;

            let mut parsed: ParsedTask = serde_json::from_str(arguments_str)
                .context("Failed to parse tool call arguments as ParsedTask")?;

            // 标准化和验证
            parsed.normalize_priority();
            parsed.validate_priority()?;

            if let Some(ref deadline) = parsed.deadline {
                parsed.parse_deadline().map_err(|e| {
                    anyhow::anyhow!("Invalid deadline format '{}': {}", deadline, e)
                })?;
            }

            return Ok(parsed);
        }
    }

    // 如果模型没有调用工具，返回错误提示
    Err(anyhow::anyhow!(
        "图片中未识别到任务信息。请确保图片包含清晰的待办事项或任务内容。"
    ))
}

/// 调用 Kimi Vision API with tools
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
            "temperature": 0.3,  // 降低随机性，提高准确性
        }))
        .send()
        .await
        .context("Failed to call Kimi vision API")?;

    let status = response.status();
    let response_text = response.text().await.context("Failed to read response")?;

    println!("Kimi API response: status={}, body={}", status, response_text);

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
```

#### 3.2.2 更新 parse_image_input 命令

**src/commands/ai.rs**

```rust
#[tauri::command]
pub async fn parse_image_input(
    image_base64: String,
    image_type: String,
    state: State<'_, AiClientState>,
) -> Result<ParsedTask, String> {
    if image_base64.trim().is_empty() {
        return Err("图片数据不能为空".to_string());
    }

    if !image_type.starts_with("image/") {
        return Err(format!("无效的图片类型: {}", image_type));
    }

    let client = state.get_or_init().await?;

    // 使用 Tool Use 方式解析图片
    client
        .parse_image_input_with_tools(&image_base64, &image_type)
        .await
        .map_err(|e| format!("图片识别失败: {}", e))
}
```

## 4. 技术优势

### 4.1 相比直接解析 JSON 的优势

| 方面 | 直接 JSON 解析 | Tool Use 方案 |
|-----|--------------|--------------|
| **准确性** | 模型可能返回格式不规范的 JSON | 模型严格按照 tool schema 生成参数，格式保证 |
| **错误处理** | 需要大量边界条件处理 | Schema 验证由 API 层完成，减少错误 |
| **语义理解** | 依赖 prompt 引导 | Function description 提供明确的调用意图 |
| **扩展性** | 修改格式需要重写 prompt | 修改 tool schema 即可，向后兼容 |
| **可维护性** | Prompt 难以版本管理 | Schema 是结构化配置，易于管理 |

### 4.2 Tool Use 的其他优势

1. **智能决策**：模型会自动判断图片是否包含任务信息，避免强制解析
2. **多工具支持**：未来可以扩展多个工具（如 `create_event`、`set_reminder`）
3. **参数验证**：利用 JSON Schema 在 API 层验证参数合法性
4. **Token 效率**：相比冗长的 JSON 示例，tool schema 更简洁

## 5. 实现细节

### 5.1 Prompt 优化

在 tool description 中提供清晰的指导：

```rust
"description": "根据图片中识别的任务相关信息创建任务。\
  当图片中包含待办事项、任务列表、日程安排、提醒事项时调用此工具。\
  如果图片是普通风景照、人物照片等不包含任务信息的内容，请不要调用此工具。"
```

### 5.2 时间处理

在 `deadline` 参数描述中提供当前时间上下文：

```rust
"description": format!(
    "任务截止时间，ISO8601 格式。当前时间: {}。\
     根据图片中的时间信息转换：\
     - '明天' → 当前时间+1天\
     - '下周五' → 计算具体日期\
     - '3月15日' → 2024-03-15T23:59:59+08:00",
    current_time
)
```

### 5.3 优先级推断

在 `priority` 参数中提供关键词映射规则：

```rust
"description": "任务优先级。\
  high=紧急重要，medium=一般，low=不紧急。\
  根据图片中的关键词判断：\
  - 'urgent'/'紧急'/'ASAP'/'!!!' → high\
  - '重要'/'important'/'高优先级' → high\
  - 其他默认 medium"
```

## 6. 错误处理

### 6.1 图片不包含任务信息

```rust
if !response.has_tool_calls() {
    return Err(anyhow::anyhow!(
        "图片中未识别到任务信息。\
         请确保图片包含清晰的待办事项或任务内容。"
    ));
}
```

### 6.2 Tool 参数验证失败

```rust
parsed.normalize_priority();
if let Err(e) = parsed.validate_priority() {
    return Err(anyhow::anyhow!(
        "识别的优先级无效: {}。系统已自动调整为 medium",
        e
    ));
}
```

### 6.3 截止时间解析失败

```rust
if let Some(ref deadline) = parsed.deadline {
    if let Err(e) = parsed.parse_deadline() {
        // 记录警告但不阻止任务创建
        eprintln!("Warning: Invalid deadline '{}': {}", deadline, e);
        parsed.deadline = None;
    }
}
```

## 7. 测试场景

### 7.1 待办列表截图

**输入图片**：手写待办清单
```
✓ 买菜
✗ 完成报告（周五前）
✗ 预约牙医
```

**期望输出**：
```json
{
  "title": "完成报告",
  "description": "根据待办清单，需要在周五前完成报告",
  "priority": "medium",
  "deadline": "2024-03-15T23:59:59+08:00",
  "tags": ["工作"]
}
```

### 7.2 日历/日程截图

**输入图片**：手机日历截图显示 "3月10日 15:00 团队会议"

**期望输出**：
```json
{
  "title": "团队会议",
  "description": "根据日历显示，3月10日下午3点有团队会议",
  "deadline": "2024-03-10T15:00:00+08:00",
  "tags": ["会议", "工作"]
}
```

### 7.3 便签/Post-it 截图

**输入图片**：黄色便签写着 "记得买牛奶!!!"

**期望输出**：
```json
{
  "title": "买牛奶",
  "description": "便签提醒需要购买牛奶",
  "priority": "high",
  "tags": ["购物", "生活"]
}
```

### 7.4 非任务图片

**输入图片**：风景照片

**期望输出**：
```
Error: "图片中未识别到任务信息。请确保图片包含清晰的待办事项或任务内容。"
```

## 8. 性能优化

### 8.1 Token 使用

- Vision 模型 token 消耗较高
- 建议压缩图片到合理尺寸（推荐 1024x1024 以下）
- 使用 `temperature=0.3` 降低随机性，提高稳定性

### 8.2 响应时间

- Vision API 响应时间：2-5 秒
- 前端需要显示加载状态
- 考虑添加超时处理（建议 10 秒）

### 8.3 错误重试

```rust
// 添加重试机制
const MAX_RETRIES: u32 = 2;
for attempt in 0..MAX_RETRIES {
    match self.call_kimi_vision_with_tools(...).await {
        Ok(result) => return Ok(result),
        Err(e) if attempt < MAX_RETRIES - 1 => {
            eprintln!("Retry {}/{}: {}", attempt + 1, MAX_RETRIES, e);
            tokio::time::sleep(Duration::from_secs(1)).await;
            continue;
        }
        Err(e) => return Err(e),
    }
}
```

## 9. 未来扩展

### 9.1 多工具支持

```rust
let tools = vec![
    create_task_tool(),
    create_event_tool(),      // 创建日历事件
    set_reminder_tool(),      // 设置提醒
    extract_contacts_tool(),  // 提取联系信息
];
```

### 9.2 OCR 优化

结合传统 OCR 预处理，提高文字识别准确率：
```
图片 → OCR 提取文字 → 文字 + 图片一起发送给 Vision API
```

### 9.3 批量处理

支持一次上传多张图片，批量创建任务：
```rust
pub async fn parse_multiple_images(
    &self,
    images: Vec<(String, String)>,  // (base64, type)
) -> Result<Vec<ParsedTask>>
```

## 10. 总结

### 10.1 为什么选择 Tool Use

1. **结构化输出保证**：相比让模型直接输出 JSON，Tool Use 通过 Schema 严格约束输出格式
2. **语义清晰**：Function description 明确告诉模型"何时调用"和"如何调用"
3. **错误减少**：API 层面的参数验证，减少格式错误
4. **扩展性强**：未来可以轻松添加更多工具，无需修改核心逻辑

### 10.2 实施建议

1. **先实现 Tool Use 版本**，替换现有的直接 JSON 解析
2. **充分测试**各种图片场景（待办列表、日历、便签等）
3. **优化 tool description**，提高识别准确率
4. **添加详细日志**，便于调试和性能分析
5. **前端增加重试机制**，处理偶发性 API 错误

### 10.3 预期效果

- ✅ 图片识别准确率提升（Schema 约束）
- ✅ 错误率降低（格式验证）
- ✅ 代码可维护性提高（结构化配置）
- ✅ 用户体验改善（更准确的任务解析）

---

**文档版本**: v1.0
**作者**: Claude
**日期**: 2024-03-12
