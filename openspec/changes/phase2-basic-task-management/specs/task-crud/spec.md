## ADDED Requirements

### Requirement: Create task via Tauri command
The system SHALL provide a `create_task` Tauri command that creates a new task in the database and returns its ID.

#### Scenario: Create task with required fields only
- **WHEN** frontend invokes `create_task` with `title: "Buy milk"`
- **THEN** system creates task with status "todo", priority "medium", current timestamp, and returns task ID

#### Scenario: Create task with all fields
- **WHEN** frontend invokes `create_task` with title, description, priority "high", deadline timestamp, and tags ["shopping", "urgent"]
- **THEN** system creates task with all provided fields and returns task ID

#### Scenario: Create task with empty title
- **WHEN** frontend invokes `create_task` with empty or whitespace-only title
- **THEN** system returns error "Title cannot be empty"

### Requirement: Retrieve task by ID
The system SHALL provide a `get_task` Tauri command that retrieves a single task by ID.

#### Scenario: Get existing task
- **WHEN** frontend invokes `get_task` with id 123
- **THEN** system returns Task object with all fields populated

#### Scenario: Get non-existent task
- **WHEN** frontend invokes `get_task` with id that doesn't exist
- **THEN** system returns `None` (not an error)

#### Scenario: Get deleted task
- **WHEN** frontend invokes `get_task` with id of soft-deleted task
- **THEN** system returns `None` as if task doesn't exist

### Requirement: Update task fields
The system SHALL provide an `update_task` Tauri command that updates specific fields of an existing task.

#### Scenario: Update task status to done
- **WHEN** frontend invokes `update_task` with id 123 and status "done"
- **THEN** system updates task status to "done", sets `completed_at` to current timestamp, updates `updated_at`, and returns success

#### Scenario: Update multiple fields
- **WHEN** frontend invokes `update_task` with id 123, title "New title", description "New desc", and priority "low"
- **THEN** system updates specified fields, leaves other fields unchanged, updates `updated_at`, and returns success

#### Scenario: Update non-existent task
- **WHEN** frontend invokes `update_task` with id that doesn't exist
- **THEN** system returns error "Task with id X not found"

#### Scenario: Update with invalid status
- **WHEN** frontend invokes `update_task` with status "invalid"
- **THEN** system returns error "Invalid task status: invalid"

### Requirement: Delete task (soft delete)
The system SHALL provide a `delete_task` Tauri command that soft-deletes a task by setting `is_deleted` flag.

#### Scenario: Delete existing task
- **WHEN** frontend invokes `delete_task` with id 123
- **THEN** system sets `is_deleted` to true, updates `updated_at`, and returns success

#### Scenario: Delete already deleted task
- **WHEN** frontend invokes `delete_task` with id of already deleted task
- **THEN** system returns success (idempotent operation)

#### Scenario: Delete non-existent task
- **WHEN** frontend invokes `delete_task` with id that doesn't exist
- **THEN** system returns error "Failed to delete task"

### Requirement: List tasks with optional filtering
The system SHALL provide a `list_tasks` Tauri command that returns all non-deleted tasks, optionally filtered by status.

#### Scenario: List all tasks
- **WHEN** frontend invokes `list_tasks` with status `None`
- **THEN** system returns array of all non-deleted tasks ordered by `created_at` DESC

#### Scenario: List tasks by status
- **WHEN** frontend invokes `list_tasks` with status "todo"
- **THEN** system returns array of tasks with status "todo", excluding deleted tasks, ordered by `created_at` DESC

#### Scenario: List tasks when none exist
- **WHEN** frontend invokes `list_tasks` on empty database
- **THEN** system returns empty array (not an error)

#### Scenario: List excludes deleted tasks
- **WHEN** frontend invokes `list_tasks` and database contains soft-deleted tasks
- **THEN** system returns only non-deleted tasks in the result
