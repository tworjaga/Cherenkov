import * as React from 'react';
import { cn } from '@/lib/utils';

export interface SpinnerProps extends React.HTMLAttributes<HTMLDivElement> {
  size?: 'sm' | 'md' | 'lg' | 'xl';
  variant?: 'default' | 'accent' | 'white';
  label?: string;
}

export const Spinner = React.forwardRef<HTMLDivElement, SpinnerProps>(
  ({ className, size = 'md', variant = 'default', label = 'Loading', ...props }, ref) => {
    const sizeClasses = {
      sm: 'h-4 w-4 border-2',
      md: 'h-6 w-6 border-2',
      lg: 'h-8 w-8 border-[3px]',
      xl: 'h-12 w-12 border-4',
    };

    const variantClasses = {
      default: 'border-[#00d4ff] border-t-transparent',
      accent: 'border-[#00d4ff] border-t-transparent',
      white: 'border-white border-t-transparent',
    };

    return (
      <div
        ref={ref}
        role="status"
        aria-label={label}
        aria-live="polite"
        data-state="loading"
        className={cn(
          'animate-spin rounded-full',
          sizeClasses[size],
          variantClasses[variant],
          className
        )}
        {...props}
      >
        <span className="sr-only">{label}</span>
      </div>
    );
  }
);

Spinner.displayName = 'Spinner';
