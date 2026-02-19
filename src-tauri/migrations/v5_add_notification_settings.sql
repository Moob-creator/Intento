-- Migration v5: Add notification settings to settings table
-- These settings control notification preferences for the application

-- Insert default notification settings
INSERT OR IGNORE INTO settings (key, value) VALUES
    -- Enable/disable notifications globally
    ('notification_enabled', 'true'),

    -- Deadline reminders
    ('notification_deadline_enabled', 'true'),
    ('notification_deadline_advance_hours', '24'),  -- Notify X hours before deadline

    -- Daily review reminders
    ('notification_daily_review_enabled', 'true'),
    ('notification_daily_review_time', '09:00'),    -- Time in HH:MM format

    -- Task completion notifications
    ('notification_task_completion_enabled', 'true'),

    -- Sound settings
    ('notification_sound_enabled', 'true'),

    -- Do not disturb settings
    ('notification_dnd_enabled', 'false'),
    ('notification_dnd_start_time', '22:00'),       -- Do not disturb start time
    ('notification_dnd_end_time', '08:00');         -- Do not disturb end time

-- Update database version to 5
PRAGMA user_version = 5;
