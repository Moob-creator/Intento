import { NotificationSettings } from './NotificationSettings';

/**
 * Test/Demo page for NotificationSettings component
 *
 * Usage:
 * 1. Import this in your App.tsx for testing
 * 2. Or create a dedicated test route
 * 3. View different states and interactions
 *
 * This component helps verify:
 * - Visual appearance matches design
 * - All interactions work correctly
 * - Animations are smooth
 * - Error states display properly
 * - Loading states show correctly
 */
export function NotificationSettingsTest() {
  return (
    <div className="min-h-screen bg-background p-8">
      <div className="max-w-3xl mx-auto space-y-8">
        {/* Header */}
        <div className="text-center">
          <h1 className="text-3xl font-bold text-neutral-dark mb-2">
            Notification Settings Test
          </h1>
          <p className="text-neutral-dark/60">
            Verify the component works correctly before deployment
          </p>
        </div>

        {/* Component under test */}
        <div className="bg-white rounded-2xl shadow-soft p-6">
          <NotificationSettings />
        </div>

        {/* Test checklist */}
        <div className="bg-amber-50 border border-amber-200 rounded-2xl p-6">
          <h2 className="text-lg font-semibold text-neutral-dark mb-4">
            Testing Checklist
          </h2>

          <div className="space-y-3 text-sm">
            <label className="flex items-start gap-3 cursor-pointer group">
              <input
                type="checkbox"
                className="mt-1 w-4 h-4 text-primary rounded"
              />
              <span className="text-neutral-dark group-hover:text-primary transition-colors">
                <strong>Visual Design:</strong> Colors are warm (soft coral, peach, cream).
                No harsh blacks or neon colors.
              </span>
            </label>

            <label className="flex items-start gap-3 cursor-pointer group">
              <input
                type="checkbox"
                className="mt-1 w-4 h-4 text-primary rounded"
              />
              <span className="text-neutral-dark group-hover:text-primary transition-colors">
                <strong>Master Toggle:</strong> Enabling shows sub-settings.
                Disabling hides them and disables test button.
              </span>
            </label>

            <label className="flex items-start gap-3 cursor-pointer group">
              <input
                type="checkbox"
                className="mt-1 w-4 h-4 text-primary rounded"
              />
              <span className="text-neutral-dark group-hover:text-primary transition-colors">
                <strong>Reminder Toggle:</strong> Shows/hides time selector.
                Border highlights the section.
              </span>
            </label>

            <label className="flex items-start gap-3 cursor-pointer group">
              <input
                type="checkbox"
                className="mt-1 w-4 h-4 text-primary rounded"
              />
              <span className="text-neutral-dark group-hover:text-primary transition-colors">
                <strong>Time Dropdown:</strong> Opens with smooth animation.
                Shows 5 options (15min, 30min, 1h, 2h, 1 day).
              </span>
            </label>

            <label className="flex items-start gap-3 cursor-pointer group">
              <input
                type="checkbox"
                className="mt-1 w-4 h-4 text-primary rounded"
              />
              <span className="text-neutral-dark group-hover:text-primary transition-colors">
                <strong>Test Button:</strong> Shows loading spinner.
                Displays success or error message. Auto-dismisses after 5 seconds.
              </span>
            </label>

            <label className="flex items-start gap-3 cursor-pointer group">
              <input
                type="checkbox"
                className="mt-1 w-4 h-4 text-primary rounded"
              />
              <span className="text-neutral-dark group-hover:text-primary transition-colors">
                <strong>Animations:</strong> All transitions are smooth (200-300ms).
                Hover effects are gentle.
              </span>
            </label>

            <label className="flex items-start gap-3 cursor-pointer group">
              <input
                type="checkbox"
                className="mt-1 w-4 h-4 text-primary rounded"
              />
              <span className="text-neutral-dark group-hover:text-primary transition-colors">
                <strong>Accessibility:</strong> Can navigate with keyboard (Tab).
                Focus states are visible. Screen reader labels make sense.
              </span>
            </label>

            <label className="flex items-start gap-3 cursor-pointer group">
              <input
                type="checkbox"
                className="mt-1 w-4 h-4 text-primary rounded"
              />
              <span className="text-neutral-dark group-hover:text-primary transition-colors">
                <strong>Loading State:</strong> Shows skeleton on initial load.
                Doesn't block interaction unnecessarily.
              </span>
            </label>

            <label className="flex items-start gap-3 cursor-pointer group">
              <input
                type="checkbox"
                className="mt-1 w-4 h-4 text-primary rounded"
              />
              <span className="text-neutral-dark group-hover:text-primary transition-colors">
                <strong>Auto-save:</strong> Changes save immediately.
                No confusing "Save" button needed.
              </span>
            </label>

            <label className="flex items-start gap-3 cursor-pointer group">
              <input
                type="checkbox"
                className="mt-1 w-4 h-4 text-primary rounded"
              />
              <span className="text-neutral-dark group-hover:text-primary transition-colors">
                <strong>Error Handling:</strong> If backend fails, shows friendly error.
                Doesn't crash the app.
              </span>
            </label>
          </div>
        </div>

        {/* Backend status */}
        <div className="bg-blue-50 border border-blue-200 rounded-2xl p-6">
          <h2 className="text-lg font-semibold text-neutral-dark mb-3">
            Backend Commands Required
          </h2>
          <p className="text-sm text-neutral-dark/70 mb-4">
            The component will try to call these Tauri commands.
            Make sure they're implemented in your Rust backend:
          </p>
          <ul className="space-y-2 text-sm font-mono">
            <li className="flex items-center gap-2">
              <code className="bg-white px-2 py-1 rounded border border-blue-200">
                get_notification_settings
              </code>
              <span className="text-neutral-dark/60">→ Returns NotificationConfig</span>
            </li>
            <li className="flex items-center gap-2">
              <code className="bg-white px-2 py-1 rounded border border-blue-200">
                update_notification_settings
              </code>
              <span className="text-neutral-dark/60">→ Saves settings</span>
            </li>
            <li className="flex items-center gap-2">
              <code className="bg-white px-2 py-1 rounded border border-blue-200">
                test_notification
              </code>
              <span className="text-neutral-dark/60">→ Sends test notification</span>
            </li>
          </ul>
          <p className="text-xs text-neutral-dark/60 mt-4">
            See <code>docs/notification-quick-start.md</code> for implementation guide
          </p>
        </div>

        {/* Tips */}
        <div className="bg-emerald-50 border border-emerald-200 rounded-2xl p-6">
          <h2 className="text-lg font-semibold text-neutral-dark mb-3">
            Testing Tips
          </h2>
          <ul className="space-y-2 text-sm text-neutral-dark/80">
            <li className="flex items-start gap-2">
              <span className="text-emerald-600 font-bold">→</span>
              <span>Test with keyboard only (Tab to navigate, Space to toggle)</span>
            </li>
            <li className="flex items-start gap-2">
              <span className="text-emerald-600 font-bold">→</span>
              <span>Try on different screen sizes (resize window)</span>
            </li>
            <li className="flex items-start gap-2">
              <span className="text-emerald-600 font-bold">→</span>
              <span>Check system notifications after clicking test button</span>
            </li>
            <li className="flex items-start gap-2">
              <span className="text-emerald-600 font-bold">→</span>
              <span>Verify settings persist after restarting the app</span>
            </li>
            <li className="flex items-start gap-2">
              <span className="text-emerald-600 font-bold">→</span>
              <span>Test with backend disconnected (should show friendly errors)</span>
            </li>
          </ul>
        </div>
      </div>
    </div>
  );
}

/**
 * How to use this test component:
 *
 * Option 1 - Temporary testing in App.tsx:
 * ```tsx
 * import { NotificationSettingsTest } from './components/NotificationSettingsTest';
 *
 * // Replace your App component temporarily with:
 * function App() {
 *   return <NotificationSettingsTest />;
 * }
 * ```
 *
 * Option 2 - Add as a route:
 * ```tsx
 * <Route path="/test/notifications" element={<NotificationSettingsTest />} />
 * ```
 *
 * Option 3 - Add to command palette:
 * ```tsx
 * {
 *   id: 'test-notifications',
 *   label: 'Test Notification Settings',
 *   icon: <TestTube2 size={18} />,
 *   action: () => navigate('/test/notifications')
 * }
 * ```
 */
