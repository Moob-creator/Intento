-- Migration V3: Fix summary types to support weekly and semi_annual
-- Created: 2026-02-12
-- Purpose: Update CHECK constraint to support new summary types

-- SQLite doesn't support ALTER TABLE to modify CHECK constraints
-- We need to recreate the table

-- 1. Create new table with correct constraint
CREATE TABLE IF NOT EXISTS summaries_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    summary_type TEXT NOT NULL
        CHECK(summary_type IN ('daily', 'weekly', 'monthly', 'semi_annual', 'yearly', 'quarterly')),
    period_start INTEGER NOT NULL,
    period_end INTEGER NOT NULL,
    tag TEXT,
    tag_filter TEXT,
    content TEXT NOT NULL,
    statistics TEXT,
    task_ids TEXT,
    created_at INTEGER NOT NULL,
    is_deleted INTEGER DEFAULT 0
);

-- 2. Copy data from old table
INSERT INTO summaries_new
SELECT id, summary_type, period_start, period_end, tag, tag_filter,
       content, statistics, task_ids, created_at, is_deleted
FROM summaries;

-- 3. Drop old table
DROP TABLE summaries;

-- 4. Rename new table
ALTER TABLE summaries_new RENAME TO summaries;

-- 5. Recreate indexes
CREATE INDEX IF NOT EXISTS idx_summaries_type ON summaries(summary_type);
CREATE INDEX IF NOT EXISTS idx_summaries_period ON summaries(period_start, period_end);
CREATE INDEX IF NOT EXISTS idx_summaries_created_at ON summaries(created_at);
CREATE INDEX IF NOT EXISTS idx_summaries_is_deleted ON summaries(is_deleted);
CREATE INDEX IF NOT EXISTS idx_summaries_tag ON summaries(tag);
CREATE INDEX IF NOT EXISTS idx_summaries_composite ON summaries(tag, summary_type, period_start, period_end);

-- Update database version
PRAGMA user_version = 3;
