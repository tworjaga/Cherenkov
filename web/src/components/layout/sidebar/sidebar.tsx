'use client';

import { motion, AnimatePresence } from 'framer-motion';
import {
  LayoutDashboard,
  Globe,
  Radio,
  AlertTriangle,
  Wind,
  Settings,
  X,
} from 'lucide-react';
import { useAppStore } from '@/stores';
import { useLayout } from '@/components/providers';
import { ViewType } from '@/types';
import { Tooltip, TooltipTrigger, TooltipContent, TooltipProvider } from '@/components/ui/tooltip';
import { useCallback, useEffect } from 'react';

const navItems: { id: ViewType; icon: React.ElementType; label: string }[] = [
  { id: 'dashboard', icon: LayoutDashboard, label: 'Dashboard' },
  { id: 'globe', icon: Globe, label: 'Globe' },
  { id: 'sensors', icon: Radio, label: 'Sensors' },
  { id: 'anomalies', icon: AlertTriangle, label: 'Anomalies' },
  { id: 'plume', icon: Wind, label: 'Plume' },
  { id: 'settings', icon: Settings, label: 'Settings' },
];

export const Sidebar = () => {
  const { view, setView, sidebarCollapsed } = useAppStore();
  const { isMobile, sidebarVisible, toggleSidebar } = useLayout();

  const handleKeyDown = useCallback((e: React.KeyboardEvent, itemId: ViewType) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      setView(itemId);
    }
  }, [setView]);

  // Close sidebar on mobile when navigating
  const handleNavClick = useCallback((itemId: ViewType) => {
    setView(itemId);
    if (isMobile) {
      toggleSidebar();
    }
  }, [setView, isMobile, toggleSidebar]);

  // Handle escape key to close mobile sidebar
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && isMobile && sidebarVisible) {
        toggleSidebar();
      }
    };
    window.addEventListener('keydown', handleEscape);
    return () => window.removeEventListener('keydown', handleEscape);
  }, [isMobile, sidebarVisible, toggleSidebar]);

  // Mobile overlay backdrop
  const MobileOverlay = () => (
    <AnimatePresence>
      {isMobile && sidebarVisible && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.2 }}
          className="fixed inset-0 bg-black/50 z-30 md:hidden"
          onClick={toggleSidebar}
          aria-hidden="true"
        />
      )}
    </AnimatePresence>
  );

  return (
    <TooltipProvider>
      <>
        <MobileOverlay />
        <motion.aside
          data-testid="sidebar"
          initial={{ x: isMobile ? -280 : -64, opacity: 0 }}
          animate={{ 
            x: sidebarVisible ? 0 : (isMobile ? -280 : 0), 
            opacity: sidebarVisible ? 1 : (isMobile ? 0 : 1),
            width: sidebarCollapsed && !isMobile ? 64 : 240
          }}
          transition={{ duration: 0.3, ease: [0.4, 0, 0.2, 1] }}
          className={`
            fixed left-0 top-header h-[calc(100vh-56px)] bg-bg-secondary border-r border-border-subtle z-40
            ${isMobile ? 'w-[240px] shadow-2xl' : ''}
            ${!sidebarVisible && !isMobile ? 'hidden md:block' : ''}
          `}
          role="navigation"
          aria-label="Main navigation"

        >
          {/* Mobile close button */}
          {isMobile && (
            <div className="flex justify-end p-2 md:hidden">
              <button
                onClick={toggleSidebar}
                className="p-2 rounded-md hover:bg-bg-hover transition-colors"
                aria-label="Close navigation menu"
              >
                <X className="w-5 h-5 text-text-secondary" aria-hidden="true" />
              </button>
            </div>
          )}

          <nav className="flex flex-col gap-1 p-2" role="menubar" aria-label="Application views">
          {navItems.map((item) => {

              const Icon = item.icon;
              const isActive = view === item.id;

              return (
                <Tooltip key={item.id} delayDuration={sidebarCollapsed ? 100 : 1000}>
                  <TooltipTrigger asChild>
                    <motion.button
                      data-testid={`nav-${item.id}`}
                      onClick={() => handleNavClick(item.id)}
                      onKeyDown={(e) => handleKeyDown(e, item.id)}
                      whileHover={{ scale: 1.02 }}
                      whileTap={{ scale: 0.98 }}
                      className={`
                        flex items-center gap-3 px-3 py-3 rounded-md transition-all duration-fast
                        focus:outline-none focus:ring-2 focus:ring-accent-primary/50
                        min-h-[44px] touch-manipulation
                        ${isActive 
                          ? 'bg-bg-active text-accent-primary border-l-2 border-accent-primary' 
                          : 'text-text-secondary hover:bg-bg-hover hover:text-text-primary'
                        }
                      `}
                      role="menuitem"
                      aria-current={isActive ? 'page' : undefined}
                      aria-label={item.label}
                      tabIndex={0}
                    >
                      <Icon className="w-6 h-6 flex-shrink-0" strokeWidth={1.5} aria-hidden="true" />
                      <AnimatePresence mode="wait">
                        {(!sidebarCollapsed || isMobile) && (
                          <motion.span
                            initial={{ opacity: 0, width: 0 }}
                            animate={{ opacity: 1, width: 'auto' }}
                            exit={{ opacity: 0, width: 0 }}
                            transition={{ duration: 0.2 }}
                            className="text-body-sm font-medium whitespace-nowrap overflow-hidden"
                          >
                            {item.label}
                          </motion.span>
                        )}
                      </AnimatePresence>
                    </motion.button>
                  </TooltipTrigger>
                  {sidebarCollapsed && !isMobile && (
                    <TooltipContent side="right" sideOffset={10}>
                      {item.label}
                    </TooltipContent>
                  )}
                </Tooltip>
              );
            })}
          </nav>

          {/* Keyboard shortcut hint */}
          <div className="absolute bottom-4 left-0 right-0 px-4 hidden md:block">
            <p className="text-mono-xs text-text-tertiary text-center">
              Press <kbd className="px-1.5 py-0.5 bg-bg-tertiary rounded text-text-secondary">1-6</kbd> to navigate
            </p>
          </div>
        </motion.aside>
      </>
    </TooltipProvider>
  );
};
