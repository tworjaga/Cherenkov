'use client';

import { useDataStore } from '@/stores';
import { Card } from '@/components/ui/card';
import { Activity, AlertTriangle, Radio, Building2 } from 'lucide-react';
import { formatNumber } from '@/lib/utils/formatters';

export const StatCards = () => {
  const { sensors, facilities, alerts } = useDataStore();

  const activeSensors = sensors.filter(s => s.status === 'active').length;
  const totalFacilities = facilities.length;
  const criticalAlerts = alerts.filter(a => a.severity === 'critical').length;
  const avgDoseRate = sensors.length > 0
    ? sensors.reduce((sum, s) => sum + (s.lastReading?.doseRate || 0), 0) / sensors.length
    : 0;

  const stats = [
    {
      label: 'Active Sensors',
      value: formatNumber(activeSensors),
      total: sensors.length,
      icon: Radio,
      color: 'text-accent-primary',
    },
    {
      label: 'Facilities',
      value: formatNumber(totalFacilities),
      icon: Building2,
      color: 'text-accent-secondary',
    },
    {
      label: 'Critical Alerts',
      value: formatNumber(criticalAlerts),
      icon: AlertTriangle,
      color: criticalAlerts > 0 ? 'text-severity-critical' : 'text-text-secondary',
    },
    {
      label: 'Avg Dose Rate',
      value: `${avgDoseRate.toFixed(2)} μSv/h`,
      icon: Activity,
      color: 'text-accent-tertiary',
    },
  ];

  return (
    <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
      {stats.map((stat) => (
        <Card key={stat.label} className="p-4">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-body-xs text-text-secondary uppercase tracking-wider">
                {stat.label}
              </p>
              <p className="text-heading-lg text-text-primary mt-1">
                {stat.value}
              </p>
              {stat.total !== undefined && (
                <p className="text-mono-xs text-text-tertiary">
                  of {stat.total} total
                </p>
              )}
            </div>
            <stat.icon className={`w-8 h-8 ${stat.color}`} />
          </div>
        </Card>
      ))}
    </div>
  );
};
