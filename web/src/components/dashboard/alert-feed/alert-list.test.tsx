import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { AlertList } from './alert-list';

const mockAlerts = [
  {
    id: '1',
    type: 'anomaly' as const,
    severity: 'critical' as const,
    message: 'Critical anomaly detected',
    timestamp: Date.now(),
    acknowledged: false,
  },
  {
    id: '2',
    type: 'system' as const,
    severity: 'medium' as const,
    message: 'System warning',
    timestamp: Date.now() - 1000,
    acknowledged: true,
  },
];

describe('AlertList', () => {
  it('renders list of alerts', () => {
    render(
      <AlertList
        alerts={mockAlerts}
        selectedAlertId={null}
        onSelect={vi.fn()}
        onAcknowledge={vi.fn()}
      />
    );

    expect(screen.getByText('Critical anomaly detected')).toBeInTheDocument();
    expect(screen.getByText('System warning')).toBeInTheDocument();
  });

  it('calls onSelect when alert is clicked', () => {
    const onSelect = vi.fn();
    render(
      <AlertList
        alerts={mockAlerts}
        selectedAlertId={null}
        onSelect={onSelect}
        onAcknowledge={vi.fn()}
      />
    );

    fireEvent.click(screen.getByText('Critical anomaly detected'));
    expect(onSelect).toHaveBeenCalledWith(mockAlerts[0]);
  });

  it('calls onAcknowledge when acknowledge button is clicked', () => {
    const onAcknowledge = vi.fn();
    render(
      <AlertList
        alerts={mockAlerts}
        selectedAlertId={null}
        onSelect={vi.fn()}
        onAcknowledge={onAcknowledge}
      />
    );

    const acknowledgeButtons = screen.getAllByRole('button', { name: /acknowledge/i });
    fireEvent.click(acknowledgeButtons[0]);
    expect(onAcknowledge).toHaveBeenCalledWith('1');
  });

  it('shows empty state when no alerts', () => {
    render(
      <AlertList
        alerts={[]}
        selectedAlertId={null}
        onSelect={vi.fn()}
        onAcknowledge={vi.fn()}
      />
    );

    expect(screen.getByText('No active alerts')).toBeInTheDocument();
  });
});
