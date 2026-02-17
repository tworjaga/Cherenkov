'use client';

import { motion } from 'framer-motion';
import { getDefconColor } from '@/lib/utils/calculations';

interface DefconIndicatorProps {
  level: number;
}

export const DefconIndicator = ({ level }: DefconIndicatorProps) => {
  const color = getDefconColor(level);
  const shouldPulse = level <= 3;

  return (
    <motion.div
      data-testid="defcon-indicator"
      className="flex items-center gap-2 px-3 py-1.5 rounded-md border border-border-subtle bg-bg-tertiary"

      animate={shouldPulse ? {
        boxShadow: [
          `0 0 0px ${color}40`,
          `0 0 20px ${color}60`,
          `0 0 0px ${color}40`,
        ],
      } : {}}
      transition={shouldPulse ? {
        duration: level === 1 ? 0.5 : level === 2 ? 1 : 2,
        repeat: Infinity,
        ease: 'easeInOut',
      } : {}}
    >
      <div
        className="w-3 h-3 rounded-sm"
        style={{
          backgroundColor: color,
          boxShadow: `0 0 10px ${color}`,
        }}
      />
      <span
        className="text-heading-xs"
        style={{ color }}
      >
        DEFCON {level}
      </span>
    </motion.div>
  );
};
