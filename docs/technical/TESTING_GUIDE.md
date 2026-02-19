# Task Management UI - Testing & Usage Guide

## Quick Start

### Running the Application

```bash
# Terminal 1: Start the Tauri app
npm run tauri dev

# The app will open in a native window with full backend integration
```

## Feature Testing Checklist

### ✅ Task Creation

**Steps:**
1. Click "New Task" button in the left sidebar
2. Verify the detail panel opens on the right
3. Fill in the form:
   - Title: "Complete project documentation"
   - Description: "Write comprehensive docs for the task management feature"
   - Status: To Do
   - Priority: High
   - Due Date: Select tomorrow's date
4. Click "Save"
5. Verify:
   - New task appears in the task list
   - Task is automatically selected (highlighted)
   - Detail panel shows the saved task

**Expected Result:**
- Task created successfully
- Task appears with "To Do" gray badge
- Deadline shows "Tomorrow"

---

### ✅ Task Editing

**Steps:**
1. Click on any existing task card in the list
2. Verify the detail panel updates with task data
3. Modify the status from "To Do" to "Doing"
4. Add/edit description text
5. Click "Save"
6. Verify:
   - Task card updates with blue "Doing" badge
   - If detail panel was showing different task, it updates
   - Changes are persisted (refresh and check)

**Expected Result:**
- Task updates immediately in the list
- Status badge color changes
- No errors in console

---

### ✅ Status Filtering

**Steps:**
1. Create tasks with different statuses (To Do, Doing, Done)
2. Click "All" filter - see all tasks
3. Click "To Do" filter - only gray-badged tasks visible
4. Click "Doing" filter - only blue-badged tasks visible
5. Click "Done" filter - only green-badged tasks visible (with strikethrough)
6. Click "All" again - all tasks reappear

**Expected Result:**
- Filter buttons highlight when active (warm coral background)
- Task list updates instantly
- No flicker or loading delay
- Count of visible tasks changes appropriately

---

### ✅ Search Functionality

**Steps:**
1. Create several tasks with distinct titles:
   - "Review code changes"
   - "Write documentation"
   - "Update dependencies"
   - "Code review meeting"
2. Type "review" in search bar
3. Verify only matching tasks appear ("Review code changes", "Code review meeting")
4. Clear search
5. Type "doc"
6. Verify only "Write documentation" appears
7. Test search with partial words and description text

**Expected Result:**
- Search is case-insensitive
- Searches both title and description
- Results update as you type (no delay)
- Empty state shown if no matches

---

### ✅ Task Deletion

**Steps:**
1. Select a task (click on task card)
2. Detail panel opens
3. Click the trash icon (bottom-left of panel)
4. Verify confirmation dialog appears
5. Click "OK" to confirm
6. Verify:
   - Task is removed from the list
   - Detail panel closes (or shows empty state)
   - Task count decreases

**Alternative:**
- Click "Cancel" in confirmation dialog
- Task should NOT be deleted

**Expected Result:**
- Confirmation prevents accidental deletion
- Task removed cleanly
- No errors in console

---

### ✅ Deadline Display

**Steps:**
1. Create tasks with various deadlines:
   - Today
   - Tomorrow
   - 3 days from now
   - 10 days from now
   - Yesterday (overdue)
2. Verify display text:
   - "Due: Today"
   - "Due: Tomorrow"
   - "Due: In 3 days"
   - "Due: Dec 19" (month + day)
   - "Overdue" (with red badge instead of status)

**Expected Result:**
- Smart date formatting
- Overdue tasks have red badge
- Overdue tasks show red badge instead of status badge

---

### ✅ Completed Tasks

**Steps:**
1. Create a task
2. Edit and change status to "Done"
3. Save
4. Verify appearance changes:
   - Opacity reduced (60%)
   - Title has strikethrough
   - Description has strikethrough
   - Green "Done" badge
   - Shows "Completed: [date]" instead of deadline
5. Task should still be selectable and editable

**Expected Result:**
- Visual distinction for completed tasks
- Completion date displayed
- Can still be edited or reopened

---

### ✅ Form Validation

**Steps:**
1. Click "New Task"
2. Leave title empty
3. Fill in description
4. Click "Save"
5. Verify alert: "Please enter a task title"
6. Fill in title
7. Click "Save"
8. Task should be created successfully

**Expected Result:**
- Title is required
- Other fields are optional
- Clear error messaging

---

### ✅ Cancel Editing

**Steps:**
1. Select a task
2. Make changes to title and description (don't save)
3. Click "Cancel"
4. Verify:
   - Detail panel closes
   - Changes are NOT saved
   - Original task data unchanged
5. Select the same task again
6. Verify original values are shown

**Expected Result:**
- Cancel discards unsaved changes
- No data loss of original task

---

## UI/UX Testing

### Visual Design Checks

**Color Palette:**
- [ ] Primary actions use soft coral (#FF8B7B)
- [ ] Backgrounds are warm and comfortable
- [ ] Text is readable (warm dark gray #4A4A4A, not pure black)
- [ ] Borders are subtle (neutral-light/60)
- [ ] Shadows are soft and diffused

**Spacing & Layout:**
- [ ] Consistent padding (16px, 24px)
- [ ] Adequate line height for readability
- [ ] Cards have breathing room (12px gaps)
- [ ] Sidebar is fixed width (256px)
- [ ] Detail panel is fixed width (384px)

**Interactive Elements:**
- [ ] Buttons have hover states (darker background)
- [ ] Buttons show loading states ("Saving...")
- [ ] Buttons are disabled during operations
- [ ] Clickable areas are large enough (minimum 40px height)
- [ ] Cursor changes to pointer on interactive elements

**Typography:**
- [ ] Hierarchy is clear (h1 > h3 > body > small)
- [ ] Font weights are appropriate (bold for headings, medium for buttons)
- [ ] Line clamping works for long descriptions (2 lines max)
- [ ] Text doesn't overflow containers

**Transitions:**
- [ ] All animations are 200ms
- [ ] Smooth easing (ease-in-out)
- [ ] No jarring color changes
- [ ] Hover effects are subtle

---

## Responsive Testing

### Window Resizing

**Steps:**
1. Start with full-screen window
2. Resize window to smaller width (1024px)
3. Resize to minimum viable width (800px)
4. Verify:
   - Sidebar stays fixed width
   - Detail panel stays fixed width
   - Middle content area scrolls properly
   - No horizontal scrollbar
   - Search and filters stack on mobile (if window too narrow)

**Expected Result:**
- Layout doesn't break at any size
- Content remains accessible
- Scrolling works correctly

---

## Performance Testing

### Large Task Lists

**Steps:**
1. Create 50-100 tasks programmatically (via backend if possible)
2. Load the task list
3. Verify:
   - Initial load is quick (< 1 second)
   - Scrolling is smooth
   - Filtering is instant
   - Search is responsive
   - No lag when selecting tasks

**Expected Result:**
- No performance degradation
- Smooth 60fps scrolling
- No memory leaks (check DevTools)

---

## Error Handling Testing

### Backend Errors

**Steps:**
1. Stop the Tauri backend (if possible, or simulate error)
2. Try to create a task
3. Verify:
   - Error message appears at top of task list
   - Error is readable and helpful
   - "Dismiss" button works
   - UI doesn't crash

**Expected Result:**
- Graceful error handling
- User-friendly error messages
- No app crash

---

## Accessibility Testing

### Keyboard Navigation

**Steps:**
1. Use Tab key to navigate through interface
2. Verify tab order is logical:
   - Sidebar buttons
   - New Task button
   - Search bar
   - Filter buttons
   - Task cards
   - Detail panel fields
   - Action buttons
3. Press Enter on focused button
4. Verify it activates

**Expected Result:**
- All interactive elements are keyboard accessible
- Focus indicators are visible
- Logical tab order

### Screen Reader Testing (Optional)

**Steps:**
1. Enable screen reader (VoiceOver on Mac, NVDA on Windows)
2. Navigate through the interface
3. Verify:
   - Semantic HTML is used
   - Form labels are associated with inputs
   - Buttons have descriptive text
   - Status changes are announced

**Expected Result:**
- Interface is navigable by screen reader
- Content is understandable when read aloud

---

## Integration Testing

### Full User Journey

**Scenario: New user's first day**

1. **App opens** - Clean slate, no tasks
2. **See empty state** - Friendly message prompts to create task
3. **Click "New Task"** - Panel opens with empty form
4. **Create first task:**
   - Title: "Set up development environment"
   - Description: "Install Node.js, Rust, and Tauri CLI"
   - Status: Doing
   - Priority: High
   - Due Date: Today
5. **Save** - Task appears with blue badge
6. **Create second task:**
   - Title: "Read Tauri documentation"
   - Status: To Do
   - Priority: Medium
   - Due Date: Tomorrow
7. **Save** - Task appears with gray badge
8. **Filter to "Doing"** - Only first task visible
9. **Click first task** - Detail panel opens
10. **Change status to "Done"** - Save
11. **Verify** - Task has strikethrough, green badge, completion date
12. **Search "documentation"** - Only second task visible
13. **Clear search** - All tasks visible
14. **Click second task** - Edit and mark as "Doing"
15. **Delete first task** - Confirm, verify it's gone
16. **End state** - One task remaining, "Doing" status

**Expected Result:**
- Entire workflow is smooth
- No errors or confusion
- Interface is intuitive
- Actions are responsive

---

## Browser DevTools Checks

### Console

- [ ] No errors in console during normal operation
- [ ] No warnings (except development mode warnings)
- [ ] Tauri invoke calls log properly (if logging enabled)

### Network

- [ ] No unnecessary network requests
- [ ] All Tauri IPC calls complete successfully

### Performance

- [ ] React DevTools Profiler shows efficient renders
- [ ] No unnecessary re-renders on state changes
- [ ] useMemo prevents expensive recalculations

### React DevTools

- [ ] Component tree is clean
- [ ] Props are passed correctly
- [ ] State updates are localized
- [ ] No prop drilling (state managed in Zustand)

---

## Edge Cases to Test

### Empty States
- [ ] No tasks at all - friendly message
- [ ] Search with no results - helpful message
- [ ] Filter with no matching tasks - clear indication

### Data Edge Cases
- [ ] Task with very long title (100+ characters)
- [ ] Task with very long description (1000+ characters)
- [ ] Task with no description
- [ ] Task with no deadline
- [ ] Task deadline in distant future (years)
- [ ] Task deadline in distant past

### User Interactions
- [ ] Rapid clicking on buttons (debouncing needed?)
- [ ] Saving task multiple times quickly
- [ ] Switching between tasks rapidly
- [ ] Typing fast in search bar
- [ ] Pressing Save without changes

---

## Comparison with Reference Design

**Reference:** `specs/front-reference-pages/tasks_page/code.html`

### Matching Elements
- ✅ Left sidebar with navigation
- ✅ Main content area with task list
- ✅ Right detail panel for editing
- ✅ Search bar with icon
- ✅ Status filter buttons
- ✅ Task cards with badges
- ✅ Deadline display
- ✅ New Task button prominence
- ✅ Save/Cancel/Delete action buttons

### Improvements Over Reference
- ✅ TypeScript for type safety
- ✅ React hooks for better state management
- ✅ Proper error handling
- ✅ Loading states
- ✅ Smart deadline formatting
- ✅ Visual distinction for completed tasks
- ✅ Confirmation dialogs
- ✅ Real-time search and filtering

---

## Known Limitations & Future Work

### Current Limitations
- No drag-and-drop for reordering
- No multi-select for batch operations
- No task priority visual indicators (only in form)
- No tags display in task cards
- No attachments support
- No undo/redo functionality
- No keyboard shortcuts

### Planned Enhancements
- Virtual scrolling for very large lists (100+ tasks)
- Animations for task creation/deletion
- Dark mode toggle
- Task sorting options (by date, priority, title)
- Task grouping by project/category
- Quick actions menu on task cards
- Inline editing (edit title without opening panel)
- Task templates for recurring tasks

---

## Debugging Tips

### Common Issues

**Issue: Tasks don't load**
- Check Tauri backend is running
- Check database connection
- Open DevTools console for errors
- Verify `list_tasks` command is registered

**Issue: Tasks don't update**
- Check if `update_task` completes successfully
- Verify state is reloading after update
- Check for race conditions (rapid updates)

**Issue: UI looks broken**
- Clear build cache: `npm run build` again
- Check Tailwind CSS is compiling
- Verify `tailwind.config.js` has correct paths
- Check browser DevTools for CSS errors

**Issue: Performance is slow**
- Check React DevTools Profiler
- Look for unnecessary re-renders
- Verify useMemo dependencies
- Check if Zustand store is subscribing correctly

---

**Last Updated**: 2026-02-09
**Test Coverage**: Manual testing recommended
**Status**: Ready for QA
