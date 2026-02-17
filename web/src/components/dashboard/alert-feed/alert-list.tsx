'use client';

import { motion, AnimatePresence } from 'framer-motion';
import { Alert } from '@/types/models';
import { AlertCard } from '../alert-card';

interface AlertListProps {
  alerts: Alert[];
  onAcknowledge: (alertId: string) => void;
  onSelect: (alert: Alert) => void;
  selectedAlertId?: string | null;
}

export function AlertList({
  alerts,
  onAcknowledge,
  onSelect,
  selectedAlertId,
}: AlertListProps) {
  return (
    <div className="space-y-2">
      <AnimatePresence mode="popLayout">
        {alerts.map((alert) => (
          <motion.div
            key={alert.id}
            layout
            initial={{ opacity: 0, y: -20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, x: 100 }}
            transition={{ duration: 0.2 }}
          >
            <AlertCard
              alert={alert}
              isSelected={selectedAlertId === alert.id}
              onAcknowledge={() => onAcknowledge(alert.id)}
              onClick={() => onSelect(alert)}
            />
          </motion.div>
        ))}
      </AnimatePresence>

      {alerts.length === 0 && (
        <div className="flex h-32 items-center justify-center text-sm text-[#606070]">
          No active alerts
        </div>
      )}
    </div>
  );
}
