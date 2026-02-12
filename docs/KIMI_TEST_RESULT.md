# Kimi API 连接测试结果

## 测试时间
2026-02-09

## 测试结果
❌ **API Key 无效**

## 错误信息
```json
{
  "error": {
    "message": "Invalid Authentication",
    "type": "invalid_authentication_error"
  }
}
```

## 测试的 API Key
```
sk-kimi-o7TWwjNceg6enIUtGqCkbt2XaD2AhJgdxH9UPW9LmFkiHqkjlNJZWDKYyhHSJTme
```

## 可能的原因

1. **API Key 已过期**
   - Kimi API Key 可能有有效期限制
   - 需要在控制台重新生成

2. **API Key 格式错误**
   - 复制时可能遗漏或多了字符
   - 请从控制台重新复制完整的 Key

3. **权限不足**
   - API Key 可能被撤销或禁用
   - 余额不足导致无法调用

4. **账号问题**
   - 账号可能被暂停或限制

## 解决方案

### 方法 1：重新获取 API Key

1. 访问 Kimi 控制台：https://platform.moonshot.cn/console/api-keys
2. 点击"创建新的 API Key"
3. 复制完整的 API Key（包括 `sk-` 前缀）
4. 更新 `.env` 文件：
   ```bash
   MOONSHOT_API_KEY=sk-your-new-api-key-here
   ```

### 方法 2：检查余额

1. 登录 Kimi 控制台
2. 查看账户余额
3. 如果余额不足，需要充值

### 方法 3：验证 API Key

使用以下命令测试 API Key 是否有效：

```bash
curl https://api.moonshot.cn/v1/users/me/balance \
  -H "Authorization: Bearer YOUR_API_KEY_HERE"
```

**成功响应示例**：
```json
{
  "code": 0,
  "data": {
    "available_balance": 49.58894,
    "voucher_balance": 46.58893,
    "cash_balance": 3.00001
  }
}
```

**失败响应示例**：
```json
{
  "error": {
    "message": "Invalid Authentication",
    "type": "invalid_authentication_error"
  }
}
```

## 测试命令

在获取新的 API Key 后，可以使用以下命令测试：

```bash
# 1. 测试余额接口
curl https://api.moonshot.cn/v1/users/me/balance \
  -H "Authorization: Bearer YOUR_API_KEY"

# 2. 测试 Chat API
curl https://api.moonshot.cn/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "model": "kimi-k2-turbo-preview",
    "messages": [{"role": "user", "content": "Hello"}]
  }'
```

## 项目测试步骤

API Key 验证通过后：

1. 更新 `.env` 文件
2. 运行单元测试：
   ```bash
   cd src-tauri
   cargo test test_kimi_api_hello_world -- --ignored --nocapture
   ```
3. 启动应用测试：
   ```bash
   npm run tauri dev
   ```
4. 点击 "AI Add Task" 按钮测试

## 参考资源

- Kimi 控制台：https://platform.moonshot.cn/console
- API Key 管理：https://platform.moonshot.cn/console/api-keys
- 官方文档：https://platform.moonshot.cn/docs
- 定价信息：https://platform.moonshot.cn/pricing

---

**下一步**: 请获取有效的 Kimi API Key 后重新测试。
