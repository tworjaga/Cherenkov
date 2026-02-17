'use client';

import { motion } from 'framer-motion';
import { useDataStore, useAppStore } from '@/stores';
import { animations } from '@/styles/theme';
import { formatTimestamp, getSeverityColor } from '@/lib/utils';
import { ScrollArea } from '@/components/ui/scroll-area';
import { AlertTriangle, Wind, Activity } from 'lucide-react';


export const RecentEventsPanel = () => {
  const { alerts, anomalies } = useDataStore();
  const { setView } = useAppStore();


  // Combine alerts and anomalies into events
  const events = [
    ...alerts.slice(0, 10).map((alert) => ({
      id: alert.id,
      type: alert.type,
      severity: alert.severity,
      message: alert.message,
      timestamp: alert.timestamp,
      location: alert.location,
      icon: alert.type === 'anomaly' ? AlertTriangle : alert.type === 'facility' ? Wind : Activity,
    })),
    ...anomalies.slice(0, 5).map((anomaly) => ({
      id: anomaly.id,
      type: 'anomaly' as const,
      severity: anomaly.severity,
      message: anomaly.message,
      timestamp: anomaly.detectedAt,
      location: anomaly.location,
      icon: AlertTriangle,
    })),
  ].sort((a, b) => b.timestamp - a.timestamp).slice(0, 10);

  const handleEventClick = (event: typeof events[0]) => {
    if (event.location) {
      // Center globe on event location
      // This would trigger a fly-to animation
    }
    if (event.type === 'anomaly') {
      setView('sensors');
    }

  };

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={animations.slideIn.transition}
      className="h-full flex flex-col"
    >
      <div className="flex items-center justify-between mb-3">
        <h3 className="text-heading-xs">Recent Events</h3>
        <span className="text-body-xs text-text-tertiary">
          {events.length} events
        </span>
      </div>

      <ScrollArea className="flex-1">
        <div className="space-y-2 pr-2">
          {events.map((event) => {
            const Icon = event.icon;
            const severityColor = getSeverityColor(event.severity);

            return (
              <motion.button
                key={event.id}
                onClick={() => handleEventClick(event)}
                className="w-full text-left p-2 bg-bg-tertiary hover:bg-bg-hover rounded transition-colors"
                whileHover={{ scale: 1.01 }}
                whileTap={{ scale: 0.99 }}
              >
                <div className="flex items-start gap-2">
                  <Icon
                    size={14}
                    className="mt-0.5 shrink-0"
                    style={{ color: severityColor }}
                  />
                  <div className="flex-1 min-w-0">
                    <p className="text-body-xs text-text-primary truncate">
                      {event.message}
                    </p>
                    <div className="flex items-center gap-2 mt-1">
                      <span
                        className="text-[10px] uppercase tracking-wider px-1.5 py-0.5 rounded"
                        style={{
                          backgroundColor: `${severityColor}20`,
                          color: severityColor,
                        }}
                      >
                        {event.severity}
                      </span>
                      <span className="text-body-xs text-text-tertiary">
                        {formatTimestamp(event.timestamp)}
                      </span>
                    </div>
                  </div>
                </div>
              </motion.button>
            );
          })}
          {events.length === 0 && (
            <div className="flex items-center justify-center h-32 text-text-tertiary text-body-xs">
              No recent events
            </div>
          )}
        </div>
      </ScrollArea>
    </motion.div>
  );
};
