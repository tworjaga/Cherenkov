'use client';

import { useRef, useState } from 'react';
import { motion } from 'framer-motion';
import { Alert } from '@/types';
import { AlertCard } from './alert-card';

interface AlertFeedProps {
  alerts: Alert[];
  onAcknowledge: (id: string) => void;
  onAlertClick: (alert: Alert) => void;
}

export const AlertFeed = ({ alerts, onAcknowledge, onAlertClick }: AlertFeedProps) => {
  const [isPaused, setIsPaused] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  const sortedAlerts = [...alerts].sort((a, b) => b.timestamp - a.timestamp);
  const unreadCount = alerts.filter(a => !a.acknowledged).length;

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between p-3 border-b border-border-subtle">
        <div className="flex items-center gap-2">
          <span className="text-heading-xs text-text-secondary">ALERTS</span>
          {unreadCount > 0 && (
            <span className="px-2 py-0.5 bg-alert-critical text-white text-mono-xs rounded-full">
              {unreadCount}
            </span>
          )}
        </div>
        <span className="text-body-xs text-text-tertiary">
          {alerts.length} total
        </span>
      </div>
      
      <div
        ref={containerRef}
        className="flex-1 overflow-y-auto p-2"
        onMouseEnter={() => setIsPaused(true)}
        onMouseLeave={() => setIsPaused(false)}
      >
        {sortedAlerts.length === 0 ? (
          <div className="flex items-center justify-center h-full text-text-tertiary text-body-sm">
            No alerts
          </div>
        ) : (
          <motion.div
            animate={isPaused ? {} : { y: [0, -10, 0] }}
            transition={{ duration: 0.5 }}
          >
            {sortedAlerts.map((alert) => (
              <div key={alert.id} onClick={() => onAlertClick(alert)}>
                <AlertCard alert={alert} onAcknowledge={onAcknowledge} />
              </div>
            ))}
          </motion.div>
        )}
      </div>
    </div>
  );
};
