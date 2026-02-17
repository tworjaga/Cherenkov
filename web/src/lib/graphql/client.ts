import { GraphQLClient } from 'graphql-request';
import { createClient, Client } from 'graphql-ws';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/graphql';
const WS_URL = process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:8080/ws';

export const graphqlClient = new GraphQLClient(API_URL, {
  headers: (): Record<string, string> => {
    const token = typeof window !== 'undefined' ? localStorage.getItem('token') : null;
    return token ? { Authorization: `Bearer ${token}` } : {};
  },
});


let wsClient: Client | null = null;

export const getWsClient = (): Client => {
  if (!wsClient) {
    wsClient = createClient({
      url: WS_URL,
      connectionParams: () => {
        const token = typeof window !== 'undefined' ? localStorage.getItem('token') : null;
        return token ? { Authorization: `Bearer ${token}` } : {};
      },
      retryAttempts: 5,
      shouldRetry: () => true,
    });
  }
  return wsClient;
};

export const closeWsClient = (): void => {
  if (wsClient) {
    wsClient.dispose();
    wsClient = null;
  }
};
