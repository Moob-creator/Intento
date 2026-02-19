/**
 * Notification Settings Type Definitions
 *
 * These types match the Rust backend NotificationSettings structure
 * and can be used for type-safe Tauri command invocations.
 */

export interface NotificationSettings {
  /** Global notification toggle */
  enabled: boolean;

  /** Deadline reminder settings */
  deadline_enabled: boolean;
  /** Hours before deadline to send notification (1-168) */
  deadline_advance_hours: number;

  /** Daily review reminder settings */
  daily_review_enabled: boolean;
  /** Time for daily review in HH:MM format (24-hour) */
  daily_review_time: string;

  /** Task completion notification settings */
  task_completion_enabled: boolean;

  /** Sound settings */
  sound_enabled: boolean;

  /** Do Not Disturb settings */
  dnd_enabled: boolean;
  /** DND start time in HH:MM format (24-hour) */
  dnd_start_time: string;
  /** DND end time in HH:MM format (24-hour) */
  dnd_end_time: string;
}

/**
 * Default notification settings
 */
export const DEFAULT_NOTIFICATION_SETTINGS: NotificationSettings = {
  enabled: true,
  deadline_enabled: true,
  deadline_advance_hours: 24,
  daily_review_enabled: true,
  daily_review_time: "09:00",
  task_completion_enabled: true,
  sound_enabled: true,
  dnd_enabled: false,
  dnd_start_time: "22:00",
  dnd_end_time: "08:00",
};

/**
 * Validates a time string in HH:MM format (24-hour)
 */
export function isValidTimeFormat(time: string): boolean {
  const parts = time.split(':');
  if (parts.length !== 2) return false;

  const hours = parseInt(parts[0], 10);
  const minutes = parseInt(parts[1], 10);

  return !isNaN(hours) && !isNaN(minutes) && hours >= 0 && hours < 24 && minutes >= 0 && minutes < 60;
}

/**
 * Validates notification settings
 */
export function validateNotificationSettings(settings: NotificationSettings): string | null {
  // Validate deadline_advance_hours range
  if (settings.deadline_advance_hours < 1 || settings.deadline_advance_hours > 168) {
    return `Invalid deadline_advance_hours: ${settings.deadline_advance_hours}. Must be between 1 and 168.`;
  }

  // Validate time formats
  if (!isValidTimeFormat(settings.daily_review_time)) {
    return `Invalid daily_review_time format: ${settings.daily_review_time}. Expected HH:MM (24-hour format).`;
  }

  if (!isValidTimeFormat(settings.dnd_start_time)) {
    return `Invalid dnd_start_time format: ${settings.dnd_start_time}. Expected HH:MM (24-hour format).`;
  }

  if (!isValidTimeFormat(settings.dnd_end_time)) {
    return `Invalid dnd_end_time format: ${settings.dnd_end_time}. Expected HH:MM (24-hour format).`;
  }

  return null; // Valid
}
