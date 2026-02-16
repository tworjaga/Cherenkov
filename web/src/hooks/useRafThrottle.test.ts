import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { renderHook } from '@testing-library/react';
import { useRafThrottle, useRafDebounce } from './useRafThrottle';

describe('useRafThrottle', () => {
  let now = 0;

  beforeEach(() => {
    now = 0;
    vi.spyOn(performance, 'now').mockImplementation(() => now);
    vi.spyOn(window, 'requestAnimationFrame').mockImplementation((cb: FrameRequestCallback) => {
      // Execute callback synchronously for testing
      setTimeout(() => cb(now), 0);
      return Math.floor(Math.random() * 10000);
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('returns a throttled function', () => {
    const mockFn = vi.fn();
    const { result } = renderHook(() => useRafThrottle(mockFn));
    
    expect(typeof result.current).toBe('function');
  });

  it('executes function immediately on first call', async () => {
    const mockFn = vi.fn();
    const { result } = renderHook(() => useRafThrottle(mockFn, 60));
    
    result.current('test');
    
    // Wait for RAF to execute
    await new Promise(resolve => setTimeout(resolve, 10));
    
    expect(mockFn).toHaveBeenCalledWith('test');
  });

  it('throttles calls within frame interval', async () => {
    const mockFn = vi.fn();
    const { result } = renderHook(() => useRafThrottle(mockFn, 60));
    
    // First call
    result.current('first');
    await new Promise(resolve => setTimeout(resolve, 10));
    expect(mockFn).toHaveBeenCalledTimes(1);
    
    // Reset mock
    mockFn.mockClear();
    
    // Call again immediately (within 16ms) - should be throttled
    result.current('second');
    await new Promise(resolve => setTimeout(resolve, 10));
    
    // Should not execute because throttled
    expect(mockFn).toHaveBeenCalledTimes(0);
  });

  it('allows new calls after frame interval', async () => {
    const mockFn = vi.fn();
    const { result } = renderHook(() => useRafThrottle(mockFn, 60));
    
    // First call
    result.current('first');
    await new Promise(resolve => setTimeout(resolve, 10));
    expect(mockFn).toHaveBeenCalledTimes(1);
    
    // Advance time past frame interval (16.67ms)
    now = 20;
    
    // New call should execute
    result.current('second');
    await new Promise(resolve => setTimeout(resolve, 10));
    
    expect(mockFn).toHaveBeenCalledTimes(2);
    expect(mockFn).toHaveBeenLastCalledWith('second');
  });

  it('cancels pending animation frame on unmount', () => {
    const mockFn = vi.fn();
    const { result, unmount } = renderHook(() => useRafThrottle(mockFn));
    
    result.current('test');
    
    // Unmount should not throw
    expect(() => unmount()).not.toThrow();
  });
});

describe('useRafDebounce', () => {
  beforeEach(() => {
    vi.spyOn(window, 'requestAnimationFrame').mockImplementation((cb: FrameRequestCallback) => {
      setTimeout(() => cb(performance.now()), 0);
      return Math.floor(Math.random() * 10000);
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('returns a debounced function', () => {
    const mockFn = vi.fn();
    const { result } = renderHook(() => useRafDebounce(mockFn));
    
    expect(typeof result.current).toBe('function');
  });

  it('executes function on animation frame', async () => {
    const mockFn = vi.fn();
    const { result } = renderHook(() => useRafDebounce(mockFn));
    
    result.current('test');
    
    // Wait for RAF
    await new Promise(resolve => setTimeout(resolve, 10));
    
    expect(mockFn).toHaveBeenCalledWith('test');
  });

  it('cancels previous RAF on new call', async () => {
    const mockFn = vi.fn();
    const { result } = renderHook(() => useRafDebounce(mockFn));
    
    // Multiple rapid calls
    result.current('first');
    result.current('second');
    result.current('third');
    
    // Wait for RAF
    await new Promise(resolve => setTimeout(resolve, 10));
    
    // Should only execute last call
    expect(mockFn).toHaveBeenCalledTimes(1);
    expect(mockFn).toHaveBeenCalledWith('third');
  });
});
