'use client';

import { useDataStore } from '@/stores';
import { Card } from '@/components/ui/card';
import { AlertCard } from '@/components/dashboard/alert-card';
import { ScrollArea } from '@/components/ui/scroll-area';
import { AlertTriangle } from 'lucide-react';

export const AlertSummary = () => {
  const { alerts } = useDataStore();

  const recentAlerts = alerts.slice(0, 5);

  return (
    <Card className="h-full flex flex-col">
      <div className="flex items-center justify-between p-4 border-b border-border-subtle">
        <div className="flex items-center gap-2">
          <AlertTriangle className="w-5 h-5 text-severity-high" />
          <h2 className="text-heading-sm text-text-primary">Recent Alerts</h2>
        </div>
        <span className="text-mono-xs text-text-tertiary">
          {alerts.length} total
        </span>
      </div>
      
      <ScrollArea className="flex-1 p-4">
        {recentAlerts.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-32 text-text-tertiary">
            <AlertTriangle className="w-8 h-8 mb-2 opacity-50" />
            <p className="text-body-sm">No active alerts</p>
          </div>
        ) : (
          <div className="space-y-3">
            {recentAlerts.map((alert) => (
              <AlertCard key={alert.id} alert={alert} />
            ))}

          </div>
        )}
      </ScrollArea>
    </Card>
  );
};
