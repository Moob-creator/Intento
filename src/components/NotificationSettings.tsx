import { Bell, BellRing, Clock, TestTube2 } from 'lucide-react';
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { CustomSelect } from './CustomSelect';

interface NotificationSettingsProps {
  className?: string;
}

// Match the backend NotificationSettings structure
interface NotificationConfig {
  enabled: boolean;
  deadline_enabled: boolean;
  deadline_advance_hours: number; // Hours before deadline
  daily_review_enabled: boolean;
  daily_review_time: string; // HH:MM format
  task_completion_enabled: boolean;
  sound_enabled: boolean;
  dnd_enabled: boolean;
  dnd_start_time: string; // HH:MM format
  dnd_end_time: string; // HH:MM format
}

// Reminder time options (in hours)
const REMINDER_OPTIONS = [
  { value: '1', label: '1 hour before' },
  { value: '2', label: '2 hours before' },
  { value: '4', label: '4 hours before' },
  { value: '12', label: '12 hours before' },
  { value: '24', label: '1 day before' },
  { value: '48', label: '2 days before' },
];

export function NotificationSettings({ className = '' }: NotificationSettingsProps) {
  const [config, setConfig] = useState<NotificationConfig>({
    enabled: true,
    deadline_enabled: true,
    deadline_advance_hours: 24, // Default: 1 day before
    daily_review_enabled: true,
    daily_review_time: '09:00',
    task_completion_enabled: true,
    sound_enabled: true,
    dnd_enabled: false,
    dnd_start_time: '22:00',
    dnd_end_time: '08:00',
  });
  const [isLoading, setIsLoading] = useState(false);
  const [isTesting, setIsTesting] = useState(false);
  const [testResult, setTestResult] = useState<{ type: 'success' | 'error'; message: string } | null>(null);

  // Load notification settings when component mounts
  useEffect(() => {
    loadSettings();
  }, []);

  // Load settings from backend
  const loadSettings = async () => {
    setIsLoading(true);
    try {
      const settings = await invoke<NotificationConfig>('get_notification_settings');
      setConfig(settings);
    } catch (err) {
      console.error('Failed to load notification settings:', err);
      // Use default settings if load fails
    } finally {
      setIsLoading(false);
    }
  };

  // Save settings to backend
  const saveSettings = async (newConfig: NotificationConfig) => {
    try {
      await invoke('update_notification_settings', { settings: newConfig });
    } catch (err) {
      console.error('Failed to save notification settings:', err);
      // Optionally show error to user
    }
  };

  // Handle master toggle
  const handleToggleEnabled = () => {
    const newConfig = { ...config, enabled: !config.enabled };
    setConfig(newConfig);
    saveSettings(newConfig);
  };

  // Handle reminder toggle
  const handleToggleReminder = () => {
    const newConfig = { ...config, deadline_enabled: !config.deadline_enabled };
    setConfig(newConfig);
    saveSettings(newConfig);
  };

  // Handle reminder time change
  const handleReminderTimeChange = (value: string) => {
    const hours = parseInt(value);
    const newConfig = { ...config, deadline_advance_hours: hours };
    setConfig(newConfig);
    saveSettings(newConfig);
  };

  // Test notification
  const handleTestNotification = async () => {
    setIsTesting(true);
    setTestResult(null);

    try {
      await invoke('test_notification');
      setTestResult({
        type: 'success',
        message: 'Test notification sent successfully! Check your notification center.',
      });
    } catch (error) {
      console.error('Failed to send test notification:', error);
      setTestResult({
        type: 'error',
        message: error instanceof Error ? error.message : 'Failed to send test notification',
      });
    } finally {
      setIsTesting(false);
      // Auto-dismiss success message after 5 seconds
      setTimeout(() => setTestResult(null), 5000);
    }
  };

  if (isLoading) {
    return (
      <div className={`${className} animate-pulse`}>
        <div className="flex items-center gap-2 mb-3">
          <div className="w-5 h-5 bg-neutral-light/50 rounded" />
          <div className="h-5 w-32 bg-neutral-light/50 rounded" />
        </div>
        <div className="space-y-3 pl-7">
          <div className="h-16 bg-neutral-light/30 rounded-lg" />
          <div className="h-16 bg-neutral-light/30 rounded-lg" />
        </div>
      </div>
    );
  }

  return (
    <div className={className}>
      {/* Section header */}
      <div className="flex items-center gap-2 mb-3">
        <Bell size={20} className="text-primary" />
        <h3 className="text-base font-semibold text-neutral-dark">Notifications</h3>
      </div>

      <div className="space-y-3 pl-7">
        {/* Master toggle */}
        <label
          className={`flex items-center justify-between p-3 rounded-lg cursor-pointer transition-all duration-200 ${
            config.enabled
              ? 'bg-primary/10 border-2 border-primary/20'
              : 'bg-neutral-light/30 hover:bg-neutral-light/50'
          }`}
        >
          <div className="flex items-center gap-3">
            {config.enabled ? (
              <BellRing size={20} className="text-primary animate-pulse" />
            ) : (
              <Bell size={20} className="text-neutral-dark/40" />
            )}
            <div>
              <span className="text-sm font-medium text-neutral-dark block">
                Enable desktop notifications
              </span>
              <span className="text-xs text-neutral-dark/60">
                Get notified about tasks and deadlines
              </span>
            </div>
          </div>
          <input
            type="checkbox"
            checked={config.enabled}
            onChange={handleToggleEnabled}
            className="w-5 h-5 text-primary rounded focus:ring-2 focus:ring-primary/50 cursor-pointer"
          />
        </label>

        {/* Reminder settings - only show when notifications are enabled */}
        {config.enabled && (
          <div className="space-y-3 pl-4 border-l-2 border-primary/20">
            {/* Reminder toggle */}
            <label className="flex items-center justify-between p-3 bg-neutral-light/30 rounded-lg cursor-pointer hover:bg-neutral-light/50 transition-all duration-200">
              <div className="flex items-center gap-3">
                <Clock size={18} className="text-primary-light" />
                <div>
                  <span className="text-sm font-medium text-neutral-dark block">
                    Deadline reminders
                  </span>
                  <span className="text-xs text-neutral-dark/60">
                    Remind me before tasks are due
                  </span>
                </div>
              </div>
              <input
                type="checkbox"
                checked={config.deadline_enabled}
                onChange={handleToggleReminder}
                className="w-5 h-5 text-primary rounded focus:ring-2 focus:ring-primary/50 cursor-pointer"
              />
            </label>

            {/* Reminder time selector - only show when reminders are enabled */}
            {config.deadline_enabled && (
              <div className="p-3 bg-accent-peach rounded-lg border border-primary/10">
                <label className="block text-sm font-medium text-neutral-dark mb-2">
                  Remind me...
                </label>
                <CustomSelect
                  value={config.deadline_advance_hours.toString()}
                  options={REMINDER_OPTIONS}
                  onChange={handleReminderTimeChange}
                />
                <p className="text-xs text-neutral-dark/60 mt-2 leading-relaxed">
                  You'll receive a notification {REMINDER_OPTIONS.find(opt => opt.value === config.deadline_advance_hours.toString())?.label || '1 day before'} each task's deadline
                </p>
              </div>
            )}
          </div>
        )}

        {/* Test notification button */}
        <div className="pt-2">
          <button
            onClick={handleTestNotification}
            disabled={!config.enabled || isTesting}
            className={`w-full flex items-center justify-center gap-2 px-4 py-3 rounded-lg font-medium transition-all duration-200 ${
              config.enabled
                ? 'bg-gradient-to-r from-primary-light to-primary text-white hover:shadow-warm hover:scale-[1.02] active:scale-100'
                : 'bg-neutral-light/50 text-neutral-dark/40 cursor-not-allowed'
            }`}
          >
            {isTesting ? (
              <>
                <svg
                  className="animate-spin h-5 w-5"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                >
                  <circle
                    className="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    strokeWidth="4"
                  />
                  <path
                    className="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                  />
                </svg>
                <span>Sending notification...</span>
              </>
            ) : (
              <>
                <TestTube2 size={18} />
                <span>Test Notification</span>
              </>
            )}
          </button>
        </div>

        {/* Test result message */}
        {testResult && (
          <div
            className={`p-3 rounded-lg border-2 animate-slide-down ${
              testResult.type === 'success'
                ? 'bg-emerald-50 border-emerald-200'
                : 'bg-rose-50 border-rose-200'
            }`}
          >
            <p
              className={`text-sm leading-relaxed ${
                testResult.type === 'success' ? 'text-emerald-800' : 'text-rose-800'
              }`}
            >
              {testResult.type === 'success' ? '✓' : '✗'} {testResult.message}
            </p>
          </div>
        )}

        {/* Info note */}
        <div className="p-3 bg-amber-50/50 border border-amber-200/50 rounded-lg">
          <p className="text-xs text-neutral-dark/70 leading-relaxed">
            <span className="font-semibold text-amber-700">Note:</span> Make sure system
            notifications are enabled for this app in your system settings.
          </p>
        </div>
      </div>
    </div>
  );
}
