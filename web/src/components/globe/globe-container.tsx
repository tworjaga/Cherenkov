'use client';

import { ReactNode } from 'react';
import { motion } from 'framer-motion';

interface GlobeContainerProps {
  children: ReactNode;
  isLoading?: boolean;
}

export const GlobeContainer = ({ children, isLoading = false }: GlobeContainerProps) => {
  return (
    <motion.div 
      className="relative w-full h-full bg-bg-primary overflow-hidden"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.5 }}
    >
      {children}
      
      {isLoading && (
        <div className="absolute inset-0 flex items-center justify-center bg-bg-primary/80 backdrop-blur-sm z-50">
          <div className="flex flex-col items-center gap-4">
            <div className="w-12 h-12 border-2 border-accent-primary border-t-transparent rounded-full animate-spin" />
            <span className="text-body-sm text-text-secondary">Loading globe...</span>
          </div>
        </div>
      )}
    </motion.div>
  );
};
