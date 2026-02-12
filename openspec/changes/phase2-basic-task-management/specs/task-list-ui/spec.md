## ADDED Requirements

### Requirement: Display list of tasks
The system SHALL provide a `TaskList` component that displays all tasks from the store in a visually organized layout using shadcn/ui Card components.

#### Scenario: Display tasks when loaded
- **WHEN** store contains tasks and `<TaskList />` is rendered
- **THEN** component displays each task as a Card with title, description, status badge, priority, and action buttons

#### Scenario: Display empty state
- **WHEN** store has zero tasks and `<TaskList />` is rendered
- **THEN** component displays "No tasks yet" message with prompt to create first task

#### Scenario: Display loading state
- **WHEN** store `loading` is true and `<TaskList />` is rendered
- **THEN** component displays loading skeleton or spinner instead of task cards

### Requirement: Filter tasks by status
The system SHALL provide status filter UI that allows users to view tasks by status (All/Todo/Doing/Done).

#### Scenario: Show all tasks by default
- **WHEN** `<TaskList />` is rendered with no filter applied
- **THEN** component displays all tasks from store

#### Scenario: Filter to Todo tasks
- **WHEN** user clicks "Todo" filter button
- **THEN** component displays only tasks with status "todo"

#### Scenario: Filter to Doing tasks
- **WHEN** user clicks "Doing" filter button
- **THEN** component displays only tasks with status "doing"

#### Scenario: Filter to Done tasks
- **WHEN** user clicks "Done" filter button
- **THEN** component displays only tasks with status "done"

#### Scenario: Clear filter to show all
- **WHEN** user clicks "All" filter button
- **THEN** component displays all tasks regardless of status

### Requirement: Toggle task status inline
The system SHALL provide UI to change task status directly from the task card without opening a form.

#### Scenario: Change status from todo to doing
- **WHEN** user clicks status dropdown on a todo task and selects "doing"
- **THEN** component calls `updateTask(id, { status: "doing" })` and UI updates immediately

#### Scenario: Change status to done
- **WHEN** user changes task status to "done"
- **THEN** component calls `updateTask` with status "done" and task shows completion timestamp

#### Scenario: Visual feedback on status change
- **WHEN** status change is in progress
- **THEN** UI shows loading indicator on the affected task card

### Requirement: Delete task from list
The system SHALL provide a delete button on each task card that removes the task after confirmation.

#### Scenario: Delete task with confirmation
- **WHEN** user clicks delete button on task card
- **THEN** component shows confirmation dialog "Are you sure you want to delete this task?"

#### Scenario: Confirm deletion
- **WHEN** user confirms deletion in dialog
- **THEN** component calls `deleteTask(id)` and task is removed from list immediately

#### Scenario: Cancel deletion
- **WHEN** user cancels deletion in dialog
- **THEN** dialog closes and task remains in list unchanged

### Requirement: Display task metadata
The system SHALL display key task metadata in each task card for quick reference.

#### Scenario: Display task priority
- **WHEN** task card is rendered
- **THEN** component displays priority badge with color coding (low=gray, medium=blue, high=red)

#### Scenario: Display task deadline
- **WHEN** task has a deadline set
- **THEN** component displays deadline timestamp in human-readable format (e.g., "Due in 2 days")

#### Scenario: Display task tags
- **WHEN** task has tags
- **THEN** component displays tags as small badges below task description

#### Scenario: Display completed timestamp
- **WHEN** task status is "done" and `completed_at` is set
- **THEN** component displays "Completed: <timestamp>" in task card

### Requirement: Warm and soft UI aesthetic
The system SHALL style the TaskList component following the warm, soft design principles specified in project requirements.

#### Scenario: Card styling
- **WHEN** task cards are rendered
- **THEN** cards use soft shadows, rounded corners, warm neutral colors, and comfortable spacing

#### Scenario: Status badge colors
- **WHEN** status badges are rendered
- **THEN** badges use warm color palette (todo=warm gray, doing=soft amber, done=soft green)

#### Scenario: Hover interactions
- **WHEN** user hovers over task card or action buttons
- **THEN** UI provides subtle, smooth transitions with warm color shifts
