-- Migration V2: Add tag support to summaries
-- Created: 2026-02-12
-- Purpose: Add tag filtering for summaries and extend summary types

-- Update summary_type to support new types
-- Note: SQLite doesn't support ALTER COLUMN, so we handle this in application code

-- We need to check if columns exist before adding them
-- SQLite doesn't support IF NOT EXISTS for ALTER TABLE ADD COLUMN
-- So we'll handle this gracefully in the Rust code by catching the error

-- Create indexes for better query performance (these are idempotent)
CREATE INDEX IF NOT EXISTS idx_summaries_tag ON summaries(tag);
CREATE INDEX IF NOT EXISTS idx_summaries_composite ON summaries(tag, summary_type, period_start, period_end);

-- Update database version
PRAGMA user_version = 2;
