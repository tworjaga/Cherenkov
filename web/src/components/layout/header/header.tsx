'use client';

import { motion } from 'framer-motion';
import { Menu, Radio, PanelRight, PanelBottom, Layout } from 'lucide-react';
import { useAppStore } from '@/stores';
import { DefconIndicator } from './defcon-indicator';
import { ConnectionStatus } from './connection-status';
import { formatUTCTime } from '@/lib/utils/formatters';
import { useEffect, useState, useCallback } from 'react';
import { useLayout } from '@/components/providers';
import { Tooltip, TooltipTrigger, TooltipContent, TooltipProvider } from '@/components/ui/tooltip';

export const Header = () => {
  const { toggleSidebar, globalStatus } = useAppStore();
  const { toggleRightPanel, toggleBottomPanel, rightPanelVisible, bottomPanelVisible, isMobile } = useLayout();
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

  const handleKeyDown = useCallback((e: React.KeyboardEvent) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      (e.currentTarget as HTMLButtonElement).click();
    }
  }, []);

  return (
    <TooltipProvider>
      <motion.header
        data-testid="header"
        initial={{ y: -56, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        transition={{ duration: 0.3, ease: [0.4, 0, 0.2, 1] }}
        className="fixed top-0 left-0 right-0 h-header bg-bg-secondary/80 backdrop-blur-md border-b border-border-subtle z-50"
        role="banner"
        aria-label="Application header"
      >
        <div className="h-full flex items-center justify-between px-4">
          <div className="flex items-center gap-4">
            <motion.button
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              onClick={toggleSidebar}
              onKeyDown={handleKeyDown}
              className="p-2 rounded-md hover:bg-bg-hover transition-colors focus:outline-none focus:ring-2 focus:ring-accent-primary/50"
              aria-label="Toggle sidebar navigation"
              aria-expanded={!isMobile}
              tabIndex={0}
            >
              <Menu className="w-5 h-5 text-text-secondary" aria-hidden="true" />
            </motion.button>

            <div className="flex items-center gap-2" role="heading" aria-level={1}>
              <Radio className="w-5 h-5 text-accent-primary" aria-hidden="true" />
              <span
                className="text-xl font-bold tracking-tight text-text-primary"
                style={{ fontFamily: 'JetBrains Mono, monospace' }}
              >
                Cherenkov
              </span>
            </div>
          </div>

          <div className="flex items-center gap-4 md:gap-6">
            <div className="hidden md:flex items-center gap-2" role="toolbar" aria-label="Panel controls">
              <Tooltip>
                <TooltipTrigger asChild>
                  <motion.button
                    whileHover={{ scale: 1.05 }}
                    whileTap={{ scale: 0.95 }}
                    onClick={toggleRightPanel}
                    onKeyDown={handleKeyDown}
                    className={`p-2 rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-accent-primary/50 ${rightPanelVisible ? 'bg-accent-primary/20 text-accent-primary' : 'hover:bg-bg-hover text-text-secondary'}`}
                    aria-label={rightPanelVisible ? 'Hide right panel' : 'Show right panel'}
                    aria-pressed={rightPanelVisible}
                    tabIndex={0}
                  >
                    <PanelRight className="w-4 h-4" aria-hidden="true" />
                  </motion.button>
                </TooltipTrigger>
                <TooltipContent side="bottom">
                  {rightPanelVisible ? 'Hide right panel' : 'Show right panel'}
                </TooltipContent>
              </Tooltip>

              <Tooltip>
                <TooltipTrigger asChild>
                  <motion.button
                    whileHover={{ scale: 1.05 }}
                    whileTap={{ scale: 0.95 }}
                    onClick={toggleBottomPanel}
                    onKeyDown={handleKeyDown}
                    className={`p-2 rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-accent-primary/50 ${bottomPanelVisible ? 'bg-accent-primary/20 text-accent-primary' : 'hover:bg-bg-hover text-text-secondary'}`}
                    aria-label={bottomPanelVisible ? 'Hide bottom panel' : 'Show bottom panel'}
                    aria-pressed={bottomPanelVisible}
                    tabIndex={0}
                  >
                    <PanelBottom className="w-4 h-4" aria-hidden="true" />
                  </motion.button>
                </TooltipTrigger>
                <TooltipContent side="bottom">
                  {bottomPanelVisible ? 'Hide bottom panel' : 'Show bottom panel'}
                </TooltipContent>
              </Tooltip>

              <Tooltip>
                <TooltipTrigger asChild>
                  <motion.button
                    whileHover={{ scale: 1.05 }}
                    whileTap={{ scale: 0.95 }}
                    onClick={() => {
                      if (!rightPanelVisible) toggleRightPanel();
                      if (!bottomPanelVisible) toggleBottomPanel();
                    }}
                    onKeyDown={handleKeyDown}
                    className="p-2 rounded-md hover:bg-bg-hover transition-colors focus:outline-none focus:ring-2 focus:ring-accent-primary/50 text-text-secondary"
                    aria-label="Reset layout to default"
                    tabIndex={0}
                  >
                    <Layout className="w-4 h-4" aria-hidden="true" />
                  </motion.button>
                </TooltipTrigger>
                <TooltipContent side="bottom">Reset layout</TooltipContent>
              </Tooltip>
            </div>

            <div className="h-6 w-px bg-border-subtle hidden md:block" role="separator" aria-orientation="vertical" />

            <DefconIndicator level={defconLevel} />
            
            <div 
              data-testid="utc-clock" 
              className="flex items-center gap-2 px-3 py-1.5 bg-bg-tertiary rounded-md border border-border-subtle"
              aria-label="Current UTC time"
            >
              <span className="text-mono-xs text-text-tertiary">UTC</span>
              <span
                className="text-mono-sm text-text-primary"
                style={{ fontFamily: 'JetBrains Mono, monospace' }}
                aria-live="off"
              >
                {mounted ? formatUTCTime(currentTime) : '--:--:--'}
              </span>
            </div>

            <ConnectionStatus />
          </div>
        </div>
      </motion.header>
    </TooltipProvider>
  );
};
