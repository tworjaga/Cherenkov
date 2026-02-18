import * as React from 'react';
import { cn } from '@/lib/utils';

export interface SwitchProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, 'size' | 'onChange'> {
  label?: string;
  switchSize?: 'sm' | 'md' | 'lg';
  checked?: boolean;
  onCheckedChange?: (checked: boolean) => void;
}


export const Switch = React.forwardRef<HTMLInputElement, SwitchProps>(
  ({ className, label, switchSize: size = 'md', checked, onCheckedChange, ...props }, ref) => {
    
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
      onCheckedChange?.(e.target.checked);
    };




    const sizeClasses = {
      sm: { track: 'w-8 h-4', thumb: 'w-3 h-3', translate: 'translate-x-4' },
      md: { track: 'w-11 h-6', thumb: 'w-5 h-5', translate: 'translate-x-5' },
      lg: { track: 'w-14 h-7', thumb: 'w-6 h-6', translate: 'translate-x-7' },
    };

    const classes = sizeClasses[size];

    return (
      <label className="flex items-center gap-3 cursor-pointer">
        <div className="relative">
          <input
            type="checkbox"
            role="switch"
            aria-checked={checked ?? false}
            data-state={checked ? 'checked' : 'unchecked'}
            className={cn("peer sr-only", className)}
            ref={ref}
            checked={checked}
            onChange={handleChange}
            {...props}
          />


          <div
            className={cn(
              'rounded-full bg-[#2a2a3d] transition-colors duration-200',
              'peer-checked:bg-[#00d4ff]',
              'peer-focus:ring-2 peer-focus:ring-[#00d4ff] peer-focus:ring-offset-0',
              classes.track
            )}
          />

          <div
            className={cn(
              'absolute top-1/2 left-0.5 -translate-y-1/2 rounded-full bg-white transition-transform duration-200',
              classes.thumb,
              classes.translate
            )}
          />
        </div>
        {label && (
          <span className="text-sm text-[#a0a0b0] select-none">{label}</span>
        )}
      </label>
    );
  }
);

Switch.displayName = 'Switch';
