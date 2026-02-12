## ADDED Requirements

### Requirement: Open task form dialog
The system SHALL provide a `TaskForm` component that opens as a shadcn/ui Dialog when triggered by user action.

#### Scenario: Open form for new task
- **WHEN** user clicks "New Task" button
- **THEN** component opens dialog with empty form fields and "Create Task" title

#### Scenario: Open form to edit existing task
- **WHEN** user clicks "Edit" button on a task card
- **THEN** component opens dialog with pre-filled form fields and "Edit Task" title

#### Scenario: Close form dialog
- **WHEN** user clicks close button or outside dialog
- **THEN** component closes dialog and resets form state

### Requirement: Form fields for task properties
The system SHALL provide input fields for all editable task properties with appropriate UI controls.

#### Scenario: Title input field
- **WHEN** form is displayed
- **THEN** form includes required text input for title with placeholder "Enter task title"

#### Scenario: Description textarea
- **WHEN** form is displayed
- **THEN** form includes optional textarea for description with placeholder "Add details..."

#### Scenario: Status select dropdown
- **WHEN** form is displayed
- **THEN** form includes select dropdown with options "Todo", "Doing", "Done"

#### Scenario: Priority select dropdown
- **WHEN** form is displayed
- **THEN** form includes select dropdown with options "Low", "Medium", "High" (default "Medium")

#### Scenario: Deadline date picker
- **WHEN** form is displayed
- **THEN** form includes optional date/time picker for deadline

#### Scenario: Tags input
- **WHEN** form is displayed
- **THEN** form includes optional tag input with ability to add multiple tags

### Requirement: Form validation with Zod schema
The system SHALL validate form inputs using Zod schema before submission.

#### Scenario: Validate required title
- **WHEN** user attempts to submit form with empty title
- **THEN** form displays error "Title is required" below title field and prevents submission

#### Scenario: Validate title length
- **WHEN** user enters title longer than 200 characters
- **THEN** form displays error "Title must be 200 characters or less" and prevents submission

#### Scenario: Validate description length
- **WHEN** user enters description longer than 1000 characters
- **THEN** form displays error "Description must be 1000 characters or less" and prevents submission

#### Scenario: Validate status enum
- **WHEN** user selects invalid status (edge case)
- **THEN** form displays error "Invalid status" and prevents submission

#### Scenario: Validate priority enum
- **WHEN** user selects invalid priority (edge case)
- **THEN** form displays error "Invalid priority" and prevents submission

#### Scenario: Validate deadline in future
- **WHEN** user selects deadline in the past
- **THEN** form displays error "Deadline must be in the future" and prevents submission

### Requirement: Submit form to create task
The system SHALL call store `addTask` action when creating a new task via form submission.

#### Scenario: Create task successfully
- **WHEN** user fills valid form and clicks "Create"
- **THEN** component calls `addTask` with form values, closes dialog, resets form, and shows success toast

#### Scenario: Create task on error
- **WHEN** `addTask` returns error
- **THEN** component displays error toast with message and keeps dialog open for retry

#### Scenario: Disable submit during creation
- **WHEN** task creation is in progress
- **THEN** submit button shows loading spinner and is disabled

### Requirement: Submit form to update task
The system SHALL call store `updateTask` action when editing an existing task via form submission.

#### Scenario: Update task successfully
- **WHEN** user edits task form and clicks "Save"
- **THEN** component calls `updateTask` with task ID and changed fields, closes dialog, and shows success toast

#### Scenario: Update task on error
- **WHEN** `updateTask` returns error
- **THEN** component displays error toast with message and keeps dialog open for retry

#### Scenario: Disable submit during update
- **WHEN** task update is in progress
- **THEN** submit button shows loading spinner and is disabled

### Requirement: Form accessibility
The system SHALL provide accessible form controls following WCAG 2.1 AA standards via Radix UI primitives.

#### Scenario: Keyboard navigation
- **WHEN** user navigates form with Tab key
- **THEN** focus moves through fields in logical order and Enter key submits form

#### Scenario: Screen reader labels
- **WHEN** screen reader is active
- **THEN** all form fields announce labels, required status, and error messages

#### Scenario: Error announcement
- **WHEN** form validation fails
- **THEN** screen reader announces "Form has errors" and lists error fields

### Requirement: Warm and soft form UI
The system SHALL style the TaskForm component following warm, soft design principles.

#### Scenario: Dialog styling
- **WHEN** dialog is opened
- **THEN** dialog uses soft backdrop, rounded corners, warm white background, and smooth fade-in animation

#### Scenario: Input field styling
- **WHEN** form fields are rendered
- **THEN** inputs use soft borders, warm focus rings, comfortable padding, and gentle transitions

#### Scenario: Button styling
- **WHEN** submit and cancel buttons are rendered
- **THEN** buttons use warm color palette with soft shadows and smooth hover effects
