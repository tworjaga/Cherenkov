'use client';

import { AlertTriangle, Info, CheckCircle } from 'lucide-react';
import { Alert } from '@/types';
import { formatTimestamp } from '@/lib/utils';
import { cn } from '@/lib/utils';

export interface AlertCardProps {
  alert: Alert;
  onClick?: () => void;
  onAcknowledge?: (alertId: string) => void;
  isSelected?: boolean;
  className?: string;
}


export const AlertCard = ({ alert, onClick, onAcknowledge, isSelected, className }: AlertCardProps) => {

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
        return 'border-l-4 border-l-alert-critical border-alert-critical/30 bg-alert-critical/5';
      case 'high':
        return 'border-l-4 border-l-alert-medium border-alert-medium/30 bg-alert-medium/5';
      case 'medium':
        return 'border-l-4 border-l-accent-primary border-accent-primary/30 bg-accent-primary/5';
      case 'low':
        return 'border-l-4 border-l-alert-normal border-border-subtle bg-bg-secondary';
      default:
        return 'border-l-4 border-l-alert-normal border-border-subtle bg-bg-secondary';
    }
  };


  return (
    <div
      onClick={onClick}
      className={cn(
        'p-3 rounded-lg border cursor-pointer transition-all hover:shadow-md',
        getSeverityStyles(),
        isSelected && 'ring-2 ring-accent-primary',
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
              {String(formatTimestamp(alert.timestamp))}
            </span>

          </div>
          <p className="text-body-xs text-text-secondary mt-1 line-clamp-2">
            {alert.message}
          </p>
          {alert.metadata?.sensorId ? (
            <p className="text-mono-xs text-text-tertiary mt-1">
              Sensor: {alert.metadata.sensorId}
            </p>
          ) : null}

          {onAcknowledge && (
            <button
              onClick={(e) => {
                e.stopPropagation();
                onAcknowledge(alert.id);
              }}
              className="mt-2 text-xs text-accent-primary hover:text-accent-secondary transition-colors"
            >
              {alert.acknowledged ? 'Acknowledged' : 'Acknowledge'}
            </button>
          )}

        </div>
      </div>
    </div>

  );
};
