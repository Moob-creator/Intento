import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface Task {
  id?: number;
  title: string;
  description?: string;
  status: "todo" | "doing" | "done";
  priority: "low" | "medium" | "high";
  deadline?: number;
  created_at: number;
  updated_at: number;
  completed_at?: number;
  tags?: string[];
}

function App() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [newTaskTitle, setNewTaskTitle] = useState("");
  const [dbVersion, setDbVersion] = useState<number | null>(null);
  const [message, setMessage] = useState("");

  // Load tasks on mount
  useEffect(() => {
    loadTasks();
    loadDbVersion();
  }, []);

  async function loadDbVersion() {
    try {
      const version = await invoke<number>("get_db_version");
      setDbVersion(version);
    } catch (error) {
      setMessage(`Error loading DB version: ${error}`);
    }
  }

  async function loadTasks() {
    try {
      const taskList = await invoke<Task[]>("list_tasks", { status: null });
      setTasks(taskList);
      setMessage(`Loaded ${taskList.length} tasks`);
    } catch (error) {
      setMessage(`Error loading tasks: ${error}`);
    }
  }

  async function createTask() {
    if (!newTaskTitle.trim()) {
      setMessage("Please enter a task title");
      return;
    }

    try {
      const taskId = await invoke<number>("create_task", {
        title: newTaskTitle,
        description: null,
        priority: "medium",
        deadline: null,
        tags: null,
      });
      setMessage(`Created task with ID: ${taskId}`);
      setNewTaskTitle("");
      await loadTasks();
    } catch (error) {
      setMessage(`Error creating task: ${error}`);
    }
  }

  async function updateTaskStatus(id: number, newStatus: string) {
    try {
      await invoke("update_task", {
        id,
        title: null,
        description: null,
        status: newStatus,
        priority: null,
        deadline: null,
        tags: null,
        completedAt: null,
      });
      setMessage(`Updated task ${id} to ${newStatus}`);
      await loadTasks();
    } catch (error) {
      setMessage(`Error updating task: ${error}`);
    }
  }

  async function deleteTask(id: number) {
    try {
      await invoke("delete_task", { id });
      setMessage(`Deleted task ${id}`);
      await loadTasks();
    } catch (error) {
      setMessage(`Error deleting task: ${error}`);
    }
  }

  return (
    <div className="container">
      <h1>Intento - Task Manager Test</h1>

      <div style={{ marginBottom: "20px", padding: "10px", background: "#f0f0f0", borderRadius: "5px" }}>
        <p>Database Version: {dbVersion ?? "Loading..."}</p>
        <p>Status: {message || "Ready"}</p>
      </div>

      <div style={{ marginBottom: "20px" }}>
        <h2>Create New Task</h2>
        <form
          onSubmit={(e) => {
            e.preventDefault();
            createTask();
          }}
        >
          <input
            type="text"
            value={newTaskTitle}
            onChange={(e) => setNewTaskTitle(e.target.value)}
            placeholder="Enter task title..."
            style={{ marginRight: "10px", padding: "5px" }}
          />
          <button type="submit">Create Task</button>
        </form>
      </div>

      <div>
        <h2>Tasks ({tasks.length})</h2>
        {tasks.length === 0 ? (
          <p>No tasks yet. Create one above!</p>
        ) : (
          <table style={{ width: "100%", borderCollapse: "collapse" }}>
            <thead>
              <tr>
                <th style={{ border: "1px solid #ddd", padding: "8px" }}>ID</th>
                <th style={{ border: "1px solid #ddd", padding: "8px" }}>Title</th>
                <th style={{ border: "1px solid #ddd", padding: "8px" }}>Status</th>
                <th style={{ border: "1px solid #ddd", padding: "8px" }}>Priority</th>
                <th style={{ border: "1px solid #ddd", padding: "8px" }}>Actions</th>
              </tr>
            </thead>
            <tbody>
              {tasks.map((task) => (
                <tr key={task.id}>
                  <td style={{ border: "1px solid #ddd", padding: "8px" }}>{task.id}</td>
                  <td style={{ border: "1px solid #ddd", padding: "8px" }}>{task.title}</td>
                  <td style={{ border: "1px solid #ddd", padding: "8px" }}>
                    <select
                      value={task.status}
                      onChange={(e) => updateTaskStatus(task.id!, e.target.value)}
                    >
                      <option value="todo">Todo</option>
                      <option value="doing">Doing</option>
                      <option value="done">Done</option>
                    </select>
                  </td>
                  <td style={{ border: "1px solid #ddd", padding: "8px" }}>{task.priority}</td>
                  <td style={{ border: "1px solid #ddd", padding: "8px" }}>
                    <button onClick={() => deleteTask(task.id!)}>Delete</button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
}

export default App;

