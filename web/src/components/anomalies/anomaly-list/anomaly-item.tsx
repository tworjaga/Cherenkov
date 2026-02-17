'use client';

import React from 'react';
import { Anomaly } from '@/types/models';
import { Badge } from '@/components/ui/badge';
import { Card } from '@/components/ui/card';

interface AnomalyItemProps {
  anomaly: Anomaly;
  isSelected?: boolean;
  onClick?: () => void;
}

export function AnomalyItem({ anomaly, isSelected, onClick }: AnomalyItemProps) {
  const severityColors = {
    low: 'bg-yellow-500',
    medium: 'bg-orange-500',
    high: 'bg-red-500',
    critical: 'bg-red-700',
  };

  const severityLabels = {
    low: 'Low',
    medium: 'Medium',
    high: 'High',
    critical: 'Critical',
  };

  return (
    <Card
      className={`cursor-pointer p-4 transition-all hover:shadow-md ${
        isSelected ? 'ring-2 ring-primary' : ''
      }`}
      onClick={onClick}
    >
      <div className="flex items-start justify-between gap-2">
        <div className="flex-1 min-w-0">
          <h4 className="font-medium truncate">{anomaly.type}</h4>
          <p className="text-sm text-muted-foreground mt-1 line-clamp-2">
            {anomaly.description}
          </p>
        </div>
        <div className={`h-3 w-3 rounded-full flex-shrink-0 ${severityColors[anomaly.severity]}`} />
      </div>
      
      <div className="flex items-center justify-between mt-3 text-xs text-muted-foreground">
        <span>{new Date(anomaly.detectedAt).toLocaleString()}</span>
        <Badge variant="outline" className="text-xs">
          {severityLabels[anomaly.severity]}
        </Badge>
      </div>
    </Card>
  );
}
