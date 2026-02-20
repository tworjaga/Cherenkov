'use client';

import { useState, useCallback, useRef, useEffect } from 'react';

export interface PlumeSimulationState {
  isPlaying: boolean;
  currentTime: number;
  duration: number;
  progress: number;
  speed: number;
  isLooping: boolean;
}

export interface PlumeSimulationControls {
  play: () => void;
  pause: () => void;
  reset: () => void;
  seekToTime: (time: number) => void;
  seekToProgress: (progress: number) => void;
  setSpeed: (speed: number) => void;
  toggleLoop: () => void;
  setDuration: (duration: number) => void;
}

export function usePlumeSimulation(
  initialDuration: number = 3600000
): [PlumeSimulationState, PlumeSimulationControls] {
  const [state, setState] = useState<PlumeSimulationState>({
    isPlaying: false,
    currentTime: 0,
    duration: initialDuration,
    progress: 0,
    speed: 1,
    isLooping: false,
  });

  const animationRef = useRef<number | null>(null);
  const lastTimeRef = useRef<number>(0);

  const play = useCallback(() => {
    setState((prev) => ({ ...prev, isPlaying: true }));
    lastTimeRef.current = performance.now();
  }, []);

  const pause = useCallback(() => {
    setState((prev) => ({ ...prev, isPlaying: false }));
    if (animationRef.current) {
      cancelAnimationFrame(animationRef.current);
    }
  }, []);

  const reset = useCallback(() => {
    setState((prev) => ({
      ...prev,
      currentTime: 0,
      progress: 0,
      isPlaying: false,
    }));
    if (animationRef.current) {
      cancelAnimationFrame(animationRef.current);
    }
  }, []);

  const seekToTime = useCallback((time: number) => {
    setState((prev) => {
      const clampedTime = Math.max(0, Math.min(time, prev.duration));
      return {
        ...prev,
        currentTime: clampedTime,
        progress: (clampedTime / prev.duration) * 100,
      };
    });
  }, []);

  const seekToProgress = useCallback((progress: number) => {
    setState((prev) => {
      const clampedProgress = Math.max(0, Math.min(progress, 100));
      return {
        ...prev,
        progress: clampedProgress,
        currentTime: (clampedProgress / 100) * prev.duration,
      };
    });
  }, []);

  const setSpeed = useCallback((speed: number) => {
    setState((prev) => ({ ...prev, speed: Math.max(0.1, Math.min(speed, 10)) }));
  }, []);

  const toggleLoop = useCallback(() => {
    setState((prev) => ({ ...prev, isLooping: !prev.isLooping }));
  }, []);

  const setDuration = useCallback((duration: number) => {
    setState((prev) => ({
      ...prev,
      duration: Math.max(60000, duration),
      currentTime: Math.min(prev.currentTime, duration),
      progress: Math.min(prev.progress, 100),
    }));
  }, []);

  useEffect(() => {
    if (!state.isPlaying) {
      return;
    }

    const animate = (currentTime: number) => {
      const deltaTime = currentTime - lastTimeRef.current;
      lastTimeRef.current = currentTime;

      setState((prev) => {
        const timeIncrement = deltaTime * prev.speed;
        let newTime = prev.currentTime + timeIncrement;
        let newProgress = (newTime / prev.duration) * 100;

        if (newTime >= prev.duration) {
          if (prev.isLooping) {
            newTime = 0;
            newProgress = 0;
          } else {
            newTime = prev.duration;
            newProgress = 100;
            return {
              ...prev,
              currentTime: newTime,
              progress: newProgress,
              isPlaying: false,
            };
          }
        }

        return {
          ...prev,
          currentTime: newTime,
          progress: newProgress,
        };
      });

      if (state.isPlaying) {
        animationRef.current = requestAnimationFrame(animate);
      }
    };

    animationRef.current = requestAnimationFrame(animate);

    return () => {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
    };
  }, [state.isPlaying, state.speed, state.isLooping, state.duration]);

  return [
    state,
    {
      play,
      pause,
      reset,
      seekToTime,
      seekToProgress,
      setSpeed,
      toggleLoop,
      setDuration,
    },
  ];
}

export default usePlumeSimulation;
