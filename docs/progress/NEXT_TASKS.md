# 下一步开发任务清单

**更新时间:** 2026-02-19
**当前版本:** v0.1.0 MVP

---

## 🎯 本周任务（Week 1）- P0 必须完成

### 任务 1: 完善上下文缓存机制 ⚡

**优先级:** P0（最高）
**预计时间:** 1-2 天
**状态:** 待开始

#### 为什么重要？
这是 PRD v3 核心功能 4.2，直接影响 AI 智能化程度。用户应该可以说"把刚才那个任务改成高优先级"。

#### 具体任务：
- [ ] 实现 `save_context()` 保存对话历史到 `context_cache` 表
- [ ] 实现 `get_recent_context()` 获取最近 20 条上下文
- [ ] 在 AI 解析时自动注入上下文到 prompt
- [ ] 支持指代消解（"刚才那个"、"上一个任务"）
- [ ] 实现自动清理逻辑（保留最近 20 条）

#### 技术方案：
```rust
// src-tauri/src/db/mod.rs
pub fn save_context(&self, input: &str, result: &str) -> Result<i64> {
    let cache = ContextCache {
        cache_key: format!("conv_{}", chrono::Utc::now().timestamp()),
        cache_type: CacheType::Conversation,
        cache_value: json!({
            "input": input,
            "result": result,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }).to_string(),
        expires_at: chrono::Utc::now().timestamp() + 86400 * 7,
    };
    self.set_cache(&cache)
}

pub fn get_recent_context(&self, limit: usize) -> Result<Vec<String>> {
    // 获取最近的对话记录，用于 AI 上下文注入
}
```

#### 验收标准：
- ✅ 用户说"把刚才那个任务改成高优先级"能正确识别
- ✅ 上下文会自动保存每次 AI 交互
- ✅ 过期上下文自动清理

---

### 任务 2: 实现季度和年度总结 📊

**优先级:** P0
**预计时间:** 1-2 天
**状态:** 待开始

#### 为什么重要？
PRD v3 明确要求的功能（4.6.3 和 4.6.4），完成后 MVP 才算完整。

#### 具体任务：
- [ ] 扩展 `SummaryType` 枚举添加 Quarterly 和 Yearly
- [ ] 实现季度总结生成逻辑（Q1/Q2/Q3/Q4）
- [ ] 实现年度总结生成逻辑
- [ ] 更新调度器添加季度/年度任务
- [ ] 前端 SummaryPanel 支持选择季度/年度

#### 技术方案：
```rust
// src-tauri/src/db/models.rs
pub enum SummaryType {
    Daily,
    Weekly,
    Monthly,
    Quarterly,  // 新增
    Yearly,     // 新增
    SemiAnnually,
}

// src-tauri/src/summary/generator.rs
pub async fn generate_quarterly_summary(...) -> Result<Summary>
pub async fn generate_yearly_summary(...) -> Result<Summary>
```

#### 验收标准：
- ✅ 可以生成季度总结（Q1: 1-3月，Q2: 4-6月...）
- ✅ 可以生成年度总结
- ✅ SummaryPanel 显示季度/年度选项
- ✅ Timeline 正确显示季度/年度总结

---

### 任务 3: 测试和 Bug 修复 🐛

**优先级:** P0
**预计时间:** 0.5-1 天
**状态:** 待开始

#### 具体任务：
- [ ] 测试上下文缓存功能
- [ ] 测试季度/年度总结生成
- [ ] 修复发现的 Bug
- [ ] 更新文档

---

## 🎯 下周任务（Week 2）- P1 增强体验

### 任务 4: 日历周视图 📅

**优先级:** P1
**预计时间:** 2-3 天
**状态:** 待开始

#### 为什么重要？
日历月视图已有，周视图可以提供更详细的近期任务查看。

#### 具体任务：
- [ ] 创建 `WeekView.tsx` 组件
- [ ] 显示当前周（周一到周日）7 列布局
- [ ] 每天显示任务卡片列表（最多 5 个，超出可滚动）
- [ ] 左右箭头切换周
- [ ] 顶部栏添加周/月视图切换按钮
- [ ] 保持与月视图相同的筛选逻辑

#### UI 布局：
```
┌─────────────────────────────────────────────┐
│  Week of Feb 17 - Feb 23, 2026   [< Today >]│
├───────┬───────┬───────┬───────┬───────┬─────┤
│  Mon  │  Tue  │  Wed  │  Thu  │  Fri  │ ... │
│   17  │   18  │   19  │   20  │   21  │     │
├───────┼───────┼───────┼───────┼───────┼─────┤
│ ▬ 任务1│ ▬ 任务3│       │ ▬ 任务5│       │     │
│ ▬ 任务2│ ▬ 任务4│       │       │       │     │
└───────┴───────┴───────┴───────┴───────┴─────┘
```

#### 验收标准：
- ✅ 周视图清晰展示近期任务
- ✅ 可以点击任务打开详情
- ✅ 与月视图平滑切换
- ✅ 支持筛选器

---

### 任务 5: 操作撤销 (Undo) ⏮️

**优先级:** P1
**预计时间:** 2-3 天
**状态:** 待开始

#### 为什么重要？
防止误操作，提升用户体验。这是现代应用的标配功能。

#### 具体任务：
- [ ] 设计操作历史栈（支持创建/更新/删除）
- [ ] 实现 Cmd+Z / Ctrl+Z 全局快捷键
- [ ] 撤销后恢复数据到之前状态
- [ ] Toast 提示撤销操作
- [ ] 支持撤销栈深度限制（最多 10 步）

#### 技术方案：
```typescript
// src/store/historyStore.ts
interface Operation {
  type: 'create' | 'update' | 'delete';
  before: Task | null;
  after: Task | null;
  timestamp: number;
}

const useHistoryStore = create<HistoryState>((set, get) => ({
  history: [],
  push: (operation: Operation) => { /* ... */ },
  undo: () => { /* 弹出最后一个操作并恢复 */ },
}));
```

#### 验收标准：
- ✅ Cmd+Z 可以撤销最近的操作
- ✅ 创建任务 → 撤销 → 任务消失
- ✅ 更新任务 → 撤销 → 恢复旧值
- ✅ 删除任务 → 撤销 → 任务恢复
- ✅ 有明确的 Toast 提示

---

## 📋 后续任务（Week 3+）- P2 长期优化

### 任务 6: 任务拖拽重新排期 🎯

**优先级:** P2
**预计时间:** 1-2 天

- [ ] 日历单元格拖放功能
- [ ] 拖拽任务到新日期更新 deadline
- [ ] 拖拽预览效果

---

### 任务 7: 性能优化 ⚡

**优先级:** P2
**预计时间:** 2-3 天

- [ ] 虚拟滚动 - 大量任务优化
- [ ] 数据库索引优化
- [ ] AI 流式输出

---

### 任务 8: 深色模式 🌙

**优先级:** P2
**预计时间:** 3-4 天

- [ ] 完整的深色主题
- [ ] 自动切换（跟随系统）
- [ ] 温暖的深色配色

---

## 🎯 开发建议

### 工作节奏
- **每天:** 2-3 小时专注开发
- **每周:** 完成 1-2 个 P0/P1 任务
- **每月:** 发布一个小版本

### 优先级原则
1. **P0 优先** - 必须完成，影响 MVP 完整性
2. **P1 其次** - 重要功能，显著提升体验
3. **P2 最后** - 锦上添花，长期优化

### 开发流程
1. **选择任务** - 从上到下选择
2. **创建分支** - `git checkout -b feature/xxx`
3. **开发实现** - 遵循现有代码风格
4. **编写测试** - 单元测试 + 手动测试
5. **更新文档** - 同步更新用户指南
6. **提交代码** - 清晰的 commit message
7. **标记完成** - 在本文档打勾 ✅

---

## 📞 需要帮助？

遇到问题时：
1. 查看 `docs/specs/` 技术文档
2. 参考 `CLAUDE.md` 开发指引
3. 查看已有代码实现
4. 搜索 GitHub Issues

---

**祝开发顺利！** 🚀

每完成一个任务，记得在上面打勾 ✅，并更新 `docs/CHANGELOG.md`。
