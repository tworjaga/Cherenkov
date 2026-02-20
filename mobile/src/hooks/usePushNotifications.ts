import { useEffect, useCallback } from 'react';
import messaging from '@react-native-firebase/messaging';
import { useDataStore } from '@stores';
import type { Alert } from '@types';

export const usePushNotifications = () => {
  const { addAlert } = useDataStore();

  const requestPermission = useCallback(async () => {
    const authStatus = await messaging().requestPermission();
    const enabled =
      authStatus === messaging.AuthorizationStatus.AUTHORIZED ||
      authStatus === messaging.AuthorizationStatus.PROVISIONAL;

    return enabled;
  }, []);

  const getToken = useCallback(async () => {
    return await messaging().getToken();
  }, []);

  useEffect(() => {
    // Request permission on mount
    requestPermission();

    // Foreground message handler
    const unsubscribe = messaging().onMessage(async remoteMessage => {
      console.log('Foreground message received:', remoteMessage);
      
      if (remoteMessage.data?.type === 'ALERT') {
        const alert: Alert = {
          id: remoteMessage.data.alertId || Date.now().toString(),
          title: remoteMessage.notification?.title || 'New Alert',
          message: remoteMessage.notification?.body || '',
          severity: (remoteMessage.data.severity as Alert['severity']) || 'medium',
          timestamp: new Date().toISOString(),
          acknowledged: false,
          sensorId: remoteMessage.data.sensorId,
          facilityId: remoteMessage.data.facilityId,
        };
        addAlert(alert);
      }
    });

    // Background handler
    messaging().setBackgroundMessageHandler(async remoteMessage => {
      console.log('Background message received:', remoteMessage);
    });

    return unsubscribe;
  }, [addAlert, requestPermission]);

  return {
    requestPermission,
    getToken,
  };
};
