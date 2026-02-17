'use client';

import { useState, useEffect } from 'react';
import { motion } from 'framer-motion';
import { Clock } from 'lucide-react';

export const GlobalClock = () => {
  const [time, setTime] = useState<Date | null>(null);

  useEffect(() => {
    setTime(new Date());
    const interval = setInterval(() => {
      setTime(new Date());
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  if (!time) return null;

  const formatUTC = (date: Date) => {
    return date.toISOString().split('T')[1].split('.')[0];
  };

  return (
    <motion.div 
      className="flex items-center gap-2 px-3 py-1.5 bg-bg-tertiary rounded-md border border-border-subtle"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
    >
      <Clock size={14} className="text-accent-primary" />
      <span className="text-mono-sm text-text-primary tracking-wider">
        UTC {formatUTC(time)}
      </span>
    </motion.div>
  );
};
