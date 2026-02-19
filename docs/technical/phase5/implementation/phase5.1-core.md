# Phase 5.1: 核心总结功能实现指南

## 实现概述

Phase 5.1 是Phase 5 的第一阶段，专注于实现核心的总结生成和展示功能。

**目标**: 建立完整的总结生成和展示流程
**预计时间**: 3-4 天
**优先级**: P0 (必须实现)

---

## 实施步骤

### Step 1: 数据库迁移 (0.5天)

#### 1.1 创建迁移脚本

```rust
// src-tauri/src/db/migrations.rs

pub fn run_migrations(conn: &Connection) -> Result<(), rusqlite::Error> {
    // ... 现有迁移 ...

    // ✨ Phase 5 Migration: Add tag support to summaries
    conn.execute_batch(
        "ALTER TABLE summaries ADD COLUMN tag TEXT;
         ALTER TABLE summaries ADD COLUMN tag_filter TEXT;

         CREATE INDEX IF NOT EXISTS idx_summaries_tag ON summaries(tag);
         CREATE INDEX IF NOT EXISTS idx_summaries_type ON summaries(summary_type);
         CREATE INDEX IF NOT EXISTS idx_summaries_period ON summaries(period_start, period_end);
         CREATE INDEX IF NOT EXISTS idx_summaries_composite ON summaries(tag, summary_type, period_start, period_end);"
    )?;

    Ok(())
}
```

#### 1.2 测试迁移

```bash
# 备份现有数据库
cp data/intento.db data/intento.db.backup

# 运行应用，自动执行迁移
cargo run

# 验证表结构
sqlite3 data/intento.db "PRAGMA table_info(summaries);"
```

---

### Step 2: 扩展数据模型 (0.5天)

#### 2.1 扩展 Summary 结构体

```rust
// src-tauri/src/db/models.rs

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Summary {
    pub id: Option<i64>,
    pub summary_type: SummaryType,
    pub period_start: i64,
    pub period_end: i64,
    pub tag: Option<String>,              // ✨ 新增
    pub tag_filter: Option<Vec<String>>,  // ✨ 新增
    pub content: String,
    pub statistics: Option<String>,
    pub task_ids: Option<Vec<i64>>,
    pub created_at: i64,
    pub is_deleted: bool,
}
```

#### 2.2 扩展 SummaryType 枚举

```rust
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SummaryType {
    Daily,
    Weekly,      // ✨ 新增
    Monthly,
    SemiAnnual,  // ✨ 新增
    Yearly,
}
```

#### 2.3 添加 SummaryStatistics

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SummaryStatistics {
    pub total_tasks: usize,
    pub completed: usize,
    pub in_progress: usize,
    pub todo: usize,
    pub completion_rate: f64,
    pub priority_distribution: PriorityDistribution,
    pub time_stats: Option<TimeStats>,
}
```

---

### Step 3: 实现 SummaryGenerator (1天)

#### 3.1 创建 summary 模块

```bash
mkdir src-tauri/src/summary
touch src-tauri/src/summary/mod.rs
touch src-tauri/src/summary/generator.rs
touch src-tauri/src/summary/period.rs
```

#### 3.2 实现核心生成逻辑

参考 `docs/phase5/backend-design/summary-generation.md` 实现：

1. **SummaryGenerator 结构体**
   - `new()` - 构造函数
   - `generate_summary()` - 生成新总结
   - `get_or_generate_summary()` - 获取或生成
   - `query_tasks_by_period()` - 查询任务
   - `generate_summary_content()` - AI 生成内容
   - `build_summary_prompt()` - 构建 Prompt

2. **PeriodCalculator 工具**
   - `today()`, `this_week()`, `this_month()` 等

#### 3.3 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_summary() {
        let db = Database::new(":memory:").unwrap();
        let ai_client = AiClient::new_default().unwrap();
        let generator = SummaryGenerator::new(db, ai_client);

        // 插入测试任务...

        let summary = generator.generate_summary(
            Some("work".to_string()),
            SummaryType::Daily,
            today_start,
            today_end,
        ).await;

        assert!(summary.is_ok());
    }
}
```

---

### Step 4: 实现 Tauri Commands (0.5天)

#### 4.1 创建 commands/summary.rs

```rust
// src-tauri/src/commands/summary.rs

#[tauri::command]
pub async fn generate_summary(
    tag: Option<String>,
    summary_type: String,
    period_start: i64,
    period_end: i64,
    db: State<'_, Database>,
    ai_state: State<'_, AiClientState>,
) -> Result<Summary, String> {
    // ... 实现 ...
}

// 其他 commands...
```

#### 4.2 在 main.rs 中注册

```rust
.invoke_handler(tauri::generate_handler![
    generate_summary,
    get_or_generate_summary,
    list_summaries,
    get_summary,
    delete_summary,
    export_summary,
])
```

---

### Step 5: 前端组件实现 (1.5天)

#### 5.1 创建类型定义

```typescript
// src/types/summary.ts

export type SummaryType = 'daily' | 'weekly' | 'monthly' | 'semi_annual' | 'yearly';

export interface Summary {
  id: number;
  summary_type: SummaryType;
  period_start: number;
  period_end: number;
  tag: string | null;
  content: string;
  statistics: string;
  task_ids: number[];
  created_at: number;
}
```

#### 5.2 实现核心组件

按照以下顺序实现：

1. **TimeRangeSelector.tsx** (0.5h)
   - 时间范围按钮组
   - 选中状态管理

2. **TagSelector.tsx** (0.5h)
   - Tag 下拉选择器
   - 显示任务数量

3. **SummaryContent.tsx** (1h)
   - Markdown 渲染
   - 统计卡片
   - 样式美化

4. **SummaryPanel.tsx** (2h)
   - 整合所有子组件
   - 状态管理
   - 加载/错误状态
   - Footer 按钮

#### 5.3 集成到 App.tsx

```tsx
// src/App.tsx

function App() {
  const [summaryPanelOpen, setSummaryPanelOpen] = useState(false);
  const [selectedTag, setSelectedTag] = useState<string | null>(null);

  return (
    <div className="flex h-screen">
      <TopBar
        onSummaryClick={() => setSummaryPanelOpen(true)}
        // ...
      />

      <main className="flex-1 flex">
        <Sidebar selectedTag={selectedTag} onTagSelect={setSelectedTag} />
        <TaskList />
        <TaskDetailPanel />

        {/* ✨ Summary Panel */}
        {summaryPanelOpen && (
          <SummaryPanel
            isOpen={summaryPanelOpen}
            onClose={() => setSummaryPanelOpen(false)}
            selectedTag={selectedTag}
          />
        )}
      </main>
    </div>
  );
}
```

---

### Step 6: Sidebar 右键菜单集成 (0.5天)

#### 6.1 添加上下文菜单

```tsx
// src/components/Sidebar.tsx

const [contextMenu, setContextMenu] = useState<{
  tag: string;
  x: number;
  y: number;
} | null>(null);

const handleTagRightClick = (e: React.MouseEvent, tag: string) => {
  e.preventDefault();
  setContextMenu({
    tag,
    x: e.clientX,
    y: e.clientY,
  });
};

return (
  <aside>
    {allTags.map(tag => (
      <button
        key={tag}
        onClick={() => onTagSelect(tag)}
        onContextMenu={(e) => handleTagRightClick(e, tag)}
        // ...
      >
        {tag}
      </button>
    ))}

    {contextMenu && (
      <ContextMenu
        x={contextMenu.x}
        y={contextMenu.y}
        onClose={() => setContextMenu(null)}
      >
        <MenuItem onClick={() => handleGenerateSummary(contextMenu.tag)}>
          📊 Generate Summary
        </MenuItem>
        <MenuItem onClick={() => handleViewHistory(contextMenu.tag)}>
          📅 View Summary History
        </MenuItem>
      </ContextMenu>
    )}
  </aside>
);
```

---

## 验收标准

### 功能验收

- [ ] 可以为特定 tag 生成每日/周/月总结
- [ ] 可以为全局任务生成总结
- [ ] 总结内容包含 AI 生成的文本和统计数据
- [ ] 总结保存到数据库，可以查询历史记录
- [ ] SummaryPanel 正确显示总结内容
- [ ] 可以通过 Sidebar 右键菜单触发总结生成
- [ ] 时间范围选择器正常工作
- [ ] Tag 选择器正常工作

### 性能验收

- [ ] 生成总结响应时间 < 5秒
- [ ] 加载历史总结响应时间 < 1秒
- [ ] 面板打开/关闭动画流畅 (60fps)

### UI/UX 验收

- [ ] 设计符合 Notion 风格
- [ ] 加载状态有明确指示
- [ ] 错误信息清晰友好
- [ ] 支持键盘快捷键 (ESC 关闭)

---

## 测试清单

### 单元测试

```bash
# 后端测试
cd src-tauri
cargo test summary::

# 前端测试 (可选)
npm test -- SummaryPanel
```

### 集成测试

1. **生成今日总结**
   - 打开应用
   - 右键点击 "work" tag
   - 选择 "Generate Summary"
   - 验证总结内容正确

2. **切换时间范围**
   - 打开 SummaryPanel
   - 点击 "Weekly" 按钮
   - 验证显示本周总结

3. **切换 Tag**
   - 在 TagSelector 中选择不同 tag
   - 验证总结内容更新

4. **空状态**
   - 选择一个没有任务的 tag
   - 验证显示友好的空状态提示

---

## 常见问题

### Q1: AI 生成失败怎么办？

**A**: 实现重试机制和降级方案：

```rust
match ai_client.chat(&prompt).await {
    Ok(content) => content,
    Err(_) => {
        // 降级：使用模板生成简单总结
        generate_fallback_summary(&tasks, &statistics)
    }
}
```

### Q2: 总结生成太慢怎么办？

**A**:
1. 优化 Prompt 长度，只传递必要信息
2. 限制任务列表数量（最多 20 个）
3. 添加缓存机制

### Q3: 如何处理大量历史总结？

**A**: 实现分页加载和虚拟滚动（Phase 5.5）

---

## 下一步

完成 Phase 5.1 后，继续实现：
- **Phase 5.2**: 定时任务集成
- **Phase 5.3**: 设置面板集成
- **Phase 5.4**: 导出功能
- **Phase 5.5**: 历史浏览优化

---

## 参考文档

- [数据模型设计](../backend-design/data-model.md)
- [总结生成逻辑](../backend-design/summary-generation.md)
- [组件规格说明](../ui-design/component-specs.md)
- [API Commands](../backend-design/api-commands.md)
