import { useEffect, useRef, useCallback } from 'react';
import { useAppStore, Alert, GlobalStatus, Sensor } from '../stores/useAppStore';

const WS_URL = import.meta.env.VITE_WS_URL || 'wss://api.cherenkov.io/ws';

export const useWebSocket = () => {
  const wsRef = useRef<WebSocket | null>(null);
  const reconnectTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const pingIntervalRef = useRef<ReturnType<typeof setInterval> | null>(null);
  
  const setConnection = useAppStore((state) => state.setConnection);
  const setLastPing = useAppStore((state) => state.setLastPing);
  const addAlert = useAppStore((state) => state.addAlert);
  const setGlobalStatus = useAppStore((state) => state.setGlobalStatus);
  const setSensors = useAppStore((state) => state.setSensors);


  const connect = useCallback(() => {
    if (wsRef.current?.readyState === WebSocket.OPEN) return;

    setConnection('CONNECTING');

    try {
      const ws = new WebSocket(WS_URL);
      wsRef.current = ws;

      ws.onopen = () => {
        setConnection('CONNECTED');
        setLastPing(new Date());
        
        // Subscribe to channels
        ws.send(JSON.stringify({
          type: 'SUBSCRIBE',
          channels: ['readings', 'alerts', 'status']
        }));

        // Start ping interval
        pingIntervalRef.current = setInterval(() => {
          if (ws.readyState === WebSocket.OPEN) {
            ws.send(JSON.stringify({ type: 'PING' }));
          }
        }, 30000);
      };

      ws.onmessage = (event) => {
        try {
          const message = JSON.parse(event.data);
          
          switch (message.type) {
            case 'READING': {
              // Update sensor data
              const sensor: Sensor = {
                id: message.sensorId,
                location: { lat: message.lat, lon: message.lon },
                doseRate: message.value,
                unit: 'μSv/h',
                lastReading: new Date(message.timestamp),
                status: 'active',
              };
              setSensors({ [message.sensorId]: sensor });
              break;
            }
            
            case 'ALERT': {
              const alert: Alert = {
                id: message.alert.id,
                severity: message.alert.severity,
                type: message.alert.type,
                title: message.alert.title,
                description: message.alert.description,
                location: message.alert.location,
                timestamp: new Date(message.alert.timestamp),
                acknowledged: false,
                sensorId: message.alert.sensorId,
                facilityId: message.alert.facilityId,
              };
              addAlert(alert);
              break;
            }
            
            case 'STATUS_UPDATE': {
              const status: GlobalStatus = {
                level: message.status.level,
                defcon: message.status.defcon,
                lastUpdate: new Date(message.status.lastUpdate),
                activeAlerts: message.status.activeAlerts,
              };
              setGlobalStatus(status);
              break;
            }
            
            case 'PONG': {
              setLastPing(new Date());
              break;
            }
          }
        } catch (error) {
          console.error('Failed to parse WebSocket message:', error);
        }
      };

      ws.onerror = () => {
        setConnection('ERROR');
      };

      ws.onclose = () => {
        setConnection('DISCONNECTED');
        if (pingIntervalRef.current) {
          clearInterval(pingIntervalRef.current);
        }
        
        // Reconnect after 5 seconds
        reconnectTimeoutRef.current = setTimeout(() => {
          connect();
        }, 5000);
      };

    } catch (error) {
      setConnection('ERROR');
      console.error('WebSocket connection failed:', error);
    }
  }, [setConnection, setLastPing, addAlert, setGlobalStatus, setSensors]);

  const disconnect = useCallback(() => {
    if (reconnectTimeoutRef.current) {
      clearTimeout(reconnectTimeoutRef.current);
    }
    if (pingIntervalRef.current) {
      clearInterval(pingIntervalRef.current);
    }
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
    }
  }, []);

  const sendMessage = useCallback((message: unknown) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(message));
    }
  }, []);

  useEffect(() => {
    connect();
    return disconnect;
  }, [connect, disconnect]);

  return {
    sendMessage,
    isConnected: useAppStore((state) => state.connection === 'CONNECTED'),
  };
};
