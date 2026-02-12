/**
 * Task data structure matching the Rust backend
 */
export interface Task {
  id?: number;
  title: string;
  description?: string;
  status: "todo" | "doing" | "done";
  priority: "low" | "medium" | "high";
  deadline?: number; // Unix timestamp
  created_at: number;
  updated_at: number;
  completed_at?: number;
  tags?: string[];
}

export type TaskStatus = "todo" | "doing" | "done";
export type TaskPriority = "low" | "medium" | "high";

/**
 * Filter state for task list
 */
export interface TaskFilters {
  status: TaskStatus | "all";
  searchQuery: string;
}

/**
 * Parsed task from AI text input
 */
export interface ParsedTask {
  title: string;
  description?: string;
  deadline?: string;
  priority?: "low" | "medium" | "high";
  tags?: string[];
}

/**
 * Task operations extracted from image
 */
export type TaskOperation =
  | {
      type: "Create";
      data: {
        title: string;
        description?: string;
        priority?: "low" | "medium" | "high";
        deadline?: string;
        tags?: string[];
      };
    }
  | {
      type: "Update";
      data: {
        task_identifier: string;
        title?: string;
        description?: string;
        priority?: "low" | "medium" | "high";
        deadline?: string;
        tags?: string[];
      };
    }
  | {
      type: "Complete";
      data: {
        task_identifier: string;
      };
    }
  | {
      type: "Delete";
      data: {
        task_identifier: string;
      };
    }
  | {
      type: "BatchComplete";
      data: {
        task_identifiers: string[];
      };
    }
  | {
      type: "SetStatus";
      data: {
        task_identifier: string;
        status: "todo" | "doing" | "done";
      };
    };

/**
 * Result from image parsing with operations
 */
export interface ImageParseResult {
  operations: TaskOperation[];
  confidence: number;
  image_description?: string;
  warnings: string[];
}
