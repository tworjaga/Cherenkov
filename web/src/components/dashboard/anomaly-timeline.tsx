'use client';

import { motion } from 'framer-motion';
import { AlertTriangle, Clock } from 'lucide-react';
import { Anomaly } from '@/types';
import { getSeverityColor, formatTimestamp } from '@/lib/utils';

interface AnomalyTimelineProps {
  anomalies: Anomaly[];
  onSelect: (anomaly: Anomaly) => void;
}

export const AnomalyTimeline = ({ anomalies, onSelect }: AnomalyTimelineProps) => {
  const sortedAnomalies = [...anomalies].sort((a, b) => b.detectedAt - a.detectedAt);

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between p-3 border-b border-border-subtle">
        <span className="text-heading-xs text-text-secondary">ANOMALIES</span>
        <span className="text-body-xs text-text-tertiary">
          {anomalies.length} detected
        </span>
      </div>
      
      <div className="flex-1 overflow-y-auto p-2">
        {sortedAnomalies.length === 0 ? (
          <div className="flex items-center justify-center h-full text-text-tertiary text-body-sm">
            No anomalies detected
          </div>
        ) : (
          <div className="relative">
            <div className="absolute left-4 top-0 bottom-0 w-px bg-border-subtle" />
            {sortedAnomalies.map((anomaly, index) => (
              <motion.button
                key={anomaly.id}
                initial={{ opacity: 0, x: -20 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ delay: index * 0.05 }}
                onClick={() => onSelect(anomaly)}
                className="relative flex items-start gap-3 p-3 w-full text-left hover:bg-bg-hover rounded-lg transition-colors mb-2"
              >
                <div
                  className="relative z-10 w-8 h-8 rounded-full flex items-center justify-center flex-shrink-0"
                  style={{ backgroundColor: `${getSeverityColor(anomaly.severity)}20` }}
                >
                  <AlertTriangle
                    size={14}
                    style={{ color: getSeverityColor(anomaly.severity) }}
                  />
                </div>
                
                <div className="flex-1 min-w-0">
                  <p className="text-body-sm font-medium text-text-primary truncate">
                    {anomaly.message}
                  </p>
                  <div className="flex items-center gap-2 mt-1">
                    <Clock size={12} className="text-text-tertiary" />
                    <span className="text-body-xs text-text-tertiary">
                      {formatTimestamp(anomaly.detectedAt)}
                    </span>
                  </div>
                  <div className="flex items-center gap-3 mt-1">
                    <span className="text-mono-xs text-accent-primary">
                      Z-Score: {anomaly.zScore.toFixed(2)}
                    </span>
                    <span className="text-mono-xs text-text-secondary">
                      {anomaly.doseRate.toFixed(3)} μSv/h
                    </span>
                  </div>
                </div>
              </motion.button>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};
