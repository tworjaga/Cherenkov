import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { ViewType, TimeMode, ConnectionStatus } from '@/types';

interface AppState {
  view: ViewType;
  sidebarCollapsed: boolean;
  rightPanelOpen: boolean;
  bottomPanelOpen: boolean;
  selectedSensorId: string | null;
  selectedFacilityId: string | null;
  hoveredSensorId: string | null;
  timeMode: TimeMode;
  currentTime: number;
  playbackSpeed: number;
  connectionStatus: ConnectionStatus;
  lastPing: number;

  setView: (view: ViewType) => void;
  toggleSidebar: () => void;
  toggleRightPanel: () => void;
  toggleBottomPanel: () => void;
  selectSensor: (id: string | null) => void;
  selectFacility: (id: string | null) => void;
  setHoveredSensor: (id: string | null) => void;
  setTimeMode: (mode: TimeMode) => void;
  setCurrentTime: (time: number) => void;
  setPlaybackSpeed: (speed: number) => void;
  setConnectionStatus: (status: ConnectionStatus) => void;
  updatePing: () => void;
  stepTime: (direction: 'forward' | 'backward') => void;
}

export const useAppStore = create<AppState>()(
  persist(
    (set, get) => ({
      view: 'globe',
      sidebarCollapsed: false,
      rightPanelOpen: true,
      bottomPanelOpen: true,
      selectedSensorId: null,
      selectedFacilityId: null,
      hoveredSensorId: null,
      timeMode: 'live',
      currentTime: Date.now(),
      playbackSpeed: 1,
      connectionStatus: 'disconnected',
      lastPing: 0,

      setView: (view) => set({ view }),
      
      toggleSidebar: () => set((state) => ({ 
        sidebarCollapsed: !state.sidebarCollapsed 
      })),
      
      toggleRightPanel: () => set((state) => ({ 
        rightPanelOpen: !state.rightPanelOpen 
      })),
      
      toggleBottomPanel: () => set((state) => ({ 
        bottomPanelOpen: !state.bottomPanelOpen 
      })),
      
      selectSensor: (id) => set({ 
        selectedSensorId: id,
        selectedFacilityId: null 
      }),
      
      selectFacility: (id) => set({ 
        selectedFacilityId: id,
        selectedSensorId: null 
      }),
      
      setHoveredSensor: (id) => set({ hoveredSensorId: id }),
      
      setTimeMode: (mode) => set({ timeMode: mode }),
      
      setCurrentTime: (time) => set({ currentTime: time }),
      
      setPlaybackSpeed: (speed) => set({ playbackSpeed: speed }),
      
      setConnectionStatus: (status) => set({ connectionStatus: status }),
      
      updatePing: () => set({ lastPing: Date.now() }),
      
      stepTime: (direction) => {
        const { currentTime, playbackSpeed, timeMode } = get();
        if (timeMode === 'live') return;
        
        const step = 5 * 60 * 1000 * playbackSpeed; // 5 minutes * speed
        const newTime = direction === 'forward' 
          ? currentTime + step 
          : currentTime - step;
        set({ currentTime: newTime });
      },
    }),
    {
      name: 'cherenkov-app-storage',
      partialize: (state) => ({
        view: state.view,
        sidebarCollapsed: state.sidebarCollapsed,
        rightPanelOpen: state.rightPanelOpen,
        bottomPanelOpen: state.bottomPanelOpen,
        playbackSpeed: state.playbackSpeed,
      }),
    }
  )
);
