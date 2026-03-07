import { useState, useMemo } from 'react';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import type { Task } from '../types/task';

interface CalendarViewProps {
  tasks: Task[];
  onTaskClick: (task: Task) => void;
  selectedTag?: string | null;
}

interface DayCell {
  date: Date;
  tasks: Task[];
  isCurrentMonth: boolean;
  isToday: boolean;
}

export function CalendarView({ tasks, onTaskClick, selectedTag }: CalendarViewProps) {
  const [currentDate, setCurrentDate] = useState(new Date());

  // Navigate to previous month
  const goToPreviousMonth = () => {
    setCurrentDate(new Date(currentDate.getFullYear(), currentDate.getMonth() - 1, 1));
  };

  // Navigate to next month
  const goToNextMonth = () => {
    setCurrentDate(new Date(currentDate.getFullYear(), currentDate.getMonth() + 1, 1));
  };

  // Go to today
  const goToToday = () => {
    setCurrentDate(new Date());
  };

  // Generate calendar grid
  const calendarGrid = useMemo(() => {
    const year = currentDate.getFullYear();
    const month = currentDate.getMonth();

    // First day of current month
    const firstDay = new Date(year, month, 1);

    // Start from Monday of the week containing the first day
    const startDate = new Date(firstDay);
    const firstDayOfWeek = firstDay.getDay();
    const daysToSubtract = firstDayOfWeek === 0 ? 6 : firstDayOfWeek - 1; // Monday = 0
    startDate.setDate(firstDay.getDate() - daysToSubtract);

    // Generate 6 weeks (42 days) to ensure full calendar
    const grid: DayCell[] = [];
    const today = new Date();
    today.setHours(0, 0, 0, 0);

    for (let i = 0; i < 42; i++) {
      const date = new Date(startDate.getTime() + i * 86400 * 1000);

      const dayStart = Math.floor(date.getTime() / 1000);
      const dayEnd = dayStart + 86400 - 1;

      // Filter tasks for this day (by deadline)
      const dayTasks = tasks.filter(task => {
        if (!task.deadline) return false;
        if (selectedTag && !task.tags?.includes(selectedTag)) return false;
        return task.deadline >= dayStart && task.deadline <= dayEnd;
      });

      grid.push({
        date,
        tasks: dayTasks,
        isCurrentMonth: date.getMonth() === month,
        isToday: date.toDateString() === today.toDateString(),
      });
    }

    return grid;
  }, [currentDate, tasks, selectedTag]);

  // Tasks with no deadline
  const noDeadlineTasks = useMemo(() => {
    return tasks.filter(task => {
      if (!task.deadline) {
        if (selectedTag && !task.tags?.includes(selectedTag)) return false;
        return true;
      }
      return false;
    });
  }, [tasks, selectedTag]);

  // Format month/year
  const monthYearText = currentDate.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
  });

  // Day of week labels
  const weekDays = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];

  return (
    <div className="flex flex-col h-full">
      {/* Calendar Header */}
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-2xl font-bold text-neutral-dark">{monthYearText}</h2>

        <div className="flex items-center gap-2">
          <button
            onClick={goToToday}
            className="px-4 py-2 rounded-lg text-sm font-medium bg-neutral-light/40 text-neutral-dark/70 hover:bg-neutral-light/60 transition-colors"
          >
            Today
          </button>
          <button
            onClick={goToPreviousMonth}
            className="p-2 rounded-lg hover:bg-neutral-light/40 transition-colors"
            aria-label="Previous month"
          >
            <ChevronLeft size={20} className="text-neutral-dark/60" />
          </button>
          <button
            onClick={goToNextMonth}
            className="p-2 rounded-lg hover:bg-neutral-light/40 transition-colors"
            aria-label="Next month"
          >
            <ChevronRight size={20} className="text-neutral-dark/60" />
          </button>
        </div>
      </div>

      {/* Calendar Grid */}
      <div className="flex-1 overflow-y-auto">
        {/* Week day headers */}
        <div className="grid grid-cols-7 gap-2 mb-2">
          {weekDays.map(day => (
            <div
              key={day}
              className="text-center text-xs font-semibold text-neutral-dark/60 py-2"
            >
              {day}
            </div>
          ))}
        </div>

        {/* Calendar cells */}
        <div className="grid grid-cols-7 gap-2">
          {calendarGrid.map((cell, index) => (
            <DayCell
              key={index}
              cell={cell}
              onTaskClick={onTaskClick}
            />
          ))}
        </div>

        {/* No deadline section */}
        {noDeadlineTasks.length > 0 && (
          <div className="mt-6 pt-6 border-t border-neutral-light/60">
            <h3 className="text-sm font-semibold text-neutral-dark/60 mb-3 uppercase tracking-wide">
              No Due Date ({noDeadlineTasks.length})
            </h3>
            <div className="flex flex-wrap gap-2">
              {noDeadlineTasks.map(task => (
                <button
                  key={task.id}
                  onClick={() => onTaskClick(task)}
                  className="px-3 py-2 rounded-lg bg-neutral-light/40 hover:bg-neutral-light/60
                           text-sm text-neutral-dark transition-colors text-left max-w-xs truncate"
                >
                  <span
                    className={`inline-block w-2 h-2 rounded-full mr-2 ${
                      task.priority === 'high'
                        ? 'bg-red-400'
                        : task.priority === 'medium'
                        ? 'bg-amber-400'
                        : 'bg-blue-400'
                    }`}
                  />
                  {task.title}
                </button>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

// Individual day cell component
interface DayCellProps {
  cell: DayCell;
  onTaskClick: (task: Task) => void;
}

function DayCell({ cell, onTaskClick }: DayCellProps) {
  const [showDrawer, setShowDrawer] = useState(false);

  const { date, tasks, isCurrentMonth, isToday } = cell;
  const dayNumber = date.getDate();

  // Calculate priority counts
  const highPriorityCount = tasks.filter(t => t.priority === 'high').length;
  const mediumPriorityCount = tasks.filter(t => t.priority === 'medium').length;
  const lowPriorityCount = tasks.filter(t => t.priority === 'low').length;

  // Check for overdue tasks - only relevant for today and future dates
  // Past dates should just show their tasks without "overdue" styling
  const todayStart = new Date();
  todayStart.setHours(0, 0, 0, 0);
  const todayStartTimestamp = Math.floor(todayStart.getTime() / 1000);

  const cellDateStart = new Date(date);
  cellDateStart.setHours(0, 0, 0, 0);
  const cellDateTimestamp = Math.floor(cellDateStart.getTime() / 1000);

  // Only mark as overdue if:
  // 1. This cell is today or future (cellDate >= today)
  // 2. AND it has tasks with deadlines before today that are not done
  const hasOverdue = cellDateTimestamp >= todayStartTimestamp &&
                     tasks.some(t => t.deadline && t.deadline < todayStartTimestamp && t.status !== 'done');

  // Determine border style (priority: overdue > today > normal)
  let borderClass = 'border border-neutral-light/40';
  if (hasOverdue) {
    borderClass = 'border-2 border-red-400';
  } else if (isToday) {
    borderClass = 'border-2 border-primary shadow-md';
  }

  return (
    <>
      <button
        onClick={() => tasks.length > 0 && setShowDrawer(true)}
        className={`aspect-square p-2 rounded-xl transition-all duration-200 flex flex-col ${borderClass}
                   ${isCurrentMonth ? 'bg-white' : 'bg-neutral-light/20'}
                   ${tasks.length > 0 ? 'hover:shadow-lg hover:scale-105 cursor-pointer' : 'cursor-default'}
                 `}
      >
        {/* Date number */}
        <div
          className={`text-sm font-semibold mb-1 ${
            isToday
              ? 'text-primary'
              : isCurrentMonth
              ? 'text-neutral-dark'
              : 'text-neutral-dark/40'
          }`}
        >
          {dayNumber}
        </div>

        {/* Priority indicators + count */}
        {tasks.length > 0 && (
          <div className="flex items-center justify-center gap-1 mt-auto">
            {/* Priority dots */}
            <div className="flex gap-0.5">
              {highPriorityCount > 0 && (
                <div className="w-1.5 h-1.5 rounded-full bg-red-400" />
              )}
              {mediumPriorityCount > 0 && (
                <div className="w-1.5 h-1.5 rounded-full bg-amber-400" />
              )}
              {lowPriorityCount > 0 && (
                <div className="w-1.5 h-1.5 rounded-full bg-blue-400" />
              )}
            </div>

            {/* Task count */}
            {tasks.length > 3 && (
              <span className="text-xs text-neutral-dark/60 font-medium">
                ({tasks.length})
              </span>
            )}
          </div>
        )}
      </button>

      {/* Task drawer - slide from bottom */}
      {showDrawer && (
        <>
          {/* Backdrop */}
          <div
            className="fixed inset-0 bg-black/40 backdrop-blur-sm z-50 animate-fade-in"
            onClick={() => setShowDrawer(false)}
          />

          {/* Drawer */}
          <div className="fixed bottom-0 left-0 right-0 z-50 bg-white rounded-t-3xl shadow-2xl max-h-[70vh] overflow-hidden animate-slide-up">
            <div className="p-6">
              {/* Drawer header */}
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-lg font-bold text-neutral-dark">
                  {date.toLocaleDateString('en-US', {
                    month: 'long',
                    day: 'numeric',
                    year: 'numeric',
                  })}
                </h3>
                <button
                  onClick={() => setShowDrawer(false)}
                  className="text-neutral-dark/60 hover:text-neutral-dark"
                >
                  ×
                </button>
              </div>

              {/* Task list */}
              <div className="space-y-2 overflow-y-auto max-h-[calc(70vh-120px)]">
                {tasks.map(task => (
                  <button
                    key={task.id}
                    onClick={() => {
                      setShowDrawer(false);
                      onTaskClick(task);
                    }}
                    className={`w-full p-4 rounded-xl text-left transition-all duration-200
                               hover:shadow-md hover:scale-[1.02] border-l-4
                               ${
                                 task.priority === 'high'
                                   ? 'bg-red-50 border-red-400'
                                   : task.priority === 'medium'
                                   ? 'bg-amber-50 border-amber-400'
                                   : 'bg-blue-50 border-blue-400'
                               }`}
                  >
                    <div className="font-medium text-neutral-dark mb-1">
                      {task.title}
                    </div>
                    {task.description && (
                      <div className="text-sm text-neutral-dark/60 line-clamp-2">
                        {task.description}
                      </div>
                    )}
                    <div className="flex items-center gap-2 mt-2">
                      <span
                        className={`text-xs px-2 py-0.5 rounded-full font-medium ${
                          task.status === 'done'
                            ? 'bg-emerald-100 text-emerald-700'
                            : task.status === 'doing'
                            ? 'bg-amber-100 text-amber-700'
                            : 'bg-blue-100 text-blue-700'
                        }`}
                      >
                        {task.status}
                      </span>
                      {task.tags && task.tags.length > 0 && (
                        <div className="flex gap-1 flex-wrap">
                          {task.tags.map(tag => (
                            <span
                              key={tag}
                              className="text-xs text-neutral-dark/60"
                            >
                              #{tag}
                            </span>
                          ))}
                        </div>
                      )}
                    </div>
                  </button>
                ))}
              </div>
            </div>
          </div>
        </>
      )}
    </>
  );
}
