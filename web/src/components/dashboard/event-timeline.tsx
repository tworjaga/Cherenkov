'use client';

import { motion } from 'framer-motion';
import { History, AlertTriangle, Activity, Info } from 'lucide-react';
import { Alert } from '@/types';
import { getSeverityColor, formatTimestamp } from '@/lib/utils';

interface EventTimelineProps {
  events: Alert[];
  onSelect: (event: Alert) => void;
}

export const EventTimeline = ({ events, onSelect }: EventTimelineProps) => {
  const sortedEvents = [...events].sort((a, b) => b.timestamp - a.timestamp);

  const getEventIcon = (type: Alert['type']) => {
    switch (type) {
      case 'anomaly': return <AlertTriangle size={12} />;
      case 'system': return <Activity size={12} />;
      default: return <Info size={12} />;
    }
  };

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between p-3 border-b border-border-subtle">
        <div className="flex items-center gap-2">
          <History size={14} className="text-text-secondary" />
          <span className="text-heading-xs text-text-secondary">EVENTS</span>
        </div>
        <span className="text-body-xs text-text-tertiary">
          {events.length} total
        </span>
      </div>
      
      <div className="flex-1 overflow-y-auto p-2">
        {sortedEvents.length === 0 ? (
          <div className="flex items-center justify-center h-full text-text-tertiary text-body-xs">
            No events
          </div>
        ) : (
          <div className="space-y-1">
            {sortedEvents.map((event, index) => (
              <motion.button
                key={event.id}
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: index * 0.03 }}
                onClick={() => onSelect(event)}
                className="w-full p-2 text-left hover:bg-bg-hover rounded transition-colors flex items-center gap-2"
              >
                <div
                  className="w-6 h-6 rounded flex items-center justify-center flex-shrink-0"
                  style={{ backgroundColor: `${getSeverityColor(event.severity)}20` }}
                >
                  <span style={{ color: getSeverityColor(event.severity) }}>
                    {getEventIcon(event.type)}
                  </span>
                </div>
                
                <div className="flex-1 min-w-0">
                  <p className="text-body-xs text-text-primary truncate">
                    {event.message}
                  </p>
                  <p className="text-mono-xs text-text-tertiary">
                    {formatTimestamp(event.timestamp)}
                  </p>
                </div>
              </motion.button>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};
