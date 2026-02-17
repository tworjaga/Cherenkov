'use client';

import { Sensor } from '@/types';
import { Card, CardHeader, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { formatDoseRate, formatTimestamp } from '@/lib/utils';
import { Activity, MapPin, Database, Clock } from 'lucide-react';

interface SensorDetailProps {
  sensor: Sensor;
  className?: string;
}

export const SensorDetail = ({ sensor, className }: SensorDetailProps) => {
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
          <div className="flex items-center gap-2">
            <Activity className="w-5 h-5 text-accent-primary" />
            <h3 className="text-body-lg font-semibold text-text-primary">{sensor.name}</h3>
          </div>
          <Badge variant={getStatusVariant(sensor.status)}>{sensor.status}</Badge>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="grid grid-cols-2 gap-4">
          <div className="space-y-1">
            <div className="flex items-center gap-2 text-text-tertiary">
              <MapPin className="w-4 h-4" />
              <span className="text-body-xs">Location</span>
            </div>
            <p className="text-mono-sm text-text-primary">
              {sensor.location.lat.toFixed(6)}, {sensor.location.lon.toFixed(6)}
            </p>
          </div>
          <div className="space-y-1">
            <div className="flex items-center gap-2 text-text-tertiary">
              <Database className="w-4 h-4" />
              <span className="text-body-xs">Source</span>
            </div>
            <p className="text-body-sm text-text-primary">{sensor.source}</p>
          </div>
        </div>

        {sensor.lastReading && (
          <div className="pt-4 border-t border-border-subtle">
            <h4 className="text-body-sm font-medium text-text-primary mb-3">Last Reading</h4>
            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-1">
                <span className="text-body-xs text-text-tertiary">Dose Rate</span>
                <p className="text-mono-lg text-accent-primary">
                  {formatDoseRate(sensor.lastReading.doseRate)}
                </p>
                <span className="text-mono-xs text-text-tertiary">{sensor.lastReading.unit}</span>
              </div>
              <div className="space-y-1">
                <span className="text-body-xs text-text-tertiary">Quality</span>
                <Badge 
                  variant={sensor.lastReading.qualityFlag === 'good' ? 'success' : 
                          sensor.lastReading.qualityFlag === 'suspect' ? 'warning' : 'danger'}
                >
                  {sensor.lastReading.qualityFlag}
                </Badge>
              </div>
              <div className="space-y-1">
                <div className="flex items-center gap-2 text-text-tertiary">
                  <Clock className="w-4 h-4" />
                  <span className="text-body-xs">Timestamp</span>
                </div>
                <p className="text-mono-sm text-text-primary">
                  {formatTimestamp(sensor.lastReading.timestamp)}
                </p>
              </div>
              {sensor.lastReading.uncertainty && (
                <div className="space-y-1">
                  <span className="text-body-xs text-text-tertiary">Uncertainty</span>
                  <p className="text-mono-sm text-text-primary">
                    ±{sensor.lastReading.uncertainty.toFixed(2)}
                  </p>
                </div>
              )}
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
};
