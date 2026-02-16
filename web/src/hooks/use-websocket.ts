import { useEffect, useRef, useCallback } from 'react';
import { getWsClient, closeWsClient } from '@/lib/graphql/client';
import { useAppStore } from '@/stores';
import { useDataStore } from '@/stores';

export const useWebSocket = () => {
  const { setConnectionStatus, updatePing } = useAppStore();
  const { addAnomaly, updateReading } = useDataStore();
  const unsubscribeRef = useRef<(() => void) | null>(null);

  const connect = useCallback(() => {
    setConnectionStatus('connecting');

    try {
      const client = getWsClient();

      const unsubscribe = client.subscribe(
        {
          query: `
            subscription {
              allSensorUpdates {
                sensorId
                timestamp
                doseRate
                latitude
                longitude
              }
            }
          `,
        },
        {
          next: (data) => {
            updatePing();
            setConnectionStatus('connected');

            if (data.data?.allSensorUpdates) {
              const update = data.data.allSensorUpdates;
              updateReading(update.sensorId, {
                timestamp: update.timestamp,
                doseRate: update.doseRate,
                unit: 'microsieverts_per_hour',
                qualityFlag: 'good',
              });
            }
          },
          error: (err) => {
            console.error('WebSocket error:', err);
            setConnectionStatus('disconnected');
          },
          complete: () => {
            setConnectionStatus('disconnected');
          },
        }
      );

      unsubscribeRef.current = unsubscribe;
    } catch (error) {
      console.error('Failed to connect WebSocket:', error);
      setConnectionStatus('disconnected');
    }
  }, [setConnectionStatus, updatePing, updateReading]);

  const disconnect = useCallback(() => {
    if (unsubscribeRef.current) {
      unsubscribeRef.current();
      unsubscribeRef.current = null;
    }
    closeWsClient();
    setConnectionStatus('disconnected');
  }, [setConnectionStatus]);

  useEffect(() => {
    connect();

    return () => {
      disconnect();
    };
  }, [connect, disconnect]);

  return { connect, disconnect };
};
