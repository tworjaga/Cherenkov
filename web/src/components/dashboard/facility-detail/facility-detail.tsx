'use client';

import { Facility } from '@/types';
import { Card, CardHeader, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { MapPin, Zap, Factory, Info, Gauge } from 'lucide-react';

interface FacilityDetailProps {
  facility: Facility;
  className?: string;
}

export const FacilityDetail = ({ facility, className }: FacilityDetailProps) => {
  const getStatusVariant = (status: Facility['status']) => {
    switch (status) {
      case 'operating':
        return 'success';
      case 'shutdown':
        return 'warning';
      case 'incident':
        return 'danger';
      case 'decommissioned':
        return 'default';
      default:
        return 'default';
    }
  };

  const getTypeIcon = (type: Facility['type']) => {
    switch (type) {
      case 'nuclear':
        return <Zap className="w-5 h-5 text-alert-critical" />;
      case 'research':
        return <Factory className="w-5 h-5 text-accent-primary" />;
      default:
        return <Factory className="w-5 h-5 text-text-tertiary" />;
    }
  };

  return (
    <Card className={className}>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            {getTypeIcon(facility.type)}
            <h3 className="text-body-lg font-semibold text-text-primary">{facility.name}</h3>
          </div>
          <Badge variant={getStatusVariant(facility.status)}>{facility.status}</Badge>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="grid grid-cols-2 gap-4">
          <div className="space-y-1">
            <div className="flex items-center gap-2 text-text-tertiary">
              <MapPin className="w-4 h-4" />
              <span className="text-body-xs">Location</span>
            </div>
            <p className="text-mono-sm text-text-primary">
              {facility.location.lat.toFixed(6)}, {facility.location.lon.toFixed(6)}
            </p>
          </div>
          <div className="space-y-1">
            <div className="flex items-center gap-2 text-text-tertiary">
              <Info className="w-4 h-4" />
              <span className="text-body-xs">Type</span>
            </div>
            <p className="text-body-sm text-text-primary capitalize">{facility.type}</p>
          </div>
        </div>

        {(facility.reactorType || facility.reactorCount) && (
          <div className="pt-4 border-t border-border-subtle">
            <h4 className="text-body-sm font-medium text-text-primary mb-3">Reactor Information</h4>
            <div className="grid grid-cols-2 gap-4">
              {facility.reactorType && (
                <div className="space-y-1">
                  <span className="text-body-xs text-text-tertiary">Reactor Type</span>
                  <p className="text-body-sm text-text-primary">{facility.reactorType}</p>
                </div>
              )}
              {facility.reactorCount && (
                <div className="space-y-1">
                  <span className="text-body-xs text-text-tertiary">Reactor Count</span>
                  <p className="text-mono-sm text-text-primary">{facility.reactorCount}</p>
                </div>
              )}
            </div>
          </div>
        )}

        {(facility.capacity || facility.currentOutput) && (
          <div className="pt-4 border-t border-border-subtle">
            <h4 className="text-body-sm font-medium text-text-primary mb-3">Output</h4>
            <div className="grid grid-cols-2 gap-4">
              {facility.capacity && (
                <div className="space-y-1">
                  <span className="text-body-xs text-text-tertiary">Capacity</span>
                  <p className="text-mono-sm text-text-primary">{facility.capacity} MW</p>
                </div>
              )}
              {facility.currentOutput && (
                <div className="space-y-1">
                  <div className="flex items-center gap-2 text-text-tertiary">
                    <Gauge className="w-4 h-4" />
                    <span className="text-body-xs">Current Output</span>
                  </div>
                  <p className="text-mono-sm text-text-primary">{facility.currentOutput} MW</p>
                </div>
              )}
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
};
