-- Migration V1: Initial database schema
-- Created: 2026-02-09

-- Set database version
PRAGMA user_version = 1;

-- Enable foreign keys
PRAGMA foreign_keys = ON;

-- ==============================================
-- Table: tasks
-- Purpose: Store user tasks
-- ==============================================
CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'todo'
        CHECK(status IN ('todo', 'doing', 'done')),
    priority TEXT DEFAULT 'medium'
        CHECK(priority IN ('low', 'medium', 'high')),
    deadline INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    completed_at INTEGER,
    context TEXT,
    tags TEXT,
    attachments TEXT,
    reminder_time INTEGER,
    is_deleted INTEGER DEFAULT 0
);

-- Indexes for tasks table
CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_tasks_deadline ON tasks(deadline);
CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at);
CREATE INDEX IF NOT EXISTS idx_tasks_is_deleted ON tasks(is_deleted);

-- ==============================================
-- Table: summaries
-- Purpose: Store periodic summaries
-- ==============================================
CREATE TABLE IF NOT EXISTS summaries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    summary_type TEXT NOT NULL
        CHECK(summary_type IN ('daily', 'monthly', 'quarterly', 'yearly')),
    period_start INTEGER NOT NULL,
    period_end INTEGER NOT NULL,
    content TEXT NOT NULL,
    statistics TEXT,
    task_ids TEXT,
    created_at INTEGER NOT NULL,
    is_deleted INTEGER DEFAULT 0
);

-- Indexes for summaries table
CREATE INDEX IF NOT EXISTS idx_summaries_type ON summaries(summary_type);
CREATE INDEX IF NOT EXISTS idx_summaries_period ON summaries(period_start, period_end);
CREATE INDEX IF NOT EXISTS idx_summaries_created_at ON summaries(created_at);
CREATE INDEX IF NOT EXISTS idx_summaries_is_deleted ON summaries(is_deleted);

-- ==============================================
-- Table: context_cache
-- Purpose: Cache AI processing results
-- ==============================================
CREATE TABLE IF NOT EXISTS context_cache (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cache_key TEXT NOT NULL UNIQUE,
    cache_type TEXT NOT NULL
        CHECK(cache_type IN ('task_context', 'image_analysis', 'text_parse')),
    content TEXT NOT NULL,
    source_data TEXT,
    created_at INTEGER NOT NULL,
    last_accessed_at INTEGER NOT NULL,
    access_count INTEGER DEFAULT 1,
    expires_at INTEGER,
    is_deleted INTEGER DEFAULT 0
);

-- Indexes for context_cache table
CREATE INDEX IF NOT EXISTS idx_context_cache_key ON context_cache(cache_key);
CREATE INDEX IF NOT EXISTS idx_context_cache_type ON context_cache(cache_type);
CREATE INDEX IF NOT EXISTS idx_context_cache_expires ON context_cache(expires_at);
CREATE INDEX IF NOT EXISTS idx_context_cache_accessed ON context_cache(last_accessed_at);
CREATE INDEX IF NOT EXISTS idx_context_cache_is_deleted ON context_cache(is_deleted);
