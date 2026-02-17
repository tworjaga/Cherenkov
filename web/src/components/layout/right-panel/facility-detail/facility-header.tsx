'use client';

import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { CardTitle } from '@/components/ui/card';
import { Facility } from '@/types/models';
import { Building2, Settings, ExternalLink } from 'lucide-react';

interface FacilityHeaderProps {
  facility: Facility;
  onConfigure?: () => void;
  onViewDetails?: () => void;
}

export function FacilityHeader({ facility, onConfigure, onViewDetails }: FacilityHeaderProps) {
  const statusColors: Record<string, string> = {
    operating: 'bg-green-500',
    shutdown: 'bg-gray-500',
    incident: 'bg-red-500',
    decommissioned: 'bg-yellow-500',
  };

  const typeLabels: Record<string, string> = {
    nuclear: 'Nuclear Power Plant',
    research: 'Research Facility',
    medical: 'Medical Facility',
    industrial: 'Industrial Facility',
  };

  return (
    <div className="flex items-start justify-between p-4 border-b">
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2 mb-1">
          <CardTitle className="text-lg truncate">{facility.name}</CardTitle>
          <div className={`w-2 h-2 rounded-full ${statusColors[facility.status]}`} />
        </div>
        <p className="text-sm text-muted-foreground truncate">
          {typeLabels[facility.type] || facility.type}
        </p>
        <div className="flex items-center gap-2 mt-2">
          <Badge variant="outline" className="text-xs">
            ID: {facility.id.slice(0, 8)}
          </Badge>
          <Badge variant="outline" className="text-xs">
            {facility.status}
          </Badge>
        </div>
      </div>
      <div className="flex items-center gap-1">
        <Button
          variant="ghost"
          size="icon"
          className="h-8 w-8"
          onClick={onViewDetails}
          title="View full details"
        >
          <ExternalLink className="h-4 w-4" />
        </Button>
        <Button
          variant="ghost"
          size="icon"
          className="h-8 w-8"
          onClick={onConfigure}
          title="Configure facility"
        >
          <Settings className="h-4 w-4" />
        </Button>
      </div>
    </div>
  );
}
