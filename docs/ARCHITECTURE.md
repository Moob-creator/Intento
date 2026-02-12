# Component Architecture Summary

## New Component Tree

```
App.tsx (Redesigned)
├── TopBar
│   ├── Logo (Intento)
│   ├── Search Trigger (→ Command Palette)
│   └── Quick Actions (AI, Settings, User)
│
├── Main Content Area
│   ├── Quick Actions Bar
│   │   ├── "All Tasks" Header + Count
│   │   ├── AI Add Button
│   │   └── New Task Button
│   │
│   └── TaskList
│       └── TaskCard (Enhanced)
│           ├── Title + Priority Badge
│           ├── Description
│           ├── Status Badge + Deadline + Tags
│           └── Hover Actions
│               ├── Mark Done/Todo
│               ├── Edit
│               └── Delete
│
├── TaskDetailPanel (Slide from Right)
│   └── [Existing component - preserved]
│
├── AI Text Input Panel (Slide from Bottom)
│   ├── Header (Sparkles icon + title)
│   ├── Textarea (Natural language input)
│   ├── Submit Button
│   └── Keyboard hint (⌘Enter)
│
├── CommandPalette (⌘K)
│   ├── Search Input
│   ├── Commands Section
│   │   ├── AI Add Task
│   │   ├── New Task
│   │   ├── View Statistics
│   │   ├── Today's Tasks
│   │   ├── Due Soon
│   │   ├── Settings
│   │   ├── Help & Support
│   │   └── Test Notification
│   ├── Tasks Section (Search Results)
│   └── Footer (Keyboard hints)
│
├── StatisticsPanel (Modal)
│   ├── Overview Cards
│   │   ├── To Do Count
│   │   ├── In Progress Count
│   │   ├── Completed Count
│   │   └── Total Count
│   ├── Completion Rate Bar
│   ├── Priority Distribution
│   │   ├── High Priority
│   │   ├── Medium Priority
│   │   └── Low Priority
│   └── Recent Activity (Last 7 days)
│
├── SettingsPanel (Modal)
│   ├── Notifications Section
│   │   ├── Enable desktop notifications
│   │   ├── Remind me of deadlines
│   │   └── Reminder hours before
│   ├── Appearance Section
│   │   ├── Theme selector
│   │   └── Compact mode toggle
│   ├── Keyboard Shortcuts Reference
│   └── About Section
│
└── TaskConfirmDialog
    └── [Existing component - preserved]
```

## File Structure

```
src/
├── App.tsx (✨ Completely redesigned)
├── App.css (✨ Added animations)
│
├── components/
│   ├── CommandPalette.tsx (✨ NEW)
│   ├── TopBar.tsx (✨ NEW)
│   ├── StatisticsPanel.tsx (✨ NEW)
│   ├── SettingsPanel.tsx (✨ NEW)
│   ├── TaskCard.tsx (🔄 Enhanced with inline actions)
│   ├── TaskList.tsx (🔄 Enhanced with better empty state)
│   ├── TaskDetailPanel.tsx (✅ Preserved)
│   ├── TaskConfirmDialog.tsx (✅ Preserved)
│   ├── TaskSearchBar.tsx (❌ Removed - replaced by command palette)
│   └── StatusFilter.tsx (❌ Removed - replaced by command palette)
│
├── hooks/
│   └── useKeyboardShortcuts.ts (✨ NEW)
│
├── store/
│   └── taskStore.ts (✅ Preserved)
│
└── types/
    └── task.ts (✅ Preserved)
```

## State Management

### Local State in App.tsx
```typescript
// Panel visibility
const [commandPaletteOpen, setCommandPaletteOpen] = useState(false);
const [statisticsPanelOpen, setStatisticsPanelOpen] = useState(false);
const [settingsPanelOpen, setSettingsPanelOpen] = useState(false);

// AI input
const [textInputVisible, setTextInputVisible] = useState(false);
const [textInput, setTextInput] = useState('');
const [isParsing, setIsParsing] = useState(false);
const [parseError, setParseError] = useState<string | null>(null);

// Task confirmation
const [showConfirmDialog, setShowConfirmDialog] = useState(false);
const [parsedTask, setParsedTask] = useState<ParsedTask | null>(null);
```

### Global State (Zustand)
- `tasks` - All tasks
- `selectedTask` - Currently selected task for editing
- `isLoading` - Loading state
- `error` - Error message
- Task CRUD operations (preserved)

## Keyboard Shortcuts Implementation

```typescript
useKeyboardShortcuts([
  { key: 'k', metaKey: true, handler: () => setCommandPaletteOpen(prev => !prev) },
  { key: 'n', metaKey: true, handler: handleNewTask },
  { key: '/', metaKey: true, handler: handleOpenTextInput },
  { key: ',', metaKey: true, handler: () => setSettingsPanelOpen(true) },
  { key: 'Escape', handler: handleEscapeKey },
]);
```

Cross-platform support:
- macOS: Uses `metaKey` (⌘)
- Windows/Linux: Uses `ctrlKey` (Ctrl)
- Automatically detected via `navigator.platform`

## Panel System

### Modal Panels (Center Screen)
- **CommandPalette**: Full modal with backdrop
- **StatisticsPanel**: Center modal with backdrop
- **SettingsPanel**: Center modal with backdrop

### Slide Panels
- **TaskDetailPanel**: Slides from right
- **AI Text Input**: Slides from bottom

### Stacking Order (z-index)
```
50 - Command Palette, Stats Panel, Settings Panel
40 - AI Text Input Panel, Task Confirm Dialog
30 - Task Detail Panel
10 - Top Bar (sticky)
0  - Main content
```

## Animation System

### CSS Animations
```css
@keyframes slideUp {
  from { transform: translateY(100%); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}
```

### Transition Classes
- All interactive elements: `transition-all duration-200`
- Hover effects: Scale, shadow, color changes
- Focus states: Ring with primary color

## Responsive Breakpoints

```css
sm: 640px   // Mobile landscape
md: 768px   // Tablet
lg: 1024px  // Desktop
xl: 1280px  // Large desktop
```

Grid adapts:
- 1 column on mobile
- 2 columns on tablet
- 4 columns on desktop (stats cards)

## Color Palette

### Primary Colors
- `primary`: #FF8B7B (Soft coral)
- `primary-light`: #FFB88C (Warm peach)
- `primary-dark`: #E07A5F (Muted terracotta)

### Backgrounds
- `background`: #FAFAFA (Soft white)
- `background-warm`: #F8F6F4 (Warm gray)
- `background-card`: #FFFFFF (Pure white)

### Neutrals
- `neutral-light`: #F5E6D3 (Cream)
- `neutral`: #E8DCC8 (Light beige)
- `neutral-dark`: #4A4A4A (Warm dark gray)

### Accents
- `accent-gold`: #FFD966
- `accent-terracotta`: #E07A5F
- `accent-peach`: #FFF5E6

### Status Colors
- `status-todo`: #9CA3AF (Gray)
- `status-doing`: #60A5FA (Blue)
- `status-done`: #34D399 (Green)
- `status-overdue`: #EF4444 (Red)

## Accessibility Features

1. **Keyboard Navigation**
   - All actions accessible via keyboard
   - Visible focus states
   - Logical tab order

2. **ARIA Labels**
   - All buttons have aria-label or title
   - Panels have proper roles
   - Live regions for dynamic content

3. **Focus Management**
   - Auto-focus on panel open (command palette, AI input)
   - Focus trap in modals
   - Return focus on close

4. **Color Contrast**
   - All text meets WCAG AA standards
   - Status colors have sufficient contrast
   - Hover states clearly visible

5. **Screen Readers**
   - Semantic HTML (header, main, section)
   - Descriptive button labels
   - Status announcements for loading/error states
