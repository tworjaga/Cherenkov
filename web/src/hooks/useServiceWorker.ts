import { useEffect, useState, useCallback } from 'react';

interface ServiceWorkerState {
  isSupported: boolean;
  isRegistered: boolean;
  isOffline: boolean;
  updateAvailable: boolean;
}

export function useServiceWorker(): ServiceWorkerState & {
  register: () => Promise<void>;
  unregister: () => Promise<void>;
  skipWaiting: () => void;
} {
  const [state, setState] = useState<ServiceWorkerState>({
    isSupported: 'serviceWorker' in navigator,
    isRegistered: false,
    isOffline: !navigator.onLine,
    updateAvailable: false,
  });

  useEffect(() => {
    const handleOnline = () => setState((s) => ({ ...s, isOffline: false }));
    const handleOffline = () => setState((s) => ({ ...s, isOffline: true }));

    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  const register = useCallback(async () => {
    if (!state.isSupported) return;

    try {
      const registration = await navigator.serviceWorker.register('/service-worker.js', {
        scope: '/',
      });

      registration.addEventListener('updatefound', () => {
        const newWorker = registration.installing;
        if (newWorker) {
          newWorker.addEventListener('statechange', () => {
            if (newWorker.state === 'installed' && navigator.serviceWorker.controller) {
              setState((s) => ({ ...s, updateAvailable: true }));
            }
          });
        }
      });

      setState((s) => ({ ...s, isRegistered: true }));
    } catch (error) {
      console.error('Service Worker registration failed:', error);
    }
  }, [state.isSupported]);

  const unregister = useCallback(async () => {
    if (!state.isSupported) return;

    const registration = await navigator.serviceWorker.ready;
    await registration.unregister();
    setState((s) => ({ ...s, isRegistered: false }));
  }, [state.isSupported]);

  const skipWaiting = useCallback(() => {
    navigator.serviceWorker.ready.then((registration) => {
      registration.waiting?.postMessage({ type: 'SKIP_WAITING' });
      window.location.reload();
    });
  }, []);

  return {
    ...state,
    register,
    unregister,
    skipWaiting,
  };
}
