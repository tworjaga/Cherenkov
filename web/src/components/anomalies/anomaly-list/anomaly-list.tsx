'use client';

import React from 'react';
import { Anomaly } from '@/types/models';
import { AnomalyItem } from './anomaly-item';
import { ScrollArea } from '@/components/ui/scroll-area';

interface AnomalyListProps {
  anomalies: Anomaly[];
  selectedAnomalyId?: string;
  onAnomalySelect?: (anomaly: Anomaly) => void;
}

export function AnomalyList({ anomalies, selectedAnomalyId, onAnomalySelect }: AnomalyListProps) {
  const sortedAnomalies = [...anomalies].sort((a, b) => 
    new Date(b.detectedAt).getTime() - new Date(a.detectedAt).getTime()
  );

  return (
    <ScrollArea className="h-full">
      <div className="space-y-2 p-4">
        {sortedAnomalies.length === 0 ? (
          <p className="text-center text-muted-foreground py-8">No anomalies detected</p>
        ) : (
          sortedAnomalies.map((anomaly) => (
            <AnomalyItem
              key={anomaly.id}
              anomaly={anomaly}
              isSelected={anomaly.id === selectedAnomalyId}
              onClick={() => onAnomalySelect?.(anomaly)}
            />
          ))
        )}
      </div>
    </ScrollArea>
  );
}
