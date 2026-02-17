'use client';

import { Facility } from '@/types';
import { Card, CardHeader, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { MapPin, Zap, Factory } from 'lucide-react';

interface FacilityListProps {
  facilities: Facility[];
  onFacilityClick?: (facility: Facility) => void;
  className?: string;
}

export const FacilityList = ({ facilities, onFacilityClick, className }: FacilityListProps) => {
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
        return <Zap className="w-4 h-4 text-alert-critical" />;
      case 'research':
        return <Factory className="w-4 h-4 text-accent-primary" />;
      default:
        return <Factory className="w-4 h-4 text-text-tertiary" />;
    }
  };

  return (
    <Card className={className}>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <h3 className="text-body-lg font-semibold text-text-primary">Facilities</h3>
          <Badge variant="default">{facilities.length} total</Badge>
        </div>
      </CardHeader>
      <CardContent className="space-y-2">
        {facilities.map((facility) => (
          <div
            key={facility.id}
            onClick={() => onFacilityClick?.(facility)}
            className="p-3 rounded-lg border border-border-subtle bg-bg-secondary cursor-pointer transition-all hover:shadow-md"
          >
            <div className="flex items-start justify-between gap-2">
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  {getTypeIcon(facility.type)}
                  <span className="text-body-sm font-medium text-text-primary truncate">
                    {facility.name}
                  </span>
                </div>
                <div className="mt-1 flex items-center gap-1 text-text-tertiary">
                  <MapPin className="w-3 h-3" />
                  <span className="text-mono-xs">
                    {facility.location.lat.toFixed(4)}, {facility.location.lon.toFixed(4)}
                  </span>
                </div>
              </div>
              <Badge variant={getStatusVariant(facility.status)}>{facility.status}</Badge>
            </div>
          </div>
        ))}
      </CardContent>
    </Card>
  );
};
