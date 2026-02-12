# Intento Todo - Modern Command-Based Interface

A beautifully redesigned task management application with a Linear/Height-inspired command palette interface, powered by Tauri and React.

## What's New

### Removed Traditional Sidebar
- Clean, maximized workspace for tasks
- All navigation moved to command palette (⌘K)
- Focus on content, not navigation chrome

### New Top Bar
Simple and minimal design:
- **Left**: Intento logo (small, subtle branding)
- **Center**: Global search trigger (opens command palette)
- **Right**: Quick action buttons (AI, Settings, User)

### Command Palette (⌘K)
Powerful keyboard-first navigation:
- **Search tasks**: Type to filter all your tasks
- **Quick actions**:
  - ✨ AI Add Task - Natural language task creation
  - ➕ New Task - Create structured task
  - 📊 View Statistics - Task analytics dashboard
  - 📅 Today's Tasks - Filter to today
  - ⏰ Due Soon - Show expiring tasks
  - ⚙️ Settings - App configuration
  - 💡 Help & Support - Get help
  - 🔔 Test Notification - Test reminder system

### Enhanced Task Cards
Larger, more spacious cards with:
- **Inline quick actions on hover**:
  - ✓ Mark as done/todo
  - ✏️ Edit task
  - 🗑️ Delete task
- Better visual hierarchy
- Priority badges for high-priority tasks
- Tag display
- Improved status indicators

### Floating Panels
Modern panel system:
- **Statistics Panel**: Beautiful analytics with charts and progress bars
- **Settings Panel**: App configuration (notifications, appearance, keyboard shortcuts)
- **Task Detail Panel**: Slides in from right for editing

### Global Keyboard Shortcuts
| Shortcut | Action |
|----------|--------|
| `⌘K` / `Ctrl+K` | Open command palette |
| `⌘N` / `Ctrl+N` | New task |
| `⌘/` / `Ctrl+/` | AI add task |
| `⌘,` / `Ctrl+,` | Settings |
| `ESC` | Close panels |
| `↑` `↓` | Navigate command palette |
| `Enter` | Select command/task |

### Design System
Maintains the warm, friendly aesthetic:
- Soft peach/coral primary colors (#FF8B7B)
- Warm backgrounds and accents
- Smooth animations (200-300ms transitions)
- Generous spacing and rounded corners (12px+)
- Subtle shadows for depth
- Accessible focus states

## Technical Architecture

### New Components

#### `/src/components/CommandPalette.tsx`
Fuzzy search command palette with keyboard navigation:
- Searches both commands and tasks
- Keyboard navigation (arrow keys, enter, escape)
- Categories: actions, filters, navigation
- Real-time filtering

#### `/src/components/TopBar.tsx`
Minimal top bar with draggable region:
- Logo and branding
- Search trigger (opens command palette)
- Quick action buttons
- macOS-style titlebar integration

#### `/src/components/StatisticsPanel.tsx`
Task analytics dashboard:
- Overview cards (Todo, In Progress, Done, Total)
- Completion rate progress bar
- Priority distribution
- Recent activity (last 7 days)

#### `/src/components/SettingsPanel.tsx`
App configuration interface:
- Notifications settings
- Appearance preferences
- Keyboard shortcuts reference
- About section

#### `/src/hooks/useKeyboardShortcuts.ts`
Custom hook for global keyboard shortcuts:
- Cross-platform support (Cmd/Ctrl)
- Clean event listener management
- Declarative shortcut configuration

### Enhanced Components

#### `/src/components/TaskCard.tsx`
- Added inline quick actions (on hover)
- Enhanced visual hierarchy
- Better tag display
- Improved priority indicators
- Smooth transitions

#### `/src/components/TaskList.tsx`
- Better empty state with helpful tips
- Support for inline actions
- Improved spacing

### Updated Main App

#### `/src/App.tsx`
Complete redesign:
- No sidebar, maximized workspace
- Command palette integration
- Multiple floating panels
- Global keyboard shortcuts
- Smart task sorting (priority → status → date)
- Improved state management

## Features

### Command Palette Search
The command palette provides unified search:
- Type to filter commands by name or keywords
- Search across all tasks by title, description, or tags
- Keyboard navigation with arrow keys
- Instant results

### Smart Task Sorting
Tasks are automatically sorted by:
1. **Priority**: High → Medium → Low
2. **Status**: Doing → Todo → Done
3. **Date**: Newest first

### Empty States
Helpful empty states guide users:
- Large friendly icon
- Clear messaging
- Keyboard shortcut hints

### Responsive Design
Works across different window sizes:
- Fluid layouts
- Responsive grid system
- Touch-friendly interaction targets

## Development

### Build
```bash
npm run build
```

### Run Development Server
```bash
npm run dev
npm run tauri dev
```

### Architecture Notes

1. **State Management**: Zustand store (`/src/store/taskStore.ts`) unchanged
2. **Tauri Commands**: All backend integration preserved
3. **Type Safety**: Full TypeScript support
4. **Accessibility**: ARIA labels, keyboard navigation, focus management

## Design Philosophy

The redesign follows these principles:

1. **Keyboard-First**: Power users can accomplish everything via keyboard
2. **Progressive Disclosure**: Advanced features hidden until needed
3. **Visual Hierarchy**: Important information stands out
4. **Smooth Interactions**: All transitions feel natural (200-300ms)
5. **Warm Aesthetics**: Soft colors, rounded corners, gentle shadows
6. **Clarity**: Users immediately understand what they're looking at

## Comparison: Before & After

### Before (Sidebar Interface)
- Fixed left sidebar (264px)
- Traditional navigation menu
- Limited task display space
- Static action buttons in sidebar

### After (Command Palette Interface)
- Full-width workspace
- Command palette navigation (⌘K)
- Maximum space for tasks
- Contextual quick actions
- Floating panels for features
- Global keyboard shortcuts

## Future Enhancements

Potential additions:
- Dark mode theme
- Drag-and-drop task reordering
- Task filters in command palette (by date, priority, tags)
- Recent commands history
- Task templates
- Bulk operations
- Custom keyboard shortcuts

## Credits

Inspired by:
- [Linear](https://linear.app) - Command palette UX
- [Height](https://height.app) - Clean task interface
- Modern design systems with warm, accessible aesthetics
