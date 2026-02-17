import { describe, it, expect, vi } from 'vitest';
import { render } from '@testing-library/react';
import { AnomalyLayer } from './anomaly-layer';

// Mock the DataStore
vi.mock('@/stores', () => ({
  useDataStore: () => ({
    anomalies: [
      {
        id: 'anomaly-1',
        sensorId: 'sensor-1',
        severity: 'high',
        zScore: 3.5,
        detectedAt: new Date().toISOString(),
        message: 'High radiation detected',
        location: { lat: 35.6762, lon: 139.6503 },
      },
    ],
  }),
}));

describe('AnomalyLayer', () => {
  const defaultProps = {
    onAnomalyClick: vi.fn(),
  };

  it('renders without crashing', () => {
    const { container } = render(<AnomalyLayer {...defaultProps} />);
    expect(container).toBeInTheDocument();
  });

  it('renders without click handler', () => {
    const { container } = render(<AnomalyLayer />);
    expect(container).toBeInTheDocument();
  });
});
