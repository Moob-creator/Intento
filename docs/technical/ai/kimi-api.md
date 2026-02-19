# Kimi/Moonshot API 开发文档

## 基本信息

- **服务地址**: `https://api.moonshot.cn`
- **兼容 OpenAI SDK**: 可以直接使用 OpenAI Python/Node.js SDK 调用
- **API Key**: 需要在用户中心获取 API Key

### 可用模型列表

| 模型名称 | 描述 | 上下文长度 |
|---------|------|-----------|
| `kimi-k2.5` | Kimi K2.5 多模态模型 | 256K |
| `kimi-k2-0905-preview` | Kimi K2 预览版 | 256K |
| `kimi-k2-0711-preview` | Kimi K2 早期预览版 | 128K |
| `kimi-k2-turbo-preview` | Kimi K2 Turbo 预览版 | 128K |
| `kimi-k2-thinking-turbo` | Kimi K2 Thinking Turbo | 128K |
| `kimi-k2-thinking` | Kimi K2 Thinking | 128K |
| `moonshot-v1-8k` | 标准版，支持 8K 上下文 | 8K |
| `moonshot-v1-32k` | 标准版，支持 32K 上下文 | 32K |
| `moonshot-v1-128k` | 标准版，支持 128K 上下文 | 128K |
| `moonshot-v1-auto` | 自动选择上下文长度 | 自动 |
| `moonshot-v1-8k-vision-preview` | 视觉模型预览版，8K 上下文 | 8K |
| `moonshot-v1-32k-vision-preview` | 视觉模型预览版，32K 上下文 | 32K |
| `moonshot-v1-128k-vision-preview` | 视觉模型预览版，128K 上下文 | 128K |

---

## 1. Chat Completion API

### 接口地址

```
POST https://api.moonshot.cn/v1/chat/completions
```

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|-----|------|------|------|
| `model` | string | 是 | Model ID，如 `kimi-k2-turbo-preview` |
| `messages` | List[Dict] | 是 | 对话消息列表 |
| `max_completion_tokens` | int | 否 | 最大生成 token 数，默认 4096 |
| `temperature` | float | 否 | 温度，控制随机性，默认 0.6（kimi-k2.5 不支持修改） |
| `top_p` | float | 否 | 核采样，默认 1.0（kimi-k2.5 不支持修改） |
| `n` | int | 否 | 生成回复数量，默认 1 |
| `presence_penalty` | float | 否 | 存在惩罚，默认 0 |
| `frequency_penalty` | float | 否 | 频率惩罚，默认 0 |
| `response_format` | object | 否 | 响应格式，可选 `json_object` |
| `stop` | string/array | 否 | 停止词 |
| `thinking` | boolean | 否 | 是否启用思考模式 |
| `stream` | boolean | 否 | 是否流式输出，默认 false |
| `stream_options` | object | 否 | 流式选项，`{"include_usage": true}` |
| `prompt_cache_key` | string | 否 | Prompt 缓存键 |
| `safety_identifier` | string | 否 | 安全标识符 |
| `tools` | List[Dict] | 否 | 工具列表，用于 Function Calling |

### Messages 结构

`content` 字段支持两种格式：

1. **字符串格式**（纯文本对话）:
```json
{
  "role": "user",
  "content": "你好，请问 1+1 等于多少？"
}
```

2. **列表格式**（多模态输入）:
```json
{
  "role": "user",
  "content": [
    {
      "type": "text",
      "text": "描述这张图片"
    },
    {
      "type": "image_url",
      "image_url": {
        "url": "https://example.com/image.jpg"
      }
    },
    {
      "type": "video_url",
      "video_url": {
        "url": "https://example.com/video.mp4"
      }
    }
  ]
}
```

支持的 `role` 类型: `system`, `user`, `assistant`

### 返回内容（非流式）

```json
{
  "id": "chatcmpl-xxx",
  "object": "chat.completion",
  "created": 1700000000,
  "model": "kimi-k2-turbo-preview",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "1+1 等于 2"
      },
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 20,
    "completion_tokens": 10,
    "total_tokens": 30
  }
}
```

### 返回内容（流式）

流式输出会返回 SSE 格式数据，每个 chunk 包含部分回复内容：

```
data: {"id":"chatcmpl-xxx","object":"chat.completion.chunk","created":1700000000,"model":"kimi-k2-turbo-preview","choices":[{"index":0,"delta":{"role":"assistant"},"finish_reason":null}]}

data: {"id":"chatcmpl-xxx","object":"chat.completion.chunk","created":1700000000,"model":"kimi-k2-turbo-preview","choices":[{"index":0,"delta":{"content":"1"},"finish_reason":null}]}

data: {"id":"chatcmpl-xxx","object":"chat.completion.chunk","created":1700000000,"model":"kimi-k2-turbo-preview","choices":[{"index":0,"delta":{"content":"+"},"finish_reason":null}]}

data: {"id":"chatcmpl-xxx","object":"chat.completion.chunk","created":1700000000,"model":"kimi-k2-turbo-preview","choices":[{"index":0,"delta":{"content":"1"},"finish_reason":null}]}

data: {"id":"chatcmpl-xxx","object":"chat.completion.chunk","created":1700000000,"model":"kimi-k2-turbo-preview","choices":[{"index":0,"delta":{"content":" 等于 2"},"finish_reason":"stop"}]}

data: [DONE]
```

### 调用示例

#### Python

```python
from openai import OpenAI

client = OpenAI(
    api_key="$MOONSHOT_API_KEY",
    base_url="https://api.moonshot.cn/v1",
)

completion = client.chat.completions.create(
    model="kimi-k2-turbo-preview",
    messages=[
        {"role": "system", "content": "你是 Kimi，由 Moonshot AI 提供的人工智能助手。"},
        {"role": "user", "content": "你好，请问 1+1 等于多少？"}
    ],
    temperature=0.6,
)

print(completion.choices[0].message.content)
```

#### cURL

```bash
curl https://api.moonshot.cn/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $MOONSHOT_API_KEY" \
  -d '{
    "model": "kimi-k2-turbo-preview",
    "messages": [
      {"role": "system", "content": "你是 Kimi，由 Moonshot AI 提供的人工智能助手。"},
      {"role": "user", "content": "你好，请问 1+1 等于多少？"}
    ],
    "temperature": 0.6
  }'
```

#### Node.js

```javascript
import OpenAI from 'openai';

const client = new OpenAI({
  apiKey: process.env.MOONSHOT_API_KEY,
  baseURL: 'https://api.moonshot.cn/v1',
});

async function main() {
  const completion = await client.chat.completions.create({
    model: 'kimi-k2-turbo-preview',
    messages: [
      { role: 'system', content: '你是 Kimi，由 Moonshot AI 提供的人工智能助手。' },
      { role: 'user', content: '你好，请问 1+1 等于多少？' }
    ],
    temperature: 0.6,
  });
  
  console.log(completion.choices[0].message.content);
}

main();
```

### Vision 视觉模型

使用视觉模型时，需要使用 `vision-preview` 系列模型，并通过 `content` 列表格式传入图片：

```python
from openai import OpenAI
import base64

client = OpenAI(
    api_key="$MOONSHOT_API_KEY",
    base_url="https://api.moonshot.cn/v1",
)

# 读取本地图片并编码为 base64
with open("image.jpg", "rb") as f:
    image_data = base64.b64encode(f.read()).decode('utf-8')

completion = client.chat.completions.create(
    model="moonshot-v1-8k-vision-preview",
    messages=[
        {
            "role": "user",
            "content": [
                {"type": "text", "text": "描述这张图片的内容"},
                {
                    "type": "image_url",
                    "image_url": {
                        "url": f"data:image/jpeg;base64,{image_data}"
                    }
                }
            ]
        }
    ],
)

print(completion.choices[0].message.content)
```

### List Models 获取模型列表

```
GET https://api.moonshot.cn/v1/models
```

返回当前可用的所有模型列表。

---

## 2. Tool Use API

Tool Use（Function Calling）是 Kimi 大模型的重要功能，可以让模型智能地选择调用外部工具。

### 基本用法

在请求中通过 `tools` 字段定义可用工具：

```json
{
  "model": "kimi-k2-turbo-preview",
  "messages": [
    {"role": "user", "content": "编程判断 3214567 是否是素数。"}
  ],
  "tools": [
    {
      "type": "function",
      "function": {
        "name": "CodeRunner",
        "description": "代码执行器，支持运行 python 和 javascript 代码",
        "parameters": {
          "properties": {
            "language": {
              "type": "string",
              "enum": ["python", "javascript"]
            },
            "code": {
              "type": "string",
              "description": "代码写在这里"
            }
          },
          "type": "object"
        }
      }
    }
  ]
}
```

### 工具配置规范

- `name`: 工具名称，需符合正则表达式 `^[a-zA-Z_][a-zA-Z0-9-_]{0,63}$`
- `description`: 工具功能描述，帮助模型判断是否调用
- `parameters`: 参数定义，使用 JSON Schema 子集
- 最多支持 128 个工具

### 调用示例

```python
from openai import OpenAI

client = OpenAI(
    api_key="$MOONSHOT_API_KEY",
    base_url="https://api.moonshot.cn/v1",
)

completion = client.chat.completions.create(
    model="kimi-k2-turbo-preview",
    messages=[
        {"role": "system", "content": "你是 Kimi，由 Moonshot AI 提供的人工智能助手。"},
        {"role": "user", "content": "编程判断 3214567 是否是素数。"}
    ],
    tools=[{
        "type": "function",
        "function": {
            "name": "CodeRunner",
            "description": "代码执行器，支持运行 python 和 javascript 代码",
            "parameters": {
                "properties": {
                    "language": {
                        "type": "string",
                        "enum": ["python", "javascript"]
                    },
                    "code": {
                        "type": "string",
                        "description": "代码写在这里"
                    }
                },
                "type": "object"
            }
        }
    }],
    temperature=0.6,
)

print(completion.choices[0].message)
```

模型可能会返回包含 `tool_calls` 的消息，你需要执行相应工具并将结果回传给模型：

```python
# 检查是否需要调用工具
if completion.choices[0].message.tool_calls:
    tool_call = completion.choices[0].message.tool_calls[0]
    function_name = tool_call.function.name
    arguments = json.loads(tool_call.function.arguments)
    
    # 执行工具调用
    result = run_tool(function_name, arguments)
    
    # 将结果回传给模型
    messages.append(completion.choices[0].message)
    messages.append({
        "role": "tool",
        "tool_call_id": tool_call.id,
        "content": str(result)
    })
    
    # 再次调用获取最终回复
    completion = client.chat.completions.create(
        model="kimi-k2-turbo-preview",
        messages=messages,
        tools=tools,
    )
```

### 推荐的 Agent 平台

- [Coze](https://coze.cn/)
- [Bisheng](https://github.com/dataelement/bisheng)
- [Dify](https://github.com/langgenius/dify/)
- [LangChain](https://github.com/langchain-ai/langchain)

---

## 3. Partial Mode API

Partial Mode（部分模式）可以通过预填模型回复来引导输出格式和内容，实现更可控的生成。

### 使用方法

在最后一个 `role` 为 `assistant` 的 message 中，增加 `"partial": True`：

```json
{
  "role": "assistant",
  "content": "预填的回复开头",
  "partial": true
}
```

**注意**：请勿混用 partial mode 和 `response_format=json_object`，否则可能获得预期外的回复。

### JSON Mode 示例

使用 Partial Mode 实现 JSON 格式输出：

```python
from openai import OpenAI

client = OpenAI(
    api_key="$MOONSHOT_API_KEY",
    base_url="https://api.moonshot.cn/v1",
)

completion = client.chat.completions.create(
    model="kimi-k2-turbo-preview",
    messages=[
        {
            "role": "system",
            "content": "请从产品描述中提取名称、尺寸、价格和颜色，并在一个 JSON 对象中输出。"
        },
        {
            "role": "user",
            "content": "大米 SmartHome Mini 是一款小巧的智能家居助手，有黑色和银色两种颜色，售价为 998 元，尺寸为 256 x 128 x 128mm。"
        },
        {
            "role": "assistant",
            "content": "{",
            "partial": True
        }
    ],
    temperature=0.6,
)

# 手动拼接预填内容和返回内容
full_response = '{' + completion.choices[0].message.content
print(full_response)
# 输出: {"name": "SmartHome Mini", "size": "256 x 128 x 128mm", "price": "998元", "colors": ["黑色", "银色"]}
```

### 角色扮演示例

使用 Partial Mode 提高角色扮演的一致性：

```python
from openai import OpenAI

client = OpenAI(
    api_key="$MOONSHOT_API_KEY",
    base_url="https://api.moonshot.cn/v1",
)

completion = client.chat.completions.create(
    model="kimi-k2-turbo-preview",
    messages=[
        {
            "role": "system",
            "content": "下面你扮演凯尔希，请用凯尔希的语气和我对话..."
        },
        {
            "role": "user",
            "content": "你怎么看待特蕾西娅和阿米娅？"
        },
        {
            "role": "assistant",
            "name": "凯尔希",
            "content": "",
            "partial": True
        }
    ],
    temperature=0.6,
    max_tokens=65536,
)

print(completion.choices[0].message.content)
```

### 保持角色一致性的技巧

1. **提供清晰的角色描述**：详细介绍角色的个性、背景、特征
2. **增加角色细节**：包括说话语气、风格、背景故事
3. **指导特定情况下的行为**：在提示中提供明确的行动指南
4. **定期强化角色设定**：长对话中定期使用 prompt 强化角色

---

## 4. 文件接口

### 限制说明

- 单个用户最多上传 1000 个文件
- 单文件大小不超过 100MB
- 所有已上传文件总和不超过 10GB
- 文件解析服务限时免费，高峰期可能有限流

### 4.1 上传文件

```
POST https://api.moonshot.cn/v1/files
```

#### 支持的文件格式

文档：`.pdf`, `.txt`, `.csv`, `.doc`, `.docx`, `.xls`, `.xlsx`, `.ppt`, `.pptx`, `.md`, `.epub`, `.html`, `.json`, `.mobi`, `.log`

图片：`.jpeg`, `.png`, `.bmp`, `.gif`, `.svg`, `.webp`, `.ico`, `.tif`, `.avif`, `.apng`

代码文件：`.go`, `.h`, `.c`, `.cpp`, `.cs`, `.java`, `.js`, `.css`, `.php`, `.py`, `.ts`, `.tsx`, `.yaml`, `.yml`, `.ini`, `.conf`

#### Python 调用示例

```python
from pathlib import Path
from openai import OpenAI

client = OpenAI(
    api_key="$MOONSHOT_API_KEY",
    base_url="https://api.moonshot.cn/v1",
)

# 上传文件用于内容抽取
file_object = client.files.create(
    file=Path("document.pdf"),
    purpose="file-extract"
)

# 上传图片用于视觉理解
image_object = client.files.create(
    file=Path("image.jpg"),
    purpose="image"
)

# 上传视频
video_object = client.files.create(
    file=Path("video.mp4"),
    purpose="video"
)
```

#### Purpose 参数说明

| 值 | 说明 |
|----|------|
| `file-extract` | 用于文件内容抽取 |
| `image` | 用于图片视觉理解 |
| `video` | 用于视频理解 |

### 4.2 文件内容抽取使用示例

```python
from pathlib import Path
from openai import OpenAI

client = OpenAI(
    api_key="$MOONSHOT_API_KEY",
    base_url="https://api.moonshot.cn/v1",
)

# 1. 上传文件
file_object = client.files.create(
    file=Path("xlnet.pdf"),
    purpose="file-extract"
)

# 2. 获取文件内容
file_content = client.files.content(file_id=file_object.id).text

# 3. 将内容加入对话上下文
messages = [
    {
        "role": "system",
        "content": "你是 Kimi，由 Moonshot AI 提供的人工智能助手。"
    },
    {
        "role": "system",
        "content": file_content,  # 文件内容作为 system message
    },
    {"role": "user", "content": "请简单介绍 xlnet.pdf 讲了什么"}
]

# 4. 调用 Chat API
completion = client.chat.completions.create(
    model="kimi-k2-turbo-preview",
    messages=messages,
    temperature=0.6,
)

print(completion.choices[0].message.content)
```

### 4.3 多文件对话示例

```python
from typing import List, Dict, Any
import os
from pathlib import Path
from openai import OpenAI

client = OpenAI(
    base_url="https://api.moonshot.cn/v1",
    api_key=os.environ["MOONSHOT_API_KEY"],
)

def upload_files(files: List[str]) -> List[Dict[str, Any]]:
    """
    上传多个文件并生成文件 messages
    """
    messages = []
    for file in files:
        file_object = client.files.create(
            file=Path(file),
            purpose="file-extract"
        )
        file_content = client.files.content(file_id=file_object.id).text
        messages.append({
            "role": "system",
            "content": file_content,
        })
    return messages

# 使用示例
file_messages = upload_files(files=["file1.pdf", "file2.docx"])

messages = [
    *file_messages,  # 展开文件 messages
    {
        "role": "system",
        "content": "你是 Kimi，由 Moonshot AI 提供的人工智能助手。"
    },
    {
        "role": "user",
        "content": "总结一下这些文件的内容。"
    }
]

completion = client.chat.completions.create(
    model="kimi-k2-turbo-preview",
    messages=messages,
)

print(completion.choices[0].message.content)
```

### 4.4 列出文件

```
GET https://api.moonshot.cn/v1/files
```

```python
file_list = client.files.list()
for file in file_list.data:
    print(file)
```

### 4.5 删除文件

```
DELETE https://api.moonshot.cn/v1/files/{file_id}
```

```python
client.files.delete(file_id=file_id)
```

### 4.6 获取文件信息

```
GET https://api.moonshot.cn/v1/files/{file_id}
```

```python
file_info = client.files.retrieve(file_id=file_id)
# 返回 FileObject，包含 id, bytes, created_at, filename, purpose, status 等
```

### 4.7 获取文件内容

```
GET https://api.moonshot.cn/v1/files/{file_id}/content
```

```python
file_content = client.files.content(file_id=file_object.id).text
```

---

## 5. 计算 Token

该接口用于计算请求的 token 数量，包括纯文本输入和视觉输入。

### 接口地址

```
POST https://api.moonshot.cn/v1/tokenizers/estimate-token-count
```

### 请求参数

请求结构与 Chat Completion 基本一致：

| 字段 | 类型 | 必填 | 说明 |
|-----|------|------|------|
| `model` | string | 是 | Model ID |
| `messages` | List[Dict] | 是 | 对话消息列表 |

### 调用示例

#### 纯文本调用

```bash
curl 'https://api.moonshot.cn/v1/tokenizers/estimate-token-count' \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $MOONSHOT_API_KEY" \
  -d '{
    "model": "kimi-k2-turbo-preview",
    "messages": [
      {
        "role": "system",
        "content": "你是 Kimi，由 Moonshot AI 提供的人工智能助手。"
      },
      {
        "role": "user",
        "content": "你好，我叫李雷，1+1等于多少？"
      }
    ]
  }'
```

#### 包含视觉的调用

```python
import os
import base64
import json
import requests

api_key = os.environ.get("MOONSHOT_API_KEY")
endpoint = "https://api.moonshot.cn/v1/tokenizers/estimate-token-count"

image_path = "image.png"
with open(image_path, "rb") as f:
    image_data = f.read()

# 将图片编码为 base64
image_url = f"data:image/{os.path.splitext(image_path)[1]};base64,{base64.b64encode(image_data).decode('utf-8')}"

payload = {
    "model": "kimi-k2.5",
    "messages": [
        {
            "role": "system",
            "content": "你是 Kimi，由 Moonshot AI 提供的人工智能助手。"
        },
        {
            "role": "user",
            "content": [
                {
                    "type": "image_url",
                    "image_url": {"url": image_url}
                },
                {
                    "type": "text",
                    "text": "请描述图片的内容。"
                }
            ]
        }
    ]
}

response = requests.post(
    endpoint,
    headers={
        "Authorization": f"Bearer {api_key}",
        "Content-Type": "application/json"
    },
    data=json.dumps(payload)
)

print(response.json())
```

### 返回内容

```json
{
  "data": {
    "total_tokens": 80
  }
}
```

当没有 `error` 字段时，取 `data.total_tokens` 作为计算结果。

---

## 6. 查询余额

### 接口地址

```
GET https://api.moonshot.cn/v1/users/me/balance
```

### 调用示例

```bash
curl https://api.moonshot.cn/v1/users/me/balance \
  -H "Authorization: Bearer $MOONSHOT_API_KEY"
```

### 返回内容

```json
{
  "code": 0,
  "data": {
    "available_balance": 49.58894,
    "voucher_balance": 46.58893,
    "cash_balance": 3.00001
  },
  "scode": "0x0",
  "status": true
}
```

### 字段说明

| 字段 | 说明 | 类型 | 单位 |
|-----|------|------|------|
| `available_balance` | 可用余额（现金+代金券），≤0 时不可调用 API | float | 人民币元（CNY） |
| `voucher_balance` | 代金券余额，不会为负数 | float | 人民币元（CNY） |
| `cash_balance` | 现金余额，可能为负数（欠费） | float | 人民币元（CNY） |

**注意**：当 `cash_balance` 为负数时，`available_balance` 等于 `voucher_balance` 的值。

---

## 7. 错误码说明

### HTTP 状态码

| 状态码 | 错误类型 | 说明 |
|-------|---------|------|
| `400` | `content_filter` | 内容被安全过滤器拦截 |
| `400` | `invalid_request_error` | 请求参数错误 |
| `401` | `authentication_error` | 认证失败，API Key 无效或过期 |
| `403` | `permission_error` | 权限不足 |
| `404` | `not_found` | 资源不存在 |
| `429` | `rate_limit_error` | 请求频率超限或余额不足 |
| `500` | `server_error` | 服务器内部错误 |

### 错误响应格式

```json
{
  "error": {
    "code": "invalid_request_error",
    "message": "Invalid model: xxx",
    "type": "invalid_request_error"
  }
}
```

### 常见错误处理建议

| 错误 | 建议处理方式 |
|-----|-------------|
| `401` | 检查 API Key 是否正确设置 |
| `429` | 降低请求频率，或检查余额是否充足 |
| `400` content_filter | 修改输入内容，避免敏感信息 |
| `500` | 稍后重试，如持续出现请联系客服 |

### 参数限制说明

| 模型 | 特殊限制 |
|-----|---------|
| `kimi-k2.5` | 不支持修改 `temperature` 和 `top_p` |
| `kimi-k2-thinking` | 思考模型，输出包含推理过程 |
| Vision 模型 | 图片大小建议不超过 5MB |

---

## 附录：完整参数默认值

| 参数 | 默认值 | 取值范围 |
|-----|-------|---------|
| `temperature` | 0.6 | 0-2（kimi-k2.5 固定） |
| `top_p` | 1.0 | 0-1（kimi-k2.5 固定） |
| `n` | 1 | 1-10 |
| `max_completion_tokens` | 4096 | 1-模型最大长度 |
| `presence_penalty` | 0 | -2.0 ~ 2.0 |
| `frequency_penalty` | 0 | -2.0 ~ 2.0 |
| `stream` | false | true/false |

---

*文档生成时间: 2025年*
*官方文档: https://platform.moonshot.cn/docs*
