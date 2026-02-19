/**
 * Notification configuration settings
 */
export interface NotificationConfig {
  enabled: boolean;
  reminder_enabled: boolean;
  reminder_minutes: number; // Minutes before deadline to remind
}

/**
 * Notification test result
 */
export interface NotificationTestResult {
  success: boolean;
  message: string;
}
