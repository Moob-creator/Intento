import { useState, useRef, useEffect, ReactNode } from 'react';

interface TooltipProps {
  content: string;
  children: ReactNode;
  delay?: number;
}

export function Tooltip({ content, children, delay = 300 }: TooltipProps) {
  const [isVisible, setIsVisible] = useState(false);
  const [position, setPosition] = useState({ top: 0, left: 0 });
  const timeoutRef = useRef<number | undefined>(undefined);
  const triggerRef = useRef<HTMLDivElement>(null);

  const showTooltip = () => {
    timeoutRef.current = window.setTimeout(() => {
      if (triggerRef.current) {
        const rect = triggerRef.current.getBoundingClientRect();
        setPosition({
          top: rect.bottom + 8,
          left: rect.left + rect.width / 2,
        });
        setIsVisible(true);
      }
    }, delay);
  };

  const hideTooltip = () => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }
    setIsVisible(false);
  };

  useEffect(() => {
    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, []);

  return (
    <>
      <div
        ref={triggerRef}
        onMouseEnter={showTooltip}
        onMouseLeave={hideTooltip}
        className="inline-block"
      >
        {children}
      </div>
      {isVisible && (
        <div
          className="fixed z-50 px-3 py-2 text-sm font-medium text-neutral-dark bg-white/95 backdrop-blur-sm
                     rounded-lg shadow-lg border border-neutral-light/60 pointer-events-none
                     animate-fade-in"
          style={{
            top: `${position.top}px`,
            left: `${position.left}px`,
            transform: 'translateX(-50%)',
          }}
        >
          {content}
          {/* Arrow */}
          <div
            className="absolute w-2 h-2 bg-white border-l border-t border-neutral-light/60
                       transform rotate-45 -translate-y-1/2"
            style={{
              top: '-4px',
              left: '50%',
              marginLeft: '-4px',
            }}
          />
        </div>
      )}
    </>
  );
}
