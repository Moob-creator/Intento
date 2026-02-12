import { X, Settings as SettingsIcon, Bell, Palette, Keyboard, Info, FileText } from 'lucide-react';
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { CustomSelect } from './CustomSelect';

interface SettingsPanelProps {
  isOpen: boolean;
  onClose: () => void;
}

interface AutoSummarySettings {
  enabled: boolean;
  daily_enabled: boolean;
  weekly_enabled: boolean;
  monthly_enabled: boolean;
  semi_annual_enabled: boolean;
  yearly_enabled: boolean;
  retention_days: number;
}

export function SettingsPanel({ isOpen, onClose }: SettingsPanelProps) {
  const [autoSummarySettings, setAutoSummarySettings] = useState<AutoSummarySettings>({
    enabled: true,
    daily_enabled: true,
    weekly_enabled: true,
    monthly_enabled: true,
    semi_annual_enabled: true,
    yearly_enabled: true,
    retention_days: 365,
  });

  const [isLoading, setIsLoading] = useState(false);

  // Load settings when panel opens
  useEffect(() => {
    if (isOpen) {
      loadSettings();
    }
  }, [isOpen]);

  const loadSettings = async () => {
    try {
      const settings = await invoke<AutoSummarySettings>('get_auto_summary_settings');
      setAutoSummarySettings(settings);
    } catch (err) {
      console.error('Failed to load auto summary settings:', err);
      // Use default values on error
    }
  };

  const saveSettings = async () => {
    setIsLoading(true);
    try {
      await invoke('update_auto_summary_settings', { settings: autoSummarySettings });
      console.log('Settings saved successfully');
    } catch (err) {
      console.error('Failed to save settings:', err);
      alert('保存设置失败: ' + err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleToggle = (field: keyof AutoSummarySettings) => {
    setAutoSummarySettings(prev => ({
      ...prev,
      [field]: !prev[field],
    }));
  };

  const handleRetentionChange = (value: number) => {
    setAutoSummarySettings(prev => ({
      ...prev,
      retention_days: Math.max(30, Math.min(3650, value)),
    }));
  };
  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm"
      onClick={onClose}
    >
      <div
        className="w-full max-w-2xl bg-white rounded-xl shadow-2xl overflow-hidden"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-neutral-light/60">
          <div className="flex items-center gap-3">
            <SettingsIcon size={24} className="text-primary" />
            <h2 className="text-xl font-bold text-neutral-dark">Settings</h2>
          </div>
          <button
            onClick={onClose}
            className="p-2 text-neutral-dark/40 hover:text-neutral-dark hover:bg-neutral-light/40 rounded-lg transition-all duration-200"
            aria-label="Close"
          >
            <X size={20} />
          </button>
        </div>

        {/* Content */}
        <div className="p-6 max-h-[70vh] overflow-y-auto">
          {/* Auto Summaries - ✨ Phase 5.3 */}
          <div className="mb-6">
            <div className="flex items-center gap-2 mb-3">
              <FileText size={20} className="text-purple-500" />
              <h3 className="text-base font-semibold text-neutral-dark">自动总结</h3>
            </div>
            <div className="space-y-3 pl-7">
              {/* Master Toggle */}
              <label className="flex items-center justify-between p-3 bg-neutral-light/30 rounded-lg cursor-pointer hover:bg-neutral-light/50 transition-all duration-200">
                <div>
                  <span className="text-sm font-medium text-neutral-dark block">启用自动总结</span>
                  <span className="text-xs text-neutral-dark/60">定时生成任务总结报告</span>
                </div>
                <input
                  type="checkbox"
                  checked={autoSummarySettings.enabled}
                  onChange={() => handleToggle('enabled')}
                  className="w-5 h-5 text-purple-500 rounded focus:ring-2 focus:ring-purple-500/50"
                />
              </label>

              {/* Frequency Options */}
              {autoSummarySettings.enabled && (
                <div className="p-3 bg-neutral-light/30 rounded-lg space-y-2">
                  <span className="text-sm font-medium text-neutral-dark block mb-2">生成频率</span>

                  <label className="flex items-center justify-between py-2 cursor-pointer">
                    <span className="text-sm text-neutral-dark">每日总结 (1:00 AM)</span>
                    <input
                      type="checkbox"
                      checked={autoSummarySettings.daily_enabled}
                      onChange={() => handleToggle('daily_enabled')}
                      className="w-4 h-4 text-purple-500 rounded focus:ring-2 focus:ring-purple-500/50"
                    />
                  </label>

                  <label className="flex items-center justify-between py-2 cursor-pointer">
                    <span className="text-sm text-neutral-dark">每周总结 (周一 2:00 AM)</span>
                    <input
                      type="checkbox"
                      checked={autoSummarySettings.weekly_enabled}
                      onChange={() => handleToggle('weekly_enabled')}
                      className="w-4 h-4 text-purple-500 rounded focus:ring-2 focus:ring-purple-500/50"
                    />
                  </label>

                  <label className="flex items-center justify-between py-2 cursor-pointer">
                    <span className="text-sm text-neutral-dark">每月总结 (1号 3:00 AM)</span>
                    <input
                      type="checkbox"
                      checked={autoSummarySettings.monthly_enabled}
                      onChange={() => handleToggle('monthly_enabled')}
                      className="w-4 h-4 text-purple-500 rounded focus:ring-2 focus:ring-purple-500/50"
                    />
                  </label>

                  <label className="flex items-center justify-between py-2 cursor-pointer">
                    <span className="text-sm text-neutral-dark">半年总结 (1/1, 7/1 4:00 AM)</span>
                    <input
                      type="checkbox"
                      checked={autoSummarySettings.semi_annual_enabled}
                      onChange={() => handleToggle('semi_annual_enabled')}
                      className="w-4 h-4 text-purple-500 rounded focus:ring-2 focus:ring-purple-500/50"
                    />
                  </label>

                  <label className="flex items-center justify-between py-2 cursor-pointer">
                    <span className="text-sm text-neutral-dark">年度总结 (1/1 5:00 AM)</span>
                    <input
                      type="checkbox"
                      checked={autoSummarySettings.yearly_enabled}
                      onChange={() => handleToggle('yearly_enabled')}
                      className="w-4 h-4 text-purple-500 rounded focus:ring-2 focus:ring-purple-500/50"
                    />
                  </label>
                </div>
              )}

              {/* Retention Period */}
              {autoSummarySettings.enabled && (
                <div className="p-3 bg-neutral-light/30 rounded-lg">
                  <label className="block text-sm text-neutral-dark mb-2">
                    历史保留时长 (天)
                  </label>
                  <input
                    type="number"
                    value={autoSummarySettings.retention_days}
                    onChange={(e) => handleRetentionChange(parseInt(e.target.value) || 365)}
                    min={30}
                    max={3650}
                    className="w-full px-3 py-2 bg-white border border-neutral-light rounded-lg text-neutral-dark focus:outline-none focus:ring-2 focus:ring-purple-500/50"
                  />
                  <span className="text-xs text-neutral-dark/60 mt-1 block">
                    建议: 365 天 (1年)，最少 30 天，最多 3650 天 (10年)
                  </span>
                </div>
              )}
            </div>
          </div>

          {/* Notifications */}
          <div className="mb-6">
            <div className="flex items-center gap-2 mb-3">
              <Bell size={20} className="text-primary" />
              <h3 className="text-base font-semibold text-neutral-dark">Notifications</h3>
            </div>
            <div className="space-y-3 pl-7">
              <label className="flex items-center justify-between p-3 bg-neutral-light/30 rounded-lg cursor-pointer hover:bg-neutral-light/50 transition-all duration-200">
                <span className="text-sm text-neutral-dark">Enable desktop notifications</span>
                <input
                  type="checkbox"
                  defaultChecked
                  className="w-5 h-5 text-primary rounded focus:ring-2 focus:ring-primary/50"
                />
              </label>
              <label className="flex items-center justify-between p-3 bg-neutral-light/30 rounded-lg cursor-pointer hover:bg-neutral-light/50 transition-all duration-200">
                <span className="text-sm text-neutral-dark">Remind me of deadlines</span>
                <input
                  type="checkbox"
                  defaultChecked
                  className="w-5 h-5 text-primary rounded focus:ring-2 focus:ring-primary/50"
                />
              </label>
              <div className="p-3 bg-neutral-light/30 rounded-lg">
                <label className="block text-sm text-neutral-dark mb-2">
                  Remind me before (hours)
                </label>
                <input
                  type="number"
                  defaultValue={24}
                  min={1}
                  max={168}
                  className="w-full px-3 py-2 bg-white border border-neutral-light rounded-lg text-neutral-dark focus:outline-none focus:ring-2 focus:ring-primary/50"
                />
              </div>
            </div>
          </div>

          {/* Appearance */}
          <div className="mb-6">
            <div className="flex items-center gap-2 mb-3">
              <Palette size={20} className="text-primary" />
              <h3 className="text-base font-semibold text-neutral-dark">Appearance</h3>
            </div>
            <div className="space-y-3 pl-7">
              <div className="p-3 bg-neutral-light/30 rounded-lg">
                <CustomSelect
                  label="Theme"
                  value="light"
                  options={[
                    { value: 'light', label: 'Light (Warm)' },
                    { value: 'dark', label: 'Dark (Coming Soon)' },
                    { value: 'auto', label: 'Auto' },
                  ]}
                  onChange={() => {}}
                />
              </div>
              <label className="flex items-center justify-between p-3 bg-neutral-light/30 rounded-lg cursor-pointer hover:bg-neutral-light/50 transition-all duration-200">
                <span className="text-sm text-neutral-dark">Compact mode</span>
                <input
                  type="checkbox"
                  className="w-5 h-5 text-primary rounded focus:ring-2 focus:ring-primary/50"
                />
              </label>
            </div>
          </div>

          {/* Keyboard Shortcuts */}
          <div className="mb-6">
            <div className="flex items-center gap-2 mb-3">
              <Keyboard size={20} className="text-primary" />
              <h3 className="text-base font-semibold text-neutral-dark">Keyboard Shortcuts</h3>
            </div>
            <div className="space-y-2 pl-7">
              <div className="flex items-center justify-between py-2">
                <span className="text-sm text-neutral-dark">Open command palette</span>
                <kbd className="px-2 py-1 text-xs font-medium text-neutral-dark/80 bg-white border border-neutral-light rounded">
                  ⌘K
                </kbd>
              </div>
              <div className="flex items-center justify-between py-2">
                <span className="text-sm text-neutral-dark">New task</span>
                <kbd className="px-2 py-1 text-xs font-medium text-neutral-dark/80 bg-white border border-neutral-light rounded">
                  ⌘N
                </kbd>
              </div>
              <div className="flex items-center justify-between py-2">
                <span className="text-sm text-neutral-dark">AI add task</span>
                <kbd className="px-2 py-1 text-xs font-medium text-neutral-dark/80 bg-white border border-neutral-light rounded">
                  ⌘/
                </kbd>
              </div>
              <div className="flex items-center justify-between py-2">
                <span className="text-sm text-neutral-dark">Settings</span>
                <kbd className="px-2 py-1 text-xs font-medium text-neutral-dark/80 bg-white border border-neutral-light rounded">
                  ⌘,
                </kbd>
              </div>
              <div className="flex items-center justify-between py-2">
                <span className="text-sm text-neutral-dark">Close panel</span>
                <kbd className="px-2 py-1 text-xs font-medium text-neutral-dark/80 bg-white border border-neutral-light rounded">
                  ESC
                </kbd>
              </div>
            </div>
          </div>

          {/* About */}
          <div>
            <div className="flex items-center gap-2 mb-3">
              <Info size={20} className="text-primary" />
              <h3 className="text-base font-semibold text-neutral-dark">About</h3>
            </div>
            <div className="pl-7 space-y-2">
              <p className="text-sm text-neutral-dark">
                <span className="font-medium">Version:</span> 0.1.0
              </p>
              <p className="text-sm text-neutral-dark">
                <span className="font-medium">App:</span> Intento Todo
              </p>
              <p className="text-sm text-neutral-dark/60">
                A modern task management app with AI-powered features
              </p>
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="px-6 py-4 border-t border-neutral-light/60 bg-neutral-light/20 flex gap-3">
          <button
            onClick={saveSettings}
            disabled={isLoading}
            className="flex-1 py-2 px-4 bg-primary text-white font-semibold rounded-lg
                     hover:bg-primary-dark transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isLoading ? '保存中...' : '保存设置'}
          </button>
          <button
            onClick={onClose}
            className="flex-1 py-2 px-4 bg-neutral-light/60 text-neutral-dark font-semibold rounded-lg
                     hover:bg-neutral-light transition-all duration-200"
          >
            关闭
          </button>
        </div>
      </div>
    </div>
  );
}
