'use client';

import * as React from 'react';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '@/lib/utils';

const badgeVariants = cva(
  'inline-flex items-center rounded px-2 py-0.5 text-xs font-medium font-mono transition-colors',
  {
    variants: {
      variant: {
        default: 'bg-bg-tertiary text-text-secondary border border-border-subtle',
        primary: 'bg-accent-primary/10 text-accent-primary border border-accent-primary/20',
        success: 'bg-alert-normal/10 text-alert-normal border border-alert-normal/20',
        warning: 'bg-alert-medium/10 text-alert-medium border border-alert-medium/20',
        danger: 'bg-alert-critical/10 text-alert-critical border border-alert-critical/20',
        outline: 'border border-border-default text-text-secondary',
      },
    },
    defaultVariants: {
      variant: 'default',
    },
  }
);

export interface BadgeProps
  extends React.HTMLAttributes<HTMLDivElement>,
    VariantProps<typeof badgeVariants> {}

function Badge({ className, variant, ...props }: BadgeProps) {
  return (
    <div className={cn(badgeVariants({ variant }), className)} {...props} />
  );
}

export { Badge, badgeVariants };
export type { VariantProps };
