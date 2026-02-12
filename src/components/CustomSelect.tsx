import { useState, useRef, useEffect } from 'react';
import { ChevronDown, Check } from 'lucide-react';

export interface SelectOption {
  value: string;
  label: string;
  icon?: React.ReactNode;
  color?: string;
}

interface CustomSelectProps {
  value: string;
  options: SelectOption[];
  onChange: (value: string) => void;
  label?: string;
  placeholder?: string;
  className?: string;
}

export function CustomSelect({
  value,
  options,
  onChange,
  label,
  placeholder = '选择...',
  className = '',
}: CustomSelectProps) {
  const [isOpen, setIsOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  const selectedOption = options.find((opt) => opt.value === value);

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (containerRef.current && !containerRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };

    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside);
      return () => document.removeEventListener('mousedown', handleClickOutside);
    }
  }, [isOpen]);

  const handleSelect = (optionValue: string) => {
    onChange(optionValue);
    setIsOpen(false);
  };

  return (
    <div className={`flex flex-col gap-1.5 ${className}`} ref={containerRef}>
      {label && (
        <label className="text-sm font-medium text-neutral-dark/60">
          {label}
        </label>
      )}

      <div className="relative">
        {/* Trigger button */}
        <button
          type="button"
          onClick={() => setIsOpen(!isOpen)}
          className="w-full px-4 py-2.5 rounded-xl bg-neutral-light/40 text-neutral-dark
                   hover:bg-neutral-light/60 transition-all duration-200
                   flex items-center justify-between gap-2
                   focus:outline-none focus:ring-2 focus:ring-primary/30"
        >
          <div className="flex items-center gap-2.5 flex-1 text-left">
            {selectedOption?.icon}
            <span className={selectedOption ? '' : 'text-neutral-dark/40'}>
              {selectedOption?.label || placeholder}
            </span>
          </div>
          <ChevronDown
            size={18}
            className={`text-neutral-dark/40 transition-transform duration-200 ${
              isOpen ? 'rotate-180' : ''
            }`}
          />
        </button>

        {/* Dropdown menu */}
        {isOpen && (
          <div
            className="absolute z-50 w-full mt-2 bg-white rounded-xl shadow-lg border border-neutral-light/60
                     overflow-hidden animate-scale-up origin-top"
          >
            {options.map((option) => {
              const isSelected = option.value === value;
              return (
                <button
                  key={option.value}
                  type="button"
                  onClick={() => handleSelect(option.value)}
                  className={`w-full px-4 py-3 flex items-center justify-between gap-2
                           transition-all duration-200 text-left
                           ${
                             isSelected
                               ? 'bg-primary/10 text-primary font-medium'
                               : 'hover:bg-neutral-light/40 text-neutral-dark'
                           }`}
                >
                  <div className="flex items-center gap-2.5">
                    {option.icon}
                    <span>{option.label}</span>
                  </div>
                  {isSelected && (
                    <Check size={18} className="text-primary flex-shrink-0" />
                  )}
                </button>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
}
