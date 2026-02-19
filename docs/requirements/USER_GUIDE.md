# Intento Todo - User Guide

## Getting Started

Welcome to the redesigned Intento Todo! This app is built for speed and efficiency with keyboard shortcuts at its core.

## Core Concept: Command Palette

The heart of Intento is the **Command Palette** (⌘K or Ctrl+K). Think of it as your universal search and action center.

### What can you do with ⌘K?
- Search all your tasks
- Run quick actions
- Navigate anywhere in the app
- Filter and find tasks

**Pro tip**: You can keep your hands on the keyboard for everything!

## Keyboard Shortcuts Reference

### Global Shortcuts

| Shortcut | Action | Description |
|----------|--------|-------------|
| `⌘K` or `Ctrl+K` | Command Palette | Open the universal command center |
| `⌘N` or `Ctrl+N` | New Task | Create a new task (opens detail panel) |
| `⌘/` or `Ctrl+/` | AI Add Task | Open natural language task input |
| `⌘,` or `Ctrl+,` | Settings | Open app settings |
| `ESC` | Close | Close any open panel or dialog |

### Command Palette Shortcuts

| Key | Action |
|-----|--------|
| `↑` `↓` | Navigate up/down |
| `Enter` | Select command or task |
| `ESC` | Close palette |
| Type... | Filter results |

### Task Card Actions

Hover over any task card to reveal quick actions:

| Icon | Action | What it does |
|------|--------|--------------|
| ✓ | Mark Done/Todo | Toggle task completion status |
| ✏️ | Edit | Open task in detail panel |
| 🗑️ | Delete | Remove task (with confirmation) |

## Quick Actions

### Creating Tasks

**Method 1: Manual Entry (⌘N)**
1. Press `⌘N` or click "New Task" button
2. Fill in the details in the side panel
3. Click "Save"

**Method 2: AI Input (⌘/)**
1. Press `⌘/` or click "AI Add" button
2. Describe your task in natural language
3. AI extracts title, description, priority, deadline, and tags
4. Review and confirm
5. Task is created automatically

**Example AI inputs:**
- "Finish quarterly report by Friday, high priority"
- "Buy groceries tomorrow, low priority, personal"
- "Review code for PR #123, tag:work tag:urgent"

### Finding Tasks

**Method 1: Command Palette (⌘K)**
1. Press `⌘K`
2. Type to search
3. Results show matching tasks
4. Use arrow keys to navigate
5. Press Enter to open task

**Method 2: Visual Scan**
- Tasks are sorted by priority, then status
- High-priority tasks appear first
- Color-coded status badges
- Quick glance at deadlines

### Managing Tasks

**Quick Status Change**
1. Hover over task card
2. Click ✓ button
3. Status toggles between Done and Todo

**Full Edit**
1. Hover and click ✏️, or
2. Click anywhere on the card
3. Edit in side panel
4. Click "Save" or press `ESC` to cancel

**Delete Task**
1. Hover over task
2. Click 🗑️ button
3. Confirm deletion

## Understanding Task Cards

Each task card shows:

```
┌─────────────────────────────────────────────┐
│ Task Title                        [Priority]│ ← Priority badge (high only)
│ Description text here...                    │ ← First line of description
│ [Status] 📅 Deadline  #tag #tag            │ ← Metadata row
└─────────────────────────────────────────────┘
```

### Status Badges
- **Todo** - Gray badge, task not started
- **Doing** - Blue badge, work in progress
- **Done** - Green badge, completed (shown with opacity)

### Priority Badges
- **High** - Red badge (always shown)
- **Medium** - No badge (default)
- **Low** - No badge

### Deadline Display
- **Overdue** - Red text
- **Today** - Shows "Today"
- **Tomorrow** - Shows "Tomorrow"
- **Within 7 days** - Shows "In X days"
- **Later** - Shows date (e.g., "Dec 15")

## Using the Statistics Panel

Access with: `⌘K` → "View Statistics"

**What you'll see:**
1. **Overview Cards**
   - Total tasks
   - To Do count
   - In Progress count
   - Completed count

2. **Completion Rate**
   - Visual progress bar
   - Percentage completed

3. **Priority Distribution**
   - High, Medium, Low counts
   - Color-coded for quick scanning

4. **Recent Activity**
   - Tasks completed in last 7 days
   - Quick productivity check

## Settings Panel

Access with: `⌘,` or `⌘K` → "Settings"

### Notifications Section
- **Enable desktop notifications**: Toggle system notifications
- **Remind me of deadlines**: Get reminders before due dates
- **Reminder hours before**: How early to remind (default: 24 hours)

### Appearance Section
- **Theme**: Currently Light (Warm) - Dark mode coming soon
- **Compact mode**: Tighter spacing (future feature)

### Keyboard Shortcuts
- Quick reference card
- All shortcuts listed
- Always accessible in settings

## Tips & Tricks

### Productivity Workflows

**Morning Review**
1. Press `⌘K`
2. Type "statistics"
3. Review yesterday's completion
4. Press `ESC`
5. Start working on high-priority tasks

**Quick Task Creation**
1. Press `⌘/`
2. Describe task naturally
3. Let AI parse it
4. Confirm and done!

**Task Triage**
1. Hover over tasks
2. Use ✓ to mark simple tasks done
3. Use ✏️ for tasks needing updates
4. Keep hands on keyboard

### Keyboard Shortcuts

**Speed Navigation**
- `⌘K` to search anything
- Type to filter
- Arrow keys to navigate
- Enter to select
- ESC to close

**Quick Actions**
- `⌘N` for new task
- `⌘/` for AI task
- `⌘,` for settings
- ESC to close any panel

### Organization

**Use Tags**
- Group related tasks
- #work, #personal, #urgent
- #project-name for projects
- Quickly search by tag

**Set Priorities**
- High: Must do today
- Medium: Should do soon
- Low: When you have time

**Set Deadlines**
- Tasks with deadlines show in statistics
- Get reminded before due date
- Overdue tasks highlighted in red

## Common Questions

**Q: How do I search for a specific task?**
A: Press `⌘K` and start typing. Tasks matching your search will appear below commands.

**Q: Can I mark a task as done without opening it?**
A: Yes! Hover over the task card and click the ✓ button that appears.

**Q: What's the fastest way to create a task?**
A: Press `⌘/` and describe it naturally. For example: "Meeting with team tomorrow at 2pm, high priority"

**Q: How do I see my productivity stats?**
A: Press `⌘K`, type "statistics", and press Enter. Or click the chart icon in the command palette.

**Q: Can I use Intento without a mouse?**
A: Absolutely! That's the point. Use ⌘K for navigation, arrow keys to move around, and Enter to select.

**Q: Where did the sidebar go?**
A: We removed it to give you more space for tasks. Everything is now in the command palette (⌘K).

**Q: How do I change settings?**
A: Press `⌘,` or open the command palette (⌘K) and search for "settings".

**Q: What happens to completed tasks?**
A: They stay in your list but appear with lower opacity and a green checkmark. You can still edit or delete them.

**Q: Can I test notifications?**
A: Yes! Open the command palette (⌘K) and run "Test Notification".

## Advanced Features

### Task Sorting

Tasks are automatically sorted by:
1. **Priority** - High priority tasks always appear first
2. **Status** - Doing tasks before Todo, Todo before Done
3. **Date** - Newest tasks first within same priority/status

This ensures your most important work is always visible.

### Smart Empty States

When you have no tasks, you'll see helpful guidance on how to get started with keyboard shortcuts shown.

### AI Task Parsing

The AI can extract:
- **Title** from your description
- **Description** (optional details)
- **Priority** from keywords (high, medium, low)
- **Deadline** from natural dates (tomorrow, Friday, Dec 15)
- **Tags** from #hashtags or explicit mentions

**Example:**
> "Review the Q4 financials by end of week, this is high priority, tag it with #finance and #quarterly-review"

Becomes:
- Title: "Review the Q4 financials"
- Deadline: End of this week
- Priority: High
- Tags: finance, quarterly-review

## Best Practices

1. **Start your day with ⌘K → statistics** to review progress
2. **Use AI input (⌘/)** for quick task capture
3. **Set priorities wisely** - not everything is high priority
4. **Add deadlines** to important tasks
5. **Use tags** to group related work
6. **Review completed tasks** in statistics to feel accomplished
7. **Keep task titles clear** and action-oriented
8. **Use descriptions** for additional context
9. **Mark tasks done immediately** with hover actions
10. **Close panels with ESC** to keep workflow smooth

## Troubleshooting

**Command palette not opening?**
- Try `Ctrl+K` (Windows/Linux) or `⌘K` (Mac)
- Make sure no other app is capturing the shortcut

**Shortcuts not working?**
- Click once in the app window to focus it
- Check if another app has captured that shortcut

**Task not saving?**
- Make sure title is filled (required field)
- Check for error messages at the top

**AI parsing not working?**
- Ensure you have an internet connection
- Try rephrasing your input
- Fall back to manual entry (⌘N)

## Feedback & Support

Found a bug or have a suggestion?
- Open the command palette (⌘K)
- Select "Help & Support"

---

**Enjoy the redesigned Intento!** 🎉

Remember: Everything is just a `⌘K` away!
