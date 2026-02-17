'use client';

import { useState } from 'react';
import { Bell } from 'lucide-react';
import { AlertList, AlertFilters } from '@/components/dashboard/alert-feed';
import { Alert } from '@/types/models';
import { ScrollArea } from '@/components/ui/scroll-area';

interface AlertFeedPanelProps {
  alerts: Alert[];
  onAcknowledge: (alertId: string) => void;
  onSelectAlert: (alert: Alert) => void;
  selectedAlertId?: string | null;
}

type SeverityFilter = 'all' | 'critical' | 'high' | 'medium' | 'low';

export function AlertFeedPanel({
  alerts,
  onAcknowledge,
  onSelectAlert,
  selectedAlertId,
}: AlertFeedPanelProps) {
  const [activeFilter, setActiveFilter] = useState<SeverityFilter>('all');

  const filteredAlerts = alerts.filter((alert) => {
    if (activeFilter === 'all') return true;
    return alert.severity === activeFilter;
  });

  const counts = {
    all: alerts.length,
    critical: alerts.filter((a) => a.severity === 'critical').length,
    high: alerts.filter((a) => a.severity === 'high').length,
    medium: alerts.filter((a) => a.severity === 'medium').length,
    low: alerts.filter((a) => a.severity === 'low').length,
  };

  return (
    <div className="flex h-full flex-col">
      <div className="flex items-center justify-between border-b border-[#1f1f2e] p-3">
        <div className="flex items-center gap-2">
          <Bell size={16} className="text-[#00d4ff]" />
          <span className="text-sm font-medium text-white">Alert Feed</span>
        </div>
        <span className="text-xs text-[#606070]">
          {filteredAlerts.length} active
        </span>
      </div>

      <AlertFilters
        activeFilter={activeFilter}
        onFilterChange={setActiveFilter}
        counts={counts}
      />

      <ScrollArea className="flex-1 p-3">
        <AlertList
          alerts={filteredAlerts}
          onAcknowledge={onAcknowledge}
          onSelect={onSelectAlert}
          selectedAlertId={selectedAlertId}
        />
      </ScrollArea>
    </div>
  );
}
