import type { TaskStatus } from '../types/task';

interface StatusFilterProps {
  activeStatus: TaskStatus | 'all';
  onStatusChange: (status: TaskStatus | 'all') => void;
}

const filterOptions: { value: TaskStatus | 'all'; label: string }[] = [
  { value: 'all', label: 'All' },
  { value: 'todo', label: 'To Do' },
  { value: 'doing', label: 'Doing' },
  { value: 'done', label: 'Done' },
];

export function StatusFilter({ activeStatus, onStatusChange }: StatusFilterProps) {
  return (
    <div className="flex gap-2 overflow-x-auto">
      {filterOptions.map((option) => (
        <button
          key={option.value}
          onClick={() => onStatusChange(option.value)}
          className={`
            flex h-10 shrink-0 items-center justify-center px-5 rounded-lg
            text-sm font-medium transition-all duration-200
            ${
              activeStatus === option.value
                ? 'bg-primary/20 text-primary shadow-warm'
                : 'bg-neutral-light/60 text-neutral-dark hover:bg-neutral-light hover:shadow-soft'
            }
          `}
        >
          {option.label}
        </button>
      ))}
    </div>
  );
}
