'use client';

import { RegionalStat } from '@/types';
import { Card, CardHeader, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { formatDoseRate } from '@/lib/utils';
import { Globe, AlertTriangle } from 'lucide-react';

interface RegionalStatsProps {
  regions: RegionalStat[];
  className?: string;
}

export const RegionalStats = ({ regions, className }: RegionalStatsProps) => {
  const totalSensors = regions.reduce((sum, r) => sum + r.sensorCount, 0);
  const totalAlerts = regions.reduce((sum, r) => sum + r.alertCount, 0);
  const avgDose = regions.reduce((sum, r) => sum + r.averageDose, 0) / (regions.length || 1);

  const getStatusVariant = (alertCount: number) => {
    if (alertCount > 5) return 'danger';
    if (alertCount > 0) return 'warning';
    return 'success';
  };

  return (
    <Card className={className}>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Globe className="w-5 h-5 text-accent-primary" />
            <h3 className="text-body-lg font-semibold text-text-primary">Regional Statistics</h3>
          </div>
          <div className="flex items-center gap-2">
            <Badge variant="default">{regions.length} regions</Badge>
          </div>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="grid grid-cols-3 gap-4">
          <div className="space-y-1">
            <span className="text-body-xs text-text-tertiary">Total Sensors</span>
            <p className="text-mono-lg text-text-primary">{totalSensors}</p>
          </div>
          <div className="space-y-1">
            <span className="text-body-xs text-text-tertiary">Alerts</span>
            <p className="text-mono-lg text-alert-warning">{totalAlerts}</p>
          </div>
          <div className="space-y-1">
            <span className="text-body-xs text-text-tertiary">Avg Dose</span>
            <p className="text-mono-lg text-accent-primary">{formatDoseRate(avgDose)}</p>
          </div>
        </div>

        <div className="space-y-3 pt-2">
          {regions.map((region) => (
            <div key={region.region} className="space-y-2">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <span className="text-body-sm font-medium text-text-primary">{region.region}</span>
                  <Badge variant={getStatusVariant(region.alertCount)}>
                    {region.alertCount > 0 ? `${region.alertCount} alerts` : 'normal'}
                  </Badge>
                </div>
                <span className="text-mono-sm text-text-secondary">
                  {region.sensorCount} sensors
                </span>
              </div>
              <div className="flex items-center gap-2">
                <Progress 
                  value={Math.min(region.averageDose / 10, 100)} 
                  className="flex-1 h-2"
                />
                <span className="text-mono-xs text-text-tertiary w-16 text-right">
                  {formatDoseRate(region.averageDose)}
                </span>
              </div>
              {region.alertCount > 0 && (
                <div className="flex items-center gap-1 text-alert-warning">
                  <AlertTriangle className="w-3 h-3" />
                  <span className="text-body-xs">{region.alertCount} alerts in region</span>
                </div>
              )}
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  );
};
