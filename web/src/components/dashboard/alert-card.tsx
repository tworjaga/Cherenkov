'use client';

import { motion } from 'framer-motion';
import { AlertTriangle, Check } from 'lucide-react';
import { Alert } from '@/types';
import { formatTimestamp } from '@/lib/utils';

interface AlertCardProps {
  alert: Alert;
  onAcknowledge: (id: string) => void;
  isSelected?: boolean;
  onClick?: () => void;
}

const getSeverityBorderClass = (severity: string): string => {
  switch (severity) {
    case 'critical':
      return 'border-l-alert-critical';
    case 'high':
      return 'border-l-alert-high';
    case 'medium':
      return 'border-l-alert-medium';
    case 'low':
      return 'border-l-alert-low';
    default:
      return 'border-l-alert-normal';
  }
};

const getSeverityTextClass = (severity: string): string => {
  switch (severity) {
    case 'critical':
      return 'text-alert-critical';
    case 'high':
      return 'text-alert-high';
    case 'medium':
      return 'text-alert-medium';
    case 'low':
      return 'text-alert-low';
    default:
      return 'text-alert-normal';
  }
};

export const AlertCard = ({ alert, onAcknowledge, isSelected, onClick }: AlertCardProps) => {
  const severityBorderClass = getSeverityBorderClass(alert.severity);
  const severityTextClass = getSeverityTextClass(alert.severity);
  
  return (
    <motion.div
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      onClick={onClick}
      className={`p-3 border-l-4 ${severityBorderClass} bg-bg-secondary mb-2 rounded-r-lg cursor-pointer ${
        alert.acknowledged ? 'opacity-50' : ''
      } ${isSelected ? 'ring-1 ring-accent-primary bg-bg-tertiary' : ''}`}
    >

      <div className="flex items-start justify-between">
        <div className="flex items-start gap-2">
          <AlertTriangle 
            size={16} 
            className={`mt-0.5 ${severityTextClass}`}
          />

          <div>
            <p className={`text-body-sm font-medium ${alert.acknowledged ? 'line-through' : ''}`}>
              {alert.message}
            </p>
            <p className="text-body-xs text-text-tertiary mt-1">
              {formatTimestamp(alert.timestamp)} • {alert.type} • {alert.metadata?.sensorId}
            </p>
            {alert.acknowledged && (
              <p className="text-body-xs text-text-secondary mt-1">Acknowledged</p>
            )}
          </div>
        </div>
        {!alert.acknowledged ? (
          <button
            onClick={() => onAcknowledge(alert.id)}
            className="px-2 py-1 text-body-xs bg-bg-hover hover:bg-bg-active rounded transition-colors"
          >
            Acknowledge
          </button>
        ) : null}
      </div>

    </motion.div>
  );
};
