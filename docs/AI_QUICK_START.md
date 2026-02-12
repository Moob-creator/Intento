# AI 模块快速开始指南

## 5 分钟快速开始

### 1. 配置 API Key (1 分钟)

```bash
# 复制配置模板
cp .env.example .env

# 编辑 .env 文件，添加你的 API Key
echo "OPENAI_API_KEY=sk-your-actual-key-here" > .env

# 或使用 Anthropic
echo "ANTHROPIC_API_KEY=sk-ant-your-key-here" > .env
echo "AI_PROVIDER=anthropic" >> .env
```

### 2. 测试 AI 模块 (2 分钟)

```bash
cd src-tauri

# 运行单元测试（不需要 API key）
cargo test

# 运行集成测试（需要 API key）
export $(cat ../.env | xargs)  # 加载环境变量
cargo test -- --ignored
```

### 3. 构建并运行 (2 分钟)

```bash
# 回到项目根目录
cd ..

# 安装依赖（如果还没安装）
npm install

# 启动开发服务器
npm run tauri dev
```

### 4. 测试前端调用

在浏览器开发者工具中运行：

```javascript
// 测试 AI 解析
const task = await window.__TAURI_INTERNALS__.invoke('parse_text_input', {
  text: '明天下午3点开会讨论项目进度，高优先级'
});
console.log('解析结果:', task);

// 测试健康检查
const healthy = await window.__TAURI_INTERNALS__.invoke('ai_health_check');
console.log('AI 服务状态:', healthy);

// 获取 provider
const provider = await window.__TAURI_INTERNALS__.invoke('get_ai_provider');
console.log('当前 Provider:', provider);
```

## 常见问题

### Q: 遇到 "API key not set" 错误？
A: 确保：
1. `.env` 文件在项目根目录
2. API Key 格式正确（OpenAI: `sk-...`, Anthropic: `sk-ant-...`）
3. 重启开发服务器以加载新的环境变量

### Q: 测试失败？
A:
- 单元测试失败：检查代码是否正确编译
- 集成测试失败：检查 API Key 是否有效，是否有余额

### Q: API 调用很慢？
A:
- 正常情况下 500ms-2s 是正常的
- 如果超过 5s，检查网络连接
- 可以考虑使用国内代理或切换到其他 provider

### Q: 如何切换 OpenAI 和 Claude？
A: 修改 `.env` 文件：
```bash
# 使用 OpenAI
OPENAI_API_KEY=sk-...
# AI_PROVIDER=openai  # 可选，默认就是 openai

# 使用 Claude
ANTHROPIC_API_KEY=sk-ant-...
AI_PROVIDER=anthropic
```

### Q: 如何自定义模型？
A: 在 `.env` 中添加：
```bash
AI_MODEL=gpt-4o  # 或 claude-opus-4 等
```

## 示例代码

### Rust 内部使用

```rust
use crate::ai::{AiClient, ModelProvider};

async fn example() -> anyhow::Result<()> {
    let client = AiClient::new_default()?;
    let parsed = client.parse_text_input("明天开会").await?;
    println!("任务: {}", parsed.title);
    Ok(())
}
```

### 前端 TypeScript 使用

```typescript
import { invoke } from '@tauri-apps/api/core';

interface ParsedTask {
  title: string;
  description?: string;
  deadline?: string;
  priority?: 'low' | 'medium' | 'high';
  tags?: string[];
}

async function parseTask(input: string): Promise<ParsedTask> {
  return await invoke('parse_text_input', { text: input });
}

// 使用
const task = await parseTask('明天下午3点发布新版本，紧急');
console.log(task);
// {
//   title: "发布新版本",
//   deadline: "2024-12-01T15:00:00Z",
//   priority: "high",
//   tags: ["release", "urgent"]
// }
```

## 解析示例

| 输入 | 输出 |
|-----|------|
| "明天开会" | `{title: "开会", deadline: "2024-12-01T00:00:00Z"}` |
| "紧急：修复 bug" | `{title: "修复 bug", priority: "high"}` |
| "下周一下午3点提交报告" | `{title: "提交报告", deadline: "2024-12-04T15:00:00Z"}` |
| "购买食材和做饭" | `{title: "购买食材和做饭", tags: ["shopping", "cooking"]}` |

## 下一步

- 📖 阅读 [完整文档](./AI_MODULE_GUIDE.md)
- 🔍 查看 [示例代码](../src-tauri/src/ai/examples.rs)
- 🚀 开始 [前端集成](./TASK_3_2_FRONTEND_INTEGRATION.md) (即将到来)

## 获取帮助

如果遇到问题：
1. 查看 [故障排除](./AI_MODULE_GUIDE.md#troubleshooting)
2. 检查 [常见错误](./TASK_3_1_COMPLETION_REPORT.md#troubleshooting)
3. 提交 Issue

---

**提示**: 第一次使用建议先运行单元测试熟悉 API，然后再配置 API Key 进行集成测试。
