import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { renderHook, act } from '@testing-library/react';

describe('useRafThrottle', () => {
  let rafCallbacks: Map<number, FrameRequestCallback>;
  let rafId: number;
  let useRafThrottle: typeof import('./useRafThrottle').useRafThrottle;

  beforeEach(async () => {
    rafCallbacks = new Map();
    rafId = 0;

    // Mock requestAnimationFrame before importing
    vi.stubGlobal('requestAnimationFrame', (callback: FrameRequestCallback) => {
      rafId++;
      rafCallbacks.set(rafId, callback);
      return rafId;
    });

    vi.stubGlobal('cancelAnimationFrame', (id: number) => {
      rafCallbacks.delete(id);
    });

    // Mock performance.now with increasing time
    let now = 0;
    vi.stubGlobal('performance', {
      now: () => {
        now += 20; // Increment by 20ms each call
        return now;
      },
    });

    // Dynamic import to get module with mocks applied
    const module = await import('./useRafThrottle');
    useRafThrottle = module.useRafThrottle;
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  const flushRaf = () => {
    const callbacks = Array.from(rafCallbacks.entries());
    rafCallbacks.clear();
    callbacks.forEach(([, cb]) => cb(performance.now()));
  };

  it('returns a throttled function', () => {
    const mockFn = vi.fn();
    const { result } = renderHook(() => useRafThrottle(mockFn));
    
    expect(typeof result.current).toBe('function');
  });

  it('executes function immediately on first call', () => {
    const mockFn = vi.fn();
    const { result } = renderHook(() => useRafThrottle(mockFn));
    
    act(() => {
      result.current('test');
    });
    
    flushRaf();
    
    expect(mockFn).toHaveBeenCalledWith('test');
  });

  it('throttles calls within frame interval', () => {
    const mockFn = vi.fn();
    const { result } = renderHook(() => useRafThrottle(mockFn, 60));
    
    // First call
    act(() => {
      result.current('first');
    });
    flushRaf();
    expect(mockFn).toHaveBeenCalledTimes(1);
    
    // Reset mock
    mockFn.mockClear();
    
    // Second call immediately after - should be throttled
    act(() => {
      result.current('second');
    });
    flushRaf();
    
    // Should not have been called again due to throttling
    expect(mockFn).not.toHaveBeenCalled();
  });

  it('allows new calls after frame interval', () => {
    const mockFn = vi.fn();
    const { result } = renderHook(() => useRafThrottle(mockFn, 60));
    
    // First call
    act(() => {
      result.current('first');
    });
    flushRaf();
    expect(mockFn).toHaveBeenCalledTimes(1);
    
    // Reset mock
    mockFn.mockClear();
    
    // Second call after interval
    act(() => {
      result.current('second');
    });
    flushRaf();
    
    expect(mockFn).toHaveBeenCalledWith('second');
  });

  it('cancels pending animation frame on unmount', () => {
    const mockFn = vi.fn();
    const { result, unmount } = renderHook(() => useRafThrottle(mockFn));
    
    // Queue a call
    act(() => {
      result.current('test');
    });
    
    // Unmount before RAF executes
    unmount();
    
    flushRaf();
    
    expect(mockFn).not.toHaveBeenCalled();
  });
});

describe('useRafDebounce', () => {
  let rafCallbacks: Map<number, FrameRequestCallback>;
  let rafId: number;
  let useRafDebounce: typeof import('./useRafThrottle').useRafDebounce;

  beforeEach(async () => {
    rafCallbacks = new Map();
    rafId = 0;

    vi.stubGlobal('requestAnimationFrame', (callback: FrameRequestCallback) => {
      rafId++;
      rafCallbacks.set(rafId, callback);
      return rafId;
    });

    vi.stubGlobal('cancelAnimationFrame', (id: number) => {
      rafCallbacks.delete(id);
    });

    // Dynamic import to get module with mocks applied
    const module = await import('./useRafThrottle');
    useRafDebounce = module.useRafDebounce;
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  const flushRaf = () => {
    const callbacks = Array.from(rafCallbacks.entries());
    rafCallbacks.clear();
    callbacks.forEach(([, cb]) => cb(performance.now()));
  };

  it('returns a debounced function', () => {
    const mockFn = vi.fn();
    const { result } = renderHook(() => useRafDebounce(mockFn));
    
    expect(typeof result.current).toBe('function');
  });

  it('executes function on animation frame', () => {
    const mockFn = vi.fn();
    const { result } = renderHook(() => useRafDebounce(mockFn));
    
    act(() => {
      result.current('test');
    });
    
    flushRaf();
    
    expect(mockFn).toHaveBeenCalledWith('test');
  });

  it('cancels previous RAF on new call', () => {
    const mockFn = vi.fn();
    const { result } = renderHook(() => useRafDebounce(mockFn));
    
    // First call
    act(() => {
      result.current('first');
    });
    
    // Second call before first executes - cancels first
    act(() => {
      result.current('second');
    });
    
    // Third call before second executes - cancels second
    act(() => {
      result.current('third');
    });
    
    // Execute RAF
    flushRaf();
    
    // Should only execute last call
    expect(mockFn).toHaveBeenCalledTimes(1);
    expect(mockFn).toHaveBeenCalledWith('third');
  });
});
