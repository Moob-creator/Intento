import { X, Settings as SettingsIcon, Palette, Keyboard, Info, FileText, Key, CheckCircle, XCircle, RefreshCw } from 'lucide-react';
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { CustomSelect, SelectOption } from './CustomSelect';
import { NotificationSettings } from './NotificationSettings';

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

interface ApiKeysSettings {
  base_url?: string;
  api_key?: string;
}

// 常见的 API 域名预设
const API_BASE_URLS: SelectOption[] = [
  { label: 'OpenAI 官方', value: 'https://api.openai.com/v1' },
  { label: 'Anthropic 官方', value: 'https://api.anthropic.com' },
  { label: 'Moonshot 官方', value: 'https://api.moonshot.cn/v1' },
  { label: 'Azure OpenAI', value: 'https://your-resource.openai.azure.com' },
  { label: 'Cloudflare AI Gateway', value: 'https://gateway.ai.cloudflare.com/v1' },
  { label: '自定义', value: 'custom' },
];

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

  const [apiKeys, setApiKeys] = useState<ApiKeysSettings>({});
  const [showApiKey, setShowApiKey] = useState(false);
  const [isSavingApiKeys, setIsSavingApiKeys] = useState(false);
  const [selectedBaseUrl, setSelectedBaseUrl] = useState('https://api.openai.com/v1');
  const [isTestingConnection, setIsTestingConnection] = useState(false);
  const [connectionStatus, setConnectionStatus] = useState<'idle' | 'success' | 'error' | 'test-first'>('idle');

  // Load settings when panel opens
  useEffect(() => {
    if (isOpen) {
      loadSettings();
      loadApiKeys();
    }
  }, [isOpen]);

  const loadSettings = async () => {
    try {
      const settings = await invoke<AutoSummarySettings>('get_auto_summary_settings');
      setAutoSummarySettings(settings);
    } catch (err) {
      console.error('Failed to load auto summary settings:', err);
    }
  };

  const loadApiKeys = async () => {
    try {
      const keys = await invoke<ApiKeysSettings>('get_api_keys');
      setApiKeys(keys);

      // Set initial base URL selection
      if (keys.base_url) {
        const preset = API_BASE_URLS.find(u => u.value === keys.base_url);
        setSelectedBaseUrl(preset ? keys.base_url : 'custom');
      }
    } catch (err) {
      console.error('Failed to load API keys:', err);
    }
  };

  const saveApiKeys = async () => {
    setIsSavingApiKeys(true);
    try {
      await invoke('update_api_keys', {
        baseUrl: apiKeys.base_url || null,
        apiKey: apiKeys.api_key || null,
      });
      alert('API Keys 已保存！');
      setConnectionStatus('test-first');
    } catch (err) {
      console.error('Failed to save API keys:', err);
      alert('保存失败：' + err);
    } finally {
      setIsSavingApiKeys(false);
    }
  };

  const testConnection = async () => {
    setIsTestingConnection(true);
    setConnectionStatus('idle');
    try {
      const isHealthy = await invoke<boolean>('ai_health_check');
      if (isHealthy) {
        setConnectionStatus('success');
      } else {
        setConnectionStatus('error');
      }
    } catch (err) {
      console.error('Connection test failed:', err);
      setConnectionStatus('error');
    } finally {
      setIsTestingConnection(false);
    }
  };

  // 自动保存：设置变更后自动持久化
  const saveSettings = async (settings: AutoSummarySettings) => {
    try {
      await invoke('update_auto_summary_settings', { settings });
    } catch (err) {
      console.error('Failed to save settings:', err);
    }
  };

  const handleToggle = (field: keyof AutoSummarySettings) => {
    setAutoSummarySettings(prev => {
      const updated = { ...prev, [field]: !prev[field] };
      saveSettings(updated);
      return updated;
    });
  };

  const handleRetentionChange = (value: number) => {
    const clamped = Math.max(30, Math.min(3650, value));
    setAutoSummarySettings(prev => {
      const updated = { ...prev, retention_days: clamped };
      saveSettings(updated);
      return updated;
    });
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
          <NotificationSettings className="mb-6" />

          {/* API Keys */}
          <div className="mb-6">
            <div className="flex items-center gap-2 mb-3">
              <Key size={20} className="text-amber-500" />
              <h3 className="text-base font-semibold text-neutral-dark">API Keys</h3>
            </div>
            <div className="space-y-3">
              <p className="text-sm text-neutral-dark/60">
                配置 AI 功能所需的 API Keys。如未设置，将尝试从环境变量读取。
              </p>

              <div className="space-y-4">
                {/* Base URL Select */}
                <div className="p-3 bg-neutral-light/30 rounded-lg">
                  <CustomSelect
                    label="Base URL"
                    value={selectedBaseUrl}
                    options={API_BASE_URLS}
                    onChange={(value) => {
                      setSelectedBaseUrl(value);
                      if (value !== 'custom') {
                        setApiKeys({ ...apiKeys, base_url: value });
                      }
                    }}
                  />

                  {/* Custom URL Input */}
                  {selectedBaseUrl === 'custom' && (
                    <div className="mt-3">
                      <input
                        type="text"
                        value={apiKeys.base_url || ''}
                        onChange={(e) => setApiKeys({ ...apiKeys, base_url: e.target.value })}
                        placeholder="https://api.example.com/v1"
                        className="w-full px-3 py-2.5 text-sm bg-white border border-neutral-light rounded-lg
                                 focus:outline-none focus:ring-2 focus:ring-amber-500/50 select-text transition-all duration-200
                                 placeholder:text-neutral-dark/40"
                      />
                    </div>
                  )}
                </div>

                {/* API Key Input */}
                <div className="p-3 bg-neutral-light/30 rounded-lg">
                  <label className="block text-sm font-medium text-neutral-dark mb-2">
                    API Key
                  </label>
                  <div className="relative">
                    <input
                      type={showApiKey ? 'text' : 'password'}
                      value={apiKeys.api_key || ''}
                      onChange={(e) => setApiKeys({ ...apiKeys, api_key: e.target.value })}
                      placeholder="sk-..."
                      className="w-full px-3 py-2.5 text-sm bg-white border border-neutral-light rounded-lg
                               focus:outline-none focus:ring-2 focus:ring-amber-500/50 select-text transition-all duration-200
                               placeholder:text-neutral-dark/40 pr-10"
                    />
                    <button
                      type="button"
                      onClick={() => setShowApiKey(!showApiKey)}
                      className="absolute right-3 top-1/2 -translate-y-1/2 text-neutral-dark/40 hover:text-neutral-dark
                               transition-colors duration-200"
                      aria-label={showApiKey ? 'Hide API key' : 'Show API key'}
                    >
                      {showApiKey ? (
                        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21" />
                        </svg>
                      ) : (
                        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                        </svg>
                      )}
                    </button>
                  </div>
                </div>

                {/* Action Buttons */}
                <div className="flex gap-3">
                  {/* Test Connection Button */}
                  <button
                    onClick={testConnection}
                    disabled={isTestingConnection || !apiKeys.api_key || !apiKeys.base_url}
                    className="px-4 py-2.5 bg-white border border-neutral-light text-neutral-dark text-sm font-medium rounded-lg
                             hover:bg-neutral-light/30 active:bg-neutral-light/40 disabled:opacity-50 disabled:cursor-not-allowed
                             transition-all duration-200 shadow-sm hover:shadow-md flex items-center gap-2"
                  >
                    {isTestingConnection ? (
                      <>
                        <RefreshCw size={16} className="animate-spin" />
                        测试中...
                      </>
                    ) : (
                      <>
                        <RefreshCw size={16} />
                        测试连接
                      </>
                    )}
                  </button>

                  {/* Save Button */}
                  <button
                    onClick={saveApiKeys}
                    disabled={isSavingApiKeys}
                    className="px-6 py-2.5 bg-amber-500 text-white text-sm font-medium rounded-lg
                             hover:bg-amber-600 active:bg-amber-700 disabled:opacity-50 disabled:cursor-not-allowed
                             transition-all duration-200 shadow-sm hover:shadow-md"
                  >
                    {isSavingApiKeys ? '保存中...' : '保存'}
                  </button>
                </div>

                {/* Connection Status */}
                {connectionStatus !== 'idle' && (
                  <div
                    className={`flex items-center gap-2 p-3 rounded-lg text-sm ${
                      connectionStatus === 'success'
                        ? 'bg-green-50 text-green-700 border border-green-200'
                        : connectionStatus === 'error'
                          ? 'bg-red-50 text-red-700 border border-red-200'
                          : 'bg-amber-50 text-amber-700 border border-amber-200'
                    }`}
                  >
                    {connectionStatus === 'success' && <CheckCircle size={16} />}
                    {connectionStatus === 'error' && <XCircle size={16} />}
                    {connectionStatus === 'test-first' && <RefreshCw size={16} />}
                    <span>
                      {connectionStatus === 'success' && 'API 连接成功！配置有效。'}
                      {connectionStatus === 'error' && 'API 连接失败。请检查 Base URL 和 API Key 是否正确。'}
                      {connectionStatus === 'test-first' && '配置已保存，建议测试连接以确保 API 可用。'}
                    </span>
                  </div>
                )}
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
        <div className="px-6 py-4 border-t border-neutral-light/60 bg-neutral-light/20">
          <button
            onClick={onClose}
            className="w-full py-2 px-4 bg-primary text-white font-semibold rounded-lg
                     hover:bg-primary-dark transition-all duration-200"
          >
            关闭
          </button>
        </div>
      </div>
    </div>
  );
}
