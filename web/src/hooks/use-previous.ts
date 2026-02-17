/**
 * Previous value hook
 * Stores the previous value of a state variable
 */

import { useRef, useEffect } from 'react';

export function usePrevious<T>(value: T): T | undefined {
  const ref = useRef<T | undefined>(undefined);

  useEffect(() => {
    ref.current = value;
  }, [value]);

  return ref.current;
}

export function usePreviousDistinct<T>(value: T, isEqual: (a: T, b: T) => boolean = (a, b) => a === b): T | undefined {
  const ref = useRef<T | undefined>(undefined);
  const prevValue = ref.current;

  useEffect(() => {
    if (prevValue === undefined || !isEqual(prevValue, value)) {
      ref.current = value;
    }
  }, [value, isEqual, prevValue]);

  return prevValue;
}
