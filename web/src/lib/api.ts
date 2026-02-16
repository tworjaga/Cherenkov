import { useState, useEffect, useCallback, useRef } from 'react';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/graphql';
const WS_URL = import.meta.env.VITE_WS_URL || 'ws://localhost:8080/ws';

// Rate limiting configuration
const RATE_LIMIT = {
  maxRequests: 100,
  windowMs: 60000, // 1 minute
};

interface CacheEntry<T> {
  data: T;
  timestamp: number;
  ttl: number;
}

class APICache {
  private cache = new Map<string, CacheEntry<unknown>>();

  get<T>(key: string): T | null {
    const entry = this.cache.get(key);
    if (!entry) return null;

    if (Date.now() - entry.timestamp > entry.ttl) {
      this.cache.delete(key);
      return null;
    }

    return entry.data as T;
  }

  set<T>(key: string, data: T, ttl: number = 30000): void {
    this.cache.set(key, {
      data,
      timestamp: Date.now(),
      ttl,
    });
  }

  invalidate(pattern?: string): void {
    if (!pattern) {
      this.cache.clear();
      return;
    }

    for (const key of this.cache.keys()) {
      if (key.includes(pattern)) {
        this.cache.delete(key);
      }
    }
  }
}

export const apiCache = new APICache();

// Rate limiter
class RateLimiter {
  private requests: number[] = [];

  canMakeRequest(): boolean {
    const now = Date.now();
    this.requests = this.requests.filter(
      (time) => now - time < RATE_LIMIT.windowMs
    );

    if (this.requests.length >= RATE_LIMIT.maxRequests) {
      return false;
    }

    this.requests.push(now);
    return true;
  }

  getRetryAfter(): number {
    if (this.requests.length === 0) return 0;
    const oldestRequest = this.requests[0];
    return Math.max(0, RATE_LIMIT.windowMs - (Date.now() - oldestRequest));
  }
}

export const rateLimiter = new RateLimiter();

// Auth token management
class AuthManager {
  private token: string | null = null;
  private refreshToken: string | null = null;
  private listeners: Set<(token: string | null) => void> = new Set();

  getToken(): string | null {
    if (!this.token) {
      this.token = localStorage.getItem('cherenkov_token');
    }
    return this.token;
  }

  setToken(token: string, refresh?: string): void {
    this.token = token;
    localStorage.setItem('cherenkov_token', token);
    if (refresh) {
      this.refreshToken = refresh;
      localStorage.setItem('cherenkov_refresh_token', refresh);
    }
    this.notifyListeners();
  }

  clearToken(): void {
    this.token = null;
    this.refreshToken = null;
    localStorage.removeItem('cherenkov_token');
    localStorage.removeItem('cherenkov_refresh_token');
    this.notifyListeners();
  }

  onTokenChange(callback: (token: string | null) => void): () => void {
    this.listeners.add(callback);
    return () => this.listeners.delete(callback);
  }

  private notifyListeners(): void {
    this.listeners.forEach((cb) => cb(this.token));
  }

  async refresh(): Promise<boolean> {
    if (!this.refreshToken) return false;

    try {
      const response = await fetch(`${API_URL}/refresh`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ refreshToken: this.refreshToken }),
      });

      if (response.ok) {
        const data = await response.json();
        this.setToken(data.token, data.refreshToken);
        return true;
      }
    } catch (error) {
      console.error('Token refresh failed:', error);
    }

    this.clearToken();
    return false;
  }
}

export const authManager = new AuthManager();

// GraphQL request helper
export async function graphqlRequest<T>(
  query: string,
  variables?: Record<string, unknown>,
  options: { useCache?: boolean; cacheTTL?: number; skipAuth?: boolean } = {}
): Promise<T> {
  // Rate limiting
  if (!rateLimiter.canMakeRequest()) {
    const retryAfter = rateLimiter.getRetryAfter();
    throw new Error(`Rate limit exceeded. Retry after ${retryAfter}ms`);
  }

  // Check cache for queries (not mutations)
  const cacheKey = `${query}:${JSON.stringify(variables)}`;
  if (options.useCache && !query.trim().startsWith('mutation')) {
    const cached = apiCache.get<T>(cacheKey);
    if (cached) return cached;
  }

  const token = options.skipAuth ? null : authManager.getToken();

  const response = await fetch(API_URL, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Accept': 'application/json',
      ...(token && { Authorization: `Bearer ${token}` }),
    },
    body: JSON.stringify({ query, variables }),
  });

  if (response.status === 401) {
    // Try to refresh token
    const refreshed = await authManager.refresh();
    if (refreshed) {
      // Retry request
      return graphqlRequest<T>(query, variables, options);
    }
    throw new Error('Authentication required');
  }

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  const result = await response.json();

  if (result.errors) {
    throw new Error(result.errors.map((e: { message: string }) => e.message).join(', '));
  }

  // Cache successful query results
  if (options.useCache && !query.trim().startsWith('mutation')) {
    apiCache.set(cacheKey, result.data, options.cacheTTL);
  }

  return result.data;
}

// React hook for GraphQL queries
export function useGraphQLQuery<T>(
  query: string,
  variables?: Record<string, unknown>,
  options: { useCache?: boolean; cacheTTL?: number; pollInterval?: number } = {}
) {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const execute = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const result = await graphqlRequest<T>(query, variables, {
        useCache: options.useCache,
        cacheTTL: options.cacheTTL,
      });
      setData(result);
    } catch (err) {
      setError(err as Error);
    } finally {
      setLoading(false);
    }
  }, [query, JSON.stringify(variables), options.useCache, options.cacheTTL]);

  useEffect(() => {
    execute();

    if (options.pollInterval) {
      const interval = setInterval(execute, options.pollInterval);
      return () => clearInterval(interval);
    }
  }, [execute, options.pollInterval]);

  return { data, loading, error, refetch: execute };
}

// React hook for GraphQL mutations
export function useGraphQLMutation<T>() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const execute = useCallback(
    async (mutation: string, variables?: Record<string, unknown>) => {
      setLoading(true);
      setError(null);

      try {
        const result = await graphqlRequest<T>(mutation, variables);
        // Invalidate cache after mutations
        apiCache.invalidate();
        return result;
      } catch (err) {
        setError(err as Error);
        throw err;
      } finally {
        setLoading(false);
      }
    },
    []
  );

  return { execute, loading, error };
}

// Historical data queries
export const QUERIES = {
  SENSOR_HISTORY: `
    query GetSensorHistory($sensorId: ID!, $from: DateTime!, $to: DateTime!) {
      sensorHistory(sensorId: $sensorId, from: $from, to: $to) {
        timestamp
        doseRate
        unit
        status
      }
    }
  `,

  GLOBAL_STATS: `
    query GetGlobalStats($from: DateTime!, $to: DateTime!) {
      globalStats(from: $from, to: $to) {
        avgDoseRate
        maxDoseRate
        activeSensors
        totalReadings
        alertsCount
      }
    }
  `,

  ANOMALIES: `
    query GetAnomalies($from: DateTime!, $limit: Int) {
      anomalies(from: $from, limit: $limit) {
        id
        sensorId
        timestamp
        severity
        zScore
        doseRate
        baseline
        location {
          lat
          lon
        }
      }
    }
  `,

  FACILITIES: `
    query GetFacilities {
      facilities {
        id
        name
        location {
          lat
          lon
        }
        type
        status
        readings {
          doseRate
          timestamp
        }
      }
    }
  `,
};

// Mutations
export const MUTATIONS = {
  ACKNOWLEDGE_ALERT: `
    mutation AcknowledgeAlert($alertId: ID!) {
      acknowledgeAlert(alertId: $alertId) {
        id
        acknowledged
        acknowledgedAt
      }
    }
  `,

  UPDATE_SENSOR_STATUS: `
    mutation UpdateSensorStatus($sensorId: ID!, $status: SensorStatus!) {
      updateSensorStatus(sensorId: $sensorId, status: $status) {
        id
        status
        updatedAt
      }
    }
  `,
};
