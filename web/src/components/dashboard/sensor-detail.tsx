'use client';

import { Radio, MapPin, Clock, TrendingUp } from 'lucide-react';
import { Sensor } from '@/types';
import { formatDoseRate, formatTimestamp, formatCoordinate } from '@/lib/utils';
import { ReadingChart } from './reading-chart';

interface SensorDetailProps {
  sensor: Sensor;
  onClose: () => void;
}

export const SensorDetail = ({ sensor, onClose }: SensorDetailProps) => {
  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between p-3 border-b border-border-subtle">
        <div className="flex items-center gap-2">
          <Radio size={16} className="text-accent-primary" />
          <span className="text-body-sm font-medium text-text-primary">
            {sensor.name}
          </span>
        </div>
        <button
          onClick={onClose}
          className="text-text-tertiary hover:text-text-primary transition-colors"
        >
          ×
        </button>
      </div>
      
      <div className="p-4 space-y-4">
        <div className="flex items-center justify-between p-3 bg-bg-tertiary rounded-lg">
          <span className="text-heading-xs text-text-secondary">CURRENT READING</span>
          <span className="text-display-md font-mono text-accent-primary">
            {sensor.lastReading ? formatDoseRate(sensor.lastReading.doseRate) : 'N/A'}
          </span>
        </div>
        
        <div className="space-y-2">
          <div className="flex items-center gap-2 text-body-sm text-text-secondary">
            <MapPin size={14} />
            <span>
              {formatCoordinate(sensor.location.lat, sensor.location.lon)}
            </span>
          </div>

          <div className="flex items-center gap-2 text-body-sm text-text-secondary">
            <Clock size={14} />
            <span>
              {sensor.lastReading ? formatTimestamp(sensor.lastReading.timestamp) : 'No data'}
            </span>
          </div>
          <div className="flex items-center gap-2 text-body-sm text-text-secondary">
            <TrendingUp size={14} />
            <span>Source: {sensor.source}</span>
          </div>
        </div>
        
        <div className="h-32">
          <ReadingChart sensorId={sensor.id} />
        </div>
      </div>
    </div>
  );
};
