import { cva } from 'class-variance-authority';

export const buttonVariants = cva(
  'inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[#00d4ff] disabled:pointer-events-none disabled:opacity-50',
  {
    variants: {
      variant: {
        default: 'bg-[#00d4ff] text-[#050508] hover:bg-[#00a8cc]',
        secondary: 'bg-[#12121a] text-white border border-[#2a2a3d] hover:bg-[#1a1a25]',
        outline: 'border border-[#2a2a3d] bg-transparent hover:bg-[#1a1a25] text-white',
        ghost: 'hover:bg-[#1a1a25] text-white',
        danger: 'bg-[#ff3366] text-white hover:bg-[#ff1a53]',
      },
      size: {
        default: 'h-10 px-4 py-2',
        sm: 'h-8 px-3 text-xs',
        lg: 'h-12 px-6',
        icon: 'h-10 w-10',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
    },
  }
);
