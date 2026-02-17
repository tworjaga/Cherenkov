'use client';

import { useDataStore } from '@/stores';
import { Card } from '@/components/ui/card';
import { Activity, Radio, AlertTriangle, CheckCircle } from 'lucide-react';

export const SensorOverview = () => {
  const { sensors } = useDataStore();

  const activeSensors = sensors.filter((s) => s.status === 'active').length;
  const inactiveSensors = sensors.filter((s) => s.status === 'inactive').length;
  const maintenanceSensors = sensors.filter((s) => s.status === 'maintenance').length;


  return (
    <Card className="p-4">
      <div className="flex items-center gap-2 mb-4">
        <Activity className="w-5 h-5 text-accent-primary" />
        <h2 className="text-heading-sm text-text-primary">Sensor Overview</h2>
      </div>

      <div className="grid grid-cols-3 gap-4">
        <div className="text-center p-3 rounded-lg bg-bg-secondary">
          <div className="flex items-center justify-center gap-2 mb-1">
            <Radio className="w-4 h-4 text-status-online" />
            <span className="text-mono-lg font-semibold text-text-primary">
              {activeSensors}
            </span>
          </div>
          <span className="text-body-xs text-text-secondary">Active</span>
        </div>

        <div className="text-center p-3 rounded-lg bg-bg-secondary">
          <div className="flex items-center justify-center gap-2 mb-1">
            <AlertTriangle className="w-4 h-4 text-severity-high" />
            <span className="text-mono-lg font-semibold text-text-primary">
              {maintenanceSensors}
            </span>
          </div>
          <span className="text-body-xs text-text-secondary">Maintenance</span>
        </div>


        <div className="text-center p-3 rounded-lg bg-bg-secondary">
          <div className="flex items-center justify-center gap-2 mb-1">
            <CheckCircle className="w-4 h-4 text-text-tertiary" />
            <span className="text-mono-lg font-semibold text-text-primary">
              {inactiveSensors}
            </span>
          </div>
          <span className="text-body-xs text-text-secondary">Inactive</span>
        </div>
      </div>

      <div className="mt-4 pt-4 border-t border-border-subtle">
        <div className="flex items-center justify-between text-body-sm">
          <span className="text-text-secondary">Total Sensors</span>
          <span className="text-mono-md font-medium text-text-primary">
            {sensors.length}
          </span>
        </div>
      </div>
    </Card>
  );
};
