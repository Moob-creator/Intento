# 数据库设计文档

## 概述

Intento 使用 SQLite 作为本地数据库，存储任务、总结和上下文缓存数据。

## 表结构设计

### 1. tasks 表（任务管理）

存储用户创建的所有任务信息。

```sql
CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,                    -- 任务标题
    description TEXT,                       -- 任务详细描述
    status TEXT NOT NULL DEFAULT 'todo'     -- 任务状态: todo, doing, done
        CHECK(status IN ('todo', 'doing', 'done')),
    priority TEXT DEFAULT 'medium'          -- 优先级: low, medium, high
        CHECK(priority IN ('low', 'medium', 'high')),
    deadline INTEGER,                       -- 截止时间（Unix 时间戳）
    created_at INTEGER NOT NULL,            -- 创建时间（Unix 时间戳）
    updated_at INTEGER NOT NULL,            -- 更新时间（Unix 时间戳）
    completed_at INTEGER,                   -- 完成时间（Unix 时间戳）
    context TEXT,                           -- AI 理解的上下文（JSON 格式）
    tags TEXT,                              -- 标签（JSON 数组）
    attachments TEXT,                       -- 附件信息（JSON 数组）
    reminder_time INTEGER,                  -- 提醒时间（Unix 时间戳）
    is_deleted INTEGER DEFAULT 0            -- 软删除标记: 0=正常, 1=已删除
);

-- 索引优化
CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_tasks_deadline ON tasks(deadline);
CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at);
CREATE INDEX IF NOT EXISTS idx_tasks_is_deleted ON tasks(is_deleted);
```

**字段说明：**
- `id`: 主键，自增
- `title`: 任务标题，必填
- `description`: 任务详细描述
- `status`: 任务状态（todo/doing/done）
- `priority`: 优先级（low/medium/high）
- `deadline`: 截止日期时间戳
- `created_at`: 创建时间戳
- `updated_at`: 最后更新时间戳
- `completed_at`: 完成时间戳
- `context`: AI 解析的上下文信息（JSON）
- `tags`: 标签列表（JSON 数组）
- `attachments`: 附件信息（JSON 数组，包含图片路径等）
- `reminder_time`: 提醒时间戳
- `is_deleted`: 软删除标记

### 2. summaries 表（总结记录）

存储系统自动生成的各类总结。

```sql
CREATE TABLE IF NOT EXISTS summaries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    summary_type TEXT NOT NULL              -- 总结类型: daily, monthly, quarterly, yearly
        CHECK(summary_type IN ('daily', 'monthly', 'quarterly', 'yearly')),
    period_start INTEGER NOT NULL,          -- 统计周期开始时间（Unix 时间戳）
    period_end INTEGER NOT NULL,            -- 统计周期结束时间（Unix 时间戳）
    content TEXT NOT NULL,                  -- 总结内容
    statistics TEXT,                        -- 统计数据（JSON 格式）
    task_ids TEXT,                          -- 关联的任务 ID 列表（JSON 数组）
    created_at INTEGER NOT NULL,            -- 生成时间（Unix 时间戳）
    is_deleted INTEGER DEFAULT 0            -- 软删除标记
);

-- 索引优化
CREATE INDEX IF NOT EXISTS idx_summaries_type ON summaries(summary_type);
CREATE INDEX IF NOT EXISTS idx_summaries_period ON summaries(period_start, period_end);
CREATE INDEX IF NOT EXISTS idx_summaries_created_at ON summaries(created_at);
CREATE INDEX IF NOT EXISTS idx_summaries_is_deleted ON summaries(is_deleted);
```

**字段说明：**
- `id`: 主键，自增
- `summary_type`: 总结类型（daily/monthly/quarterly/yearly）
- `period_start`: 统计周期开始时间
- `period_end`: 统计周期结束时间
- `content`: AI 生成的总结内容
- `statistics`: 统计数据（JSON，包含任务完成率、数量等）
- `task_ids`: 本次总结涵盖的任务 ID 列表
- `created_at`: 生成时间
- `is_deleted`: 软删除标记

### 3. context_cache 表（上下文缓存）

存储 AI 处理过程中的上下文信息，用于优化响应速度。

```sql
CREATE TABLE IF NOT EXISTS context_cache (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cache_key TEXT NOT NULL UNIQUE,         -- 缓存键（通常是内容的 hash）
    cache_type TEXT NOT NULL                -- 缓存类型: task_context, image_analysis, text_parse
        CHECK(cache_type IN ('task_context', 'image_analysis', 'text_parse')),
    content TEXT NOT NULL,                  -- 缓存内容（JSON 格式）
    source_data TEXT,                       -- 源数据引用
    created_at INTEGER NOT NULL,            -- 创建时间（Unix 时间戳）
    last_accessed_at INTEGER NOT NULL,      -- 最后访问时间（Unix 时间戳）
    access_count INTEGER DEFAULT 1,         -- 访问次数
    expires_at INTEGER,                     -- 过期时间（Unix 时间戳，NULL 表示永不过期）
    is_deleted INTEGER DEFAULT 0            -- 软删除标记
);

-- 索引优化
CREATE INDEX IF NOT EXISTS idx_context_cache_key ON context_cache(cache_key);
CREATE INDEX IF NOT EXISTS idx_context_cache_type ON context_cache(cache_type);
CREATE INDEX IF NOT EXISTS idx_context_cache_expires ON context_cache(expires_at);
CREATE INDEX IF NOT EXISTS idx_context_cache_accessed ON context_cache(last_accessed_at);
CREATE INDEX IF NOT EXISTS idx_context_cache_is_deleted ON context_cache(is_deleted);
```

**字段说明：**
- `id`: 主键，自增
- `cache_key`: 缓存键，唯一标识
- `cache_type`: 缓存类型（task_context/image_analysis/text_parse）
- `content`: 缓存的内容（JSON）
- `source_data`: 原始数据的引用或摘要
- `created_at`: 创建时间
- `last_accessed_at`: 最后访问时间
- `access_count`: 访问次数统计
- `expires_at`: 过期时间，用于缓存清理
- `is_deleted`: 软删除标记

## 数据关系

```
tasks (1) ----< (N) summaries.task_ids
  |
  +--> context_cache (通过 cache_key 关联任务上下文)
```

## 性能优化策略

1. **索引设计**
   - 所有时间字段添加索引，支持时间范围查询
   - status、type 等常用过滤字段添加索引
   - 软删除字段添加索引，优化查询性能

2. **软删除**
   - 所有表使用软删除机制，避免误删除
   - 定期清理已删除数据（可选）

3. **缓存过期机制**
   - context_cache 表支持 TTL
   - 定期清理过期缓存

4. **JSON 存储**
   - 使用 JSON 存储灵活字段（tags, attachments, statistics 等）
   - SQLite 3.38+ 支持 JSON 函数查询

## 迁移策略

版本号存储在 SQLite 的 `PRAGMA user_version` 中，便于追踪数据库版本和执行迁移。

```rust
// 获取当前版本
PRAGMA user_version;

// 设置版本号
PRAGMA user_version = 1;
```

## 备份策略

1. 定期自动备份数据库文件
2. 导出为 JSON 格式供用户查看
3. 支持导入/恢复功能
