'use client';

import { Alert } from '@/types';
import { Card, CardHeader, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { formatTimestamp } from '@/lib/utils';
import { History, AlertTriangle, Info, CheckCircle } from 'lucide-react';

interface EventTimelineProps {
  events: Alert[];
  onEventClick?: (event: Alert) => void;
  className?: string;
}

export const EventTimeline = ({ events, onEventClick, className }: EventTimelineProps) => {
  const getSeverityIcon = (severity: Alert['severity']) => {
    switch (severity) {
      case 'critical':
      case 'high':
        return <AlertTriangle className="w-4 h-4 text-alert-danger" />;
      case 'medium':
        return <AlertTriangle className="w-4 h-4 text-alert-warning" />;
      case 'low':
        return <Info className="w-4 h-4 text-accent-primary" />;
      default:
        return <Info className="w-4 h-4 text-text-tertiary" />;
    }
  };

  const getSeverityVariant = (severity: Alert['severity']) => {
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

  const sortedEvents = [...events].sort((a, b) => b.timestamp - a.timestamp);

  return (
    <Card className={className}>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <History className="w-5 h-5 text-accent-primary" />
            <h3 className="text-body-lg font-semibold text-text-primary">Event Timeline</h3>
          </div>
          <Badge variant="default">{events.length} events</Badge>
        </div>
      </CardHeader>
      <CardContent>
        <div className="space-y-2 max-h-64 overflow-y-auto">
          {sortedEvents.map((event, index) => (
            <div
              key={event.id}
              onClick={() => onEventClick?.(event)}
              className={`flex items-start gap-3 p-2 rounded-lg cursor-pointer transition-all hover:bg-bg-secondary ${
                event.acknowledged ? 'opacity-50' : ''
              }`}
            >
              <div className="mt-0.5">{getSeverityIcon(event.severity)}</div>
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  <Badge variant={getSeverityVariant(event.severity)}>{event.severity}</Badge>
                  <span className="text-mono-xs text-text-tertiary">{event.type}</span>
                </div>
                <p className="mt-1 text-body-sm text-text-primary truncate">{event.message}</p>
                <span className="text-mono-xs text-text-tertiary">
                  {formatTimestamp(event.timestamp)}
                </span>
              </div>
              {event.acknowledged && (
                <CheckCircle className="w-4 h-4 text-alert-success mt-0.5" />
              )}
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  );
};
