'use client';

import { motion } from 'framer-motion';
import { Radio, Activity } from 'lucide-react';
import { Sensor } from '@/types';
import { formatDoseRate } from '@/lib/utils';

interface SensorListProps {
  sensors: Sensor[];
  selectedId: string | null;
  onSelect: (sensor: Sensor) => void;
}

export const SensorList = ({ sensors, selectedId, onSelect }: SensorListProps) => {
  const sortedSensors = [...sensors].sort((a, b) => {
    const aDose = a.lastReading?.doseRate || 0;
    const bDose = b.lastReading?.doseRate || 0;
    return bDose - aDose;
  });

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between p-3 border-b border-border-subtle">
        <span className="text-heading-xs text-text-secondary">SENSORS</span>
        <span className="text-body-xs text-text-tertiary">
          {sensors.length} active
        </span>
      </div>
      
      <div className="flex-1 overflow-y-auto">
        {sortedSensors.map((sensor) => (
          <motion.button
            key={sensor.id}
            onClick={() => onSelect(sensor)}
            className={`w-full p-3 text-left border-b border-border-subtle transition-colors ${
              selectedId === sensor.id ? 'bg-bg-active' : 'hover:bg-bg-hover'
            }`}
            whileHover={{ x: 2 }}
          >
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Radio size={14} className="text-accent-primary" />
                <span className="text-body-sm font-medium text-text-primary">
                  {sensor.name}
                </span>
              </div>
              <div className="flex items-center gap-1">
                <Activity size={12} className={
                  sensor.status === 'active' ? 'text-alert-normal' : 'text-alert-medium'
                } />
                <span className="text-mono-xs text-text-secondary uppercase">
                  {sensor.status}
                </span>
              </div>
            </div>
            
            {sensor.lastReading && (
              <div className="mt-1 flex items-center justify-between">
                <span className="text-mono-sm text-accent-primary">
                  {formatDoseRate(sensor.lastReading.doseRate)}
                </span>
                <span className="text-body-xs text-text-tertiary">
                  {sensor.source}
                </span>
              </div>
            )}
          </motion.button>
        ))}
      </div>
    </div>
  );
};
