'use client';

import { motion } from 'framer-motion';
import { Building2, Activity } from 'lucide-react';
import { Facility } from '@/types';

interface FacilityListProps {
  facilities: Facility[];
  selectedId: string | null;
  onSelect: (facility: Facility) => void;
}

export const FacilityList = ({ facilities, selectedId, onSelect }: FacilityListProps) => {
  const sortedFacilities = [...facilities].sort((a, b) => a.name.localeCompare(b.name));

  const getStatusColor = (status: Facility['status']) => {
    switch (status) {
      case 'operating': return 'text-alert-normal';
      case 'shutdown': return 'text-text-tertiary';
      case 'incident': return 'text-alert-critical';
      case 'decommissioned': return 'text-text-disabled';
      default: return 'text-text-secondary';
    }
  };

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between p-3 border-b border-border-subtle">
        <span className="text-heading-xs text-text-secondary">FACILITIES</span>
        <span className="text-body-xs text-text-tertiary">
          {facilities.length} total
        </span>
      </div>
      
      <div className="flex-1 overflow-y-auto">
        {sortedFacilities.map((facility) => (
          <motion.button
            key={facility.id}
            onClick={() => onSelect(facility)}
            className={`w-full p-3 text-left border-b border-border-subtle transition-colors ${
              selectedId === facility.id ? 'bg-bg-active' : 'hover:bg-bg-hover'
            }`}
            whileHover={{ x: 2 }}
          >
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Building2 size={14} className="text-accent-secondary" />
                <span className="text-body-sm font-medium text-text-primary">
                  {facility.name}
                </span>
              </div>
              <div className="flex items-center gap-1">
                <Activity size={12} className={getStatusColor(facility.status)} />
                <span className="text-mono-xs text-text-secondary uppercase">
                  {facility.status}
                </span>
              </div>
            </div>
            
            <div className="mt-1 flex items-center justify-between text-body-xs text-text-tertiary">
              <span>{facility.type}</span>
              {facility.reactorCount && (
                <span>{facility.reactorCount} reactors</span>
              )}
            </div>
          </motion.button>
        ))}
      </div>
    </div>
  );
};
