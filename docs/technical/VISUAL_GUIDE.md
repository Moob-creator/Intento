# Visual Layout Guide

## Main Interface Layout

```
┌─────────────────────────────────────────────────────────────────────┐
│  TOPBAR (h-14, bg-white/80, backdrop-blur)                         │
│  ┌─────────┬────────────────────────────────────┬─────────────────┐│
│  │  [I]    │  [Search tasks... ⌘K]              │ [✨] [⚙️] [👤]  ││
│  │ Intento │                                     │                 ││
│  └─────────┴────────────────────────────────────┴─────────────────┘│
├─────────────────────────────────────────────────────────────────────┤
│  MAIN CONTENT (flex-1, overflow-hidden)                            │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │  Quick Actions Bar                                           │ │
│  │  ┌────────────────────────┬──────────────────────────────┐  │ │
│  │  │ All Tasks              │  [✨ AI Add] [➕ New Task]   │  │ │
│  │  │ X tasks total          │                              │  │ │
│  │  └────────────────────────┴──────────────────────────────┘  │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │  Task Card (hover for actions)                               │ │
│  │  ┌─────────────────────────────────────────────┬──────────┐  │ │
│  │  │ Task Title                             [High]│ [✓][✏️][🗑]│ │
│  │  │ Description text here...                    │          │  │ │
│  │  │ [Todo] 📅 Due: Tomorrow  #tag #tag         │          │  │ │
│  │  └─────────────────────────────────────────────┴──────────┘  │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                     │
│  [More task cards...]                                              │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

## Command Palette (⌘K)

```
┌─────────────────────────────────────────────────────────────────────┐
│                     BACKDROP (black/40, blur)                       │
│                                                                     │
│         ┌─────────────────────────────────────────────────┐        │
│         │  COMMAND PALETTE (white, rounded-xl, shadow-2xl)│        │
│         │ ┌─────────────────────────────────────────────┐ │        │
│         │ │ [🔍] Search tasks or run a command... [ESC]│ │        │
│         │ └─────────────────────────────────────────────┘ │        │
│         │ ┌─────────────────────────────────────────────┐ │        │
│         │ │ COMMANDS                                    │ │        │
│         │ │ [✨] AI Add Task                           │ │        │
│         │ │ [➕] New Task                              │ │        │
│         │ │ [📊] View Statistics                       │ │        │
│         │ │ [📅] Today's Tasks                         │ │        │
│         │ │ [⏰] Due Soon                              │ │        │
│         │ │ [⚙️] Settings                              │ │        │
│         │ │ [💡] Help & Support                        │ │        │
│         │ │ [🔔] Test Notification                     │ │        │
│         │ └─────────────────────────────────────────────┘ │        │
│         │ ┌─────────────────────────────────────────────┐ │        │
│         │ │ TASKS (5)                                   │ │        │
│         │ │ [○] Task title here                        │ │        │
│         │ │ [✓] Completed task                         │ │        │
│         │ └─────────────────────────────────────────────┘ │        │
│         │ ┌─────────────────────────────────────────────┐ │        │
│         │ │ [↑][↓] navigate  [↵] select      [⌘K] open│ │        │
│         │ └─────────────────────────────────────────────┘ │        │
│         └─────────────────────────────────────────────────┘        │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

## Statistics Panel

```
┌─────────────────────────────────────────────────────────────────────┐
│                     BACKDROP (black/40, blur)                       │
│                                                                     │
│         ┌─────────────────────────────────────────────────┐        │
│         │  STATISTICS PANEL                         [✕]  │        │
│         │ ┌───────────────────────────────────────────┐   │        │
│         │ │ 📊 Task Statistics                        │   │        │
│         │ └───────────────────────────────────────────┘   │        │
│         │ ┌─────────┬─────────┬─────────┬─────────────┐  │        │
│         │ │  [○]    │  [⏳]   │  [✓]    │   [📅]      │  │        │
│         │ │  Todo   │ In Prog │ Done    │   Total     │  │        │
│         │ │   12    │    5    │   23    │    40       │  │        │
│         │ └─────────┴─────────┴─────────┴─────────────┘  │        │
│         │ ┌───────────────────────────────────────────┐  │        │
│         │ │ Completion Rate                      65%  │  │        │
│         │ │ ████████████████░░░░░░░░░░░░░░░░░░░░     │  │        │
│         │ └───────────────────────────────────────────┘  │        │
│         │ ┌───────────────────────────────────────────┐  │        │
│         │ │ Tasks by Priority                         │  │        │
│         │ │ High Priority              5              │  │        │
│         │ │ Medium Priority           20              │  │        │
│         │ │ Low Priority              15              │  │        │
│         │ └───────────────────────────────────────────┘  │        │
│         │ ┌───────────────────────────────────────────┐  │        │
│         │ │ Recent Activity                           │  │        │
│         │ │ Tasks completed in last 7 days:    8      │  │        │
│         │ └───────────────────────────────────────────┘  │        │
│         │ ┌───────────────────────────────────────────┐  │        │
│         │ │              [Close]                      │  │        │
│         │ └───────────────────────────────────────────┘  │        │
│         └─────────────────────────────────────────────────┘        │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

## Settings Panel

```
┌─────────────────────────────────────────────────────────────────────┐
│                     BACKDROP (black/40, blur)                       │
│                                                                     │
│         ┌─────────────────────────────────────────────────┐        │
│         │  ⚙️ Settings                              [✕]  │        │
│         │ ┌───────────────────────────────────────────┐   │        │
│         │ │ 🔔 Notifications                          │   │        │
│         │ │   □ Enable desktop notifications         │   │        │
│         │ │   ☑ Remind me of deadlines               │   │        │
│         │ │   Remind me before: [24] hours           │   │        │
│         │ └───────────────────────────────────────────┘   │        │
│         │ ┌───────────────────────────────────────────┐   │        │
│         │ │ 🎨 Appearance                             │   │        │
│         │ │   Theme: [Light (Warm) ▼]                │   │        │
│         │ │   □ Compact mode                         │   │        │
│         │ └───────────────────────────────────────────┘   │        │
│         │ ┌───────────────────────────────────────────┐   │        │
│         │ │ ⌨️ Keyboard Shortcuts                     │   │        │
│         │ │   Open command palette         [⌘K]      │   │        │
│         │ │   New task                     [⌘N]      │   │        │
│         │ │   AI add task                  [⌘/]      │   │        │
│         │ │   Settings                     [⌘,]      │   │        │
│         │ │   Close panel                  [ESC]     │   │        │
│         │ └───────────────────────────────────────────┘   │        │
│         │ ┌───────────────────────────────────────────┐   │        │
│         │ │ ℹ️ About                                  │   │        │
│         │ │   Version: 0.1.0                         │   │        │
│         │ │   App: Intento Todo                      │   │        │
│         │ └───────────────────────────────────────────┘   │        │
│         │ ┌───────────────────────────────────────────┐   │        │
│         │ │              [Close]                      │   │        │
│         │ └───────────────────────────────────────────┘   │        │
│         └─────────────────────────────────────────────────┘        │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

## AI Text Input (Slide from Bottom)

```
┌─────────────────────────────────────────────────────────────────────┐
│  Main Content                                                       │
│                                                                     │
│  [Tasks displayed here...]                                         │
│                                                                     │
├─────────────────────────────────────────────────────────────────────┤
│  AI INPUT PANEL (slide-up animation)                               │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ ✨ Describe your task and AI will help create it        [✕] │  │
│  │ ┌────────────────────────────────────────────────┬─────────┐ │  │
│  │ │ e.g., 'Finish the quarterly report             │ [Send]  │ │  │
│  │ │ by Friday, high priority, work project'        │    📤   │ │  │
│  │ │                                                 │         │ │  │
│  │ └────────────────────────────────────────────────┴─────────┘ │  │
│  │ Press ⌘/Ctrl + Enter to submit                               │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

## Task Detail Panel (Slide from Right)

```
┌───────────────────────────────────┬─────────────────────────────────┐
│  Main Content                     │  TASK DETAIL PANEL              │
│                                   │  (slide-in from right)          │
│  [Tasks list here...]            │ ┌─────────────────────────────┐ │
│                                   │ │ Edit Task              [✕] │ │
│                                   │ └─────────────────────────────┘ │
│                                   │ ┌─────────────────────────────┐ │
│                                   │ │ Title:                      │ │
│                                   │ │ [___________________________]│ │
│                                   │ │                             │ │
│                                   │ │ Description:                │ │
│                                   │ │ [___________________________]│ │
│                                   │ │ [___________________________]│ │
│                                   │ │ [___________________________]│ │
│                                   │ │                             │ │
│                                   │ │ Status: [Todo ▼]           │ │
│                                   │ │ Priority: [Medium ▼]       │ │
│                                   │ │ Deadline: [Pick date...]   │ │
│                                   │ │ Tags: [tag1, tag2...]      │ │
│                                   │ │                             │ │
│                                   │ │ [Save]  [Delete]  [Cancel] │ │
│                                   │ └─────────────────────────────┘ │
└───────────────────────────────────┴─────────────────────────────────┘
```

## Responsive Layouts

### Desktop (> 1024px)
- Full layout as shown above
- 4-column statistics grid
- Side-by-side panels possible

### Tablet (768px - 1024px)
- Top bar unchanged
- 2-column statistics grid
- Full-width task cards
- Panels stack vertically

### Mobile (< 768px)
- Simplified top bar
- 1-column layout throughout
- Full-screen panels
- Touch-optimized spacing

## Color Coding

### Status Colors
```
🔵 Todo (doing)    - Blue backgrounds
⚪ Todo (not started) - Gray backgrounds
🟢 Done           - Green backgrounds
🔴 Overdue        - Red backgrounds
```

### Priority Colors
```
🔴 High    - Red badges
🟡 Medium  - Amber badges
🔵 Low     - Gray badges
```

### Interactive States
```
Default    - Neutral colors
Hover      - Soft shadow, border highlight
Active     - Pressed state, darker
Focus      - Ring with primary color
Disabled   - Opacity 50%, no interaction
```

## Z-Index Layers

```
Layer 50: Command Palette, Modal Panels
Layer 40: AI Input, Confirmation Dialogs
Layer 30: Task Detail Panel (side)
Layer 20: Tooltips, Dropdowns
Layer 10: Top Bar (sticky)
Layer 0:  Main Content
```

## Animation Timings

```
Fast:    150ms - Hover effects, ripples
Normal:  200ms - Default transitions
Medium:  300ms - Panel slides
Slow:    500ms - Page transitions (future)
```

## Spacing System

```
xs: 0.25rem (4px)   - Tiny gaps
sm: 0.5rem  (8px)   - Small gaps
md: 1rem    (16px)  - Default gaps
lg: 1.5rem  (24px)  - Large gaps
xl: 2rem    (32px)  - Extra large gaps
2xl: 3rem   (48px)  - Section spacing
```
