'use client';

import React from 'react';
import { Anomaly } from '@/types/models';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';

interface AnomalyTimelineProps {
  anomalies: Anomaly[];
  onAnomalyClick?: (anomaly: Anomaly) => void;
}

export function AnomalyTimeline({ anomalies, onAnomalyClick }: AnomalyTimelineProps) {
  const sortedAnomalies = [...anomalies].sort((a, b) => 
    new Date(b.detectedAt).getTime() - new Date(a.detectedAt).getTime()
  );

  const severityColors = {
    low: 'bg-yellow-500',
    medium: 'bg-orange-500',
    high: 'bg-red-500',
    critical: 'bg-red-700',
  };

  return (
    <div className="relative">
      <div className="absolute left-4 top-0 bottom-0 w-0.5 bg-border" />
      
      <div className="space-y-4">
        {sortedAnomalies.map((anomaly, index) => (
          <div key={anomaly.id} className="relative flex items-start gap-4">
            <div className={`relative z-10 w-8 h-8 rounded-full border-2 border-background ${severityColors[anomaly.severity]} flex items-center justify-center`}>
              <span className="text-xs font-bold text-white">{sortedAnomalies.length - index}</span>
            </div>
            
            <Card 
              className="flex-1 p-4 cursor-pointer hover:shadow-md transition-shadow"
              onClick={() => onAnomalyClick?.(anomaly)}
            >
              <div className="flex items-start justify-between">
                <div>
                  <p className="font-medium">{anomaly.id}</p>
                  <p className="text-sm text-muted-foreground">
                    {new Date(anomaly.detectedAt).toLocaleString()}
                  </p>
                </div>
                <Badge variant="outline">{anomaly.severity}</Badge>
              </div>
            </Card>
          </div>
        ))}
      </div>
    </div>
  );
}
