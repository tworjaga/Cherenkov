import * as React from 'react';
import { cn } from '@/lib/utils';

export interface ModalContentProps extends React.HTMLAttributes<HTMLDivElement> {
  children: React.ReactNode;
}

export function ModalContent({ className, children, ...props }: ModalContentProps) {
  return (
    <div
      className={cn('text-sm text-text-secondary', className)}
      {...props}
    >
      {children}
    </div>
  );
}
