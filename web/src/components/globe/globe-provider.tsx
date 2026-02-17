'use client';

import { ReactNode, createContext, useContext, useState } from 'react';
import type { GlobeViewport } from '@/types';

interface GlobeContextType {
  viewport: GlobeViewport;
  setViewport: (viewport: Partial<GlobeViewport>) => void;
  isLoading: boolean;
  setIsLoading: (loading: boolean) => void;
}

const GlobeContext = createContext<GlobeContextType | undefined>(undefined);

export const GlobeProvider = ({ children }: { children: ReactNode }) => {
  const [viewport, setViewportState] = useState<GlobeViewport>({
    latitude: 20,
    longitude: 0,
    zoom: 2,
    pitch: 0,
    bearing: 0,
  });
  
  const [isLoading, setIsLoading] = useState(true);

  const setViewport = (updates: Partial<GlobeViewport>) => {
    setViewportState(prev => ({ ...prev, ...updates }));
  };

  return (
    <GlobeContext.Provider value={{ viewport, setViewport, isLoading, setIsLoading }}>
      {children}
    </GlobeContext.Provider>
  );
};

export const useGlobeContext = () => {
  const context = useContext(GlobeContext);
  if (!context) throw new Error('useGlobeContext must be used within GlobeProvider');
  return context;
};
