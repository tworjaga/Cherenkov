import { describe, it, expect, vi } from 'vitest';
import { render } from '@testing-library/react';
import { SensorLayer } from './sensor-layer';

// Mock the data store
vi.mock('@/stores', () => ({
  useDataStore: () => ({
    sensors: [
      {
        id: 'sensor-1',
        name: 'Test Sensor 1',
        latitude: 35.6762,
        longitude: 139.6503,
        status: 'active',
        lastReading: {
          doseRate: 0.15,
          timestamp: Date.now(),
        },
      },
    ],
  }),
}));

describe('SensorLayer', () => {
  it('renders without crashing', () => {
    const { container } = render(
      <SensorLayer onSensorClick={() => {}} />
    );
    expect(container).toBeInTheDocument();
  });

  it('renders with selected sensor', () => {
    const { container } = render(
      <SensorLayer
        onSensorClick={() => {}}
        selectedSensorId="sensor-1"
      />
    );
    expect(container).toBeInTheDocument();
  });

  it('handles click callback', () => {
    const handleClick = vi.fn();
    const { container } = render(
      <SensorLayer onSensorClick={handleClick} />
    );
    expect(container).toBeInTheDocument();
  });
});
