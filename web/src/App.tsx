import React from 'react';
import { Header } from './components/layout/Header';
import { Sidebar } from './components/layout/Sidebar';
import { RightPanel } from './components/layout/RightPanel';
import { BottomPanel } from './components/layout/BottomPanel';
import { GlobeContainer } from './components/globe/GlobeContainer';
import { useAppStore } from './stores/useAppStore';
import { useWebSocket } from './hooks/useWebSocket';

const App: React.FC = () => {
  const view = useAppStore((state) => state.view);
  
  // Initialize WebSocket connection
  useWebSocket();


  return (
    <div className="h-screen w-screen bg-bg-primary text-text-primary overflow-hidden flex flex-col">
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
