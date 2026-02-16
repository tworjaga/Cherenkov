'use client';

import { motion } from 'framer-motion';
import { useAppStore } from '@/stores';

export const ConnectionStatus = () => {
  const { connectionStatus } = useAppStore();

  const getStatusColor = () => {
    switch (connectionStatus) {
      case 'connected':
        return '#00ff88';
      case 'connecting':
        return '#ffb800';
      case 'disconnected':
        return '#ff3366';
      default:
        return '#606070';
    }
  };

  const getStatusText = () => {
    switch (connectionStatus) {
      case 'connected':
        return 'Connected';
      case 'connecting':
        return 'Connecting...';
      case 'disconnected':
        return 'Disconnected';
      default:
        return 'Unknown';
    }
  };

  return (
    <div className="flex items-center gap-2">
      <motion.div
        className="w-2 h-2 rounded-full"
        style={{ backgroundColor: getStatusColor() }}
        animate={connectionStatus === 'connecting' ? {
          scale: [1, 1.5, 1],
          opacity: [1, 0.5, 1],
        } : {}}
        transition={{
          duration: 1,
          repeat: Infinity,
          ease: 'easeInOut',
        }}
      />
      <span className="text-body-xs text-text-tertiary">
        {getStatusText()}
      </span>
    </div>
  );
};
