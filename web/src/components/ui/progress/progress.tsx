import * as React from 'react';
import { cn } from '@/lib/utils';

export interface ProgressProps extends React.HTMLAttributes<HTMLDivElement> {
  value: number;
  max?: number;
  size?: 'sm' | 'md' | 'lg';
  variant?: 'default' | 'accent' | 'success' | 'warning' | 'error';
  indeterminate?: boolean;
}

export const Progress = React.forwardRef<HTMLDivElement, ProgressProps>(
  ({ className, value, max = 100, size = 'md', variant = 'default', indeterminate = false, ...props }, ref) => {
    const percentage = Math.min(100, Math.max(0, (value / max) * 100));

    const sizeClasses = {
      sm: 'h-1',
      md: 'h-2',
      lg: 'h-3',
    };

    const variantClasses = {
      default: 'bg-[#00d4ff]',
      accent: 'bg-[#00d4ff]',
      success: 'bg-[#00ff88]',
      warning: 'bg-[#ffb800]',
      error: 'bg-[#ff3366]',
    };

    return (
      <div
        ref={ref}
        role="progressbar"
        aria-valuenow={indeterminate ? undefined : value}
        aria-valuemin={0}
        aria-valuemax={max}
        aria-valuetext={indeterminate ? undefined : `${Math.round(percentage)}%`}
        data-state={indeterminate ? 'indeterminate' : value === max ? 'complete' : 'loading'}
        className={cn(
          'w-full rounded-full bg-[#1f1f2e] overflow-hidden',
          sizeClasses[size],
          className
        )}
        {...props}
      >
        <div
          className={cn(
            'h-full rounded-full transition-all duration-300 ease-out',
            variantClasses[variant],
            indeterminate && 'animate-progress-indeterminate w-1/3'
          )}
          style={!indeterminate ? { width: `${percentage}%` } : undefined}
        />
      </div>
    );
  }
);

Progress.displayName = 'Progress';
