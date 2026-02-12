-- Migration V2: Add tag support to summaries
-- Created: 2026-02-12
-- Purpose: Add tag filtering for summaries and extend summary types

-- Update summary_type to support new types
-- Note: SQLite doesn't support ALTER COLUMN, so we handle this in application code

-- Add tag columns
ALTER TABLE summaries ADD COLUMN tag TEXT;
ALTER TABLE summaries ADD COLUMN tag_filter TEXT;

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_summaries_tag ON summaries(tag);
CREATE INDEX IF NOT EXISTS idx_summaries_composite ON summaries(tag, summary_type, period_start, period_end);

-- Update database version
PRAGMA user_version = 2;
