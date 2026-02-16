import { useState, useCallback } from 'react';
import { Toast, ToastSeverity } from '../components/ui/Toast';

export const useToast = () => {
  const [toasts, setToasts] = useState<Toast[]>([]);

  const addToast = useCallback((toast: Omit<Toast, 'id'>) => {
    const id = `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    setToasts((prev) => [...prev, { ...toast, id }]);
    return id;
  }, []);

  const removeToast = useCallback((id: string) => {
    setToasts((prev) => prev.filter((t) => t.id !== id));
  }, []);

  const clearAll = useCallback(() => {
    setToasts([]);
  }, []);

  const toast = useCallback(
    (title: string, message: string, severity: ToastSeverity = 'info', duration?: number) => {
      return addToast({ title, message, severity, duration });
    },
    [addToast]
  );

  return {
    toasts,
    addToast,
    removeToast,
    clearAll,
    toast,
  };
};
