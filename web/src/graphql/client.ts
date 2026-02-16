import { createSignal, createResource, createEffect } from 'solid-js';
import type { Resource, ResourceActions } from 'solid-js';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/graphql';
const WS_URL = import.meta.env.VITE_WS_URL || 'ws://localhost:8080/graphql';

export interface GraphQLResponse<T> {
  data?: T;
  errors?: Array<{ message: string; path?: string[] }>;
}

export interface Sensor {
  id: string;
  name: string;
  location: {
    lat: number;
    lon: number;
  };
  doseRate: number;
  unit: string;
  lastReading: string;
  status: 'active' | 'inactive' | 'alert';
  source: string;
}

export interface Anomaly {
  id: string;
  sensorId: string;
  timestamp: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  zScore: number;
  doseRate: number;
  baseline: number;
  location: {
    lat: number;
    lon: number;
  };
  acknowledged: boolean;
}

export interface PlumeSimulation {
  id: string;
  releaseLocation: {
    lat: number;
    lon: number;
  };
  isotope: string;
  releaseRate: number;
  duration: number;
  grid: Array<{
    lat: number;
    lon: number;
    concentration: number;
  }>;
  timestamp: string;
}

export interface Alert {
  id: string;
  type: 'anomaly' | 'facility' | 'seismic' | 'system';
  severity: 'info' | 'warning' | 'critical';
  message: string;
  timestamp: string;
  acknowledged: boolean;
  metadata?: Record<string, unknown>;
}

class GraphQLClient {
  private url: string;
  private wsUrl: string;
  private ws: WebSocket | null = null;
  private subscriptions: Map<string, (data: unknown) => void> = new Map();
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;

  constructor(url: string, wsUrl: string) {
    this.url = url;
    this.wsUrl = wsUrl;
  }

  async query<T>(query: string, variables?: Record<string, unknown>): Promise<GraphQLResponse<T>> {
    const response = await fetch(this.url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      },
      body: JSON.stringify({ query, variables }),
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return response.json();
  }

  async mutation<T>(mutation: string, variables?: Record<string, unknown>): Promise<GraphQLResponse<T>> {
    return this.query<T>(mutation, variables);
  }

  subscribe<T>(subscription: string, variables: Record<string, unknown>, callback: (data: T) => void): () => void {
    const id = Math.random().toString(36).substring(7);
    this.subscriptions.set(id, callback as (data: unknown) => void);

    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      this.connect();
    }

    const message = {
      type: 'subscribe',
      id,
      payload: {
        query: subscription,
        variables,
      },
    };

    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message));
    }

    return () => {
      this.unsubscribe(id);
    };
  }

  private connect(): void {
    this.ws = new WebSocket(this.wsUrl);

    this.ws.onopen = () => {
      console.log('WebSocket connected');
      this.reconnectAttempts = 0;
      
      this.subscriptions.forEach((_, id) => {
        const message = {
          type: 'subscribe',
          id,
          payload: {},
        };
        this.ws?.send(JSON.stringify(message));
      });
    };

    this.ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      if (data.type === 'data' && data.id) {
        const callback = this.subscriptions.get(data.id);
        if (callback) {
          callback(data.payload);
        }
      }
    };

    this.ws.onclose = () => {
      if (this.reconnectAttempts < this.maxReconnectAttempts) {
        this.reconnectAttempts++;
        setTimeout(() => this.connect(), 1000 * this.reconnectAttempts);
      }
    };

    this.ws.onerror = (error) => {
      console.error('WebSocket error:', error);
    };
  }

  private unsubscribe(id: string): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({ type: 'unsubscribe', id }));
    }
    this.subscriptions.delete(id);
  }
}

export const graphqlClient = new GraphQLClient(API_URL, WS_URL);

export function createGraphQLQuery<T>(
  query: string,
  variables?: () => Record<string, unknown>
): Resource<T | undefined> & ResourceActions<T | undefined> {
  const [resource, actions] = createResource<T | undefined, Record<string, unknown>>(
    variables || (() => ({})),
    async (vars) => {
      const response = await graphqlClient.query<T>(query, vars);
      if (response.errors) {
        throw new Error(response.errors.map(e => e.message).join(', '));
      }
      return response.data;
    }
  );
  
  return Object.assign(resource, actions);
}

export function createGraphQLMutation<T>(): [
  (mutation: string, variables?: Record<string, unknown>) => Promise<T>,
  { loading: () => boolean; error: () => Error | undefined }
] {
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal<Error | undefined>(undefined);

  const execute = async (mutation: string, variables?: Record<string, unknown>): Promise<T> => {
    setLoading(true);
    setError(undefined);
    
    try {
      const response = await graphqlClient.mutation<T>(mutation, variables);
      if (response.errors) {
        throw new Error(response.errors.map(e => e.message).join(', '));
      }
      if (!response.data) {
        throw new Error('No data returned from mutation');
      }
      return response.data;
    } catch (err) {
      setError(err as Error);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  return [execute, { loading, error }];
}

export function createGraphQLSubscription<T>(
  subscription: string,
  variables: Record<string, unknown> = {}
): () => T | undefined {
  const [data, setData] = createSignal<T | undefined>(undefined);

  createEffect(() => {
    const unsubscribe = graphqlClient.subscribe<T>(subscription, variables, (newData) => {
      setData(() => newData);
    });

    return unsubscribe;
  });

  return data;
}
