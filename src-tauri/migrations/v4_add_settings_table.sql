-- Migration v4: Add settings table for app configuration
-- This table stores key-value settings for the application

CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- Create index for faster lookups
CREATE INDEX IF NOT EXISTS idx_settings_key ON settings(key);

-- Insert default auto summary settings
INSERT OR IGNORE INTO settings (key, value) VALUES
    ('auto_summary_enabled', 'true'),
    ('auto_summary_daily_enabled', 'true'),
    ('auto_summary_weekly_enabled', 'true'),
    ('auto_summary_monthly_enabled', 'true'),
    ('auto_summary_semi_annual_enabled', 'true'),
    ('auto_summary_yearly_enabled', 'true'),
    ('auto_summary_retention_days', '365');
