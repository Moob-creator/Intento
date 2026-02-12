import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { Task, TaskStatus } from '../types/task';

interface TaskStore {
  tasks: Task[];
  selectedTask: Task | null;
  isLoading: boolean;
  error: string | null;

  // Actions
  loadTasks: (status?: TaskStatus | null) => Promise<void>;
  createTask: (task: Omit<Task, 'id' | 'created_at' | 'updated_at'>) => Promise<void>;
  updateTask: (id: number, updates: Partial<Task>) => Promise<void>;
  deleteTask: (id: number) => Promise<void>;
  selectTask: (task: Task | null) => void;
  clearError: () => void;
}

export const useTaskStore = create<TaskStore>((set, get) => ({
  tasks: [],
  selectedTask: null,
  isLoading: false,
  error: null,

  loadTasks: async (status = null) => {
    set({ isLoading: true, error: null });
    try {
      const tasks = await invoke<Task[]>('list_tasks', { status });
      set({ tasks, isLoading: false });
    } catch (error) {
      set({ error: String(error), isLoading: false });
      console.error('Failed to load tasks:', error);
    }
  },

  createTask: async (taskData) => {
    set({ isLoading: true, error: null });
    try {
      const taskId = await invoke<number>('create_task', {
        title: taskData.title,
        description: taskData.description || null,
        priority: taskData.priority || 'medium',
        deadline: taskData.deadline || null,
        tags: taskData.tags || null,
      });

      // Reload tasks after creation
      await get().loadTasks();

      // Select the newly created task
      const newTask = get().tasks.find(t => t.id === taskId);
      if (newTask) {
        set({ selectedTask: newTask });
      }
    } catch (error) {
      set({ error: String(error), isLoading: false });
      console.error('Failed to create task:', error);
    }
  },

  updateTask: async (id, updates) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('update_task', {
        id,
        title: updates.title || null,
        description: updates.description || null,
        status: updates.status || null,
        priority: updates.priority || null,
        deadline: updates.deadline || null,
        tags: updates.tags || null,
        completedAt: updates.completed_at || null,
      });

      // Reload tasks after update
      await get().loadTasks();

      // Update selected task if it was the one being edited
      if (get().selectedTask?.id === id) {
        const updatedTask = get().tasks.find(t => t.id === id);
        set({ selectedTask: updatedTask || null });
      }
    } catch (error) {
      set({ error: String(error), isLoading: false });
      console.error('Failed to update task:', error);
    }
  },

  deleteTask: async (id) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('delete_task', { id });

      // Clear selection if the deleted task was selected
      if (get().selectedTask?.id === id) {
        set({ selectedTask: null });
      }

      // Reload tasks after deletion
      await get().loadTasks();
    } catch (error) {
      set({ error: String(error), isLoading: false });
      console.error('Failed to delete task:', error);
    }
  },

  selectTask: (task) => {
    set({ selectedTask: task });
  },

  clearError: () => {
    set({ error: null });
  },
}));
