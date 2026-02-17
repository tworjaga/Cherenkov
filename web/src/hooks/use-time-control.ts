'use client';

import { useCallback } from 'react';
import { useAppStore } from '@/stores/app-store';

export const useTimeControl = () => {
  const {
    timeMode,
    currentTime,
    playbackSpeed,
    setTimeMode,
    setCurrentTime,
    setPlaybackSpeed,
  } = useAppStore();

  const play = useCallback(() => {
    setTimeMode('live');
  }, [setTimeMode]);

  const pause = useCallback(() => {
    setTimeMode('paused');
  }, [setTimeMode]);

  const togglePlayPause = useCallback(() => {
    setTimeMode(timeMode === 'live' ? 'paused' : 'live');
  }, [timeMode, setTimeMode]);

  const stepForward = useCallback(() => {
    const newTime = currentTime + 5 * 60 * 1000;
    setCurrentTime(newTime);
  }, [currentTime, setCurrentTime]);

  const stepBackward = useCallback(() => {
    const newTime = currentTime - 5 * 60 * 1000;
    setCurrentTime(newTime);
  }, [currentTime, setCurrentTime]);

  const setSpeed = useCallback((speed: number) => {
    setPlaybackSpeed(speed);
  }, [setPlaybackSpeed]);

  return {
    timeMode,
    currentTime,
    playbackSpeed,
    play,
    pause,
    togglePlayPause,
    stepForward,
    stepBackward,
    setSpeed,
  };
};
