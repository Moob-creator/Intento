# Notification Settings Component

A warm, user-friendly notification settings component for the Intento Todo application. This component allows users to configure desktop notifications and deadline reminders with a beautiful, accessible interface.

## Features

- **Master Toggle**: Enable/disable all desktop notifications with a single switch
- **Deadline Reminders**: Configure when to be reminded before task deadlines
- **Flexible Timing**: Choose from multiple reminder options (15min, 30min, 1h, 2h, 1 day before)
- **Test Functionality**: Send a test notification to verify everything is working
- **Auto-save**: Settings are automatically persisted when changed
- **Loading States**: Smooth loading animation while fetching settings
- **Error Handling**: Graceful error messages with helpful feedback
- **Visual Feedback**: Active states, animations, and clear visual hierarchy

## Design Philosophy

The component follows Intento's warm, soft design aesthetic:

- **Warm Color Palette**: Uses soft coral, peach, cream, and warm beige tones
- **Gentle Interactions**: Smooth transitions (200-300ms) and rounded corners (8-12px)
- **Clear Hierarchy**: Visual nesting and borders show relationships between settings
- **Accessible**: Proper labels, keyboard navigation, and focus states
- **Contextual Help**: Helpful descriptions and notes guide users

## Usage

### Basic Integration

The component is designed to be used within the `SettingsPanel`:

```tsx
import { NotificationSettings } from './components/NotificationSettings';

// In your SettingsPanel or other parent component
<NotificationSettings className="mb-6" />
```

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `className` | `string` | `''` | Additional CSS classes for styling |

## Component Structure

```
NotificationSettings
├── Header (Bell icon + "Notifications" title)
├── Master Toggle
│   ├── Icon (animated when enabled)
│   ├── Label & Description
│   └── Checkbox
├── Reminder Settings (shown when enabled)
│   ├── Reminder Toggle
│   │   ├── Clock icon
│   │   ├── Label & Description
│   │   └── Checkbox
│   └── Time Selector (shown when reminder enabled)
│       ├── Dropdown (15min, 30min, 1h, 2h, 1 day)
│       └── Helper text
├── Test Button
│   ├── Loading spinner (when testing)
│   └── Success/Error message
└── Info Note
    └── System settings reminder
```

## Backend Integration

The component communicates with the Rust backend via Tauri commands:

### Commands Used

1. **`get_notification_settings`**
   - Loads current settings on mount
   - Returns: `NotificationConfig`

2. **`update_notification_settings`**
   - Auto-saves when settings change
   - Params: `{ settings: NotificationConfig }`

3. **`test_notification`**
   - Sends a test notification
   - Returns: `Result<(), String>`

### Data Structure

```typescript
interface NotificationConfig {
  enabled: boolean;
  reminder_enabled: boolean;
  reminder_minutes: number; // Minutes before deadline
}
```

See [Notification Backend Implementation Guide](./notification-backend-implementation.md) for Rust implementation details.

## Visual States

### 1. Disabled State
- Gray background, muted colors
- Test button disabled
- No sub-settings visible

### 2. Enabled State
- Warm peach/coral background with border
- Animated bell icon (pulse effect)
- Reminder settings become available

### 3. Loading State
- Skeleton loading animation
- Prevents interaction during load

### 4. Testing State
- Button shows spinner
- Disabled during test
- Result message appears below

## Animations

All animations use smooth, gentle easing for a warm feel:

- **Transitions**: `200ms ease-in-out`
- **Hover effects**: `scale-[1.02]` on buttons
- **Slide down**: Test results animate from top
- **Pulse**: Bell icon when notifications enabled

## Accessibility

- **Semantic HTML**: Proper `<label>` elements for all inputs
- **ARIA Labels**: Clear labels for screen readers
- **Keyboard Navigation**: Full keyboard support
- **Focus States**: Visible focus rings with warm colors
- **Color Contrast**: WCAG AA compliant text contrast
- **Error Messages**: Clear, actionable error text

## Customization

### Colors

The component uses Tailwind CSS with custom theme colors:

```javascript
// tailwind.config.js
theme: {
  extend: {
    colors: {
      primary: {
        DEFAULT: '#FF8B7B',
        light: '#FFB88C',
        dark: '#E07A5F',
      },
      accent: {
        peach: '#FFF5E6',
      },
      // ... other colors
    }
  }
}
```

### Reminder Options

To add or modify reminder time options, edit the `REMINDER_OPTIONS` array:

```typescript
const REMINDER_OPTIONS = [
  { value: '15', label: '15 minutes before' },
  { value: '30', label: '30 minutes before' },
  { value: '60', label: '1 hour before' },
  { value: '120', label: '2 hours before' },
  { value: '1440', label: '1 day before' },
  // Add more options here
];
```

## Error Handling

The component handles errors gracefully:

1. **Load Failure**: Falls back to default settings
2. **Save Failure**: Logs error but doesn't break UI
3. **Test Failure**: Shows user-friendly error message with details
4. **Permission Errors**: Info note reminds users to check system settings

## Best Practices

### When to Use

- In app settings or preferences
- During onboarding to configure notifications
- Anywhere users need notification control

### Integration Tips

1. **Loading State**: Always show loading skeleton on initial load
2. **Auto-save**: Save immediately on change, don't require "Save" button
3. **Test Feature**: Encourage users to test after enabling
4. **System Settings**: Remind users about OS-level permissions
5. **Error Recovery**: Don't lose user changes if save fails

## Platform Support

- **macOS**: Full support with notification permissions
- **Windows 10+**: Native WinRT notifications
- **Linux**: Requires notification daemon (e.g., dunst, notify-osd)

## Future Enhancements

Potential improvements for future versions:

- [ ] Sound selection for notifications
- [ ] Quiet hours (don't disturb mode)
- [ ] Different reminder times per priority level
- [ ] Notification history/log
- [ ] Custom notification templates
- [ ] Multiple reminders per task (e.g., 1 day + 1 hour before)
- [ ] Weekly digest notifications
- [ ] In-app notification center

## Dependencies

- **React**: 18+
- **TypeScript**: 5+
- **Tauri**: 2+
- **Tailwind CSS**: 3+
- **lucide-react**: For icons

## File Structure

```
src/
├── components/
│   ├── NotificationSettings.tsx    # Main component
│   └── CustomSelect.tsx             # Used for dropdown
├── types/
│   └── notification.ts              # TypeScript types
└── App.css                          # Animations

docs/
├── notification-backend-implementation.md  # Rust guide
└── notification-settings-readme.md         # This file
```

## Testing Checklist

- [ ] Component renders without backend connection
- [ ] Loading state appears during initial load
- [ ] Master toggle enables/disables all settings
- [ ] Reminder toggle shows/hides time selector
- [ ] Dropdown changes save automatically
- [ ] Test button is disabled when notifications off
- [ ] Test button shows loading state
- [ ] Success message appears after test
- [ ] Error messages are user-friendly
- [ ] All animations are smooth
- [ ] Keyboard navigation works
- [ ] Focus states are visible
- [ ] Works on all platforms (macOS, Windows, Linux)

## Troubleshooting

### Notifications not appearing?

1. Check system notification permissions
2. Verify notification daemon is running (Linux)
3. Test with the "Test Notification" button
4. Check browser/app console for errors

### Settings not saving?

1. Check backend Tauri commands are implemented
2. Verify app has write permissions to data directory
3. Check console for save errors
4. Ensure `update_notification_settings` command exists

### Component not loading?

1. Verify `get_notification_settings` command is implemented
2. Check component import path
3. Ensure `CustomSelect` component is available
4. Check console for errors

## License

Part of the Intento Todo application. See main project license.

## Contributing

When modifying this component:

1. Maintain the warm, soft aesthetic
2. Keep animations smooth and gentle
3. Test on all platforms
4. Update this README with changes
5. Add TypeScript types for new features
6. Follow accessibility guidelines

## Support

For issues or questions:
- Check the main Intento documentation
- Review the backend implementation guide
- Test with the included test button
- Check system notification settings
