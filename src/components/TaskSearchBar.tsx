import { Search } from 'lucide-react';

interface TaskSearchBarProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
}

export function TaskSearchBar({ value, onChange, placeholder = "Search tasks..." }: TaskSearchBarProps) {
  return (
    <div className="relative flex-1 min-w-[240px]">
      <div className="absolute inset-y-0 left-0 flex items-center pl-4 pointer-events-none text-neutral-dark/40">
        <Search size={20} />
      </div>
      <input
        type="text"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        className="w-full h-12 pl-12 pr-4 text-base rounded-lg bg-neutral-light/60 border-none
                 text-neutral-dark placeholder:text-neutral-dark/40
                 focus:outline-none focus:ring-2 focus:ring-primary/30 focus:bg-white
                 transition-all duration-200"
      />
    </div>
  );
}
