import React, { useEffect } from 'react';
import { Header } from './components/layout/Header';
import { Sidebar } from './components/layout/Sidebar';
import { RightPanel } from './components/layout/RightPanel';
import { BottomPanel } from './components/layout/BottomPanel';
import { GlobeContainer } from './components/globe/GlobeContainer';
import { useAppStore } from './stores/useAppStore';
import { useWebSocket } from './hooks/useWebSocket';
import { useMockData } from './hooks/useMockData';


const App: React.FC = () => {
  const view = useAppStore((state) => state.view);
  const theme = useAppStore((state) => state.theme);
  
  // Initialize WebSocket connection
  useWebSocket();
  
  // Load mock data for demo
  useMockData();

  // Apply theme class on mount and theme change
  useEffect(() => {
    document.documentElement.setAttribute('data-theme', theme.toLowerCase());
  }, [theme]);


  return (
    <div className="h-screen w-screen bg-bg-primary text-text-primary overflow-hidden flex flex-col md:pb-0 pb-[64px]">

      <Header />
      
      <div className="flex-1 flex overflow-hidden">
        <Sidebar />
        
        <main className="flex-1 flex flex-col relative">
          <div className="flex-1 relative">
            <GlobeContainer />
          </div>
          
          <BottomPanel />
        </main>
        
        <RightPanel />
      </div>
    </div>
  );
};

export default App;
