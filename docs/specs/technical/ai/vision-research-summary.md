# 图片识别功能技术调研总结

## 问题 1: Vision 模型的 Tool Calling 能力

### 调研结论

根据行业调研，主流 AI 提供商（OpenAI、Anthropic、Google）都使用**统一模型**同时处理 Vision 和 Tool Calling，无需分离。

| 提供商 | 模型 | Vision + Tool Calling |
|-------|------|---------------------|
| OpenAI | gpt-4o | ✅ 同一模型，官方推荐 |
| Anthropic | Claude 3.5 Sonnet | ✅ 同一模型，无限制 |
| Google | Gemini 3 Pro/Flash | ✅ 同一模型，支持并行调用 |
| Kimi | moonshot-v1-8k-vision-preview | ❓ 待测试验证 |

### 关键发现

1. **行业标准**: 所有主流提供商都支持在同一请求中组合 Vision 和 Tool Calling
2. **无性能损失**: 没有文档表明 Vision 模型的 Tool Calling 能力弱于纯文本模型
3. **架构优势**: 统一模型能更好理解视觉上下文并做出工具调用决策

### Kimi 特殊情况

**文档证据**:
- ✅ Kimi API 文档明确支持 `tools` 参数（第 56 行）
- ✅ Vision 模型文档有完整示例（207-243 行）
- ❓ **但没有明确示例展示 Vision + Tool Calling 组合使用**

**需要验证**:
```rust
// 已创建测试文件: src-tauri/tests/test_kimi_vision_tools.rs
// 运行测试: cargo test --test test_kimi_vision_tools -- --ignored

// 测试场景:
// 1. Vision 模型 + Tool Calling 是否返回 tool_calls
// 2. 对比纯文本模型的 Tool Calling 表现
```

### 推荐方案

**方案 A: 单模型方案（推荐优先尝试）**
```rust
moonshot-v1-8k-vision-preview + tools
    ↓
一次 API 调用完成 Vision + Tool Calling
```
- ✅ 架构简单，延迟低
- ✅ 成本低（单次调用）
- ✅ 上下文统一
- ⚠️  需要测试验证效果

**方案 B: 双模型方案（备选）**
```rust
Step 1: moonshot-v1-8k-vision-preview (纯描述)
    ↓ 提取图片中的文本信息
Step 2: kimi-k2-turbo-preview + tools
    ↓ 基于文本调用工具
```
- ✅ Tool Calling 准确率有保证
- ❌ 两次 API 调用，成本翻倍
- ❌ 延迟增加（2-5秒 × 2）
- ❌ 可能丢失视觉上下文

### 实施策略

```
1. 实现方案 A
   ├─ 使用 Vision 模型 + Tool Calling
   └─ 运行测试用例验证

2. 测试评估
   ├─ 准确率测试（10+ 张不同类型图片）
   ├─ Tool Call 成功率
   └─ 参数提取质量

3. 根据测试结果决策
   ├─ 成功率 > 85% → 采用方案 A
   └─ 成功率 < 85% → 切换方案 B

4. 架构设计支持切换
   └─ 通过配置项切换 Vision-only 或 Vision+Text 模式
```

## 问题 2: 多操作支持架构设计

### 核心设计

#### 2.1 操作类型枚举（可扩展）

```rust
pub enum TaskOperation {
    Create { ... },      // ✅ Phase 1 - 当前实现
    Update { ... },      // 🔵 Phase 2 - 未来扩展
    Complete { ... },    // 🔵 Phase 2
    Delete { ... },      // 🔵 Phase 2
    SetStatus { ... },   // 🔵 Phase 2
    BatchComplete { ... }, // 🔵 Phase 3
}
```

#### 2.2 Tool Registry（统一管理）

```rust
pub struct TaskToolRegistry;

impl TaskToolRegistry {
    // 获取所有工具
    pub fn all_tools() -> Vec<Value>;

    // 获取基础工具（仅创建）
    pub fn basic_tools() -> Vec<Value>;

    // 获取管理工具（更新、完成、删除）
    pub fn management_tools() -> Vec<Value>;
}

// 使用示例:
let tools = TaskToolRegistry::all_tools();  // 6 个工具
let tools = TaskToolRegistry::basic_tools(); // 仅 create_task
```

**优势**:
- ✅ 集中管理所有工具定义
- ✅ 添加新工具只需在一处修改
- ✅ 支持按场景选择工具集
- ✅ 易于测试和维护

#### 2.3 解析器（类型安全）

```rust
pub struct ToolCallParser;

impl ToolCallParser {
    // 解析 API 返回的 tool_calls
    pub fn parse_tool_calls(tool_calls: &[Value]) -> Result<Vec<TaskOperation>>;

    // 每个工具有独立的解析函数
    fn parse_create_task(args: &Value) -> Result<TaskOperation>;
    fn parse_update_task(args: &Value) -> Result<TaskOperation>;
    // ...
}
```

**优势**:
- ✅ 类型安全的转换
- ✅ 统一的错误处理
- ✅ 易于添加新的解析逻辑

#### 2.4 任务标识符匹配

```rust
// 智能匹配任务
pub async fn find_task_by_identifier(
    db: &Database,
    identifier: &str,
) -> Result<Option<Task>> {
    // 1. 精确匹配 ID
    // 2. 精确匹配标题
    // 3. 模糊匹配（包含关键词）
    // 4. Fuzzy search (可选)
}
```

### 扩展路径

#### Phase 1: 基础实现（当前）
- [x] `TaskOperation` 枚举定义
- [x] `TaskToolRegistry` 实现
- [x] `create_task` 工具完整流程
- [ ] **测试 Vision + Tool Calling**

#### Phase 2: 管理功能（下一步）
- [ ] `update_task` 工具
- [ ] `complete_task` 工具
- [ ] `delete_task` 工具
- [ ] 任务标识符匹配逻辑
- [ ] 前端多操作确认对话框

#### Phase 3: 高级功能（未来）
- [ ] 批量操作
- [ ] 操作历史记录
- [ ] 撤销/重做
- [ ] 智能建议

### 前端集成

```typescript
// 解析结果包含多个操作
interface ImageParseResult {
  operations: TaskOperation[];  // 可能包含多个操作
  confidence: number;
  warnings: string[];
}

// 用户可以选择执行哪些操作
<TaskOperationsConfirmDialog
  operations={parseResult.operations}
  onConfirm={(selectedOps) => executeOperations(selectedOps)}
/>
```

### 使用场景示例

#### 场景 1: 待办列表截图
```
图片内容:
✓ 买菜（已完成）
✗ 完成报告（周五前，高优先级）
✗ 预约牙医

识别结果:
Operation 1: Complete { task_identifier: "买菜" }
Operation 2: Create { title: "完成报告", priority: "high", deadline: "..." }
Operation 3: Create { title: "预约牙医" }
```

#### 场景 2: 任务更新便签
```
图片内容:
"完成报告" 任务改为高优先级，截止时间改到明天

识别结果:
Operation 1: Update {
  task_identifier: "完成报告",
  priority: Some("high"),
  deadline: Some("2024-03-13T23:59:59+08:00")
}
```

## 总结与行动计划

### 立即行动

1. **运行测试验证 Kimi Vision + Tool Calling**
   ```bash
   cd src-tauri
   cargo test --test test_kimi_vision_tools -- --ignored --nocapture
   ```

2. **实现基础架构**
   - 创建 `src-tauri/src/ai/task_operations.rs` 文件
   - 实现 `TaskOperation` 枚举
   - 实现 `TaskToolRegistry`
   - 实现 `ToolCallParser`

3. **更新 `parse_image_input`**
   - 从返回 `ParsedTask` 改为返回 `ImageParseResult`
   - 支持多操作解析

### 后续规划

**Week 1-2**: 基础功能
- 完成单模型方案实现
- 测试 Vision + Tool Calling 效果
- 如果效果不佳，实现双模型备选方案

**Week 3-4**: 扩展功能
- 实现 Update/Complete/Delete 工具
- 实现任务匹配逻辑
- 前端多操作确认对话框

**Week 5+**: 优化与高级功能
- 批量操作
- 性能优化
- 用户体验改进

## 参考文档

1. `docs/vision-tool-use-design.md` - 原始技术方案
2. `docs/task-operations-architecture.md` - 架构设计详细文档
3. `src-tauri/tests/test_kimi_vision_tools.rs` - 测试用例
4. `docs/kimi-api.md` - Kimi API 完整文档

---

**文档版本**: v1.0
**最后更新**: 2024-03-12
**状态**: 待测试验证
