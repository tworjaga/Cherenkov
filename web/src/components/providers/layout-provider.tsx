'use client';

import React from 'react';

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
  rightPanelOpen: boolean;
  bottomPanelOpen: boolean;
  resetLayout: () => void;
}

const LayoutContext = React.createContext<LayoutContextType | null>(null);

export function useLayout(): LayoutContextType {
  const context = React.useContext(LayoutContext);
  if (!context) {
    throw new Error('useLayout must be used within LayoutProvider');
  }
  return context;
}

interface ProviderProps {
  children: React.ReactNode;
}

export function LayoutProvider(props: ProviderProps): React.ReactElement {

  const [sidebarVisible, setSidebarVisible] = React.useState(true);
  const [rightPanelVisible, setRightPanelVisible] = React.useState(true);
  const [bottomPanelVisible, setBottomPanelVisible] = React.useState(true);

  const toggleSidebar = React.useCallback(() => {
    setSidebarVisible(prev => !prev);
  }, []);

  const toggleRightPanel = React.useCallback(() => {
    setRightPanelVisible(prev => !prev);
  }, []);

  const toggleBottomPanel = React.useCallback(() => {
    setBottomPanelVisible(prev => !prev);
  }, []);

  const resetLayout = React.useCallback(() => {
    setSidebarVisible(true);
    setRightPanelVisible(true);
    setBottomPanelVisible(true);
  }, []);

  const value = React.useMemo<LayoutContextType>(() => ({
    isMobile: false,
    isTablet: false,
    isDesktop: true,
    sidebarVisible,
    rightPanelVisible,
    bottomPanelVisible,
    toggleSidebar,
    toggleRightPanel,
    toggleBottomPanel,
    rightPanelOpen: rightPanelVisible,
    bottomPanelOpen: bottomPanelVisible,
    resetLayout,
  }), [sidebarVisible, rightPanelVisible, bottomPanelVisible, toggleSidebar, toggleRightPanel, toggleBottomPanel, resetLayout]);

  return (
    <LayoutContext.Provider value={value}>
      {props.children}
    </LayoutContext.Provider>
  );
}
