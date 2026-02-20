import React, { useEffect } from 'react';
import { StatusBar, useColorScheme } from 'react-native';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import { NavigationContainer } from '@react-navigation/native';
import { ApolloProvider } from '@apollo/client';

import { apolloClient } from '@services/graphql/client';
import { useAuthStore } from '@stores/auth-store';
import { usePushNotifications } from '@hooks/use-push-notifications';
import { useOfflineSync } from '@hooks/use-offline-sync';
import { RootNavigator } from '@navigation/root-navigator';
import { ThemeProvider } from '@components/theme-provider';

const App: React.FC = () => {
  const isDarkMode = useColorScheme() === 'dark';
  const { initializeAuth } = useAuthStore();

  usePushNotifications();
  useOfflineSync();

  useEffect(() => {
    initializeAuth();
  }, [initializeAuth]);

  return (
    <ApolloProvider client={apolloClient}>
      <SafeAreaProvider>
        <ThemeProvider isDarkMode={isDarkMode}>
          <StatusBar
            barStyle={isDarkMode ? 'light-content' : 'dark-content'}
            backgroundColor="transparent"
            translucent
          />
          <NavigationContainer>
            <RootNavigator />
          </NavigationContainer>
        </ThemeProvider>
      </SafeAreaProvider>
    </ApolloProvider>
  );
};

export default App;
