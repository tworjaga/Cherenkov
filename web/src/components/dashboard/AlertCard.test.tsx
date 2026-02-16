import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import { AlertCard } from './AlertCard';

// Mock the store module
vi.mock('../../stores/useAppStore', () => ({
  useAppStore: vi.fn(() => vi.fn()),
}));

describe('AlertCard', () => {
  const mockAlert = {
    id: '1',
    severity: 'CRITICAL' as const,
    type: 'ANOMALY' as const,
    title: 'Critical Alert',
    description: 'High radiation detected',
    location: { lat: 51.5074, lon: -0.1278 },
    timestamp: new Date('2024-01-15T10:30:00Z'),
    acknowledged: false,
    sensorId: 'sensor-1',
  };

  it('renders alert title and description', () => {
    render(<AlertCard alert={mockAlert} />);
    
    expect(screen.getByText('Critical Alert')).toBeInTheDocument();
    expect(screen.getByText('High radiation detected')).toBeInTheDocument();
  });

  it('renders with correct severity styling - critical', () => {
    render(<AlertCard alert={mockAlert} />);
    
    const card = screen.getByRole('listitem');
    expect(card).toHaveClass('border-alert-critical');
  });

  it('renders with correct severity styling - high', () => {
    const highAlert = { ...mockAlert, severity: 'HIGH' as const };
    render(<AlertCard alert={highAlert} />);
    
    const card = screen.getByRole('listitem');
    expect(card).toHaveClass('border-alert-high');
  });

  it('renders with correct severity styling - medium', () => {
    const mediumAlert = { ...mockAlert, severity: 'MEDIUM' as const };
    render(<AlertCard alert={mediumAlert} />);
    
    const card = screen.getByRole('listitem');
    expect(card).toHaveClass('border-alert-medium');
  });

  it('renders with correct severity styling - low', () => {
    const lowAlert = { ...mockAlert, severity: 'LOW' as const };
    render(<AlertCard alert={lowAlert} />);
    
    const card = screen.getByRole('listitem');
    expect(card).toHaveClass('border-alert-low');
  });

  it('does not show acknowledge button for acknowledged alerts', () => {
    const acknowledgedAlert = { ...mockAlert, acknowledged: true };
    render(<AlertCard alert={acknowledgedAlert} />);
    
    expect(screen.queryByLabelText('Acknowledge alert')).not.toBeInTheDocument();
  });

  it('shows acknowledge button for unacknowledged alerts', () => {
    render(<AlertCard alert={mockAlert} />);
    
    expect(screen.getByLabelText('Acknowledge alert')).toBeInTheDocument();
  });

  it('renders timestamp', () => {
    render(<AlertCard alert={mockAlert} />);
    
    expect(screen.getByText(/ago/)).toBeInTheDocument();
  });

  it('renders location coordinates', () => {
    render(<AlertCard alert={mockAlert} />);
    
    expect(screen.getByText(/51.5074/)).toBeInTheDocument();
    expect(screen.getByText(/-0.1278/)).toBeInTheDocument();
  });

  it('has correct ARIA attributes', () => {
    render(<AlertCard alert={mockAlert} />);
    
    const card = screen.getByRole('listitem');
    expect(card).toHaveAttribute('aria-label', 'CRITICAL alert: Critical Alert');
  });

  it('applies selected styling when isSelected is true', () => {
    render(<AlertCard alert={mockAlert} isSelected={true} />);
    
    const card = screen.getByRole('listitem');
    expect(card).toHaveClass('ring-2', 'ring-accent-primary');
  });

  it('renders sensor ID when present', () => {
    render(<AlertCard alert={mockAlert} />);
    
    expect(screen.getByText('sensor-1')).toBeInTheDocument();
  });

  it('renders facility ID when sensor ID is not present', () => {
    const facilityAlert = { ...mockAlert, sensorId: undefined, facilityId: 'facility-1' };
    render(<AlertCard alert={facilityAlert} />);
    
    expect(screen.getByText('facility-1')).toBeInTheDocument();
  });
});
