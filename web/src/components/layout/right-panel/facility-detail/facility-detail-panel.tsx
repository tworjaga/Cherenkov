'use client';

import { motion, AnimatePresence } from 'framer-motion';
import { X, Building2, MapPin, Activity, Clock } from 'lucide-react';
import { useAppStore } from '@/stores';
import { FacilityDetail } from '@/components/dashboard';
import { ScrollArea } from '@/components/ui/scroll-area';
import { animations } from '@/styles/theme';

export function FacilityDetailPanel(): JSX.Element {
  const { selectedFacilityId, selectFacility } = useAppStore();

  return (
    <AnimatePresence>
      {selectedFacilityId && (
        <motion.div
          initial={{ x: '100%', opacity: 0 }}
          animate={{ x: 0, opacity: 1 }}
          exit={{ x: '100%', opacity: 0 }}
          transition={animations.slideIn.transition}
          className="absolute inset-0 bg-[#0a0a10] border-l border-[#1f1f2e] flex flex-col"
        >
          {/* Header */}
          <div className="flex items-center justify-between p-4 border-b border-[#1f1f2e]">
            <div className="flex items-center gap-2">
              <Building2 className="w-4 h-4 text-[#00d4ff]" />
              <span className="text-sm font-semibold text-white uppercase tracking-wider">
                Facility Detail
              </span>
            </div>
            <button
              onClick={() => selectFacility(null)}
              className="p-1.5 rounded hover:bg-[#1a1a25] transition-colors"
            >
              <X className="w-4 h-4 text-[#606070]" />
            </button>
          </div>

          {/* Content */}
          <ScrollArea className="flex-1">
            <div className="p-4">
              <FacilityDetail />
            </div>
          </ScrollArea>
        </motion.div>
      )}
    </AnimatePresence>
  );
}
