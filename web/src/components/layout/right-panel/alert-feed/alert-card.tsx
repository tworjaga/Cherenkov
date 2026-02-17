'use client';

import { Card, CardContent, CardHeader } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import type { Alert } from '@/types/models';


interface AlertCardProps {
  alert: Alert;
  onClick?: (alert: Alert) => void;
  isSelected?: boolean;
}

export function AlertCard({ alert, onClick, isSelected }: AlertCardProps) {
  const severityColors = {
    critical: 'bg-red-500/10 text-red-500 border-red-500/20',
    high: 'bg-orange-500/10 text-orange-500 border-orange-500/20',
    medium: 'bg-yellow-500/10 text-yellow-500 border-yellow-500/20',
    low: 'bg-blue-500/10 text-blue-500 border-blue-500/20',
    info: 'bg-slate-500/10 text-slate-500 border-slate-500/20',
  };

  const severityLabels = {
    critical: 'Critical',
    high: 'High',
    medium: 'Medium',
    low: 'Low',
    info: 'Info',
  };

  return (
    <Card
      className={`cursor-pointer transition-all hover:border-primary/50 ${
        isSelected ? 'border-primary bg-primary/5' : ''
      }`}
      onClick={() => onClick?.(alert)}
    >
      <CardHeader className="p-3 pb-0">
        <div className="flex items-start justify-between gap-2">
          <div className="flex-1 min-w-0">
            <p className="text-sm font-medium truncate">Alert #{alert.id.slice(0, 8)}</p>
            <p className="text-xs text-muted-foreground">
              {new Date(alert.timestamp).toLocaleString()}
            </p>
          </div>
          <Badge
            variant="outline"
            className={`text-xs ${severityColors[alert.severity]}`}
          >
            {severityLabels[alert.severity]}
          </Badge>
        </div>
      </CardHeader>
      <CardContent className="p-3 pt-2">
        <p className="text-xs text-muted-foreground line-clamp-2">
          {alert.message}
        </p>
      </CardContent>

    </Card>
  );
}
