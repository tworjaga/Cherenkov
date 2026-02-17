import { describe, it, expect, beforeEach } from 'vitest';
import { useAppStore } from './app-store';

describe('useAppStore', () => {
  beforeEach(() => {
    // Reset store to initial state
    const store = useAppStore.getState();
    store.setView('dashboard');
    store.setTimeMode('live');
    store.setCurrentTime(Date.now());

    store.setPlaybackSpeed(1);
    store.selectSensor(null);
    store.setConnectionStatus('disconnected');
  });

  it('should initialize with default values', () => {
    const state = useAppStore.getState();
    expect(state.view).toBe('dashboard');
    expect(state.sidebarCollapsed).toBe(false);
    expect(state.rightPanelOpen).toBe(true);
    expect(state.bottomPanelOpen).toBe(true);
    expect(state.timeMode).toBe('live');
    expect(state.playbackSpeed).toBe(1);
    expect(state.connectionStatus).toBe('disconnected');
  });

  it('should set view', () => {
    const store = useAppStore.getState();
    store.setView('globe');
    expect(useAppStore.getState().view).toBe('globe');
  });

  it('should toggle sidebar', () => {
    const store = useAppStore.getState();
    const initial = store.sidebarCollapsed;
    store.toggleSidebar();
    expect(useAppStore.getState().sidebarCollapsed).toBe(!initial);
  });

  it('should set time mode', () => {
    const store = useAppStore.getState();
    store.setTimeMode('paused');
    expect(useAppStore.getState().timeMode).toBe('paused');
  });

  it('should set playback speed', () => {
    const store = useAppStore.getState();
    store.setPlaybackSpeed(2);
    expect(useAppStore.getState().playbackSpeed).toBe(2);
  });

  it('should select sensor', () => {
    const store = useAppStore.getState();
    store.selectSensor('sensor-123');
    expect(useAppStore.getState().selectedSensorId).toBe('sensor-123');
  });

  it('should set connection status', () => {
    const store = useAppStore.getState();
    store.setConnectionStatus('connected');
    expect(useAppStore.getState().connectionStatus).toBe('connected');
  });
});
