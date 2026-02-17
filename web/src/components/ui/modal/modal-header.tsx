import * as React from 'react';
import { cn } from '@/lib/utils';

export interface ModalHeaderProps extends React.HTMLAttributes<HTMLDivElement> {
  children: React.ReactNode;
}

export function ModalHeader({ className, children, ...props }: ModalHeaderProps) {
  return (
    <div
      className={cn(
        'flex flex-col space-y-1.5 text-center sm:text-left',
        className
      )}
      {...props}
    >
      {children}
    </div>
  );
}
