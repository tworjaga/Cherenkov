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
  return (
    <div className="h-full w-full">
      <Globe
        sensors={sensors}
        selectedSensorId={selectedSensorId ?? null}
        onSensorClick={onSensorSelect}
        showFacilities={false}
        showAnomalies={false}
      />

    </div>
  );
}
