'use client';

import { Anomaly } from '@/types';
import { Card, CardHeader, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { formatTimestamp } from '@/lib/utils';
import { AlertTriangle, Clock } from 'lucide-react';

interface AnomalyTimelineProps {
  anomalies: Anomaly[];
  onAnomalyClick?: (anomaly: Anomaly) => void;
  className?: string;
}

export const AnomalyTimeline = ({ anomalies, onAnomalyClick, className }: AnomalyTimelineProps) => {
  const getSeverityVariant = (severity: Anomaly['severity']) => {
    switch (severity) {
      case 'critical':
        return 'danger';
      case 'high':
        return 'warning';
      case 'medium':
        return 'default';
      case 'low':
        return 'success';
      default:
        return 'default';
    }
  };

  const sortedAnomalies = [...anomalies].sort((a, b) => b.detectedAt - a.detectedAt);

  return (
    <Card className={className}>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <AlertTriangle className="w-5 h-5 text-alert-warning" />
            <h3 className="text-body-lg font-semibold text-text-primary">Anomaly Timeline</h3>
          </div>
          <Badge variant="default">{anomalies.length} total</Badge>
        </div>
      </CardHeader>
      <CardContent>
        <div className="space-y-3 max-h-64 overflow-y-auto">
          {sortedAnomalies.map((anomaly) => (
            <div
              key={anomaly.id}
              onClick={() => onAnomalyClick?.(anomaly)}
              className={`p-3 rounded-lg border cursor-pointer transition-all hover:shadow-md ${
                anomaly.acknowledged 
                  ? 'border-border-subtle bg-bg-secondary opacity-60' 
                  : 'border-alert-warning bg-bg-secondary'
              }`}
            >
              <div className="flex items-start justify-between gap-2">
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <Badge variant={getSeverityVariant(anomaly.severity)}>{anomaly.severity}</Badge>
                    <span className="text-mono-xs text-text-tertiary">{anomaly.algorithm}</span>
                  </div>
                  <p className="mt-1 text-body-sm text-text-primary truncate">{anomaly.message}</p>
                  <div className="mt-2 flex items-center gap-4 text-text-tertiary">
                    <div className="flex items-center gap-1">
                      <Clock className="w-3 h-3" />
                      <span className="text-mono-xs">{formatTimestamp(anomaly.detectedAt)}</span>
                    </div>
                    <span className="text-mono-xs">Z-Score: {anomaly.zScore.toFixed(2)}</span>
                  </div>
                </div>
              </div>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  );
};
