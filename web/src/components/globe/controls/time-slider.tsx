'use client';

import { useState } from 'react';
import { Play, Pause, SkipBack, SkipForward } from 'lucide-react';
import { useAppStore } from '@/stores';
import { Button } from '@/components/ui/button';
import { Slider } from '@/components/ui/slider';
import { cn } from '@/lib/utils';

interface TimeSliderProps {
  className?: string;
}

export function TimeSlider({ className }: TimeSliderProps) {
  const { timeMode, setTimeMode, currentTime, setCurrentTime } = useAppStore();
  const [playbackSpeed, setPlaybackSpeed] = useState(1);

  const speeds = [0.5, 1, 2, 5, 10];

  const handlePlayPause = () => {
    setTimeMode(timeMode === 'live' ? 'paused' : 'live');
  };

  const handleStepBack = () => {
    const newTime = currentTime - 5 * 60 * 1000; // 5 minutes back
    setCurrentTime(newTime);
  };

  const handleStepForward = () => {
    const newTime = currentTime + 5 * 60 * 1000; // 5 minutes forward
    setCurrentTime(newTime);
  };

  const formatTime = (timestamp: number) => {
    return new Date(timestamp).toLocaleTimeString('en-US', {

      hour12: false,
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  };

  return (
    <div
      className={cn(
        'bg-bg-secondary/90 backdrop-blur-sm border border-border-subtle rounded-lg p-3 flex items-center gap-4',
        className
      )}
    >
      <Button
        variant="outline"
        size="icon"
        onClick={handlePlayPause}
        className="bg-bg-tertiary border-border-subtle hover:bg-bg-hover"
      >
        {timeMode === 'live' ? (
          <Pause className="w-4 h-4" />
        ) : (
          <Play className="w-4 h-4" />
        )}
      </Button>

      <div className="flex items-center gap-1">
        <Button
          variant="ghost"
          size="icon"
          onClick={handleStepBack}
          className="h-8 w-8 hover:bg-bg-hover"
        >
          <SkipBack className="w-4 h-4" />
        </Button>
        <Button
          variant="ghost"
          size="icon"
          onClick={handleStepForward}
          className="h-8 w-8 hover:bg-bg-hover"
        >
          <SkipForward className="w-4 h-4" />
        </Button>
      </div>

      <div className="flex-1 min-w-[200px]">
        <Slider defaultValue={[50]} max={100} step={1} />
      </div>

      <div className="flex items-center gap-2">
        <span className="text-sm font-mono text-text-primary">
          {formatTime(currentTime)}
        </span>
        {timeMode === 'live' && (
          <span className="flex items-center gap-1 text-xs text-alert-critical">
            <span className="w-2 h-2 rounded-full bg-alert-critical animate-pulse" />
            LIVE
          </span>
        )}
      </div>

      <div className="flex items-center gap-1 border-l border-border-subtle pl-3">
        {speeds.map((speed) => (
          <Button
            key={speed}
            variant="ghost"
            size="sm"
            onClick={() => setPlaybackSpeed(speed)}
            className={cn(
              'h-6 px-2 text-xs',
              playbackSpeed === speed
                ? 'bg-accent-primary/20 text-accent-primary'
                : 'text-text-tertiary hover:text-text-primary'
            )}
          >
            {speed}x
          </Button>
        ))}
      </div>
    </div>
  );
}

export default TimeSlider;
