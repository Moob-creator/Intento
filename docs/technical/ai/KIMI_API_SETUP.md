# Kimi API 配置指南

Kimi (Moonshot AI) 是国内优秀的大语言模型服务，API 与 OpenAI 兼容，访问更稳定。

## 1. 获取 API Key

1. 访问 Kimi 开放平台：https://platform.moonshot.cn/
2. 注册并登录账号
3. 进入控制台：https://platform.moonshot.cn/console/api-keys
4. 点击"创建新的 API Key"
5. 复制生成的 API Key（格式：`sk-xxxxxxxxxxxxxxxx`）

## 2. 配置项目

### 方法一：使用 .env 文件（推荐）

1. 复制环境变量模板：
```bash
cp .env.example .env
```

2. 编辑 `.env` 文件，取消注释并填入你的 API Key：
```bash
# 设置为使用 Kimi
AI_PROVIDER=kimi

# 填入你的 Kimi API Key
MOONSHOT_API_KEY=sk-your-moonshot-api-key-here

# 可选：指定模型（默认 moonshot-v1-8k）
AI_MODEL=moonshot-v1-8k
```

### 方法二：环境变量

在终端中设置环境变量：
```bash
# macOS/Linux
export AI_PROVIDER=kimi
export MOONSHOT_API_KEY=sk-your-moonshot-api-key-here

# Windows PowerShell
$env:AI_PROVIDER="kimi"
$env:MOONSHOT_API_KEY="sk-your-moonshot-api-key-here"
```

## 3. 可用模型

Kimi 提供三种模型，根据需求选择：

| 模型名称 | 上下文长度 | 适用场景 |
|---------|-----------|---------|
| `moonshot-v1-8k` | 8,000 tokens | 日常对话、简单任务解析 |
| `moonshot-v1-32k` | 32,000 tokens | 较长文本处理 |
| `moonshot-v1-128k` | 128,000 tokens | 超长文本、复杂任务 |

**推荐**：对于任务解析功能，`moonshot-v1-8k` 足够使用。

## 4. API 端点

Kimi API 兼容 OpenAI API 格式：
- **Base URL**: `https://api.moonshot.cn/v1`
- **Chat Completions**: `POST /chat/completions`

项目已自动配置该端点，无需手动修改。

## 5. 测试配置

启动应用后：
```bash
npm run tauri dev
```

1. 点击 "AI Add Task" 按钮
2. 输入任务描述，例如："明天下午3点开会讨论项目进度"
3. 如果配置正确，AI 会解析并显示确认对话框

## 6. 常见问题

### Q: API Key 无效
**A**: 检查以下几点：
- API Key 是否正确复制（包括 `sk-` 前缀）
- 是否已保存 `.env` 文件
- 是否重启了应用

### Q: 请求失败
**A**: 可能原因：
- 网络连接问题
- API Key 余额不足
- 请求频率过高

查看控制台日志获取详细错误信息。

### Q: 如何切换回 OpenAI/Claude？
**A**: 修改 `.env` 文件：
```bash
# 使用 OpenAI
AI_PROVIDER=openai
OPENAI_API_KEY=sk-your-openai-key

# 或使用 Claude
AI_PROVIDER=anthropic
ANTHROPIC_API_KEY=sk-ant-your-anthropic-key
```

## 7. 价格

Kimi API 采用按需计费，价格透明：
- 查看最新价格：https://platform.moonshot.cn/docs/pricing

首次注册通常会有免费额度，足够测试使用。

## 8. 参考资源

- 官方文档：https://platform.moonshot.cn/docs
- API 参考：https://platform.moonshot.cn/docs/api/chat
- 控制台：https://platform.moonshot.cn/console

---

**配置完成！** 现在你可以用自然语言创建任务了。🎉
