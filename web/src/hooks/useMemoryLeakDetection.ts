import { useEffect, useRef, useCallback } from 'react';

interface MemorySnapshot {
  usedJSHeapSize: number;
  totalJSHeapSize: number;
  jsHeapSizeLimit: number;
  timestamp: number;
}

export function useMemoryLeakDetection(
  componentName: string,
  threshold: number = 50 * 1024 * 1024 // 50MB threshold
) {
  const snapshots = useRef<MemorySnapshot[]>([]);
  const checkInterval = useRef<ReturnType<typeof setInterval> | null>(null);


  const takeSnapshot = useCallback(() => {
    if (performance && (performance as unknown as { memory?: MemorySnapshot }).memory) {
      const memory = (performance as unknown as { memory: MemorySnapshot }).memory;
      const snapshot: MemorySnapshot = {
        usedJSHeapSize: memory.usedJSHeapSize,
        totalJSHeapSize: memory.totalJSHeapSize,
        jsHeapSizeLimit: memory.jsHeapSizeLimit,
        timestamp: Date.now(),
      };
      snapshots.current.push(snapshot);

      // Keep only last 100 snapshots
      if (snapshots.current.length > 100) {
        snapshots.current.shift();
      }

      // Check for memory leak pattern
      if (snapshots.current.length >= 10) {
        const recent = snapshots.current.slice(-10);
        const first = recent[0];
        const last = recent[recent.length - 1];
        const growth = last.usedJSHeapSize - first.usedJSHeapSize;

        if (growth > threshold) {
          console.warn(
            `[Memory Leak Detection] ${componentName}: Potential memory leak detected. ` +
            `Heap grew by ${(growth / 1024 / 1024).toFixed(2)}MB over ${recent.length} snapshots`
          );
        }
      }
    }
  }, [componentName, threshold]);

  useEffect(() => {
    // Take snapshot every 5 seconds
    checkInterval.current = setInterval(takeSnapshot, 5000);

    return () => {
      if (checkInterval.current) {
        clearInterval(checkInterval.current);
      }
      // Clear snapshots on unmount
      snapshots.current = [];
    };
  }, [takeSnapshot]);

  return {
    getSnapshots: () => [...snapshots.current],
    clearSnapshots: () => {
      snapshots.current = [];
    },
  };
}

export function useCleanupOnUnmount(cleanupFn: () => void) {
  const cleanupRef = useRef(cleanupFn);

  useEffect(() => {
    cleanupRef.current = cleanupFn;
  }, [cleanupFn]);

  useEffect(() => {
    return () => {
      cleanupRef.current();
    };
  }, []);
}

export function useWeakRef<T extends object>(initialValue: T): React.MutableRefObject<T | null> {
  const ref = useRef<T | null>(initialValue);

  useEffect(() => {
    return () => {
      // Help GC by clearing reference
      ref.current = null;
    };
  }, []);

  return ref;
}
