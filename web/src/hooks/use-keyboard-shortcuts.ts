import { useEffect, useCallback } from 'react';
import { useAppStore } from '@/stores';
import { useGlobeStore } from '@/stores';

export const useKeyboardShortcuts = () => {
  const { setView, toggleRightPanel, toggleBottomPanel, setTimeMode, stepTime, selectSensor } =
    useAppStore();
  const { resetView } = useGlobeStore();

  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      // Ignore if typing in input
      if (
        event.target instanceof HTMLInputElement ||
        event.target instanceof HTMLTextAreaElement
      ) {
        return;
      }

      switch (event.key) {
        case '1':
          setView('dashboard');
          break;
        case '2':
          setView('globe');
          break;
        case '3':
          setView('sensors');
          break;
        case '4':
          setView('anomalies');
          break;
        case '5':
          setView('plume');
          break;
        case 'g':
        case 'G':
          // Center globe on selected sensor - handled by globe component
          break;
        case 't':
        case 'T':
          toggleRightPanel();
          break;
        case ' ':
          event.preventDefault();
          setTimeMode('live');
          break;
        case 'ArrowLeft':
          stepTime('backward');
          break;
        case 'ArrowRight':
          stepTime('forward');
          break;
        case 'f':
        case 'F':
          if (document.fullscreenElement) {
            document.exitFullscreen();
          } else {
            document.documentElement.requestFullscreen();
          }
          break;
        case 'Escape':
          selectSensor(null);
          break;
        case '?':
          // Show keyboard shortcuts modal
          break;
        default:
          break;
      }
    },
    [setView, toggleRightPanel, toggleBottomPanel, setTimeMode, stepTime, selectSensor]
  );

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);
};
