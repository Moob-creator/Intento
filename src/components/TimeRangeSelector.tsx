import type { SummaryType } from '../types/summary';

interface TimeRangeSelectorProps {
  value: SummaryType;
  onChange: (value: SummaryType) => void;
}

interface TimeRange {
  value: SummaryType;
  label: string;
  icon: string;
}

const TIME_RANGES: TimeRange[] = [
  { value: 'daily', label: '每日', icon: '📅' },
  { value: 'weekly', label: '每周', icon: '📆' },
  { value: 'monthly', label: '每月', icon: '🗓️' },
  { value: 'semi_annual', label: '半年', icon: '📊' },
  { value: 'yearly', label: '年度', icon: '🎉' },
];

export function TimeRangeSelector({ value, onChange }: TimeRangeSelectorProps) {
  return (
    <div className="flex gap-2 overflow-x-auto pb-1">
      {TIME_RANGES.map((range) => (
        <button
          key={range.value}
          onClick={() => onChange(range.value)}
          className={`
            flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium
            transition-all duration-200 whitespace-nowrap
            ${value === range.value
              ? 'bg-primary text-white shadow-md'
              : 'bg-neutral-light/40 text-neutral-dark/70 hover:bg-neutral-light/60'
            }
          `}
        >
          <span>{range.icon}</span>
          <span>{range.label}</span>
        </button>
      ))}
    </div>
  );
}
