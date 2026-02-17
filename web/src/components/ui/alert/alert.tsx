import * as React from 'react';
import { AlertTriangle, CheckCircle, Info, XCircle } from 'lucide-react';
import { cn } from '@/lib/utils';

export interface AlertProps extends React.HTMLAttributes<HTMLDivElement> {
  variant?: 'info' | 'success' | 'warning' | 'error';
  title?: string;
  dismissible?: boolean;
  onDismiss?: () => void;
}

export const Alert = React.forwardRef<HTMLDivElement, AlertProps>(
  ({ className, variant = 'info', title, dismissible, onDismiss, children, ...props }, ref) => {
    const variantClasses = {
      info: {
        container: 'bg-[#00d4ff]/10 border-[#00d4ff]/30',
        icon: 'text-[#00d4ff]',
        Icon: Info,
      },
      success: {
        container: 'bg-[#00ff88]/10 border-[#00ff88]/30',
        icon: 'text-[#00ff88]',
        Icon: CheckCircle,
      },
      warning: {
        container: 'bg-[#ffb800]/10 border-[#ffb800]/30',
        icon: 'text-[#ffb800]',
        Icon: AlertTriangle,
      },
      error: {
        container: 'bg-[#ff3366]/10 border-[#ff3366]/30',
        icon: 'text-[#ff3366]',
        Icon: XCircle,
      },
    };

    const { container, icon, Icon } = variantClasses[variant];

    return (
      <div
        ref={ref}
        className={cn(
          'relative flex gap-3 rounded-lg border p-4',
          container,
          className
        )}
        {...props}
      >
        <Icon className={cn('h-5 w-5 flex-shrink-0 mt-0.5', icon)} />
        <div className="flex-1">
          {title && (
            <h5 className="mb-1 font-medium text-sm text-white">{title}</h5>
          )}
          <div className="text-sm text-[#a0a0b0]">{children}</div>
        </div>
        {dismissible && onDismiss && (
          <button
            onClick={onDismiss}
            className="flex-shrink-0 text-[#606070] hover:text-white transition-colors"
          >
            <XCircle className="h-4 w-4" />
          </button>
        )}
      </div>
    );
  }
);

Alert.displayName = 'Alert';
