import { X, Settings as SettingsIcon, Bell, Palette, Keyboard, Info } from 'lucide-react';

interface SettingsPanelProps {
  isOpen: boolean;
  onClose: () => void;
}

export function SettingsPanel({ isOpen, onClose }: SettingsPanelProps) {
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
                <label className="block text-sm text-neutral-dark mb-2">Theme</label>
                <select className="w-full px-3 py-2 bg-white border border-neutral-light rounded-lg text-neutral-dark focus:outline-none focus:ring-2 focus:ring-primary/50">
                  <option>Light (Warm)</option>
                  <option>Dark (Coming Soon)</option>
                  <option>Auto</option>
                </select>
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
            Close
          </button>
        </div>
      </div>
    </div>
  );
}
