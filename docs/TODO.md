# Intento 开发待办清单

> 基于 specs/development-plan.md 的开发进度追踪
> 最后更新：2026-02-11

---

## ✅ 已完成功能

### Phase 0: 项目初始化
- [x] Tauri 项目搭建
- [x] React + TypeScript 前端配置
- [x] Tailwind CSS 配置
- [x] ADK-Rust 依赖集成

### Phase 1: 数据库层
- [x] Task 1.1: 选择和配置 SQLite
- [x] Task 1.2: 设计数据库 Schema
- [x] Task 1.3: 创建数据模型（Task/Summary/ContextCache）
- [x] Task 1.4: 实现数据访问层（DAO）
  - 完整的 CRUD 操作
  - 状态筛选支持
  - 单元测试覆盖

### Phase 2: 基础任务管理
- [x] Task 2.1: 任务管理 Tauri Commands
- [x] Task 2.2: Zustand 任务状态管理
- [x] Task 2.3: 任务列表组件
  - 任务卡片组件（支持 hover 快捷操作）
  - 状态筛选（通过 Command Palette）
  - 搜索功能（通过 Command Palette）
- [x] Task 2.4: 任务创建/编辑表单
  - TaskDetailPanel 侧滑面板
  - 表单验证

### Phase 3: AI 能力集成
- [x] Task 3.1: ADK-Rust AI 客户端封装
  - 支持 OpenAI/Anthropic/Kimi
  - 统一的 parse_input 接口
  - 错误处理和重试逻辑
- [x] Task 3.2: 文本输入解析 Command
  - parse_text_input Tauri command
  - AI 解析任务标题、描述、截止时间
- [x] Task 3.4: 任务确认界面（部分完成）
  - TaskConfirmDialog 组件
  - 展示 AI 解析结果并允许编辑

### Phase 4: 智能提醒系统
- [x] Task 4.1: 定时任务调度器
  - TaskScheduler 基于 tokio-cron-scheduler
  - 启动/停止逻辑
  - 集成到 main.rs
- [x] Task 4.2: 到期提醒逻辑
  - 每小时检查即将到期任务
  - 自动触发通知
  - get_expiring_tasks 数据库方法
- [x] Task 4.3: 桌面通知
  - send_notification/check_expiring_tasks/test_notification commands
  - 支持多种通知类型
  - 跨平台通知支持

### 界面重构（2026-02-11）
- [x] 移除传统侧边栏，释放 30% 屏幕空间
- [x] 实现 Command Palette (⌘K)
  - 模糊搜索任务
  - 快捷操作菜单
  - 键盘导航
- [x] 极简顶部栏设计
- [x] 全局键盘快捷键系统
- [x] 统计面板（Statistics Panel）
- [x] 设置面板（Settings Panel）
- [x] 任务卡片 hover 快捷操作
- [x] 智能任务排序（优先级 > 状态 > 日期）

---

## 📋 待开发功能

### Phase 3: AI 能力集成（剩余部分）

#### Task 3.3: 实现图片识别功能
**优先级:** P1
**预计时间:** 2 天

**任务内容:**
- [ ] 在 AI 客户端中添加 `parse_image` 方法
- [ ] 使用 gpt-4o 的视觉能力识别图片
- [ ] 创建 `parse_image_input` command
- [ ] 支持图片路径或 Base64 输入
- [ ] 前端添加图片上传/拖拽功能

**验收标准:**
- 能从截图中提取文字信息
- 能识别任务相关内容（标题、描述、时间）
- 支持多种图片格式（PNG/JPG/WebP）

**技术方案:**
```rust
// src-tauri/src/ai/client.rs
impl AiClient {
    pub async fn parse_image_input(
        &self,
        image_path: &str,
    ) -> Result<ParsedTask, Box<dyn std::error::Error>> {
        let model = OpenAIClient::new(OpenAIConfig::new(api_key, "gpt-4o"))?;
        let agent = LlmAgentBuilder::new("image_parser")
            .instruction("从图片中提取任务信息")
            .model(Arc::new(model))
            .build()?;

        let response = agent.run_with_image(image_path).await?;
        let parsed: ParsedTask = serde_json::from_str(&response.content)?;
        Ok(parsed)
    }
}
```

---

#### Task 3.5: 实现上下文缓存机制
**优先级:** P2
**预计时间:** 1 天

**任务内容:**
- [ ] 每次解析后保存输入和结果到 `context_cache` 表
- [ ] 实现上下文清理逻辑（保留最近 20 条）
- [ ] 在解析时自动读取上下文
- [ ] 支持"刚才那个任务"等指代

**验收标准:**
- AI 能理解上下文关联（"把刚才那个任务的优先级改成高"）
- 上下文不会无限增长
- 定期清理过期上下文

**数据库操作:**
```rust
// src-tauri/src/db/mod.rs
pub fn save_context(&self, input: &str, result: &str) -> Result<i64> {
    let cache = ContextCache {
        cache_key: format!("task_input_{}", chrono::Utc::now().timestamp()),
        cache_type: CacheType::Conversation,
        cache_value: json!({
            "input": input,
            "result": result,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }).to_string(),
        expires_at: chrono::Utc::now().timestamp() + 86400 * 7, // 7天过期
    };
    self.set_cache(&cache)
}

pub fn get_recent_context(&self, limit: usize) -> Result<Vec<String>> {
    // 获取最近的上下文记录
}
```

---

### Phase 5: 每日总结生成

#### Task 5.1: 实现每日总结生成逻辑
**优先级:** P1
**预计时间:** 2 天

**任务内容:**
- [ ] 创建 `src-tauri/src/summary/mod.rs`
- [ ] 实现每日任务数据聚合
- [ ] 使用 AI 生成总结文本
- [ ] 保存总结到 `summaries` 表
- [ ] 创建 `generate_daily_summary` command

**验收标准:**
- 每天自动生成总结
- 总结包含完成任务数、未完成任务、重要事项
- 可以手动触发生成
- 使用自然语言描述

**AI Prompt 设计:**
```
你是一个任务总结助手。根据以下今日任务数据生成一份简洁的每日总结：

已完成任务 (3):
- [高优先级] 完成项目方案
- [中优先级] 代码审查
- [低优先级] 更新文档

未完成任务 (2):
- [高优先级] 客户会议准备（明天截止）
- [中优先级] Bug 修复

请生成：
1. 一句话总结今天的工作成果
2. 未完成任务提醒
3. 明日建议
```

---

#### Task 5.2: 开发每日总结展示界面
**优先级:** P1
**预计时间:** 1 天

**任务内容:**
- [ ] 在 Command Palette 添加"查看今日总结"选项
- [ ] 创建总结展示对话框
- [ ] 支持查看历史总结（按日期）
- [ ] 美化排版，使用温暖色系

**验收标准:**
- 总结内容清晰易读
- 可以查看过去 30 天的总结
- 支持导出为文本/Markdown

---

#### Task 5.3: 实现总结导出功能
**优先级:** P2
**预计时间:** 1 天

**任务内容:**
- [ ] 创建 `export_summary` command
- [ ] 支持导出为 Markdown 格式
- [ ] 支持导出为纯文本
- [ ] 可选择日期范围导出（本周/本月/自定义）

**验收标准:**
- 导出的文件格式规范
- 包含任务详情和统计数据
- 文件名包含日期范围

**导出格式示例:**
```markdown
# Intento 每日总结 - 2026-02-11

## 📊 今日统计
- ✅ 完成任务：3
- 🔄 进行中：1
- 📋 待办：2
- 🎯 完成率：60%

## ✨ 今日亮点
今天专注于项目方案设计，高效完成了3个重要任务...

## 📝 已完成任务
1. [高优先级] 完成项目方案
2. [中优先级] 代码审查
...

## ⏰ 未完成提醒
- 客户会议准备（明天截止）
```

---

### Phase 6: 高级功能（未来规划）

#### Task 6.1: 任务标签系统增强
**优先级:** P2
**预计时间:** 1 天

- [ ] 标签颜色自定义
- [ ] 标签管理界面
- [ ] 按标签筛选任务

---

#### Task 6.2: 任务依赖关系
**优先级:** P2
**预计时间:** 2 天

- [ ] 数据库 Schema 扩展（添加 dependencies 表）
- [ ] 任务依赖关系可视化
- [ ] 阻塞任务提示

---

#### Task 6.3: 时间追踪功能
**优先级:** P3
**预计时间:** 2 天

- [ ] 记录任务实际花费时间
- [ ] 时间统计报表
- [ ] 番茄钟集成

---

#### Task 6.4: 多用户支持
**优先级:** P3
**预计时间:** 3 天

- [ ] 用户认证系统
- [ ] 任务分配功能
- [ ] 团队协作

---

#### Task 6.5: 云端同步
**优先级:** P3
**预计时间:** 3 天

- [ ] 数据备份到云端
- [ ] 多设备同步
- [ ] 冲突解决机制

---

## 🐛 已知问题 & 待优化

### 性能优化
- [ ] 大量任务时列表渲染性能优化（虚拟滚动）
- [ ] 数据库查询性能优化（索引优化）
- [ ] AI 解析响应时间优化（流式输出）

### 用户体验
- [ ] 添加更多动画过渡效果
- [ ] 完善空状态提示
- [ ] 改进错误提示信息
- [ ] 添加操作撤销功能（Undo）

### 代码质量
- [ ] 增加单元测试覆盖率（目标 >80%）
- [ ] 代码文档补充（Rust Doc）
- [ ] 前端组件 Storybook
- [ ] E2E 测试（Playwright）

---

## 📚 文档待完善

- [ ] 用户使用手册（USER_GUIDE.md 已创建，待完善）
- [ ] 开发者贡献指南
- [ ] API 接口文档
- [ ] 部署指南
- [ ] 常见问题 FAQ

---

## 💡 未来创意功能

- [ ] AI 智能任务建议（"根据你的习惯，建议今天完成..."）
- [ ] 自然语言任务更新（"把今天的会议推迟到明天"）
- [ ] 语音输入支持
- [ ] 移动端适配（Tauri Mobile）
- [ ] Chrome 插件（网页快速添加任务）
- [ ] 与日历应用集成（Google Calendar/iCal）
- [ ] Slack/Discord 集成
- [ ] 任务模板功能
- [ ] 每周/每月总结报告
- [ ] 目标追踪（OKR）

---

## 📝 开发笔记

### 技术栈
- **前端:** React 19 + TypeScript + Tailwind CSS
- **后端:** Rust + Tauri 2.0
- **数据库:** SQLite + Rusqlite
- **AI:** ADK-Rust (支持 OpenAI/Anthropic/Kimi)
- **状态管理:** Zustand
- **调度器:** tokio-cron-scheduler

### 项目亮点
- ⚡ 极简 Command Palette 驱动界面
- 🎨 温暖色系设计语言
- ⌨️ 键盘优先交互设计
- 🤖 AI 驱动任务创建
- 🔔 智能到期提醒系统
- 📊 数据可视化统计

### 快捷键参考
- `⌘K` - 打开 Command Palette
- `⌘N` - 新建任务
- `⌘/` - AI 添加任务
- `⌘,` - 打开设置
- `ESC` - 关闭当前面板

---

## 📅 版本规划

### v0.1.0 (当前版本)
- ✅ 基础任务管理
- ✅ AI 文本解析
- ✅ 智能提醒系统
- ✅ Command Palette 界面

### v0.2.0 (下一版本，预计 2 周)
- 图片识别功能
- 上下文缓存
- 每日总结生成
- 总结导出功能

### v0.3.0 (未来版本)
- 标签系统增强
- 任务依赖关系
- 性能优化

### v1.0.0 (正式版)
- 完整功能集
- 完善文档
- 稳定性测试
- 用户手册

---

**维护者:** @wangshuo
**项目地址:** /Users/wangshuo/codes/projects/Intento
**最后更新:** 2026-02-11
