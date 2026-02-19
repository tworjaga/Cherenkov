import * as React from 'react';
import { Check } from 'lucide-react';
import { cn } from '@/lib/utils';

export interface CheckboxProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: boolean;
  onCheckedChange?: (checked: boolean) => void;
  children?: React.ReactNode;
}


export const Checkbox = React.forwardRef<HTMLInputElement, CheckboxProps>(
  ({ className, label, error, onCheckedChange, onChange, checked, children, ...props }, ref) => {
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
      onCheckedChange?.(e.target.checked);
      onChange?.(e);
    };

    return (
      <label className={cn("flex items-center gap-2 cursor-pointer", className)}>
        <div className="relative">
          <input
            type="checkbox"
            role="checkbox"
            aria-checked={checked ?? false}
            className="peer sr-only"
            ref={ref}
            checked={checked}
            onChange={handleChange}
            {...props}
          />


          <div
            className={cn(
              'w-4 h-4 rounded border border-[#2a2a3d] bg-[#0a0a10] transition-colors',
              'peer-checked:bg-[#00d4ff] peer-checked:border-[#00d4ff]',
              'peer-focus:ring-2 peer-focus:ring-[#00d4ff] peer-focus:ring-offset-0',
              error && 'border-[#ff3366]'
            )}
          >
            <Check className="w-3 h-3 text-[#050508] opacity-0 peer-checked:opacity-100 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2" />
          </div>
        </div>
        {(label || children) && (
          <span className="text-sm text-[#a0a0b0] select-none">{label || children}</span>
        )}
      </label>
    );
  }
);


Checkbox.displayName = 'Checkbox';
