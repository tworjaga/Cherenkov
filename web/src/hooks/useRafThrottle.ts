import { useRef, useCallback } from 'react';

export function useRafThrottle<T extends (...args: unknown[]) => void>(
  callback: T,
  fps: number = 60
): T {
  const rafId = useRef<number | null>(null);
  const lastTime = useRef<number>(0);
  const frameInterval = 1000 / fps;

  const throttledCallback = useCallback(
    (...args: Parameters<T>) => {
      const now = performance.now();

      if (now - lastTime.current >= frameInterval) {
        if (rafId.current !== null) {
          cancelAnimationFrame(rafId.current);
        }

        rafId.current = requestAnimationFrame(() => {
          lastTime.current = now;
          callback(...args);
          rafId.current = null;
        });
      }
    },
    [callback, frameInterval]
  );

  return throttledCallback as T;
}

export function useRafDebounce<T extends (...args: unknown[]) => void>(
  callback: T,
  delay: number = 16
): T {
  const rafId = useRef<number | null>(null);

  const debouncedCallback = useCallback(
    (...args: Parameters<T>) => {
      if (rafId.current !== null) {
        cancelAnimationFrame(rafId.current);
      }

      rafId.current = requestAnimationFrame(() => {
        callback(...args);
        rafId.current = null;
      });
    },
    [callback]
  );

  return debouncedCallback as T;
}
