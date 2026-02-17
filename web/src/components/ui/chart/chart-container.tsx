'use client';

import * as React from 'react';
import { cn } from '@/lib/utils';

interface ChartContainerProps {
  children: React.ReactNode;
  className?: string;
  title?: string;
  description?: string;
}

export const ChartContainer = ({
  children,
  className,
  title,
  description,
}: ChartContainerProps) => {
  return (
    <div className={cn('bg-bg-secondary border border-border-subtle rounded-lg p-4', className)}>
      {title && (
        <div className="mb-4">
          <h3 className="text-heading-xs text-text-primary">{title}</h3>
          {description && (
            <p className="text-body-xs text-text-secondary mt-1">{description}</p>
          )}
        </div>
      )}
      <div className="h-full">{children}</div>
    </div>
  );
};

export default ChartContainer;
