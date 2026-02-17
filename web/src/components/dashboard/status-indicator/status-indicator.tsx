'use client';

import { motion } from 'framer-motion';
import { getDefconColor } from '@/lib/utils/calculations';

interface StatusIndicatorProps {
  defcon: number;
  level: string;
  activeAlerts: number;
}

export const StatusIndicator = ({ defcon, level, activeAlerts }: StatusIndicatorProps) => {
  const color = getDefconColor(defcon);
  
  const getPulseSpeed = () => {
    if (defcon <= 2) return 0.5;
    if (defcon === 3) return 1.5;
    return 0;
  };

  return (
    <div className="flex items-center gap-3 px-4 py-2 bg-bg-secondary border border-border-subtle rounded-lg">
      <div className="relative">
        <div
          className="w-4 h-4 rounded-full"
          style={{ backgroundColor: color }}
        />
        {defcon <= 3 && (
          <motion.div
            data-testid="pulse"
            className="absolute inset-0 rounded-full"
            style={{ backgroundColor: color }}
            animate={{
              scale: [1, 1.5, 1],
              opacity: [0.5, 0, 0.5],
            }}
            transition={{
              duration: getPulseSpeed(),
              repeat: Infinity,
              ease: 'easeInOut',
            }}
          />
        )}
      </div>
      <div className="flex flex-col">
        <span className="text-heading-xs text-text-secondary">DEFCON {defcon}</span>
        <span className="text-body-sm text-text-primary uppercase">{level}</span>
      </div>

      <div className="flex flex-col ml-4 border-l border-border-subtle pl-4">
        <span className="text-heading-xs text-text-secondary">ALERTS</span>
        <span className="text-body-sm text-alert-critical font-mono">
          {activeAlerts}
        </span>
      </div>
    </div>
  );
};
