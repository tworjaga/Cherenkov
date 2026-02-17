'use client';

import React, { useState, useMemo } from 'react';
import { Sensor } from '@/types/models';
import { SensorRow } from './sensor-row';
import { SensorFilters } from './sensor-filters';

interface SensorTableProps {
  sensors: Sensor[];
  onSensorSelect?: (sensor: Sensor) => void;
  selectedSensorId?: string;
}

export function SensorTable({ sensors, onSensorSelect, selectedSensorId }: SensorTableProps) {
  const [filters, setFilters] = useState({
    status: 'all',
    type: 'all',
    search: '',
  });

  const filteredSensors = useMemo(() => {
    return sensors.filter((sensor) => {
      const matchesStatus = filters.status === 'all' || sensor.status === filters.status;
      const matchesType = filters.type === 'all' || sensor.type === filters.type;
      const locationString = `${sensor.location.lat}, ${sensor.location.lon}`;
      const matchesSearch = 
        filters.search === '' ||
        sensor.name.toLowerCase().includes(filters.search.toLowerCase()) ||
        locationString.toLowerCase().includes(filters.search.toLowerCase());

      return matchesStatus && matchesType && matchesSearch;
    });
  }, [sensors, filters]);

  return (
    <div className="flex flex-col h-full">
      <SensorFilters filters={filters} onFiltersChange={setFilters} />
      
      <div className="flex-1 overflow-auto">
        <table className="w-full text-sm">
          <thead className="bg-muted sticky top-0">
            <tr>
              <th className="px-4 py-2 text-left font-medium">Name</th>
              <th className="px-4 py-2 text-left font-medium">Location</th>
              <th className="px-4 py-2 text-left font-medium">Type</th>
              <th className="px-4 py-2 text-left font-medium">Status</th>
              <th className="px-4 py-2 text-left font-medium">Reading</th>
              <th className="px-4 py-2 text-left font-medium">Last Update</th>
            </tr>
          </thead>
          <tbody>
            {filteredSensors.map((sensor) => (
              <SensorRow
                key={sensor.id}
                sensor={sensor}
                isSelected={sensor.id === selectedSensorId}
                onClick={() => onSensorSelect?.(sensor)}
              />
            ))}
          </tbody>
        </table>
      </div>

      <div className="px-4 py-2 border-t text-xs text-muted-foreground">
        Showing {filteredSensors.length} of {sensors.length} sensors
      </div>
    </div>
  );
}
