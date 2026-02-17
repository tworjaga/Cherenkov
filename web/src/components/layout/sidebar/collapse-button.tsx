'use client';

import { motion } from 'framer-motion';
import { ChevronLeft, ChevronRight } from 'lucide-react';

interface CollapseButtonProps {
  collapsed: boolean;
  onToggle: () => void;
}

export const CollapseButton = ({ collapsed, onToggle }: CollapseButtonProps) => {
  return (
    <button
      onClick={onToggle}
      className="absolute -right-3 top-1/2 -translate-y-1/2 w-6 h-6 bg-bg-tertiary border border-border-default rounded-full flex items-center justify-center hover:border-border-active transition-colors z-10"
      aria-label={collapsed ? 'Expand sidebar' : 'Collapse sidebar'}
    >
      <motion.div
        animate={{ rotate: collapsed ? 180 : 0 }}
        transition={{ duration: 0.2 }}
      >
        {collapsed ? <ChevronRight size={12} /> : <ChevronLeft size={12} />}
      </motion.div>
    </button>
  );
};
