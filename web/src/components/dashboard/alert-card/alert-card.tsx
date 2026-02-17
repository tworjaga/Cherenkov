'use client';

import { AlertTriangle, Info, CheckCircle } from 'lucide-react';
import { Alert } from '@/types';
import { formatTimestamp } from '@/lib/utils';
import { cn } from '@/lib/utils';

interface AlertCardProps {
  alert: Alert;
  onClick?: () => void;
  className?: string;
}

export const AlertCard = ({ alert, onClick, className }: AlertCardProps) => {
  const getSeverityIcon = () => {
    switch (alert.severity) {
      case 'critical':
        return <AlertTriangle className="w-5 h-5 text-alert-critical" />;
      case 'high':
        return <AlertTriangle className="w-5 h-5 text-alert-medium" />;
      case 'medium':
        return <Info className="w-5 h-5 text-accent-primary" />;
      case 'low':
        return <CheckCircle className="w-5 h-5 text-alert-normal" />;
      default:
        return <CheckCircle className="w-5 h-5 text-alert-normal" />;
    }
  };

  const getSeverityStyles = () => {
    switch (alert.severity) {
      case 'critical':
        return 'border-alert-critical/30 bg-alert-critical/5';
      case 'high':
        return 'border-alert-medium/30 bg-alert-medium/5';
      case 'medium':
        return 'border-accent-primary/30 bg-accent-primary/5';
      case 'low':
        return 'border-border-subtle bg-bg-secondary';
      default:
        return 'border-border-subtle bg-bg-secondary';
    }
  };

  return (
    <div
      onClick={onClick}
      className={cn(
        'p-3 rounded-lg border cursor-pointer transition-all hover:shadow-md',
        getSeverityStyles(),
        className
      )}
    >
      <div className="flex items-start gap-3">
        {getSeverityIcon()}
        <div className="flex-1 min-w-0">
          <div className="flex items-center justify-between gap-2">
            <span className="text-body-sm font-medium text-text-primary truncate">
              {alert.type}
            </span>
            <span className="text-mono-xs text-text-tertiary whitespace-nowrap">
              {formatTimestamp(alert.timestamp)}
            </span>
          </div>
          <p className="text-body-xs text-text-secondary mt-1 line-clamp-2">
            {alert.message}
          </p>
        </div>
      </div>
    </div>
  );
};
