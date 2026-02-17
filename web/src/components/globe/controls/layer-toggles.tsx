'use client';

import { useGlobeStore } from '@/stores';
import { Switch } from '@/components/ui/switch';

import { Layers, Radio, Building2, AlertTriangle, Wind, Hexagon } from 'lucide-react';

interface LayerTogglesProps {
  className?: string;
}

export function LayerToggles({ className }: LayerTogglesProps) {
  const { layers, toggleLayer } = useGlobeStore();

  const layerConfig = [
    { id: 'sensors', label: 'Sensors', icon: Radio, color: 'text-alert-normal' },
    { id: 'facilities', label: 'Facilities', icon: Building2, color: 'text-text-secondary' },
    { id: 'anomalies', label: 'Anomalies', icon: AlertTriangle, color: 'text-alert-critical' },
    { id: 'plumes', label: 'Plumes', icon: Wind, color: 'text-accent-primary' },
    { id: 'heatmap', label: 'Heatmap', icon: Hexagon, color: 'text-alert-medium' },
  ] as const;

  return (
    <div className={`bg-bg-secondary/90 backdrop-blur-sm border border-border-subtle rounded-lg p-3 ${className}`}>
      <div className="flex items-center gap-2 mb-3 pb-2 border-b border-border-subtle">
        <Layers className="w-4 h-4 text-text-secondary" />
        <span className="text-xs font-medium uppercase tracking-wider text-text-secondary">
          Layers
        </span>
      </div>
      <div className="space-y-2">
        {layerConfig.map(({ id, label, icon: Icon, color }) => (
          <div key={id} className="flex items-center justify-between gap-4">
            <div className="flex items-center gap-2">
              <Icon className={`w-4 h-4 ${color}`} />
              <span className="text-sm text-text-primary">{label}</span>
            </div>
            <Switch
              checked={layers[id as keyof typeof layers]}
              onCheckedChange={() => toggleLayer(id as keyof typeof layers)}
            />


          </div>
        ))}
      </div>
    </div>
  );
}

export default LayerToggles;
