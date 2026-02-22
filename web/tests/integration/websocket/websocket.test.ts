import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { createClient } from 'graphql-ws';
import WebSocket from 'ws';

const WS_URL = 'ws://localhost:8080/ws';

describe('WebSocket Integration', () => {
  let client: ReturnType<typeof createClient>;

  beforeAll(() => {
    client = createClient({
      url: WS_URL,
      webSocketImpl: WebSocket,
    });
  });

  afterAll(() => {
    client.dispose();
  });

  it('should establish WebSocket connection', async () => {
    const result = await new Promise((resolve, reject) => {
      const unsubscribe = client.subscribe(
        {
          query: 'subscription { allSensorUpdates { sensorId value unit timestamp } }',
        },
        {
          next: (data) => {
            unsubscribe();
            resolve(data);
          },
          error: reject,
          complete: () => resolve(null),
        }
      );

      setTimeout(() => {
        unsubscribe();
        reject(new Error('WebSocket timeout'));
      }, 5000);
    });

    const typedResult = result as { data: { allSensorUpdates: unknown } };
    expect(typedResult).toBeDefined();
    expect(typedResult).toHaveProperty('data');
    expect(typedResult.data).toHaveProperty('allSensorUpdates');

  });

  it('should receive sensor updates via subscription', async () => {
    const updates: unknown[] = [];

    const unsubscribe = client.subscribe(
      {
        query: 'subscription { allSensorUpdates { sensorId value unit timestamp } }',
      },
      {
        next: (data) => {
          updates.push(data);
        },
        error: (err) => {
          console.error('Subscription error:', err);
        },
        complete: () => {},
      }
    );

    await new Promise((resolve) => setTimeout(resolve, 3500));

    unsubscribe();

    expect(updates.length).toBeGreaterThan(0);
    const firstUpdate = updates[0] as { data: { allSensorUpdates: { sensorId: string; value: number; unit: string; timestamp: string } } };
    expect(firstUpdate.data.allSensorUpdates).toHaveProperty('sensorId');
    expect(firstUpdate.data.allSensorUpdates).toHaveProperty('value');
    expect(firstUpdate.data.allSensorUpdates).toHaveProperty('unit');
    expect(firstUpdate.data.allSensorUpdates).toHaveProperty('timestamp');
  });

  it('should handle connection errors gracefully', async () => {
    const badClient = createClient({
      url: 'ws://localhost:9999/ws',
      webSocketImpl: WebSocket,
      retryAttempts: 1,
    });

    const result = await new Promise((resolve) => {
      badClient.subscribe(
        {
          query: 'subscription { allSensorUpdates { sensorId } }',
        },
        {
          next: () => resolve('success'),
          error: () => resolve('error'),
          complete: () => resolve('complete'),
        }
      );

      setTimeout(() => resolve('timeout'), 2000);
    });

    expect(result).toBe('error');
    badClient.dispose();
  });
});
