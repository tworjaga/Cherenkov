'use client';

import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Sensor, Reading } from '@/types/models';
import { Activity, TrendingUp, AlertTriangle, Clock } from 'lucide-react';

interface SensorMetricsProps {
  sensor: Sensor;
}

export function SensorMetrics({ sensor }: SensorMetricsProps) {
  const lastReading = sensor.lastReading;

  const getQualityColor = (quality: string) => {
    switch (quality) {
      case 'good':
        return 'text-green-500';
      case 'suspect':
        return 'text-yellow-500';
      case 'bad':
        return 'text-red-500';
      default:
        return 'text-gray-500';
    }
  };

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp).toLocaleString();
  };

  return (
    <div className="grid grid-cols-2 gap-3 p-4">
      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">Current Dose Rate</CardTitle>
          <Activity className="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">
            {lastReading ? `${lastReading.doseRate.toFixed(3)} ${lastReading.unit}` : 'N/A'}
          </div>
          <p className="text-xs text-muted-foreground">
            {lastReading ? `Quality: ${lastReading.qualityFlag}` : 'No data available'}
          </p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">Uncertainty</CardTitle>
          <TrendingUp className="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">
            {lastReading?.uncertainty ? `±${lastReading.uncertainty.toFixed(2)}%` : 'N/A'}
          </div>
          <p className="text-xs text-muted-foreground">
            Measurement confidence
          </p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">Data Quality</CardTitle>
          <AlertTriangle className="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div className={`text-2xl font-bold ${getQualityColor(lastReading?.qualityFlag || '')}`}>
            {lastReading?.qualityFlag ? lastReading.qualityFlag.toUpperCase() : 'N/A'}
          </div>
          <p className="text-xs text-muted-foreground">
            Last reading status
          </p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">Last Update</CardTitle>
          <Clock className="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div className="text-lg font-bold">
            {lastReading ? formatTimestamp(lastReading.timestamp) : 'N/A'}
          </div>
          <p className="text-xs text-muted-foreground">
            Timestamp of last reading
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
