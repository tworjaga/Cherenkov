'use client';

import React, { createContext, useContext, useCallback, useEffect, ReactNode } from 'react';
import { useAppStore } from '@/stores/app-store';

interface LayoutContextType {
  isMobile: boolean;
  isTablet: boolean;
  isDesktop: boolean;
  sidebarVisible: boolean;
  rightPanelVisible: boolean;
  bottomPanelVisible: boolean;
  toggleSidebar: () => void;
  toggleRightPanel: () => void;
  toggleBottomPanel: () => void;
  setSidebarVisible: (visible: boolean) => void;
  setRightPanelVisible: (visible: boolean) => void;
  setBottomPanelVisible: (visible: boolean) => void;
}

const LayoutContext = createContext<LayoutContextType | undefined>(undefined);

interface LayoutProviderProps {
  children: ReactNode;
}

export function LayoutProvider({ children }: LayoutProviderProps): JSX.Element {
  const {
    sidebarCollapsed,
    rightPanelOpen,
    bottomPanelOpen,
    toggleSidebar: storeToggleSidebar,
    toggleRightPanel: storeToggleRightPanel,
    toggleBottomPanel: storeToggleBottomPanel,
  } = useAppStore();

  const [isMobile, setIsMobile] = React.useState(false);
  const [isTablet, setIsTablet] = React.useState(false);
  const [isDesktop, setIsDesktop] = React.useState(true);

  useEffect(() => {
    const checkScreenSize = (): void => {
      const width = window.innerWidth;
      setIsMobile(width < 768);
      setIsTablet(width >= 768 && width < 1024);
      setIsDesktop(width >= 1024);
    };

    checkScreenSize();
    window.addEventListener('resize', checkScreenSize);
    return () => window.removeEventListener('resize', checkScreenSize);
  }, []);

  const setSidebarVisible = useCallback((visible: boolean): void => {
    const store = useAppStore.getState();
    if (visible !== !store.sidebarCollapsed) {
      storeToggleSidebar();
    }
  }, [storeToggleSidebar]);

  const setRightPanelVisible = useCallback((visible: boolean): void => {
    const store = useAppStore.getState();
    if (visible !== store.rightPanelOpen) {
      storeToggleRightPanel();
    }
  }, [storeToggleRightPanel]);

  const setBottomPanelVisible = useCallback((visible: boolean): void => {
    const store = useAppStore.getState();
    if (visible !== store.bottomPanelOpen) {
      storeToggleBottomPanel();
    }
  }, [storeToggleBottomPanel]);

  const value: LayoutContextType = {
    isMobile,
    isTablet,
    isDesktop,
    sidebarVisible: !sidebarCollapsed,
    rightPanelVisible: rightPanelOpen,
    bottomPanelVisible: bottomPanelOpen,
    toggleSidebar: storeToggleSidebar,
    toggleRightPanel: storeToggleRightPanel,
    toggleBottomPanel: storeToggleBottomPanel,
    setSidebarVisible,
    setRightPanelVisible,
    setBottomPanelVisible,
  };

  return (
    <LayoutContext.Provider value={value}>
      {children}
    </LayoutContext.Provider>
  );
}

export function useLayout(): LayoutContextType {
  const context = useContext(LayoutContext);
  if (context === undefined) {
    throw new Error('useLayout must be used within a LayoutProvider');
  }
  return context;
}
