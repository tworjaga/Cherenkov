import * as React from 'react';
import { Check } from 'lucide-react';
import { cn } from '@/lib/utils';

export interface CheckboxProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: boolean;
}

export const Checkbox = React.forwardRef<HTMLInputElement, CheckboxProps>(
  ({ className, label, error, ...props }, ref) => {
    return (
      <label className="flex items-center gap-2 cursor-pointer">
        <div className="relative">
          <input
            type="checkbox"
            className="peer sr-only"
            ref={ref}
            {...props}
          />
          <div
            className={cn(
              'w-4 h-4 rounded border border-[#2a2a3d] bg-[#0a0a10] transition-colors',
              'peer-checked:bg-[#00d4ff] peer-checked:border-[#00d4ff]',
              'peer-focus:ring-2 peer-focus:ring-[#00d4ff] peer-focus:ring-offset-0',
              error && 'border-[#ff3366]',
              className
            )}
          >
            <Check className="w-3 h-3 text-[#050508] opacity-0 peer-checked:opacity-100 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2" />
          </div>
        </div>
        {label && (
          <span className="text-sm text-[#a0a0b0] select-none">{label}</span>
        )}
      </label>
    );
  }
);

Checkbox.displayName = 'Checkbox';
