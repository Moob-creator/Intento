import { useEffect, useState, useMemo } from 'react';
import { Calendar, CheckCircle2, Clock, TrendingUp, Zap } from 'lucide-react';
import { useTaskStore } from '../store/taskStore';

interface TaskStats {
  total: number;
  completed: number;
  inProgress: number;
  todo: number;
  completedToday: number;
  completedThisWeek: number;
  overdue: number;
}

export function HomePage() {
  const { tasks, loadTasks } = useTaskStore();
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const loadData = async () => {
      setIsLoading(true);
      await loadTasks();
      setIsLoading(false);
    };
    loadData();
  }, [loadTasks]);

  // Calculate statistics
  const stats: TaskStats = useMemo(() => {
    const now = Date.now() / 1000;
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    const todayStart = today.getTime() / 1000;

    const weekStart = new Date(today);
    weekStart.setDate(weekStart.getDate() - weekStart.getDay());
    const weekStartTimestamp = weekStart.getTime() / 1000;

    return {
      total: tasks.length,
      completed: tasks.filter((t) => t.status === 'done').length,
      inProgress: tasks.filter((t) => t.status === 'doing').length,
      todo: tasks.filter((t) => t.status === 'todo').length,
      completedToday: tasks.filter(
        (t) => t.status === 'done' && t.completed_at && t.completed_at >= todayStart
      ).length,
      completedThisWeek: tasks.filter(
        (t) => t.status === 'done' && t.completed_at && t.completed_at >= weekStartTimestamp
      ).length,
      overdue: tasks.filter(
        (t) => t.deadline && t.deadline < now && t.status !== 'done'
      ).length,
    };
  }, [tasks]);

  // Get upcoming tasks (next 7 days)
  const upcomingTasks = useMemo(() => {
    const now = Date.now() / 1000;
    const sevenDaysLater = now + 7 * 24 * 60 * 60;

    return tasks
      .filter(
        (t) =>
          t.status !== 'done' &&
          t.deadline &&
          t.deadline >= now &&
          t.deadline <= sevenDaysLater
      )
      .sort((a, b) => (a.deadline || 0) - (b.deadline || 0))
      .slice(0, 5);
  }, [tasks]);

  // Get recent activity (last 5 completed tasks)
  const recentActivity = useMemo(() => {
    return tasks
      .filter((t) => t.status === 'done' && t.completed_at)
      .sort((a, b) => (b.completed_at || 0) - (a.completed_at || 0))
      .slice(0, 5);
  }, [tasks]);

  const formatDeadline = (timestamp: number) => {
    const date = new Date(timestamp * 1000);
    const now = new Date();
    const diffTime = date.getTime() - now.getTime();
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));

    if (diffDays === 0) return 'Today';
    if (diffDays === 1) return 'Tomorrow';
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
  };

  const formatRelativeTime = (timestamp: number) => {
    const date = new Date(timestamp * 1000);
    const now = new Date();
    const diffTime = now.getTime() - date.getTime();
    const diffDays = Math.floor(diffTime / (1000 * 60 * 60 * 24));

    if (diffDays === 0) return 'Today';
    if (diffDays === 1) return 'Yesterday';
    if (diffDays < 7) return `${diffDays} days ago`;
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
  };

  if (isLoading) {
    return (
      <div className="flex-1 flex items-center justify-center">
        <div className="text-neutral-dark/60">Loading your dashboard...</div>
      </div>
    );
  }

  return (
    <div className="flex-1 overflow-y-auto p-6">
      <div className="max-w-6xl mx-auto space-y-6">
        {/* Welcome Section */}
        <div className="space-y-2">
          <h1 className="text-neutral-dark text-4xl font-black leading-tight">
            Welcome back!
          </h1>
          <p className="text-neutral-dark/60 text-base">
            Here's an overview of your tasks and productivity
          </p>
        </div>

        {/* Statistics Cards */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          {/* Total Tasks */}
          <div className="bg-background-card border border-neutral-light/60 rounded-xl p-6 hover:shadow-soft transition-all duration-200">
            <div className="flex items-center justify-between mb-3">
              <div className="p-2 bg-primary/10 rounded-lg">
                <CheckCircle2 className="text-primary" size={24} />
              </div>
            </div>
            <p className="text-neutral-dark text-3xl font-bold">{stats.total}</p>
            <p className="text-neutral-dark/60 text-sm mt-1">Total Tasks</p>
          </div>

          {/* Completed Today */}
          <div className="bg-background-card border border-neutral-light/60 rounded-xl p-6 hover:shadow-soft transition-all duration-200">
            <div className="flex items-center justify-between mb-3">
              <div className="p-2 bg-green-100 rounded-lg">
                <Zap className="text-green-600" size={24} />
              </div>
            </div>
            <p className="text-neutral-dark text-3xl font-bold">{stats.completedToday}</p>
            <p className="text-neutral-dark/60 text-sm mt-1">Completed Today</p>
          </div>

          {/* In Progress */}
          <div className="bg-background-card border border-neutral-light/60 rounded-xl p-6 hover:shadow-soft transition-all duration-200">
            <div className="flex items-center justify-between mb-3">
              <div className="p-2 bg-blue-100 rounded-lg">
                <TrendingUp className="text-blue-600" size={24} />
              </div>
            </div>
            <p className="text-neutral-dark text-3xl font-bold">{stats.inProgress}</p>
            <p className="text-neutral-dark/60 text-sm mt-1">In Progress</p>
          </div>

          {/* Overdue */}
          <div className="bg-background-card border border-neutral-light/60 rounded-xl p-6 hover:shadow-soft transition-all duration-200">
            <div className="flex items-center justify-between mb-3">
              <div className="p-2 bg-red-100 rounded-lg">
                <Clock className="text-red-600" size={24} />
              </div>
            </div>
            <p className="text-neutral-dark text-3xl font-bold">{stats.overdue}</p>
            <p className="text-neutral-dark/60 text-sm mt-1">Overdue</p>
          </div>
        </div>

        {/* Two Column Layout */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Upcoming Tasks */}
          <div className="bg-background-card border border-neutral-light/60 rounded-xl p-6">
            <div className="flex items-center gap-2 mb-4">
              <Calendar className="text-primary" size={20} />
              <h2 className="text-neutral-dark text-lg font-bold">Upcoming Tasks</h2>
            </div>

            {upcomingTasks.length === 0 ? (
              <div className="text-center py-8">
                <p className="text-neutral-dark/60 text-sm">
                  No upcoming tasks in the next 7 days
                </p>
              </div>
            ) : (
              <div className="space-y-3">
                {upcomingTasks.map((task) => (
                  <div
                    key={task.id}
                    className="flex items-start gap-3 p-3 rounded-lg bg-background-warm/50 border border-neutral-light/40 hover:border-primary/30 transition-all duration-200"
                  >
                    <div className="flex-1 min-w-0">
                      <h3 className="text-neutral-dark font-medium text-sm leading-snug truncate">
                        {task.title}
                      </h3>
                      <div className="flex items-center gap-2 mt-1">
                        <span className="text-xs text-neutral-dark/60">
                          Due: {formatDeadline(task.deadline!)}
                        </span>
                        <span
                          className={`text-xs px-2 py-0.5 rounded-full ${
                            task.priority === 'high'
                              ? 'bg-red-100 text-red-700'
                              : task.priority === 'medium'
                              ? 'bg-amber-100 text-amber-700'
                              : 'bg-gray-100 text-gray-700'
                          }`}
                        >
                          {task.priority}
                        </span>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* Recent Activity */}
          <div className="bg-background-card border border-neutral-light/60 rounded-xl p-6">
            <div className="flex items-center gap-2 mb-4">
              <CheckCircle2 className="text-green-600" size={20} />
              <h2 className="text-neutral-dark text-lg font-bold">Recent Activity</h2>
            </div>

            {recentActivity.length === 0 ? (
              <div className="text-center py-8">
                <p className="text-neutral-dark/60 text-sm">
                  No completed tasks yet. Get started!
                </p>
              </div>
            ) : (
              <div className="space-y-3">
                {recentActivity.map((task) => (
                  <div
                    key={task.id}
                    className="flex items-start gap-3 p-3 rounded-lg bg-green-50/50 border border-green-100 opacity-80"
                  >
                    <div className="mt-0.5">
                      <CheckCircle2 className="text-green-600" size={16} />
                    </div>
                    <div className="flex-1 min-w-0">
                      <h3 className="text-neutral-dark font-medium text-sm leading-snug truncate line-through">
                        {task.title}
                      </h3>
                      <p className="text-xs text-neutral-dark/60 mt-1">
                        Completed {formatRelativeTime(task.completed_at!)}
                      </p>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>

        {/* Progress Overview */}
        <div className="bg-background-card border border-neutral-light/60 rounded-xl p-6">
          <h2 className="text-neutral-dark text-lg font-bold mb-4">This Week's Progress</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div>
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm text-neutral-dark/60">Completed</span>
                <span className="text-sm font-semibold text-neutral-dark">{stats.completed}</span>
              </div>
              <div className="h-2 bg-neutral-light/40 rounded-full overflow-hidden">
                <div
                  className="h-full bg-green-500 rounded-full transition-all duration-500"
                  style={{
                    width: `${stats.total > 0 ? (stats.completed / stats.total) * 100 : 0}%`,
                  }}
                />
              </div>
            </div>

            <div>
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm text-neutral-dark/60">In Progress</span>
                <span className="text-sm font-semibold text-neutral-dark">{stats.inProgress}</span>
              </div>
              <div className="h-2 bg-neutral-light/40 rounded-full overflow-hidden">
                <div
                  className="h-full bg-blue-500 rounded-full transition-all duration-500"
                  style={{
                    width: `${stats.total > 0 ? (stats.inProgress / stats.total) * 100 : 0}%`,
                  }}
                />
              </div>
            </div>

            <div>
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm text-neutral-dark/60">To Do</span>
                <span className="text-sm font-semibold text-neutral-dark">{stats.todo}</span>
              </div>
              <div className="h-2 bg-neutral-light/40 rounded-full overflow-hidden">
                <div
                  className="h-full bg-gray-500 rounded-full transition-all duration-500"
                  style={{
                    width: `${stats.total > 0 ? (stats.todo / stats.total) * 100 : 0}%`,
                  }}
                />
              </div>
            </div>
          </div>

          <div className="mt-6 pt-6 border-t border-neutral-light/60">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-neutral-dark/60">Completion Rate</p>
                <p className="text-2xl font-bold text-neutral-dark">
                  {stats.total > 0 ? Math.round((stats.completed / stats.total) * 100) : 0}%
                </p>
              </div>
              <div className="text-right">
                <p className="text-sm text-neutral-dark/60">Completed This Week</p>
                <p className="text-2xl font-bold text-green-600">{stats.completedThisWeek}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
