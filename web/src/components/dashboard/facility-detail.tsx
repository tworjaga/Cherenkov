'use client';

import { Building2, MapPin, Zap, Calendar } from 'lucide-react';
import { Facility } from '@/types';
import { formatCoordinate } from '@/lib/utils';

interface FacilityDetailProps {
  facility: Facility;
  onClose: () => void;
}

export const FacilityDetail = ({ facility, onClose }: FacilityDetailProps) => {
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
        <div className="flex items-center gap-2">
          <Building2 size={16} className="text-accent-secondary" />
          <span className="text-body-sm font-medium text-text-primary">
            {facility.name}
          </span>
        </div>
        <button
          onClick={onClose}
          className="text-text-tertiary hover:text-text-primary transition-colors"
        >
          ×
        </button>
      </div>
      
      <div className="p-4 space-y-4">
        <div className="flex items-center justify-between p-3 bg-bg-tertiary rounded-lg">
          <span className="text-heading-xs text-text-secondary">STATUS</span>
          <span className={`text-body-sm font-medium uppercase ${getStatusColor(facility.status)}`}>
            {facility.status}
          </span>
        </div>
        
        <div className="space-y-2">
          <div className="flex items-center gap-2 text-body-sm text-text-secondary">
            <MapPin size={14} />
            <span>
              {formatCoordinate(facility.location.lat, facility.location.lon)}
            </span>
          </div>
          <div className="flex items-center gap-2 text-body-sm text-text-secondary">
            <Zap size={14} />
            <span>Type: {facility.type}</span>
          </div>
          {facility.reactorType && (
            <div className="flex items-center gap-2 text-body-sm text-text-secondary">
              <Calendar size={14} />
              <span>Reactor: {facility.reactorType}</span>
            </div>
          )}
          {facility.capacity && (
            <div className="flex items-center gap-2 text-body-sm text-text-secondary">
              <Zap size={14} />
              <span>Capacity: {facility.capacity} MW</span>
            </div>
          )}
          {facility.reactorCount && (
            <div className="flex items-center gap-2 text-body-sm text-text-secondary">
              <Building2 size={14} />
              <span>Reactors: {facility.reactorCount}</span>
            </div>
          )}
          {facility.currentOutput && (
            <div className="flex items-center gap-2 text-body-sm text-text-secondary">
              <Zap size={14} />
              <span>Output: {facility.currentOutput} MW</span>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
