import { useEffect } from 'react';
import { CheckCircle2, XCircle, Info, X } from 'lucide-react';

export type ToastType = 'success' | 'error' | 'info';

interface ToastProps {
  message: string;
  type: ToastType;
  onClose: () => void;
  duration?: number;
}

export function Toast({ message, type, onClose, duration = 3000 }: ToastProps) {
  useEffect(() => {
    if (duration > 0) {
      const timer = setTimeout(onClose, duration);
      return () => clearTimeout(timer);
    }
  }, [duration, onClose]);

  const icons = {
    success: <CheckCircle2 size={20} className="text-green-600" />,
    error: <XCircle size={20} className="text-red-600" />,
    info: <Info size={20} className="text-blue-600" />,
  };

  const bgColors = {
    success: 'bg-green-50 border-green-200',
    error: 'bg-red-50 border-red-200',
    info: 'bg-blue-50 border-blue-200',
  };

  return (
    <div
      className={`fixed top-20 right-6 z-[100] flex items-center gap-3 px-4 py-3 rounded-xl border-2 shadow-xl
                  ${bgColors[type]} animate-slide-in-right`}
      style={{ minWidth: '320px', maxWidth: '480px' }}
    >
      {icons[type]}
      <span className="flex-1 text-sm font-medium text-neutral-dark">{message}</span>
      <button
        onClick={onClose}
        className="p-1 hover:bg-black/5 rounded-lg transition-colors"
        aria-label="关闭"
      >
        <X size={16} className="text-neutral-dark/60" />
      </button>
    </div>
  );
}
