import React, { useCallback, useState, useRef, useEffect } from 'react';
import { useAppStore } from '../../stores/useAppStore';
import { format } from 'date-fns';

export const TimeSlider: React.FC = () => {
  const timeControl = useAppStore((state) => state.globe.time);
  const setTimeControl = useAppStore((state) => state.setTimeControl);
  const [isDragging, setIsDragging] = useState(false);
  const trackRef = useRef<HTMLDivElement>(null);

  const { mode, currentTime, windowStart, windowEnd, playbackSpeed } = timeControl;

  const isLive = mode === 'LIVE';
  const isPaused = mode === 'PAUSED';
  const isReplay = mode === 'REPLAY';

  // Calculate progress percentage
  const totalWindow = windowEnd.getTime() - windowStart.getTime();
  const currentProgress = currentTime.getTime() - windowStart.getTime();
  const progressPercent = totalWindow > 0 
    ? (currentProgress / totalWindow) * 100 
    : 0;

  // Format times
  const formattedCurrent = format(currentTime, 'HH:mm:ss');
  const formattedDate = format(currentTime, 'MMM dd, yyyy');

  // Handle play/pause toggle
  const togglePlayPause = useCallback(() => {
    if (isLive) {
      setTimeControl({ mode: 'PAUSED' });
    } else {
      setTimeControl({ mode: 'LIVE' });
    }
  }, [isLive, setTimeControl]);

  // Handle speed change
  const setSpeed = useCallback((speed: typeof playbackSpeed) => {
    setTimeControl({ playbackSpeed: speed });
  }, [setTimeControl]);

  // Handle track click/drag
  const handleTrackClick = useCallback((e: React.MouseEvent<HTMLDivElement>) => {
    if (!trackRef.current) return;
    
    const rect = trackRef.current.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const percent = Math.max(0, Math.min(1, x / rect.width));
    
    const newTime = new Date(windowStart.getTime() + percent * totalWindow);
    setTimeControl({ 
      mode: 'PAUSED',
      currentTime: newTime 
    });
  }, [windowStart, totalWindow, setTimeControl]);

  // Drag handling
  useEffect(() => {
    if (!isDragging) return;

    const handleMouseMove = (e: MouseEvent) => {
      if (!trackRef.current) return;
      
      const rect = trackRef.current.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const percent = Math.max(0, Math.min(1, x / rect.width));
      
      const newTime = new Date(windowStart.getTime() + percent * totalWindow);
      setTimeControl({ currentTime: newTime });
    };

    const handleMouseUp = () => {
      setIsDragging(false);
    };

    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);
    
    return () => {
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isDragging, windowStart, totalWindow, setTimeControl]);

  // Auto-advance in live mode
  useEffect(() => {
    if (!isLive) return;

    const interval = setInterval(() => {
      setTimeControl({ currentTime: new Date() });
    }, 1000);

    return () => clearInterval(interval);
  }, [isLive, setTimeControl]);

  return (
    <div className="absolute bottom-20 left-4 right-[340px] z-20 bg-bg-secondary/90 backdrop-blur border border-border-subtle rounded-lg p-3">
      <div className="flex items-center gap-4">
        {/* Play/Pause button */}
        <button
          onClick={togglePlayPause}
          className="flex items-center justify-center w-10 h-10 rounded-lg bg-accent-primary/20 border border-accent-primary/50 text-accent-primary hover:bg-accent-primary/30 transition-colors"
          aria-label={isLive ? 'Pause' : 'Play'}
          title={isLive ? 'Pause' : 'Play'}
        >
          {isLive ? (
            <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
              <rect x="6" y="4" width="4" height="16" />
              <rect x="14" y="4" width="4" height="16" />
            </svg>
          ) : (
            <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
              <path d="M8 5v14l11-7z" />
            </svg>
          )}
        </button>

        {/* Speed selector (only in replay mode) */}
        {isReplay && (
          <div className="flex items-center gap-1">
            {[1, 2, 5, 10, 50].map((speed) => (
              <button
                key={speed}
                onClick={() => setSpeed(speed as typeof playbackSpeed)}
                className={`px-2 py-1 text-xs font-medium rounded ${
                  playbackSpeed === speed
                    ? 'bg-accent-primary text-bg-primary'
                    : 'text-text-secondary hover:text-text-primary'
                }`}
                aria-pressed={playbackSpeed === speed ? 'true' : 'false'}
              >
                {speed}x
              </button>
            ))}
          </div>
        )}

        {/* Time display */}
        <div className="flex flex-col min-w-[120px]">
          <div className="flex items-center gap-2">
            {isLive && (
              <span className="w-2 h-2 rounded-full bg-alert-normal animate-pulse" />
            )}
            <span className={`text-sm font-mono font-semibold ${
              isLive ? 'text-alert-normal' : 'text-text-primary'
            }`}>
              {isLive ? 'LIVE' : formattedCurrent}
            </span>
          </div>
          <span className="text-xs text-text-tertiary">{formattedDate}</span>
        </div>

        {/* Timeline track */}
        <div 
          ref={trackRef}
          className="flex-1 h-8 relative cursor-pointer group"
          onClick={handleTrackClick}
          role="slider"
          aria-valuemin={0}
          aria-valuemax={100}
          aria-valuenow={Math.round(progressPercent)}
          aria-label="Time slider"
        >
          {/* Track background */}
          <div className="absolute inset-y-0 left-0 right-0 flex items-center">
            <div className="w-full h-2 bg-bg-tertiary rounded-full overflow-hidden">
              {/* Past (dimmed) */}
              <div 
                className="h-full bg-accent-primary/30"
                style={{ width: `${progressPercent}%` }}
              />
              {/* Future (transparent) */}
              <div 
                className="h-full bg-transparent"
                style={{ width: `${100 - progressPercent}%` }}
              />
            </div>
          </div>

          {/* Playhead */}
          <div 
            className="absolute top-1/2 -translate-y-1/2 w-4 h-6 bg-accent-primary rounded shadow-glow cursor-grab active:cursor-grabbing transition-transform group-hover:scale-110"
            style={{ left: `calc(${progressPercent}% - 8px)` }}
            onMouseDown={(e) => {
              e.stopPropagation();
              setIsDragging(true);
            }}
          >
            <div className="absolute -top-1 left-1/2 -translate-x-1/2 w-0.5 h-1 bg-accent-primary" />
            <div className="absolute -bottom-1 left-1/2 -translate-x-1/2 w-0.5 h-1 bg-accent-primary" />
          </div>

          {/* Window handles */}
          <div className="absolute inset-y-0 left-0 w-1 bg-border-subtle rounded-l" />
          <div className="absolute inset-y-0 right-0 w-1 bg-border-subtle rounded-r" />
        </div>

        {/* Mode indicator */}
        <div className="flex items-center gap-2 px-3 py-1.5 bg-bg-tertiary rounded-lg">
          <span className="text-xs text-text-secondary uppercase tracking-wider">
            {mode}
          </span>
        </div>
      </div>
    </div>
  );
};
