import * as React from 'react';
import { cn } from '@/lib/utils';

export interface InputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {
  error?: string;
}

const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ className, type, error, ...props }, ref) => {
    return (
      <div className="w-full">
        <input
          type={type}
          className={cn(
            'flex h-9 w-full rounded-md border border-border-subtle bg-bg-primary px-3 py-1 text-sm text-text-primary shadow-sm transition-colors',
            'file:border-0 file:bg-transparent file:text-sm file:font-medium',
            'placeholder:text-text-tertiary',
            'focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-accent-primary',
            'disabled:cursor-not-allowed disabled:opacity-50',
            error && 'border-alert-critical focus-visible:ring-alert-critical',
            className
          )}
          ref={ref}
          {...props}
        />
        {error && (
          <p className="mt-1 text-body-xs text-alert-critical">{error}</p>
        )}
      </div>
    );
  }
);
Input.displayName = 'Input';

export { Input };
