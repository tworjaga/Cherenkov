import React, { useEffect } from 'react';

export type ToastSeverity = 'critical' | 'high' | 'medium' | 'low' | 'info';

export interface Toast {
  id: string;
  title: string;
  message: string;
  severity: ToastSeverity;
  duration?: number;
}

interface ToastProps extends Toast {
  onClose: (id: string) => void;
}

const severityStyles: Record<ToastSeverity, string> = {
  critical: 'border-alert-critical bg-alert-critical/10 text-alert-critical',
  high: 'border-alert-high bg-alert-high/10 text-alert-high',
  medium: 'border-alert-medium bg-alert-medium/10 text-alert-medium',
  low: 'border-alert-low bg-alert-low/10 text-alert-low',
  info: 'border-text-secondary bg-bg-tertiary text-text-secondary',
};

const severityIcons: Record<ToastSeverity, string> = {
  critical: '!',
  high: '!',
  medium: '⚠',
  low: 'ℹ',
  info: 'ℹ',
};

export const ToastItem: React.FC<ToastProps> = ({
  id,
  title,
  message,
  severity,
  duration = 5000,
  onClose,
}) => {
  useEffect(() => {
    if (duration > 0) {
      const timer = setTimeout(() => {
        onClose(id);
      }, duration);
      return () => clearTimeout(timer);
    }
  }, [id, duration, onClose]);

  return (
    <div
      className={`flex items-start gap-3 p-4 rounded-lg border-l-4 shadow-lg backdrop-blur-sm ${severityStyles[severity]} animate-slide-in`}
      role="alert"
      aria-live={severity === 'critical' || severity === 'high' ? 'assertive' : 'polite'}
      aria-atomic="true"
    >

      <span className="text-lg font-bold shrink-0">{severityIcons[severity]}</span>
      <div className="flex-1 min-w-0">
        <h4 className="font-semibold text-sm uppercase tracking-wider">{title}</h4>
        <p className="text-sm mt-1 opacity-90">{message}</p>
      </div>
      <button
        onClick={() => onClose(id)}
        className="shrink-0 p-1 hover:bg-white/10 rounded transition-colors"
        aria-label="Close notification"
      >
        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
  );
};

interface ToastContainerProps {
  toasts: Toast[];
  onClose: (id: string) => void;
}

export const ToastContainer: React.FC<ToastContainerProps> = ({ toasts, onClose }) => {
  if (toasts.length === 0) return null;

  return (
    <div className="fixed top-20 right-4 z-[100] flex flex-col gap-2 w-[400px] max-w-[calc(100vw-2rem)]">
      {toasts.map((toast) => (
        <ToastItem key={toast.id} {...toast} onClose={onClose} />
      ))}
    </div>
  );
};
