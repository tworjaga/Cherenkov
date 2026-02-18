import * as React from 'react';
import { cn } from '@/lib/utils';

export interface TextareaProps extends React.TextareaHTMLAttributes<HTMLTextAreaElement> {
  error?: boolean;
}

export const Textarea = React.forwardRef<HTMLTextAreaElement, TextareaProps>(
  ({ className, error, ...props }, ref) => {
    return (
      <textarea
        className={cn(
          'flex min-h-[80px] w-full rounded-md border border-[#2a2a3d] bg-[#0a0a10] px-3 py-2 text-sm text-white placeholder:text-[#606070] focus:outline-none focus:ring-2 focus:ring-[#00d4ff] focus:ring-offset-0 disabled:cursor-not-allowed disabled:opacity-50',
          error && 'border-alert-critical focus:ring-alert-critical',

          className
        )}
        ref={ref}
        {...props}
      />
    );
  }
);

Textarea.displayName = 'Textarea';
