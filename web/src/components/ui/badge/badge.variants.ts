import { cva } from 'class-variance-authority';

export const badgeVariants = cva(
  'inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2',
  {
    variants: {
      variant: {
        default:
          'border-transparent bg-bg-tertiary text-text-primary hover:bg-bg-hover',
        secondary:
          'border-transparent bg-bg-secondary text-text-secondary hover:bg-bg-hover',
        primary:
          'border-transparent bg-accent-primary/20 text-accent-primary hover:bg-accent-primary/30',

        success:
          'border-transparent bg-alert-normal/20 text-alert-normal hover:bg-alert-normal/30',
        warning:
          'border-transparent bg-alert-medium/20 text-alert-medium hover:bg-alert-medium/30',
        danger:
          'border-transparent bg-alert-critical/20 text-alert-critical hover:bg-alert-critical/30',
        outline: 'text-text-secondary border-border-default',
      },
    },
    defaultVariants: {
      variant: 'default',
    },
  }
);
