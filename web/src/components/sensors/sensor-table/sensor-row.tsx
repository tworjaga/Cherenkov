'use client';

import React from 'react';
import { Sensor } from '@/types/models';
import { Badge } from '@/components/ui/badge';
import { getRelativeTime } from '@/lib/utils/dates';


interface SensorRowProps {
  sensor: Sensor;
  isSelected?: boolean;
  onClick?: () => void;
}

export function SensorRow({ sensor, isSelected, onClick }: SensorRowProps) {
  const statusColors = {
    online: 'bg-green-500',
    offline: 'bg-red-500',
    maintenance: 'bg-yellow-500',
    error: 'bg-red-600',
  };

  return (
    <tr
      className={`cursor-pointer border-b transition-colors hover:bg-muted/50 ${
        isSelected ? 'bg-muted' : ''
      }`}
      onClick={onClick}
    >
      <td className="px-4 py-3 font-medium">{sensor.name}</td>
      <td className="px-4 py-3 text-muted-foreground">
        {`${sensor.location.lat.toFixed(4)}, ${sensor.location.lon.toFixed(4)}`}
      </td>
      <td className="px-4 py-3">
        <Badge variant="outline">{sensor.source}</Badge>
      </td>


      <td className="px-4 py-3">
        <div className="flex items-center gap-2">
          <div className={`h-2 w-2 rounded-full ${statusColors[sensor.status as keyof typeof statusColors] || 'bg-gray-500'}`} />
          <span className="capitalize">{sensor.status}</span>
        </div>
      </td>
      <td className="px-4 py-3 font-mono text-sm">
        {sensor.lastReading?.doseRate !== undefined 
          ? `${sensor.lastReading.doseRate} ${sensor.lastReading.unit || ''}`
          : 'N/A'}
      </td>
      <td className="px-4 py-3 text-xs text-muted-foreground">
        {sensor.lastReading?.timestamp 
          ? getRelativeTime(sensor.lastReading.timestamp)
          : 'Never'}
      </td>

    </tr>
  );
}
