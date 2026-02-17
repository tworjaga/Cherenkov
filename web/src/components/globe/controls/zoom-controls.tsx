'use client';

import { Plus, Minus, Maximize } from 'lucide-react';
import { useGlobeStore } from '@/stores';
import { Button } from '@/components/ui/button';

interface ZoomControlsProps {
  className?: string;
}

export function ZoomControls({ className }: ZoomControlsProps) {
  const { viewport, setViewport } = useGlobeStore();

  const handleZoomIn = () => {
    setViewport({ zoom: Math.min(viewport.zoom + 1, 20) });
  };

  const handleZoomOut = () => {
    setViewport({ zoom: Math.max(viewport.zoom - 1, 1) });
  };

  const handleReset = () => {
    setViewport({
      latitude: 20,
      longitude: 0,
      zoom: 2,
      pitch: 0,
      bearing: 0,
    });
  };

  return (
    <div className={`flex flex-col gap-1 ${className}`}>
      <Button
        variant="outline"
        size="icon"
        onClick={handleZoomIn}
        className="bg-bg-secondary/90 backdrop-blur-sm border-border-subtle hover:bg-bg-hover"
      >
        <Plus className="w-4 h-4" />
      </Button>
      <Button
        variant="outline"
        size="icon"
        onClick={handleZoomOut}
        className="bg-bg-secondary/90 backdrop-blur-sm border-border-subtle hover:bg-bg-hover"
      >
        <Minus className="w-4 h-4" />
      </Button>
      <Button
        variant="outline"
        size="icon"
        onClick={handleReset}
        className="bg-bg-secondary/90 backdrop-blur-sm border-border-subtle hover:bg-bg-hover"
      >
        <Maximize className="w-4 h-4" />
      </Button>
    </div>
  );
}

export default ZoomControls;
