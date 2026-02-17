import * as React from 'react';
import { cn } from '@/lib/utils';

export interface RadioProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
}

export const Radio = React.forwardRef<HTMLInputElement, RadioProps>(
  ({ className, label, ...props }, ref) => {
    return (
      <label className="flex items-center gap-2 cursor-pointer">
        <div className="relative">
          <input
            type="radio"
            className="peer sr-only"
            ref={ref}
            {...props}
          />
          <div
            className={cn(
              'w-4 h-4 rounded-full border border-[#2a2a3d] bg-[#0a0a10] transition-colors',
              'peer-checked:border-[#00d4ff] peer-checked:bg-[#00d4ff]',
              'peer-focus:ring-2 peer-focus:ring-[#00d4ff] peer-focus:ring-offset-0',
              className
            )}
          >
            <div className="w-1.5 h-1.5 rounded-full bg-[#050508] opacity-0 peer-checked:opacity-100 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2" />
          </div>
        </div>
        {label && (
          <span className="text-sm text-[#a0a0b0] select-none">{label}</span>
        )}
      </label>
    );
  }
);

Radio.displayName = 'Radio';
