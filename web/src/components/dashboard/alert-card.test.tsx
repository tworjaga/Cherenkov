import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { AlertCard } from './alert-card';

describe('AlertCard', () => {
  const mockAlert = {
    id: 'alert-1',
    type: 'anomaly' as const,
    severity: 'critical' as const,
    message: 'Radiation spike detected',
    timestamp: Date.now(),
    location: {
      lat: 35.6762,
      lon: 139.6503,
    },
    metadata: {
      sensorId: 'sensor-1',
      doseRate: 2.5,
      baseline: 0.15,
      zScore: 15.6,
    },
    acknowledged: false,
  };

  it('renders alert information', () => {
    render(
      <AlertCard
        alert={mockAlert}
        onAcknowledge={() => {}}
      />
    );

    expect(screen.getByText('Radiation spike detected')).toBeInTheDocument();
    expect(screen.getByText(/sensor-1/)).toBeInTheDocument();
  });

  it('calls onAcknowledge when acknowledge button clicked', () => {
    const handleAcknowledge = vi.fn();
    render(
      <AlertCard
        alert={mockAlert}
        onAcknowledge={handleAcknowledge}
      />
    );

    const button = screen.getByText('Acknowledge');
    fireEvent.click(button);
    expect(handleAcknowledge).toHaveBeenCalledWith('alert-1');
  });

  it('applies correct severity styling', () => {
    const { container } = render(
      <AlertCard
        alert={mockAlert}
        onAcknowledge={() => {}}
      />
    );

    const card = container.querySelector('.border-l-4');
    expect(card?.className).toContain('border-l-alert-critical');
  });

  it('renders acknowledged state', () => {
    const acknowledgedAlert = { ...mockAlert, acknowledged: true, acknowledgedAt: Date.now() };
    render(
      <AlertCard
        alert={acknowledgedAlert}
        onAcknowledge={() => {}}
      />
    );

    expect(screen.getByText('Acknowledged')).toBeInTheDocument();
  });
});
