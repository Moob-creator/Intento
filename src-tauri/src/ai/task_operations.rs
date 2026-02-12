use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use serde_json::{json, Value};

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

    /// 根据 ToolSet 获取对应工具
    pub fn get_tools(tool_set: ToolSet) -> Vec<Value> {
        match tool_set {
            ToolSet::Basic => Self::basic_tools(),
            ToolSet::Management => Self::management_tools(),
            ToolSet::All => Self::all_tools(),
        }
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
                            "description": "任务标题，简洁明了地概括任务内容"
                        },
                        "description": {
                            "type": "string",
                            "description": "任务详细描述，包含从图片中提取的所有相关信息"
                        },
                        "priority": {
                            "type": "string",
                            "enum": ["low", "medium", "high"],
                            "description": "优先级：high=紧急重要，medium=一般，low=不紧急。根据图片中的关键词判断：'urgent'/'紧急'/'ASAP'→high，'重要'/'important'→high，其他默认medium"
                        },
                        "deadline": {
                            "type": "string",
                            "description": "截止时间 ISO8601 格式，例如 2024-03-16T10:00:00+08:00。根据图片中的时间信息转换：'明天'→当前时间+1天，'下周五'→计算具体日期"
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_operation_description() {
        let op = TaskOperation::Create {
            title: "测试任务".to_string(),
            description: None,
            priority: None,
            deadline: None,
            tags: None,
        };
        assert_eq!(op.description(), "创建任务: 测试任务");
        assert_eq!(op.operation_name(), "create");
    }

    #[test]
    fn test_tool_registry() {
        let basic_tools = TaskToolRegistry::basic_tools();
        assert_eq!(basic_tools.len(), 1);

        let all_tools = TaskToolRegistry::all_tools();
        assert_eq!(all_tools.len(), 6);
    }

    #[test]
    fn test_parse_create_task() {
        let args = json!({
            "title": "买牛奶",
            "priority": "high",
            "tags": ["购物"]
        });

        let op = ToolCallParser::parse_create_task(&args).unwrap();

        match op {
            TaskOperation::Create { title, priority, tags, .. } => {
                assert_eq!(title, "买牛奶");
                assert_eq!(priority, Some("high".to_string()));
                assert_eq!(tags, Some(vec!["购物".to_string()]));
            }
            _ => panic!("Expected Create operation"),
        }
    }
}
