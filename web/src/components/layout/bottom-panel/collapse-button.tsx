'use client';

import { motion } from 'framer-motion';
import { ChevronDown } from 'lucide-react';


interface CollapseButtonProps {
  isOpen: boolean;
  onToggle: () => void;
}

export const CollapseButton = ({ isOpen, onToggle }: CollapseButtonProps) => {
  return (
    <button
      onClick={onToggle}
      className="absolute -top-3 left-1/2 -translate-x-1/2 w-8 h-6 bg-bg-tertiary border border-border-default rounded-t-md flex items-center justify-center hover:border-border-active transition-colors z-10"
      aria-label={isOpen ? 'Collapse panel' : 'Expand panel'}
    >
      <motion.div
        animate={{ rotate: isOpen ? 0 : 180 }}
        transition={{ duration: 0.2 }}
      >
        <ChevronDown size={14} />
      </motion.div>
    </button>
  );
};
