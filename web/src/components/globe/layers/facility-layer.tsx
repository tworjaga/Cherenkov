'use client';

import { useMemo } from 'react';
import { IconLayer } from '@deck.gl/layers';
import { useDataStore } from '@/stores';
import { Diamond } from 'lucide-react';

interface FacilityLayerProps {
  onFacilityClick?: (facilityId: string) => void;
  selectedFacilityId?: string | null;
}

export function FacilityLayer({ onFacilityClick, selectedFacilityId }: FacilityLayerProps) {
  const { facilities } = useDataStore();

  const layer = useMemo(() => {
    return new IconLayer({
      id: 'facility-layer',
      data: facilities,
      pickable: true,
      iconAtlas: '/icons/facilities.png',
      iconMapping: {
        nuclear: { x: 0, y: 0, width: 128, height: 128, mask: true },
        research: { x: 128, y: 0, width: 128, height: 128, mask: true },
        medical: { x: 0, y: 128, width: 128, height: 128, mask: true },
      },
      getIcon: (d) => d.type,
      sizeScale: 1,
      getPosition: (d) => [d.longitude, d.latitude],
      getSize: (d) => (d.id === selectedFacilityId ? 24 : 16),
      getColor: (d) => {
        if (d.id === selectedFacilityId) {
          return [0, 212, 255]; // accent.primary
        }
        switch (d.status) {
          case 'operating':
            return [0, 255, 136]; // alert.normal
          case 'shutdown':
            return [160, 160, 176]; // text.secondary
          case 'incident':
            return [255, 51, 102]; // alert.critical
          default:
            return [160, 160, 176];
        }
      },
      onClick: (info) => {
        if (info.object && onFacilityClick) {
          onFacilityClick(info.object.id);
        }
      },
    });
  }, [facilities, selectedFacilityId, onFacilityClick]);

  return null;
}

export default FacilityLayer;
