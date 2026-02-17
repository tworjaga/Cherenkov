'use client';

import * as React from 'react';
import { cn } from '@/lib/utils';

export interface ToggleProps {
  pressed?: boolean;
  onPressedChange?: (pressed: boolean) => void;
  disabled?: boolean;
  className?: string;
  children?: React.ReactNode;
}

export const Toggle = React.forwardRef<HTMLButtonElement, ToggleProps>(
  ({ pressed, onPressedChange, disabled, className, children, ...props }, ref) => {
    const [internalPressed, setInternalPressed] = React.useState(false);
    
    const isPressed = pressed !== undefined ? pressed : internalPressed;
    
    const handleClick = () => {
      if (disabled) return;
      
      const newPressed = !isPressed;
      if (pressed === undefined) {
        setInternalPressed(newPressed);
      }
      onPressedChange?.(newPressed);
    };

    return (
      <button
        ref={ref}
        type="button"
        aria-pressed={isPressed}
        data-state={isPressed ? 'on' : 'off'}
        disabled={disabled}
        onClick={handleClick}
        className={cn(
          'inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors',
          'hover:bg-[#1a1a25] hover:text-white',
          'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[#00d4ff]',
          'disabled:pointer-events-none disabled:opacity-50',
          'data-[state=on]:bg-[#00d4ff]/20 data-[state=on]:text-[#00d4ff]',
          'h-9 px-3',
          className
        )}
        {...props}
      >
        {children}
      </button>
    );
  }
);

Toggle.displayName = 'Toggle';
