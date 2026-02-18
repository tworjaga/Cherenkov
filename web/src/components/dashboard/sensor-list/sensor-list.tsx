'use client';

import { useState } from 'react';
import { Sensor } from '@/types';
import { Card, CardHeader, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { formatDoseRate, formatTimestamp } from '@/lib/utils';
import { Activity, MapPin, Clock } from 'lucide-react';

interface SensorListProps {
  sensors: Sensor[];
  onSensorClick?: (sensor: Sensor) => void;
  className?: string;
}

export const SensorList = ({ sensors, onSensorClick, className }: SensorListProps) => {
  const [selectedId, setSelectedId] = useState<string | null>(null);

  const handleClick = (sensor: Sensor) => {
    setSelectedId(sensor.id);
    onSensorClick?.(sensor);
  };

  const getStatusVariant = (status: Sensor['status']) => {
    switch (status) {
      case 'active':
        return 'success';
      case 'inactive':
        return 'warning';
      case 'maintenance':
        return 'default';
      case 'offline':
        return 'danger';
      default:
        return 'default';
    }
  };

  return (
    <Card className={className}>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <h3 className="text-body-lg font-semibold text-text-primary">Sensors</h3>
          <Badge variant="default">{sensors.length} total</Badge>
        </div>
      </CardHeader>
      <CardContent className="space-y-2">
        {sensors.map((sensor) => (
          <div
            key={sensor.id}
            data-testid={`sensor-item-${sensor.id}`}
            onClick={() => handleClick(sensor)}
            className={`p-3 rounded-lg border cursor-pointer transition-all hover:shadow-md ${
              selectedId === sensor.id
                ? 'border-accent-primary bg-accent-primary/5'
                : 'border-border-subtle bg-bg-secondary'
            }`}
          >

            <div className="flex items-start justify-between gap-2">
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  <Activity className="w-4 h-4 text-accent-primary" />
                  <span className="text-body-sm font-medium text-text-primary truncate">
                    {sensor.name}
                  </span>
                </div>
                <div className="mt-1 flex items-center gap-1 text-text-tertiary">
                  <MapPin className="w-3 h-3" />
                  <span className="text-mono-xs">
                    {sensor.location.lat.toFixed(4)}, {sensor.location.lon.toFixed(4)}
                  </span>
                </div>
              </div>
              <Badge variant={getStatusVariant(sensor.status)}>{sensor.status}</Badge>
            </div>
            {sensor.lastReading && (
              <div className="mt-2 pt-2 border-t border-border-subtle flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <span className="text-mono-sm text-accent-primary">
                    {formatDoseRate(sensor.lastReading.doseRate)}
                  </span>
                  <span className="text-mono-xs text-text-tertiary">
                    {sensor.lastReading.unit}
                  </span>
                </div>
                <div className="flex items-center gap-1 text-text-tertiary">
                  <Clock className="w-3 h-3" />
                  <span className="text-mono-xs">
                    {formatTimestamp(sensor.lastReading.timestamp)}
                  </span>
                </div>
              </div>
            )}
          </div>
        ))}
      </CardContent>
    </Card>
  );
};
