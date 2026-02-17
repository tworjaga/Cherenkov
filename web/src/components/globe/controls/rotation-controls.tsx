'use client';

import React from 'react';
import { Button } from '@/components/ui/button';
import { useGlobe } from '@/hooks/use-globe';

interface RotationControlsProps {
  className?: string;
}

export function RotationControls({ className }: RotationControlsProps) {
  const { isRotating, toggleRotation, resetView } = useGlobe();

  return (
    <div className={`flex items-center gap-2 ${className}`}>
      <Button
        variant="secondary"
        size="sm"
        onClick={toggleRotation}
        aria-label={isRotating ? 'Pause rotation' : 'Start rotation'}
      >
        {isRotating ? 'Pause' : 'Rotate'}
      </Button>
      <Button
        variant="outline"
        size="sm"
        onClick={resetView}
        aria-label="Reset view"
      >
        Reset
      </Button>
    </div>
  );
}
