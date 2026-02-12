# 图片识别任务操作架构设计

## 1. 架构概述

### 1.1 核心设计原则

- **可扩展性**：支持多种任务操作（创建、更新、完成、删除等）
- **统一接口**：所有操作通过统一的 Tool 定义和调用机制
- **类型安全**：使用 Rust 枚举强类型约束操作类型
- **灵活组合**：支持一张图片触发多个操作

### 1.2 架构层次

```
┌─────────────────────────────────────────────────────────────┐
│  Frontend Layer (React)                                     │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Image Upload → parse_image_input                     │  │
│  │  ↓                                                     │  │
│  │  TaskOperations[] → Confirmation Dialog               │  │
│  │  ↓                                                     │  │
│  │  Execute Operations (create/update/complete/delete)   │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ Tauri IPC
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  Backend Layer (Rust)                                       │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  TaskOperationHandler                                 │  │
│  │  ├─ parse_image_for_operations()                      │  │
│  │  ├─ execute_operation(TaskOperation)                  │  │
│  │  └─ batch_execute_operations(Vec<TaskOperation>)      │  │
│  └───────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Tool Registry                                        │  │
│  │  ├─ create_task_tool                                  │  │
│  │  ├─ update_task_tool                                  │  │
│  │  ├─ complete_task_tool                                │  │
│  │  ├─ delete_task_tool                                  │  │
│  │  └─ ... (extensible)                                  │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ HTTPS
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  AI Provider (Kimi/OpenAI/Anthropic)                        │
│  Vision API + Tool Calling                                  │
└─────────────────────────────────────────────────────────────┘
```

## 2. 数据模型设计

### 2.1 任务操作类型 (TaskOperation)

```rust
use serde::{Deserialize, Serialize};

/// 任务操作类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum TaskOperation {
    /// 创建新任务
    Create {
        title: String,
        description: Option<String>,
        priority: Option<String>,
        deadline: Option<String>,
        tags: Option<Vec<String>>,
    },

    /// 更新现有任务
    Update {
        /// 任务标识（可以是 ID 或标题）
        task_identifier: String,
        /// 更新的字段
        title: Option<String>,
        description: Option<String>,
        priority: Option<String>,
        deadline: Option<String>,
        tags: Option<Vec<String>>,
    },

    /// 完成任务
    Complete {
        /// 任务标识
        task_identifier: String,
    },

    /// 删除任务
    Delete {
        /// 任务标识
        task_identifier: String,
    },

    /// 批量完成多个任务
    BatchComplete {
        task_identifiers: Vec<String>,
    },

    /// 设置任务状态
    SetStatus {
        task_identifier: String,
        status: String, // "todo", "doing", "done"
    },
}

impl TaskOperation {
    /// 获取操作类型名称
    pub fn operation_name(&self) -> &str {
        match self {
            Self::Create { .. } => "create",
            Self::Update { .. } => "update",
            Self::Complete { .. } => "complete",
            Self::Delete { .. } => "delete",
            Self::BatchComplete { .. } => "batch_complete",
            Self::SetStatus { .. } => "set_status",
        }
    }

    /// 获取操作的人类可读描述
    pub fn description(&self) -> String {
        match self {
            Self::Create { title, .. } => format!("创建任务: {}", title),
            Self::Update { task_identifier, .. } => format!("更新任务: {}", task_identifier),
            Self::Complete { task_identifier } => format!("完成任务: {}", task_identifier),
            Self::Delete { task_identifier } => format!("删除任务: {}", task_identifier),
            Self::BatchComplete { task_identifiers } => {
                format!("完成 {} 个任务", task_identifiers.len())
            }
            Self::SetStatus { task_identifier, status } => {
                format!("设置任务状态为 {}: {}", status, task_identifier)
            }
        }
    }
}
```

### 2.2 解析结果 (ImageParseResult)

```rust
/// 图片解析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageParseResult {
    /// 识别到的操作列表
    pub operations: Vec<TaskOperation>,

    /// 识别置信度 (0.0-1.0)
    pub confidence: f32,

    /// 图片内容描述（可选）
    pub image_description: Option<String>,

    /// 警告信息（如果有）
    pub warnings: Vec<String>,
}

impl ImageParseResult {
    pub fn has_operations(&self) -> bool {
        !self.operations.is_empty()
    }

    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }
}
```

## 3. Tool Registry 设计

### 3.1 Tool 定义工厂

```rust
use serde_json::{json, Value};

/// Tool 定义注册表
pub struct TaskToolRegistry;

impl TaskToolRegistry {
    /// 获取所有可用的工具定义
    pub fn all_tools() -> Vec<Value> {
        vec![
            Self::create_task_tool(),
            Self::update_task_tool(),
            Self::complete_task_tool(),
            Self::delete_task_tool(),
            Self::batch_complete_tool(),
            Self::set_status_tool(),
        ]
    }

    /// 获取基础工具（仅创建）
    pub fn basic_tools() -> Vec<Value> {
        vec![Self::create_task_tool()]
    }

    /// 获取管理工具（更新、完成、删除）
    pub fn management_tools() -> Vec<Value> {
        vec![
            Self::update_task_tool(),
            Self::complete_task_tool(),
            Self::delete_task_tool(),
            Self::set_status_tool(),
        ]
    }

    /// 创建任务工具
    fn create_task_tool() -> Value {
        json!({
            "type": "function",
            "function": {
                "name": "create_task",
                "description": "创建新任务。当图片中包含新的待办事项、任务列表、便签提醒时调用。",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "title": {
                            "type": "string",
                            "description": "任务标题，简洁明了"
                        },
                        "description": {
                            "type": "string",
                            "description": "任务详细描述"
                        },
                        "priority": {
                            "type": "string",
                            "enum": ["low", "medium", "high"],
                            "description": "优先级：high=紧急，medium=一般，low=不紧急"
                        },
                        "deadline": {
                            "type": "string",
                            "description": "截止时间 ISO8601 格式"
                        },
                        "tags": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "标签列表"
                        }
                    },
                    "required": ["title"]
                }
            }
        })
    }

    /// 更新任务工具
    fn update_task_tool() -> Value {
        json!({
            "type": "function",
            "function": {
                "name": "update_task",
                "description": "更新现有任务的信息。当图片显示对已有任务的修改、补充、调整时调用。",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "task_identifier": {
                            "type": "string",
                            "description": "任务标识，可以是任务标题或关键词"
                        },
                        "title": {
                            "type": "string",
                            "description": "新的任务标题（如果要修改）"
                        },
                        "description": {
                            "type": "string",
                            "description": "新的任务描述（如果要修改）"
                        },
                        "priority": {
                            "type": "string",
                            "enum": ["low", "medium", "high"],
                            "description": "新的优先级（如果要修改）"
                        },
                        "deadline": {
                            "type": "string",
                            "description": "新的截止时间（如果要修改）"
                        },
                        "tags": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "新的标签列表（如果要修改）"
                        }
                    },
                    "required": ["task_identifier"]
                }
            }
        })
    }

    /// 完成任务工具
    fn complete_task_tool() -> Value {
        json!({
            "type": "function",
            "function": {
                "name": "complete_task",
                "description": "标记任务为已完成。当图片中显示任务已打钩✓、已划线、标记为done等完成状态时调用。",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "task_identifier": {
                            "type": "string",
                            "description": "要完成的任务标识，可以是标题或关键词"
                        }
                    },
                    "required": ["task_identifier"]
                }
            }
        })
    }

    /// 删除任务工具
    fn delete_task_tool() -> Value {
        json!({
            "type": "function",
            "function": {
                "name": "delete_task",
                "description": "删除任务。当图片明确显示要删除或取消某个任务时调用。",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "task_identifier": {
                            "type": "string",
                            "description": "要删除的任务标识"
                        }
                    },
                    "required": ["task_identifier"]
                }
            }
        })
    }

    /// 批量完成工具
    fn batch_complete_tool() -> Value {
        json!({
            "type": "function",
            "function": {
                "name": "batch_complete_tasks",
                "description": "批量完成多个任务。当图片显示多个任务都已完成时调用。",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "task_identifiers": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "要完成的任务标识列表"
                        }
                    },
                    "required": ["task_identifiers"]
                }
            }
        })
    }

    /// 设置状态工具
    fn set_status_tool() -> Value {
        json!({
            "type": "function",
            "function": {
                "name": "set_task_status",
                "description": "设置任务状态。当图片显示任务状态变更时调用。",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "task_identifier": {
                            "type": "string",
                            "description": "任务标识"
                        },
                        "status": {
                            "type": "string",
                            "enum": ["todo", "doing", "done"],
                            "description": "新状态：todo=待办，doing=进行中，done=已完成"
                        }
                    },
                    "required": ["task_identifier", "status"]
                }
            }
        })
    }
}
```

### 3.2 Tool Call 解析器

```rust
use anyhow::{Context, Result};

/// Tool Call 解析器
pub struct ToolCallParser;

impl ToolCallParser {
    /// 解析 API 返回的 tool_calls 为 TaskOperation
    pub fn parse_tool_calls(tool_calls: &[Value]) -> Result<Vec<TaskOperation>> {
        let mut operations = Vec::new();

        for tool_call in tool_calls {
            let function_name = tool_call["function"]["name"]
                .as_str()
                .context("Missing function name")?;

            let arguments_str = tool_call["function"]["arguments"]
                .as_str()
                .context("Missing function arguments")?;

            let args: Value = serde_json::from_str(arguments_str)
                .context("Failed to parse arguments as JSON")?;

            let operation = match function_name {
                "create_task" => Self::parse_create_task(&args)?,
                "update_task" => Self::parse_update_task(&args)?,
                "complete_task" => Self::parse_complete_task(&args)?,
                "delete_task" => Self::parse_delete_task(&args)?,
                "batch_complete_tasks" => Self::parse_batch_complete(&args)?,
                "set_task_status" => Self::parse_set_status(&args)?,
                _ => {
                    eprintln!("Unknown function: {}", function_name);
                    continue;
                }
            };

            operations.push(operation);
        }

        Ok(operations)
    }

    fn parse_create_task(args: &Value) -> Result<TaskOperation> {
        Ok(TaskOperation::Create {
            title: args["title"]
                .as_str()
                .context("Missing title")?
                .to_string(),
            description: args["description"].as_str().map(|s| s.to_string()),
            priority: args["priority"].as_str().map(|s| s.to_string()),
            deadline: args["deadline"].as_str().map(|s| s.to_string()),
            tags: args["tags"]
                .as_array()
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()),
        })
    }

    fn parse_update_task(args: &Value) -> Result<TaskOperation> {
        Ok(TaskOperation::Update {
            task_identifier: args["task_identifier"]
                .as_str()
                .context("Missing task_identifier")?
                .to_string(),
            title: args["title"].as_str().map(|s| s.to_string()),
            description: args["description"].as_str().map(|s| s.to_string()),
            priority: args["priority"].as_str().map(|s| s.to_string()),
            deadline: args["deadline"].as_str().map(|s| s.to_string()),
            tags: args["tags"]
                .as_array()
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()),
        })
    }

    fn parse_complete_task(args: &Value) -> Result<TaskOperation> {
        Ok(TaskOperation::Complete {
            task_identifier: args["task_identifier"]
                .as_str()
                .context("Missing task_identifier")?
                .to_string(),
        })
    }

    fn parse_delete_task(args: &Value) -> Result<TaskOperation> {
        Ok(TaskOperation::Delete {
            task_identifier: args["task_identifier"]
                .as_str()
                .context("Missing task_identifier")?
                .to_string(),
        })
    }

    fn parse_batch_complete(args: &Value) -> Result<TaskOperation> {
        let identifiers = args["task_identifiers"]
            .as_array()
            .context("Missing task_identifiers array")?
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        Ok(TaskOperation::BatchComplete {
            task_identifiers: identifiers,
        })
    }

    fn parse_set_status(args: &Value) -> Result<TaskOperation> {
        Ok(TaskOperation::SetStatus {
            task_identifier: args["task_identifier"]
                .as_str()
                .context("Missing task_identifier")?
                .to_string(),
            status: args["status"]
                .as_str()
                .context("Missing status")?
                .to_string(),
        })
    }
}
```

## 4. AI Client 更新

### 4.1 支持多工具调用

```rust
impl AiClient {
    /// 使用多工具解析图片
    pub async fn parse_image_for_operations(
        &self,
        image_base64: &str,
        image_type: &str,
        tool_set: ToolSet,
    ) -> Result<ImageParseResult> {
        let current_time = chrono::Utc::now().to_rfc3339();

        // 根据工具集选择工具
        let tools = match tool_set {
            ToolSet::Basic => TaskToolRegistry::basic_tools(),
            ToolSet::Management => TaskToolRegistry::management_tools(),
            ToolSet::All => TaskToolRegistry::all_tools(),
        };

        let prompt = format!(
            "当前时间: {}。请仔细分析图片中的任务信息，识别需要执行的操作。\
             可以创建新任务、更新现有任务、标记任务完成等。\
             如果图片不包含任务相关信息，请不要调用任何工具。",
            current_time
        );

        let image_data_url = format!("data:{};base64,{}", image_type, image_base64);
        let messages = vec![json!({
            "role": "user",
            "content": [
                {"type": "image_url", "image_url": {"url": image_data_url}},
                {"type": "text", "text": prompt}
            ]
        })];

        let response = self.call_vision_api_with_tools(&messages, &tools).await?;

        // 解析 tool_calls
        if let Some(tool_calls) = response["choices"][0]["message"]["tool_calls"].as_array() {
            let operations = ToolCallParser::parse_tool_calls(tool_calls)?;

            Ok(ImageParseResult {
                operations,
                confidence: 0.9, // TODO: 从 API 响应中提取真实置信度
                image_description: response["choices"][0]["message"]["content"]
                    .as_str()
                    .map(|s| s.to_string()),
                warnings: Vec::new(),
            })
        } else {
            Ok(ImageParseResult {
                operations: Vec::new(),
                confidence: 0.0,
                image_description: response["choices"][0]["message"]["content"]
                    .as_str()
                    .map(|s| s.to_string()),
                warnings: vec!["图片中未识别到任务操作".to_string()],
            })
        }
    }
}

/// 工具集选择
#[derive(Debug, Clone, Copy)]
pub enum ToolSet {
    /// 仅基础工具（创建任务）
    Basic,
    /// 管理工具（更新、完成、删除）
    Management,
    /// 所有工具
    All,
}
```

## 5. 前端集成

### 5.1 TypeScript 类型定义

```typescript
// src/types/taskOperation.ts

export type TaskOperation =
  | { type: 'Create'; data: CreateTaskData }
  | { type: 'Update'; data: UpdateTaskData }
  | { type: 'Complete'; data: { task_identifier: string } }
  | { type: 'Delete'; data: { task_identifier: string } }
  | { type: 'BatchComplete'; data: { task_identifiers: string[] } }
  | { type: 'SetStatus'; data: { task_identifier: string; status: TaskStatus } };

export interface CreateTaskData {
  title: string;
  description?: string;
  priority?: 'low' | 'medium' | 'high';
  deadline?: string;
  tags?: string[];
}

export interface UpdateTaskData {
  task_identifier: string;
  title?: string;
  description?: string;
  priority?: 'low' | 'medium' | 'high';
  deadline?: string;
  tags?: string[];
}

export interface ImageParseResult {
  operations: TaskOperation[];
  confidence: number;
  image_description?: string;
  warnings: string[];
}
```

### 5.2 前端确认对话框

```typescript
// src/components/TaskOperationsConfirmDialog.tsx

interface Props {
  isOpen: boolean;
  parseResult: ImageParseResult | null;
  onConfirm: (operations: TaskOperation[]) => Promise<void>;
  onCancel: () => void;
}

export function TaskOperationsConfirmDialog({ isOpen, parseResult, onConfirm, onCancel }: Props) {
  const [selectedOps, setSelectedOps] = useState<number[]>([]);

  if (!parseResult || !isOpen) return null;

  return (
    <div className="modal">
      <h3>识别到 {parseResult.operations.length} 个操作</h3>

      {parseResult.operations.map((op, index) => (
        <div key={index} className="operation-item">
          <input
            type="checkbox"
            checked={selectedOps.includes(index)}
            onChange={() => toggleOperation(index)}
          />
          <span>{getOperationDescription(op)}</span>
        </div>
      ))}

      <div className="actions">
        <button onClick={() => onConfirm(getSelectedOperations())}>
          执行 {selectedOps.length} 个操作
        </button>
        <button onClick={onCancel}>取消</button>
      </div>
    </div>
  );
}
```

## 6. 实施建议

### Phase 1: 基础实现（当前优先级）
1. ✅ 实现 `TaskOperation` 枚举和基本数据结构
2. ✅ 实现 `TaskToolRegistry` 注册表
3. ✅ 实现 `create_task` 工具的完整流程
4. ⚠️  测试 Kimi Vision + Tool Calling 能力

### Phase 2: 扩展管理功能（后续）
1. 实现 `update_task` 工具
2. 实现 `complete_task` 工具
3. 实现任务标识符匹配逻辑（模糊搜索）
4. 前端多操作确认对话框

### Phase 3: 高级功能（未来）
1. 批量操作支持
2. 操作历史记录
3. 撤销/重做功能
4. 智能操作建议

## 7. 技术决策记录

### 7.1 单模型 vs 双模型方案

**待验证**: Kimi Vision 模型的 Tool Calling 能力

**方案 A: 单模型方案（推荐）**
- 使用 `moonshot-v1-8k-vision-preview` 同时处理图片识别和工具调用
- 优点: 架构简单，成本低，上下文统一
- 缺点: 如果 Vision 模型 tool calling 能力弱，准确率可能下降

**方案 B: 双模型方案（备选）**
- Step 1: 用 Vision 模型提取图片中的文本和结构化信息
- Step 2: 用 Text 模型（如 `kimi-k2-turbo-preview`）+ Tool Calling 解析操作
- 优点: Tool calling 准确率可能更高
- 缺点: 两次 API 调用，成本和延迟增加

**实施策略**:
1. 先实现方案 A，用测试用例验证效果
2. 如果效果不佳，再切换到方案 B
3. 通过配置开关支持两种方案切换

### 7.2 任务标识符匹配策略

对于 `update/complete/delete` 等操作，需要匹配现有任务：

```rust
// 模糊匹配任务
pub async fn find_task_by_identifier(
    db: &Database,
    identifier: &str,
) -> Result<Option<Task>> {
    // 1. 精确匹配 ID
    if let Ok(id) = identifier.parse::<i64>() {
        if let Ok(task) = db.get_task(id).await {
            return Ok(Some(task));
        }
    }

    // 2. 精确匹配标题
    let tasks = db.list_tasks().await?;
    if let Some(task) = tasks.iter().find(|t| t.title == identifier) {
        return Ok(Some(task.clone()));
    }

    // 3. 模糊匹配标题（包含关键词）
    if let Some(task) = tasks.iter().find(|t|
        t.title.contains(identifier) || identifier.contains(&t.title)
    ) {
        return Ok(Some(task.clone()));
    }

    // 4. 使用 fuzzy search (可选)
    // ...

    Ok(None)
}
```

---

**文档版本**: v1.0
**作者**: Claude
**日期**: 2024-03-12
