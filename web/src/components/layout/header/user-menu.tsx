'use client';

import { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { User, Settings, LogOut, ChevronDown } from 'lucide-react';
import { useAuthStore } from '@/stores';

export const UserMenu = () => {
  const [isOpen, setIsOpen] = useState(false);
  const { user, logout } = useAuthStore();

  if (!user) return null;

  return (
    <div className="relative">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center gap-2 px-3 py-1.5 bg-bg-tertiary rounded-md border border-border-subtle hover:border-border-active transition-colors"
      >
        <User size={14} className="text-accent-primary" />
        <span className="text-body-sm text-text-primary">{user.name}</span>
        <ChevronDown size={12} className={`text-text-tertiary transition-transform ${isOpen ? 'rotate-180' : ''}`} />
      </button>

      <AnimatePresence>
        {isOpen && (
          <motion.div
            initial={{ opacity: 0, y: -8 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -8 }}
            className="absolute right-0 top-full mt-2 w-48 bg-bg-secondary border border-border-default rounded-md shadow-lg z-50"
          >
            <div className="p-2">
              <button className="w-full flex items-center gap-2 px-3 py-2 text-body-sm text-text-primary hover:bg-bg-hover rounded-md transition-colors">
                <Settings size={14} />
                Settings
              </button>
              <button 
                onClick={logout}
                className="w-full flex items-center gap-2 px-3 py-2 text-body-sm text-alert-critical hover:bg-bg-hover rounded-md transition-colors"
              >
                <LogOut size={14} />
                Logout
              </button>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
};
