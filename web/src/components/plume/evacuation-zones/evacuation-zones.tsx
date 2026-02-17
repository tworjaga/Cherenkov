'use client';

import React from 'react';
import { Card, CardContent, CardHeader } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { AlertTriangle, Circle, MapPin } from 'lucide-react';

interface EvacuationZone {
  id: string;
  name: string;
  radius: number;
  severity: 'critical' | 'high' | 'medium' | 'low';
  population: number;
  instructions: string;
}

interface EvacuationZonesProps {
  zones?: EvacuationZone[];
}

const defaultZones: EvacuationZone[] = [
  {
    id: '1',
    name: 'Immediate Evacuation Zone',
    radius: 2,
    severity: 'critical',
    population: 1500,
    instructions: 'Evacuate immediately. Move perpendicular to wind direction.',
  },
  {
    id: '2',
    name: 'Shelter in Place Zone',
    radius: 5,
    severity: 'high',
    population: 8500,
    instructions: 'Close all windows and doors. Turn off ventilation.',
  },
  {
    id: '3',
    name: 'Monitoring Zone',
    radius: 10,
    severity: 'medium',
    population: 25000,
    instructions: 'Stay alert for updates. Prepare for potential evacuation.',
  },
];

const severityColors = {
  critical: 'bg-red-500',
  high: 'bg-orange-500',
  medium: 'bg-yellow-500',
  low: 'bg-blue-500',
};

const severityLabels = {
  critical: 'Critical',
  high: 'High',
  medium: 'Medium',
  low: 'Low',
};

export function EvacuationZones({ zones = defaultZones }: EvacuationZonesProps) {
  return (
    <div className="space-y-4">
      <div className="flex items-center gap-2 text-amber-500">
        <AlertTriangle className="h-5 w-5" />
        <span className="font-semibold">Emergency Response Zones</span>
      </div>

      <div className="grid gap-4">
        {zones.map((zone) => (
          <Card key={zone.id}>
            <CardHeader className="pb-2">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <Circle className={`h-4 w-4 ${severityColors[zone.severity]}`} />
                  <span className="font-semibold">{zone.name}</span>
                </div>
                <Badge variant={zone.severity === 'critical' ? 'danger' : 'outline'}>

                  {severityLabels[zone.severity]}
                </Badge>
              </div>
            </CardHeader>
            <CardContent className="space-y-2">
              <div className="flex items-center gap-4 text-sm text-muted-foreground">
                <div className="flex items-center gap-1">
                  <MapPin className="h-4 w-4" />
                  <span>{zone.radius} km radius</span>
                </div>
                <div>Population: {zone.population.toLocaleString()}</div>
              </div>
              <p className="text-sm">{zone.instructions}</p>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
}
