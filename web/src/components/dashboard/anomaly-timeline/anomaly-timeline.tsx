'use client';

import React from 'react';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { getRelativeTime } from '@/lib/utils/dates';
import { Anomaly } from '@/types/models';

interface AnomalyTimelineProps {
  anomalies: Anomaly[];
  maxItems?: number;
  onAnomalyClick?: (anomaly: Anomaly) => void;
}

export function AnomalyTimeline({ 
  anomalies, 
  maxItems = 10,
  onAnomalyClick 
}: AnomalyTimelineProps) {
  const sortedAnomalies = [...anomalies]
    .sort((a, b) => b.detectedAt - a.detectedAt)
    .slice(0, maxItems);

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case 'critical':
        return 'bg-red-500';
      case 'high':
        return 'bg-orange-500';
      case 'medium':
        return 'bg-yellow-500';
      case 'low':
        return 'bg-blue-500';
      default:
        return 'bg-gray-500';
    }
  };

  const getSeverityBadgeVariant = (severity: string): 'default' | 'primary' | 'success' | 'warning' | 'danger' | 'outline' => {
    switch (severity) {
      case 'critical':
        return 'danger';
      case 'high':
        return 'warning';
      case 'medium':
        return 'primary';
      case 'low':
        return 'outline';
      default:
        return 'default';
    }
  };

  return (
    <Card className="h-full">
      <CardHeader>
        <CardTitle>Anomaly Timeline</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="relative">
          {/* Timeline line */}
          <div className="absolute left-4 top-0 bottom-0 w-0.5 bg-border" />

          <div className="space-y-4">
            {sortedAnomalies.map((anomaly) => (
              <div
                key={anomaly.id}
                className="relative flex items-start gap-4 pl-10 cursor-pointer hover:bg-muted/50 rounded-lg p-2 transition-colors"
                onClick={() => onAnomalyClick?.(anomaly)}
              >
                {/* Timeline dot */}
                <div
                  className={`absolute left-2 top-4 w-4 h-4 rounded-full border-2 border-background ${getSeverityColor(anomaly.severity)}`}
                />

                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2 mb-1">
                    <Badge variant={getSeverityBadgeVariant(anomaly.severity)}>
                      {anomaly.severity}
                    </Badge>
                    <span className="text-xs text-muted-foreground">
                      {getRelativeTime(anomaly.detectedAt)}
                    </span>
                  </div>
                  
                  <p className="text-sm font-medium truncate">
                    {anomaly.message || `Anomaly detected at ${anomaly.sensorId}`}
                  </p>
                  
                  <div className="flex items-center gap-4 mt-1 text-xs text-muted-foreground">
                    <span>Sensor: {anomaly.sensorId}</span>
                    <span>Dose: {anomaly.doseRate.toFixed(2)} μSv/h</span>
                    <span>Baseline: {anomaly.baseline.toFixed(2)}</span>
                  </div>

                  {anomaly.acknowledged && (
                    <div className="mt-1 text-xs text-green-600">
                      Acknowledged
                    </div>
                  )}
                </div>
              </div>
            ))}

            {sortedAnomalies.length === 0 && (
              <div className="text-center py-8 text-muted-foreground">
                No anomalies detected in the selected time range
              </div>
            )}
          </div>
        </div>
      </CardContent>
    </Card>
  );
}

export default AnomalyTimeline;
