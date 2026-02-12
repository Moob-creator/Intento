import { X, CheckCircle2, Circle, Loader2, TrendingUp, Calendar } from 'lucide-react';
import type { Task } from '../types/task';

interface StatisticsPanelProps {
  isOpen: boolean;
  onClose: () => void;
  tasks: Task[];
}

export function StatisticsPanel({ isOpen, onClose, tasks }: StatisticsPanelProps) {
  if (!isOpen) return null;

  // Calculate statistics
  const totalTasks = tasks.length;
  const todoTasks = tasks.filter((t) => t.status === 'todo').length;
  const doingTasks = tasks.filter((t) => t.status === 'doing').length;
  const doneTasks = tasks.filter((t) => t.status === 'done').length;
  const highPriorityTasks = tasks.filter((t) => t.priority === 'high').length;

  const completionRate = totalTasks > 0 ? Math.round((doneTasks / totalTasks) * 100) : 0;

  // Tasks by priority
  const lowPriority = tasks.filter((t) => t.priority === 'low').length;
  const mediumPriority = tasks.filter((t) => t.priority === 'medium').length;

  // Recent activity (completed tasks in the last 7 days)
  const now = Date.now() / 1000;
  const sevenDaysAgo = now - 7 * 24 * 60 * 60;
  const recentCompletions = tasks.filter(
    (t) => t.status === 'done' && t.completed_at && t.completed_at >= sevenDaysAgo
  ).length;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm"
      onClick={onClose}
    >
      <div
        className="w-full max-w-2xl bg-white rounded-xl shadow-2xl overflow-hidden"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-neutral-light/60">
          <div className="flex items-center gap-3">
            <TrendingUp size={24} className="text-primary" />
            <h2 className="text-xl font-bold text-neutral-dark">Task Statistics</h2>
          </div>
          <button
            onClick={onClose}
            className="p-2 text-neutral-dark/40 hover:text-neutral-dark hover:bg-neutral-light/40 rounded-lg transition-all duration-200"
            aria-label="Close"
          >
            <X size={20} />
          </button>
        </div>

        {/* Content */}
        <div className="p-6 max-h-[70vh] overflow-y-auto">
          {/* Overview cards */}
          <div className="grid grid-cols-2 gap-4 mb-6">
            <div className="p-4 bg-gradient-to-br from-blue-50 to-blue-100 rounded-xl">
              <div className="flex items-center gap-2 mb-2">
                <Circle size={18} className="text-blue-600" />
                <span className="text-sm font-medium text-blue-900">To Do</span>
              </div>
              <p className="text-3xl font-bold text-blue-900">{todoTasks}</p>
            </div>

            <div className="p-4 bg-gradient-to-br from-amber-50 to-amber-100 rounded-xl">
              <div className="flex items-center gap-2 mb-2">
                <Loader2 size={18} className="text-amber-600" />
                <span className="text-sm font-medium text-amber-900">In Progress</span>
              </div>
              <p className="text-3xl font-bold text-amber-900">{doingTasks}</p>
            </div>

            <div className="p-4 bg-gradient-to-br from-green-50 to-green-100 rounded-xl">
              <div className="flex items-center gap-2 mb-2">
                <CheckCircle2 size={18} className="text-green-600" />
                <span className="text-sm font-medium text-green-900">Completed</span>
              </div>
              <p className="text-3xl font-bold text-green-900">{doneTasks}</p>
            </div>

            <div className="p-4 bg-gradient-to-br from-primary-light to-primary rounded-xl">
              <div className="flex items-center gap-2 mb-2">
                <Calendar size={18} className="text-white" />
                <span className="text-sm font-medium text-white">Total Tasks</span>
              </div>
              <p className="text-3xl font-bold text-white">{totalTasks}</p>
            </div>
          </div>

          {/* Completion rate */}
          <div className="mb-6 p-5 bg-neutral-light/30 rounded-xl">
            <div className="flex items-center justify-between mb-3">
              <span className="text-sm font-semibold text-neutral-dark">Completion Rate</span>
              <span className="text-2xl font-bold text-primary">{completionRate}%</span>
            </div>
            <div className="w-full h-3 bg-neutral-light rounded-full overflow-hidden">
              <div
                className="h-full bg-gradient-to-r from-primary to-primary-dark rounded-full transition-all duration-500"
                style={{ width: `${completionRate}%` }}
              />
            </div>
          </div>

          {/* Priority distribution */}
          <div className="mb-6">
            <h3 className="text-sm font-semibold text-neutral-dark mb-3">Tasks by Priority</h3>
            <div className="space-y-3">
              <div className="flex items-center justify-between p-3 bg-red-50 rounded-lg">
                <span className="text-sm font-medium text-red-900">High Priority</span>
                <span className="text-lg font-bold text-red-900">{highPriorityTasks}</span>
              </div>
              <div className="flex items-center justify-between p-3 bg-amber-50 rounded-lg">
                <span className="text-sm font-medium text-amber-900">Medium Priority</span>
                <span className="text-lg font-bold text-amber-900">{mediumPriority}</span>
              </div>
              <div className="flex items-center justify-between p-3 bg-blue-50 rounded-lg">
                <span className="text-sm font-medium text-blue-900">Low Priority</span>
                <span className="text-lg font-bold text-blue-900">{lowPriority}</span>
              </div>
            </div>
          </div>

          {/* Recent activity */}
          <div>
            <h3 className="text-sm font-semibold text-neutral-dark mb-3">Recent Activity</h3>
            <div className="p-4 bg-gradient-to-r from-green-50 to-emerald-50 rounded-lg">
              <p className="text-sm text-green-900 mb-1">Tasks completed in the last 7 days</p>
              <p className="text-3xl font-bold text-green-900">{recentCompletions}</p>
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="px-6 py-4 border-t border-neutral-light/60 bg-neutral-light/20">
          <button
            onClick={onClose}
            className="w-full py-2 px-4 bg-primary text-white font-semibold rounded-lg
                     hover:bg-primary-dark transition-all duration-200"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
}
