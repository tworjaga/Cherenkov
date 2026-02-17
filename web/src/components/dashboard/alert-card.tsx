'use client';

import { motion } from 'framer-motion';
import { AlertTriangle, Check } from 'lucide-react';
import { Alert } from '@/types';
import { getSeverityColor, formatTimestamp } from '@/lib/utils';

interface AlertCardProps {
  alert: Alert;
  onAcknowledge: (id: string) => void;
  isSelected?: boolean;
  onClick?: () => void;
}

export const AlertCard = ({ alert, onAcknowledge, isSelected, onClick }: AlertCardProps) => {

  const severityColor = getSeverityColor(alert.severity);
  
  return (
    <motion.div
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      onClick={onClick}
      className={`p-3 border-l-2 bg-bg-secondary mb-2 rounded-r-lg cursor-pointer ${
        alert.acknowledged ? 'opacity-50' : ''
      } ${isSelected ? 'ring-1 ring-accent-primary bg-bg-tertiary' : ''}`}
      style={{ borderLeftColor: severityColor }}
    >

      <div className="flex items-start justify-between">
        <div className="flex items-start gap-2">
          <AlertTriangle 
            size={16} 
            style={{ color: severityColor }}
            className="mt-0.5"
          />
          <div>
            <p className={`text-body-sm font-medium ${alert.acknowledged ? 'line-through' : ''}`}>
              {alert.message}
            </p>
            <p className="text-body-xs text-text-tertiary mt-1">
              {formatTimestamp(alert.timestamp)} • {alert.type}
            </p>

          </div>
        </div>
        {!alert.acknowledged && (
          <button
            onClick={() => onAcknowledge(alert.id)}
            className="p-1 hover:bg-bg-hover rounded transition-colors"
          >
            <Check size={14} className="text-alert-normal" />
          </button>
        )}
      </div>
    </motion.div>
  );
};
