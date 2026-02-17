'use client';

import { motion } from 'framer-motion';
import { Menu, Radio } from 'lucide-react';
import { useAppStore } from '@/stores';
import { DefconIndicator } from './defcon-indicator';
import { ConnectionStatus } from './connection-status';
import { formatUTCTime } from '@/lib/utils/formatters';
import { useEffect, useState } from 'react';

export const Header = () => {
  const { toggleSidebar, globalStatus } = useAppStore();
  const [currentTime, setCurrentTime] = useState(new Date());
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
    const interval = setInterval(() => {
      setCurrentTime(new Date());
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  const defconLevel = globalStatus?.defcon || 5;


  return (
    <motion.header
      initial={{ y: -56, opacity: 0 }}
      animate={{ y: 0, opacity: 1 }}
      transition={{ duration: 0.3, ease: [0.4, 0, 0.2, 1] }}
      className="fixed top-0 left-0 right-0 h-header bg-bg-secondary/80 backdrop-blur-md border-b border-border-subtle z-50"
    >
      <div className="h-full flex items-center justify-between px-4">
        <div className="flex items-center gap-4">
          <motion.button
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            onClick={toggleSidebar}
            className="p-2 rounded-md hover:bg-bg-hover transition-colors"
          >
            <Menu className="w-5 h-5 text-text-secondary" />
          </motion.button>

          <div className="flex items-center gap-2">
            <Radio className="w-5 h-5 text-accent-primary" />
            <span
              className="text-xl font-bold tracking-tight text-text-primary"
              style={{ fontFamily: 'JetBrains Mono, monospace' }}
            >
              Cherenkov
            </span>
          </div>
        </div>

        <div className="flex items-center gap-6">
          <DefconIndicator level={defconLevel} />
          
          <div className="flex items-center gap-2 px-3 py-1.5 bg-bg-tertiary rounded-md border border-border-subtle">
            <span className="text-mono-xs text-text-tertiary">UTC</span>
            <span
              className="text-mono-sm text-text-primary"
              style={{ fontFamily: 'JetBrains Mono, monospace' }}
            >
              {mounted ? formatUTCTime(currentTime) : '--:--:--'}
            </span>
          </div>


          <ConnectionStatus />
        </div>
      </div>
    </motion.header>
  );
};
