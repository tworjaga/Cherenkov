/**
 * Throttle hook
 * Limits function execution to once per specified period
 */

import { useState, useEffect, useRef, useCallback } from 'react';

export function useThrottle<T>(value: T, interval: number): T {
  const [throttledValue, setThrottledValue] = useState<T>(value);
  const lastExecuted = useRef<number>(Date.now());

  useEffect(() => {
    const now = Date.now();
    const timeElapsed = now - lastExecuted.current;
    let timeoutId: ReturnType<typeof setTimeout> | undefined;

    if (timeElapsed >= interval) {
      setThrottledValue(value);
      lastExecuted.current = now;
    } else {
      timeoutId = setTimeout(() => {
        setThrottledValue(value);
        lastExecuted.current = Date.now();
      }, interval - timeElapsed);
    }

    return () => {
      if (timeoutId) clearTimeout(timeoutId);
    };
  }, [value, interval]);


  return throttledValue;
}

export function useThrottledCallback<T extends (...args: unknown[]) => unknown>(
  callback: T,
  interval: number
): (...args: Parameters<T>) => void {
  const lastExecuted = useRef<number>(0);

  return useCallback(
    (...args: Parameters<T>) => {
      const now = Date.now();
      if (now - lastExecuted.current >= interval) {
        lastExecuted.current = now;
        callback(...args);
      }
    },
    [callback, interval]
  );
}
