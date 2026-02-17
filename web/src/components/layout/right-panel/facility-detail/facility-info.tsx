'use client';

import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Facility } from '@/types/models';
import { Zap, Gauge, Hash, MapPin } from 'lucide-react';

interface FacilityInfoProps {
  facility: Facility;
}

export function FacilityInfo({ facility }: FacilityInfoProps) {
  return (
    <div className="grid grid-cols-2 gap-3 p-4">
      {facility.reactorType && (
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Reactor Type</CardTitle>
            <Zap className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-lg font-bold">{facility.reactorType}</div>
          </CardContent>
        </Card>
      )}

      {facility.capacity !== undefined && (
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Capacity</CardTitle>
            <Gauge className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-lg font-bold">{facility.capacity} MW</div>
          </CardContent>
        </Card>
      )}

      {facility.reactorCount !== undefined && (
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Reactors</CardTitle>
            <Hash className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-lg font-bold">{facility.reactorCount}</div>
          </CardContent>
        </Card>
      )}

      {facility.currentOutput !== undefined && (
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Current Output</CardTitle>
            <Gauge className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-lg font-bold">{facility.currentOutput} MW</div>
            {facility.capacity && (
              <p className="text-xs text-muted-foreground">
                {((facility.currentOutput / facility.capacity) * 100).toFixed(1)}% of capacity
              </p>
            )}
          </CardContent>
        </Card>
      )}

      <Card className="col-span-2">
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">Location</CardTitle>
          <MapPin className="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div className="text-lg font-bold">
            {facility.location.lat.toFixed(4)}, {facility.location.lon.toFixed(4)}
          </div>
          <p className="text-xs text-muted-foreground">
            Latitude, Longitude
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
