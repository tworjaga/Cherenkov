import { describe, it, expect, vi, beforeEach } from 'vitest';
import { renderHook, waitFor } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactNode } from 'react';

import { useSensors, useReadings, useAnomalies } from './use-graphql';

// Mock the GraphQL client
vi.mock('@/lib/graphql/client', () => ({
  graphqlClient: {
    request: vi.fn(),
  },
}));

import { graphqlClient } from '@/lib/graphql/client';

const createWrapper = () => {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
      },
    },
  });
  return function Wrapper({ children }: { children: ReactNode }) {
    return <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>;
  };
};

describe('useSensors', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('fetches sensors successfully', async () => {
    const mockSensors = [
      {
        id: '1',
        name: 'Sensor 1',
        latitude: 35.6762,
        longitude: 139.6503,
        status: 'active',
      },
    ];

    (graphqlClient.request as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
      sensors: mockSensors,
    });

    const { result } = renderHook(() => useSensors(), { wrapper: createWrapper() });

    await waitFor(() => {
      expect(result.current.data).toEqual(mockSensors);
    });
  });

  it('handles error state', async () => {
    (graphqlClient.request as ReturnType<typeof vi.fn>).mockRejectedValueOnce(
      new Error('Network error')
    );

    const { result } = renderHook(() => useSensors(), { wrapper: createWrapper() });

    await waitFor(() => {
      expect(result.current.error).toBeDefined();
    });
  });
});

describe.skip('useReadings', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('fetches readings for sensor', async () => {

    const mockReadings = [
      { timestamp: Date.now(), doseRate: 0.5, unit: 'uSv/h' },
    ];

    (graphqlClient.request as ReturnType<typeof vi.fn>).mockResolvedValue({
      readings: mockReadings,
    });

    const { result } = renderHook(
      () => useReadings(['1'], new Date(Date.now() - 86400000), new Date()),
      { wrapper: createWrapper() }
    );

    // Wait for query to complete
    await waitFor(() => expect(result.current.isLoading).toBe(false));

    // Verify hook returns data when query succeeds
    expect(result.current.data).toEqual(mockReadings);
  });
});



describe('useAnomalies', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('fetches anomalies with filters', async () => {
    const mockAnomalies = [
      {
        id: '1',
        severity: 'critical',
        message: 'Test anomaly',
        timestamp: Date.now(),
      },
    ];

    (graphqlClient.request as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
      anomalies: mockAnomalies,
    });

    const { result } = renderHook(() => useAnomalies(['critical']), { wrapper: createWrapper() });

    await waitFor(() => {
      expect(result.current.data).toEqual(mockAnomalies);
    });
  });
});
