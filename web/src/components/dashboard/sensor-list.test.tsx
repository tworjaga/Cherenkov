import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { SensorList } from './sensor-list';
import type { Sensor } from '@/types/models';

const mockSensors: Sensor[] = [
  {
    id: 'sensor-1',
    name: 'Tokyo Sensor',
    location: { lat: 35.6762, lon: 139.6503 },
    status: 'active',
    source: 'safecast',
    lastReading: {
      timestamp: Date.now(),
      doseRate: 0.15,
      unit: 'uSv/h',
    },
  },
  {
    id: 'sensor-2',
    name: 'Fukushima Sensor',
    location: { lat: 37.4215, lon: 141.0328 },
    status: 'active',
    source: 'uRadMonitor',
    lastReading: {
      timestamp: Date.now(),
      doseRate: 0.45,
      unit: 'uSv/h',
    },
  },
];

describe('SensorList', () => {
  const defaultProps = {
    sensors: mockSensors,
    selectedId: null as string | null,
    onSelect: vi.fn(),
  };

  it('renders sensor list', () => {
    render(<SensorList {...defaultProps} />);
    expect(screen.getByText(/Tokyo Sensor/i)).toBeInTheDocument();
    expect(screen.getByText(/Fukushima Sensor/i)).toBeInTheDocument();
  });

  it('displays sensor readings', () => {
    render(<SensorList {...defaultProps} />);
    expect(screen.getByText('0.150 μSv/h')).toBeInTheDocument();
    expect(screen.getByText('0.450 μSv/h')).toBeInTheDocument();
  });


  it('shows sensor sources', () => {
    render(<SensorList {...defaultProps} />);
    expect(screen.getByText(/safecast/i)).toBeInTheDocument();
    expect(screen.getByText(/uRadMonitor/i)).toBeInTheDocument();
  });

  it('calls onSelect when sensor clicked', () => {
    render(<SensorList {...defaultProps} />);
    fireEvent.click(screen.getByText(/Tokyo Sensor/i));
    expect(defaultProps.onSelect).toHaveBeenCalledWith(mockSensors[0]);
  });

  it('shows selected state for selected sensor', () => {
    render(<SensorList {...defaultProps} selectedId="sensor-1" />);
    const tokyoButton = screen.getByText(/Tokyo Sensor/i).closest('button');
    expect(tokyoButton).toHaveClass('bg-bg-active');
  });



});
