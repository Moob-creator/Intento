import { useState, useEffect, useRef } from 'react';
import { Calendar, Clock, ChevronLeft, ChevronRight, ChevronDown } from 'lucide-react';

interface DateTimePickerProps {
  value: { date: string; time: string };
  onChange: (value: { date: string; time: string }) => void;
  onClose: () => void;
}

export function DateTimePicker({ value, onChange, onClose }: DateTimePickerProps) {
  const [selectedDate, setSelectedDate] = useState<Date>(() => {
    if (value.date) {
      return new Date(value.date);
    }
    return new Date();
  });

  const [selectedTime, setSelectedTime] = useState(value.time || '09:00');
  const [viewMonth, setViewMonth] = useState(selectedDate.getMonth());
  const [viewYear, setViewYear] = useState(selectedDate.getFullYear());
  const [showMonthPicker, setShowMonthPicker] = useState(false);
  const [showYearPicker, setShowYearPicker] = useState(false);
  const [showTimePicker, setShowTimePicker] = useState(false);

  const pickerRef = useRef<HTMLDivElement>(null);

  // Close picker when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (pickerRef.current && !pickerRef.current.contains(event.target as Node)) {
        onClose();
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, [onClose]);

  // Get days in month
  const getDaysInMonth = (year: number, month: number) => {
    return new Date(year, month + 1, 0).getDate();
  };

  // Get first day of month (0 = Sunday, 6 = Saturday)
  const getFirstDayOfMonth = (year: number, month: number) => {
    return new Date(year, month, 1).getDay();
  };

  // Generate calendar days
  const generateCalendarDays = () => {
    const daysInMonth = getDaysInMonth(viewYear, viewMonth);
    const firstDay = getFirstDayOfMonth(viewYear, viewMonth);
    const daysInPrevMonth = getDaysInMonth(viewYear, viewMonth - 1);

    const days: Array<{ day: number; isCurrentMonth: boolean; date: Date }> = [];

    // Previous month days
    for (let i = firstDay - 1; i >= 0; i--) {
      const day = daysInPrevMonth - i;
      days.push({
        day,
        isCurrentMonth: false,
        date: new Date(viewYear, viewMonth - 1, day),
      });
    }

    // Current month days
    for (let i = 1; i <= daysInMonth; i++) {
      days.push({
        day: i,
        isCurrentMonth: true,
        date: new Date(viewYear, viewMonth, i),
      });
    }

    // Next month days to fill the grid
    const remainingDays = 42 - days.length; // 6 rows * 7 days
    for (let i = 1; i <= remainingDays; i++) {
      days.push({
        day: i,
        isCurrentMonth: false,
        date: new Date(viewYear, viewMonth + 1, i),
      });
    }

    return days;
  };

  const days = generateCalendarDays();
  const today = new Date();
  today.setHours(0, 0, 0, 0);

  const isToday = (date: Date) => {
    return date.getTime() === today.getTime();
  };

  const isSelected = (date: Date) => {
    const selected = new Date(selectedDate);
    selected.setHours(0, 0, 0, 0);
    return date.getTime() === selected.getTime();
  };

  const handleDayClick = (date: Date) => {
    setSelectedDate(date);
    const dateStr = date.toISOString().split('T')[0];
    onChange({ date: dateStr, time: selectedTime });
  };

  const handleTimeChange = (time: string) => {
    setSelectedTime(time);
    const dateStr = selectedDate.toISOString().split('T')[0];
    onChange({ date: dateStr, time });
  };

  const goToPreviousMonth = () => {
    if (viewMonth === 0) {
      setViewMonth(11);
      setViewYear(viewYear - 1);
    } else {
      setViewMonth(viewMonth - 1);
    }
  };

  const goToNextMonth = () => {
    if (viewMonth === 11) {
      setViewMonth(0);
      setViewYear(viewYear + 1);
    } else {
      setViewMonth(viewMonth + 1);
    }
  };

  const months = [
    '1 月', '2 月', '3 月', '4 月', '5 月', '6 月',
    '7 月', '8 月', '9 月', '10 月', '11 月', '12 月'
  ];

  const weekDays = ['日', '一', '二', '三', '四', '五', '六'];

  // Generate time options (30-minute intervals)
  const timeOptions = Array.from({ length: 48 }, (_, i) => {
    const hours = Math.floor(i / 2);
    const minutes = (i % 2) * 30;
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}`;
  });

  // Format date display
  const formatDateDisplay = () => {
    const weekDay = ['周日', '周一', '周二', '周三', '周四', '周五', '周六'][selectedDate.getDay()];
    return `${selectedDate.getFullYear()}/${(selectedDate.getMonth() + 1).toString().padStart(2, '0')}/${selectedDate.getDate().toString().padStart(2, '0')} ${weekDay}`;
  };

  return (
    <div
      ref={pickerRef}
      className="absolute top-full left-0 right-0 mt-2 bg-white rounded-xl shadow-2xl border border-neutral-light/60 z-50 overflow-hidden"
    >
      {/* Time and Date Display */}
      <div className="p-4 border-b border-neutral-light/60 flex items-center gap-3">
        {/* Time Selector */}
        <div className="relative flex-shrink-0">
          <button
            onClick={() => {
              setShowTimePicker(!showTimePicker);
              setShowMonthPicker(false);
              setShowYearPicker(false);
            }}
            className="flex items-center gap-2 px-4 py-2 bg-neutral-light/30 hover:bg-neutral-light/50 rounded-lg transition-all duration-200"
          >
            <Clock size={16} className="text-neutral-dark/80" />
            <span className="text-sm font-medium text-neutral-dark">{selectedTime}</span>
            <ChevronDown size={16} className="text-neutral-dark/70" />
          </button>

          {/* Time Picker Dropdown */}
          {showTimePicker && (
            <div className="absolute top-full left-0 mt-2 bg-white rounded-lg shadow-xl border border-neutral-light/60 max-h-64 overflow-y-auto z-50">
              {timeOptions.map((time) => (
                <button
                  key={time}
                  onClick={() => {
                    handleTimeChange(time);
                    setShowTimePicker(false);
                  }}
                  className={`w-full px-4 py-2 text-sm text-left transition-colors ${
                    time === selectedTime
                      ? 'bg-primary text-white'
                      : 'hover:bg-neutral-light/40 text-neutral-dark'
                  }`}
                >
                  {time}
                </button>
              ))}
            </div>
          )}
        </div>

        {/* Date Display */}
        <button
          onClick={() => {
            setShowTimePicker(false);
            setShowMonthPicker(false);
            setShowYearPicker(false);
          }}
          className="flex items-center gap-2 px-4 py-2 bg-neutral-light/30 hover:bg-neutral-light/50 rounded-lg transition-all duration-200 flex-1"
        >
          <Calendar size={16} className="text-neutral-dark/80" />
          <span className="text-sm font-medium text-neutral-dark">{formatDateDisplay()}</span>
        </button>
      </div>

      {/* Calendar View */}
      <div className="p-4">
        {/* Month and Year Selectors */}
        <div className="flex items-center justify-between mb-4">
          <button
            onClick={goToPreviousMonth}
            className="p-2 hover:bg-neutral-light/40 rounded-lg transition-colors"
          >
            <ChevronLeft size={20} className="text-neutral-dark/80" />
          </button>

          <div className="flex items-center gap-2">
            {/* Month Selector */}
            <div className="relative">
              <button
                onClick={() => {
                  setShowMonthPicker(!showMonthPicker);
                  setShowYearPicker(false);
                  setShowTimePicker(false);
                }}
                className="flex items-center gap-2 px-3 py-1.5 bg-neutral-light/30 hover:bg-neutral-light/50 rounded-lg transition-all duration-200"
              >
                <span className="text-sm font-medium text-neutral-dark">{months[viewMonth]}</span>
                <ChevronDown size={14} className="text-neutral-dark/70" />
              </button>

              {showMonthPicker && (
                <div className="absolute top-full left-0 mt-2 bg-white rounded-lg shadow-xl border border-neutral-light/60 grid grid-cols-3 gap-1 p-2 z-50">
                  {months.map((month, index) => (
                    <button
                      key={month}
                      onClick={() => {
                        setViewMonth(index);
                        setShowMonthPicker(false);
                      }}
                      className={`px-3 py-2 text-sm rounded-lg transition-colors ${
                        index === viewMonth
                          ? 'bg-primary text-white'
                          : 'hover:bg-neutral-light/40 text-neutral-dark'
                      }`}
                    >
                      {month}
                    </button>
                  ))}
                </div>
              )}
            </div>

            {/* Year Selector */}
            <div className="relative">
              <button
                onClick={() => {
                  setShowYearPicker(!showYearPicker);
                  setShowMonthPicker(false);
                  setShowTimePicker(false);
                }}
                className="flex items-center gap-2 px-3 py-1.5 bg-neutral-light/30 hover:bg-neutral-light/50 rounded-lg transition-all duration-200"
              >
                <span className="text-sm font-medium text-neutral-dark">{viewYear}</span>
                <ChevronDown size={14} className="text-neutral-dark/70" />
              </button>

              {showYearPicker && (
                <div className="absolute top-full right-0 mt-2 bg-white rounded-lg shadow-xl border border-neutral-light/60 max-h-64 overflow-y-auto z-50">
                  {Array.from({ length: 20 }, (_, i) => viewYear - 10 + i).map((year) => (
                    <button
                      key={year}
                      onClick={() => {
                        setViewYear(year);
                        setShowYearPicker(false);
                      }}
                      className={`w-full px-4 py-2 text-sm text-left transition-colors ${
                        year === viewYear
                          ? 'bg-primary text-white'
                          : 'hover:bg-neutral-light/40 text-neutral-dark'
                      }`}
                    >
                      {year}
                    </button>
                  ))}
                </div>
              )}
            </div>
          </div>

          <button
            onClick={goToNextMonth}
            className="p-2 hover:bg-neutral-light/40 rounded-lg transition-colors"
          >
            <ChevronRight size={20} className="text-neutral-dark/80" />
          </button>
        </div>

        {/* Week Days */}
        <div className="grid grid-cols-7 gap-1 mb-2">
          {weekDays.map((day) => (
            <div
              key={day}
              className="text-center text-xs font-medium text-neutral-dark/40 py-2"
            >
              {day}
            </div>
          ))}
        </div>

        {/* Calendar Grid */}
        <div className="grid grid-cols-7 gap-1">
          {days.map((dayInfo, index) => {
            const isTodayDate = isToday(dayInfo.date);
            const isSelectedDate = isSelected(dayInfo.date);

            return (
              <button
                key={index}
                onClick={() => handleDayClick(dayInfo.date)}
                className={`
                  aspect-square flex items-center justify-center rounded-lg text-sm font-medium transition-all duration-150
                  ${!dayInfo.isCurrentMonth ? 'text-neutral-dark/20' : 'text-neutral-dark'}
                  ${isSelectedDate ? 'bg-emerald-500 text-white shadow-md' : ''}
                  ${isTodayDate && !isSelectedDate ? 'border-2 border-emerald-500' : ''}
                  ${!isSelectedDate && !isTodayDate ? 'hover:bg-neutral-light/40' : ''}
                `}
              >
                {dayInfo.day}
              </button>
            );
          })}
        </div>

        {/* Quick Actions */}
        <div className="mt-4 pt-4 border-t border-neutral-light/60 flex items-center gap-2">
          <button
            onClick={() => {
              const now = new Date();
              setSelectedDate(now);
              setViewMonth(now.getMonth());
              setViewYear(now.getFullYear());
              const hours = now.getHours().toString().padStart(2, '0');
              const minutes = Math.ceil(now.getMinutes() / 30) * 30;
              const time = minutes === 60
                ? `${(parseInt(hours) + 1).toString().padStart(2, '0')}:00`
                : `${hours}:${minutes.toString().padStart(2, '0')}`;
              handleTimeChange(time);
            }}
            className="px-3 py-1.5 rounded-lg text-xs font-medium bg-blue-50 text-blue-600 hover:bg-blue-100 transition-colors"
          >
            现在
          </button>
          <button
            onClick={() => {
              const tomorrow = new Date();
              tomorrow.setDate(tomorrow.getDate() + 1);
              setSelectedDate(tomorrow);
              setViewMonth(tomorrow.getMonth());
              setViewYear(tomorrow.getFullYear());
              handleTimeChange('09:00');
            }}
            className="px-3 py-1.5 rounded-lg text-xs font-medium bg-emerald-50 text-emerald-600 hover:bg-emerald-100 transition-colors"
          >
            明天
          </button>
          <button
            onClick={onClose}
            className="ml-auto px-4 py-1.5 rounded-lg text-xs font-medium bg-primary text-white hover:bg-primary-dark transition-colors"
          >
            确定
          </button>
        </div>
      </div>
    </div>
  );
}
