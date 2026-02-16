import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { renderHook, waitFor } from '@testing-library/react';

// Mock the store with a factory function
const mockSetConnection = vi.fn();
const mockSetLastPing = vi.fn();
const mockAddAlert = vi.fn();
const mockSetGlobalStatus = vi.fn();
const mockSetSensors = vi.fn();

vi.mock('../stores/useAppStore', () => ({
  useAppStore: vi.fn((selector?: (state: unknown) => unknown) => {
    const mockState = {
      setConnection: mockSetConnection,
      setLastPing: mockSetLastPing,
      addAlert: mockAddAlert,
      setGlobalStatus: mockSetGlobalStatus,
      setSensors: mockSetSensors,
      connection: 'DISCONNECTED',
    };
    return selector ? selector(mockState as unknown) : mockState;
  }),
}));

describe('useWebSocket', () => {
  let mockWebSocket: ReturnType<typeof vi.fn>;
  let mockSend: ReturnType<typeof vi.fn>;
  let mockClose: ReturnType<typeof vi.fn>;
  let useWebSocket: typeof import('./useWebSocket').useWebSocket;

  beforeEach(async () => {
    vi.clearAllMocks();
    mockSend = vi.fn();
    mockClose = vi.fn();
    
    // Create mock WebSocket class
    const MockWebSocket = vi.fn().mockImplementation(() => ({
      send: mockSend,
      close: mockClose,
      readyState: WebSocket.CONNECTING,
      onopen: null as ((event: Event) => void) | null,
      onmessage: null as ((event: MessageEvent) => void) | null,
      onerror: null as ((event: Event) => void) | null,
      onclose: null as ((event: CloseEvent) => void) | null,
    }));
    
    mockWebSocket = MockWebSocket;
    
    // Set up global WebSocket mock before dynamic import
    window.WebSocket = MockWebSocket as unknown as typeof WebSocket;
    
    // Dynamically import to get fresh module with mock applied
    const module = await import('./useWebSocket');
    useWebSocket = module.useWebSocket;
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('should establish WebSocket connection on mount', async () => {
    renderHook(() => useWebSocket());
    
    expect(mockWebSocket).toHaveBeenCalled();
    expect(mockWebSocket.mock.calls[0][0]).toContain('ws');
  });




  it('should send subscription message on open', async () => {
    renderHook(() => useWebSocket());
    
    const wsInstance = mockWebSocket.mock.results[0]?.value;
    if (wsInstance && wsInstance.onopen) {
      wsInstance.readyState = WebSocket.OPEN;
      wsInstance.onopen(new Event('open'));
      
      await waitFor(() => {
        expect(mockSend).toHaveBeenCalledWith(
          JSON.stringify({
            type: 'SUBSCRIBE',
            channels: ['readings', 'alerts', 'status'],
          })
        );
      });
    }
  });

  it('should handle READING message', async () => {
    renderHook(() => useWebSocket());
    
    const wsInstance = mockWebSocket.mock.results[0]?.value;
    if (wsInstance && wsInstance.onmessage) {
      wsInstance.onmessage({
        data: JSON.stringify({
          type: 'READING',
          sensorId: 'sensor-1',
          lat: 51.5074,
          lon: -0.1278,
          value: 0.15,
          timestamp: new Date().toISOString(),
        }),
      } as MessageEvent);

      await waitFor(() => {
        expect(mockSetSensors).toHaveBeenCalled();
      });
    }
  });

  it('should handle ALERT message', async () => {
    renderHook(() => useWebSocket());
    
    const wsInstance = mockWebSocket.mock.results[0]?.value;
    if (wsInstance && wsInstance.onmessage) {
      wsInstance.onmessage({
        data: JSON.stringify({
          type: 'ALERT',
          alert: {
            id: 'alert-1',
            severity: 'HIGH',
            type: 'ANOMALY',
            title: 'High Radiation Detected',
            description: 'Radiation levels elevated',
            location: { lat: 51.5074, lon: -0.1278 },
            timestamp: new Date().toISOString(),
          },
        }),
      } as MessageEvent);

      await waitFor(() => {
        expect(mockAddAlert).toHaveBeenCalled();
      });
    }
  });

  it('should handle STATUS_UPDATE message', async () => {
    renderHook(() => useWebSocket());
    
    const wsInstance = mockWebSocket.mock.results[0]?.value;
    if (wsInstance && wsInstance.onmessage) {
      wsInstance.onmessage({
        data: JSON.stringify({
          type: 'STATUS_UPDATE',
          status: {
            level: 'ELEVATED',
            defcon: 4,
            lastUpdate: new Date().toISOString(),
            activeAlerts: 5,
          },
        }),
      } as MessageEvent);

      await waitFor(() => {
        expect(mockSetGlobalStatus).toHaveBeenCalled();
      });
    }
  });

  it('should handle PONG message', async () => {
    renderHook(() => useWebSocket());
    
    const wsInstance = mockWebSocket.mock.results[0]?.value;
    if (wsInstance && wsInstance.onmessage) {
      wsInstance.onmessage({
        data: JSON.stringify({
          type: 'PONG',
          timestamp: new Date().toISOString(),
        }),
      } as MessageEvent);

      await waitFor(() => {
        expect(mockSetLastPing).toHaveBeenCalled();
      });
    }
  });

  it('should handle connection error', async () => {
    renderHook(() => useWebSocket());
    
    const wsInstance = mockWebSocket.mock.results[0]?.value;
    if (wsInstance && wsInstance.onerror) {
      wsInstance.onerror(new Event('error'));
      
      await waitFor(() => {
        expect(mockSetConnection).toHaveBeenCalledWith('ERROR');
      });
    }
  });

  it('should handle connection close', async () => {
    renderHook(() => useWebSocket());
    
    const wsInstance = mockWebSocket.mock.results[0]?.value;
    if (wsInstance && wsInstance.onclose) {
      wsInstance.onclose(new CloseEvent('close'));
      
      await waitFor(() => {
        expect(mockSetConnection).toHaveBeenCalledWith('DISCONNECTED');
      });
    }
  });

  it('should send ping messages periodically', async () => {
    vi.useFakeTimers();
    
    renderHook(() => useWebSocket());
    
    const wsInstance = mockWebSocket.mock.results[0]?.value;
    if (wsInstance && wsInstance.onopen) {
      wsInstance.readyState = WebSocket.OPEN;
      wsInstance.onopen(new Event('open'));
      
      vi.advanceTimersByTime(30000);
      
      expect(mockSend).toHaveBeenCalledWith(
        JSON.stringify({ type: 'PING' })
      );
    }
  });

  it('should reconnect after connection loss', async () => {
    vi.useFakeTimers();
    
    renderHook(() => useWebSocket());
    
    const wsInstance = mockWebSocket.mock.results[0]?.value;
    if (wsInstance && wsInstance.onclose) {
      wsInstance.onclose(new CloseEvent('close'));
      
      vi.advanceTimersByTime(5000);
      
      expect(mockWebSocket).toHaveBeenCalledTimes(2);
    }
  });

  it('should not send messages when disconnected', () => {
    const { result } = renderHook(() => useWebSocket());
    
    // Try to send when not connected (ws is in CONNECTING state)
    // The hook's sendMessage should check readyState before calling send
    result.current.sendMessage({ type: 'TEST' });
    
    // send should not be called because connection is not OPEN
    expect(mockSend).not.toHaveBeenCalled();
  });



  it('should return correct connection status', () => {
    const { result } = renderHook(() => useWebSocket());
    
    // Default state is DISCONNECTED
    expect(result.current.isConnected).toBe(false);
  });
});
