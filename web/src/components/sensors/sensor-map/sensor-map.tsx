'use client';

import React from 'react';
import { Sensor } from '@/types/models';
import { Globe } from '@/components/globe/globe';

interface SensorMapProps {
  sensors: Sensor[];
  selectedSensorId?: string;
  onSensorSelect?: (sensor: Sensor) => void;
}

export function SensorMap({ sensors, selectedSensorId, onSensorSelect }: SensorMapProps) {
  const handleSensorSelect = (sensorId: string) => {
    const sensor = sensors.find(s => s.id === sensorId);
    if (sensor && onSensorSelect) {
      onSensorSelect(sensor);
    }
  };

  return (
    <div className="h-full w-full">
      <Globe
        sensors={sensors}
        selectedSensorId={selectedSensorId ?? null}
        onSensorSelect={handleSensorSelect}
        layers={{
          sensors: true,
          facilities: false,
          anomalies: false,
          plumes: false,
          heatmap: false,
        }}
      />
    </div>
  );
}
