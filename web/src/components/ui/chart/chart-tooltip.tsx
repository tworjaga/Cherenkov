'use client';

import * as React from 'react';
import { cn } from '@/lib/utils';

interface ChartTooltipProps {
  active?: boolean;
  payload?: Array<{
    name: string;
    value: number;
    color: string;
  }>;
  label?: string;
  className?: string;
}

export const ChartTooltip = ({
  active,
  payload,
  label,
  className,
}: ChartTooltipProps) => {
  if (!active || !payload || payload.length === 0) {
    return null;
  }

  return (
    <div
      className={cn(
        'bg-bg-tertiary border border-border-default rounded-lg p-3 shadow-lg',
        className
      )}
    >
      {label && (
        <p className="text-body-xs text-text-secondary mb-2 font-mono">{label}</p>
      )}
      <div className="space-y-1">
        {payload.map((entry, index) => (
          <div key={index} className="flex items-center gap-2">
            <div
              className="w-2 h-2 rounded-full"
              style={{ backgroundColor: entry.color }}
            />
            <span className="text-body-xs text-text-secondary">{entry.name}:</span>
            <span className="text-body-sm text-text-primary font-mono">
              {entry.value}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
};

export default ChartTooltip;
