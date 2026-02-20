import { useEffect, useCallback } from 'react';
import NetInfo from '@react-native-community/netinfo';
import AsyncStorage from '@react-native-async-storage/async-storage';
import { useDataStore } from '@stores';

const OFFLINE_QUEUE_KEY = '@offline_queue';

export const useOfflineSync = () => {
  const { setLastSync, setError } = useDataStore();

  const queueAction = useCallback(async (action: object) => {
    try {
      const existing = await AsyncStorage.getItem(OFFLINE_QUEUE_KEY);
      const queue = existing ? JSON.parse(existing) : [];
      queue.push({ ...action, timestamp: Date.now() });
      await AsyncStorage.setItem(OFFLINE_QUEUE_KEY, JSON.stringify(queue));
    } catch (err) {
      console.error('Failed to queue action:', err);
    }
  }, []);

  const syncPendingActions = useCallback(async () => {
    try {
      const existing = await AsyncStorage.getItem(OFFLINE_QUEUE_KEY);
      if (!existing) return;

      const queue = JSON.parse(existing);
      if (queue.length === 0) return;

      // Process queue
      const failed: typeof queue = [];
      for (const action of queue) {
        try {
          // Attempt to sync each action
          console.log('Syncing action:', action);
        } catch (err) {
          failed.push(action);
        }
      }

      // Save failed actions back to queue
      await AsyncStorage.setItem(OFFLINE_QUEUE_KEY, JSON.stringify(failed));
      setLastSync(new Date());
    } catch (err) {
      setError('Sync failed');
    }
  }, [setLastSync, setError]);

  useEffect(() => {
    const unsubscribe = NetInfo.addEventListener(state => {
      if (state.isConnected) {
        syncPendingActions();
      }
    });

    // Initial sync check
    NetInfo.fetch().then(state => {
      if (state.isConnected) {
        syncPendingActions();
      }
    });

    return () => unsubscribe();
  }, [syncPendingActions]);

  return {
    queueAction,
    syncPendingActions,
  };
};
