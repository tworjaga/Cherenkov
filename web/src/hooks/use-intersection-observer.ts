/**
 * Intersection Observer hook
 * Tracks element visibility in viewport
 */

import { useState, useEffect, useRef, RefObject } from 'react';

interface UseIntersectionObserverOptions {
  threshold?: number | number[];
  root?: Element | null;
  rootMargin?: string;
  triggerOnce?: boolean;
}

export function useIntersectionObserver<T extends Element = Element>(
  options: UseIntersectionObserverOptions = {}
): [RefObject<T | null>, boolean] {
  const threshold = options.threshold ?? 0;
  const root = options.root ?? null;
  const rootMargin = options.rootMargin ?? '0px';
  const triggerOnce = options.triggerOnce ?? false;
  
  const ref = useRef<T | null>(null);
  const [isIntersecting, setIsIntersecting] = useState(false);

  useEffect(() => {
    const element = ref.current;
    if (!element) return;

    const observer = new IntersectionObserver(
      ([entry]) => {
        setIsIntersecting(entry.isIntersecting);
        if (triggerOnce && entry.isIntersecting) {
          observer.unobserve(element);
        }
      },
      { threshold, root, rootMargin }
    );

    observer.observe(element);

    return () => {
      observer.disconnect();
    };
  }, [threshold, root, rootMargin, triggerOnce]);

  return [ref, isIntersecting];
}

export function useInView<T extends Element = Element>(
  options: UseIntersectionObserverOptions = {}
): [RefObject<T | null>, boolean] {
  return useIntersectionObserver<T>(options);
}
