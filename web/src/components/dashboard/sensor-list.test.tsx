import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { SensorList } from './sensor-list';
import type { Sensor } from '@/types/models';

const mockSensors: Sensor[] = [
  {
    id: 'sensor-1',
    name: 'Tokyo Sensor',
    location: { lat: 35.6762, lon: 139.6503 },
    latitude: 35.6762,
    longitude: 139.6503,
    status: 'active',
    source: 'safecast',
    lastReading: {
      timestamp: Date.now(),
      doseRate: 0.15,
      unit: 'uSv/h',
      qualityFlag: 'good',
    },
  },
  {
    id: 'sensor-2',
    name: 'Fukushima Sensor',
    location: { lat: 37.4215, lon: 141.0328 },
    latitude: 37.4215,
    longitude: 141.0328,
    status: 'active',
    source: 'uRadMonitor',
    lastReading: {
      timestamp: Date.now(),
      doseRate: 0.45,
      unit: 'uSv/h',
      qualityFlag: 'good',
    },
  },
];

describe('SensorList', () => {
  const defaultProps = {
    sensors: mockSensors,
    onSensorClick: vi.fn(),
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





  it('calls onSensorClick when sensor clicked', () => {
    render(<SensorList {...defaultProps} />);
    fireEvent.click(screen.getByText(/Tokyo Sensor/i));
    expect(defaultProps.onSensorClick).toHaveBeenCalledWith(mockSensors[0]);
  });

  it('shows selected state when sensor is clicked', async () => {
    render(<SensorList {...defaultProps} />);
    const tokyoElement = screen.getByTestId('sensor-item-sensor-1');
    fireEvent.click(tokyoElement);
    // Re-query element after React re-render to get updated classes
    const updatedElement = screen.getByTestId('sensor-item-sensor-1');
    expect(updatedElement).toHaveClass('border-accent-primary');
    expect(updatedElement).toHaveClass('bg-accent-primary/5');
  });





});
