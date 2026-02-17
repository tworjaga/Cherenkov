'use client';

import { Button } from '@/components/ui/button';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Slider } from '@/components/ui/slider';
import { Play, Pause, SkipBack, SkipForward } from 'lucide-react';

interface ChartControlsProps {
  isPlaying: boolean;
  onPlayPause: () => void;
  onReset: () => void;
  timeRange: number;
  onTimeRangeChange: (value: number) => void;
  speed: number;
  onSpeedChange: (value: number) => void;
}

export function ChartControls({
  isPlaying,
  onPlayPause,
  onReset,
  timeRange,
  onTimeRangeChange,
  speed,
  onSpeedChange,
}: ChartControlsProps) {
  return (
    <div className="flex items-center gap-4 p-3 border-t bg-muted/50">
      <div className="flex items-center gap-2">
        <Button
          variant="outline"
          size="icon"
          className="h-8 w-8"
          onClick={onReset}
        >
          <SkipBack className="h-4 w-4" />
        </Button>
        <Button
          variant="default"
          size="icon"
          className="h-8 w-8"
          onClick={onPlayPause}
        >
          {isPlaying ? <Pause className="h-4 w-4" /> : <Play className="h-4 w-4" />}
        </Button>
      </div>

      <div className="flex-1 px-4">
        <div className="flex items-center justify-between mb-1">
          <span className="text-xs text-muted-foreground">Time Range</span>
          <span className="text-xs font-medium">{timeRange}h</span>
        </div>
        <Slider
          value={[timeRange]}
          onValueChange={([value]) => onTimeRangeChange(value)}
          min={1}
          max={168}
          step={1}
          className="w-full"
        />
      </div>

      <div className="w-32">
        <Select value={speed.toString()} onValueChange={(v) => onSpeedChange(Number(v))}>
          <SelectTrigger className="h-8">
            <SelectValue placeholder="Speed" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="0.5">0.5x</SelectItem>
            <SelectItem value="1">1x</SelectItem>
            <SelectItem value="2">2x</SelectItem>
            <SelectItem value="5">5x</SelectItem>
            <SelectItem value="10">10x</SelectItem>
          </SelectContent>
        </Select>
      </div>
    </div>
  );
}
