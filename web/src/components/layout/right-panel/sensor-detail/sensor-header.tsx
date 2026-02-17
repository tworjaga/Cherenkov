'use client';

import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { CardTitle } from '@/components/ui/card';
import { Sensor } from '@/types/models';
import { Settings, Bell } from 'lucide-react';


interface SensorHeaderProps {
  sensor: Sensor;
  onConfigure?: () => void;
  onToggleAlerts?: () => void;
}

export function SensorHeader({ sensor, onConfigure, onToggleAlerts }: SensorHeaderProps) {
  const statusColors: Record<string, string> = {
    active: 'bg-green-500',
    inactive: 'bg-gray-500',
    maintenance: 'bg-yellow-500',
    offline: 'bg-red-500',
  };


  return (
    <div className="flex items-start justify-between p-4 border-b">
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2 mb-1">
          <CardTitle className="text-lg truncate">{sensor.name}</CardTitle>
          <div className={`w-2 h-2 rounded-full ${statusColors[sensor.status]}`} />
        </div>
        <p className="text-sm text-muted-foreground truncate">
          {sensor.source} • {sensor.location.lat.toFixed(4)}, {sensor.location.lon.toFixed(4)}
        </p>
        <div className="flex items-center gap-2 mt-2">
          <Badge variant="outline" className="text-xs">
            ID: {sensor.id.slice(0, 8)}
          </Badge>
          <Badge variant="outline" className="text-xs">
            {sensor.status}
          </Badge>
        </div>

      </div>
      <div className="flex items-center gap-1">
        <Button
          variant="ghost"
          size="icon"
          className="h-8 w-8"
          onClick={onToggleAlerts}
          title="Toggle alerts"
        >
          <Bell className="h-4 w-4" />
        </Button>
        <Button
          variant="ghost"
          size="icon"
          className="h-8 w-8"
          onClick={onConfigure}
          title="Configure sensor"
        >
          <Settings className="h-4 w-4" />
        </Button>
      </div>
    </div>
  );
}
