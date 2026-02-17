'use client';

import * as React from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { cn } from '@/lib/utils';
import { Sensor, Facility, Anomaly } from '@/types/models';

interface TooltipOverlayProps {
  feature: {
    type: 'sensor' | 'facility' | 'anomaly';
    data: Sensor | Facility | Anomaly;
  } | null;
  position: { x: number; y: number } | null;
  className?: string;
}

export const TooltipOverlay = ({
  feature,
  position,
  className,
}: TooltipOverlayProps) => {
  if (!feature || !position) return null;

  const { type, data } = feature;

  return (
    <AnimatePresence>
      <motion.div
        initial={{ opacity: 0, scale: 0.95 }}
        animate={{ opacity: 1, scale: 1 }}
        exit={{ opacity: 0, scale: 0.95 }}
        transition={{ duration: 0.15 }}
        className={cn(
          'absolute z-50 pointer-events-none',
          className
        )}
        style={{
          left: position.x,
          top: position.y,
          transform: 'translate(-50%, -100%)',
        }}
      >
        <div className="bg-bg-tertiary border border-border-default rounded-lg p-3 shadow-lg min-w-[200px]">
          {type === 'sensor' && (
            <SensorTooltipContent sensor={data as Sensor} />
          )}
          {type === 'facility' && (
            <FacilityTooltipContent facility={data as Facility} />
          )}
          {type === 'anomaly' && (
            <AnomalyTooltipContent anomaly={data as Anomaly} />
          )}
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 -bottom-2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-l-transparent border-r-transparent border-t-bg-tertiary" />
      </motion.div>
    </AnimatePresence>
  );
};

const SensorTooltipContent = ({ sensor }: { sensor: Sensor }) => (
  <div className="space-y-2">
    <div className="flex items-center justify-between">
      <span className="text-heading-xs text-text-secondary">SENSOR</span>
      <span
        className="w-2 h-2 rounded-full"
        style={{
          backgroundColor:
            sensor.status === 'active' ? '#00ff88' : '#ff3366',
        }}
      />
    </div>
    <p className="text-body-sm text-text-primary font-medium">
      {sensor.name}
    </p>
    <div className="text-mono-xs text-text-tertiary">
      <p>Lat: {sensor.latitude.toFixed(4)}</p>
      <p>Lon: {sensor.longitude.toFixed(4)}</p>
    </div>
  </div>
);

const FacilityTooltipContent = ({ facility }: { facility: Facility }) => (
  <div className="space-y-2">
    <div className="flex items-center justify-between">
      <span className="text-heading-xs text-text-secondary">FACILITY</span>
      <span
        className="w-2 h-2 rounded-full"
        style={{
          backgroundColor:
            facility.status === 'operating' ? '#00ff88' : '#ff3366',
        }}
      />
    </div>
    <p className="text-body-sm text-text-primary font-medium">
      {facility.name}
    </p>
    <p className="text-body-xs text-text-secondary">{facility.type}</p>
  </div>
);

const AnomalyTooltipContent = ({ anomaly }: { anomaly: Anomaly }) => (
  <div className="space-y-2">
    <div className="flex items-center justify-between">
      <span className="text-heading-xs text-text-secondary">ANOMALY</span>
      <span
        className="w-2 h-2 rounded-full"
        style={{
          backgroundColor:
            anomaly.severity === 'critical'
              ? '#ff3366'
              : anomaly.severity === 'high'
              ? '#ff6b35'
              : '#ffb800',
        }}
      />
    </div>
    <p className="text-body-sm text-text-primary font-medium">
      {anomaly.message}
    </p>
    <p className="text-mono-xs text-text-tertiary">
      Z-Score: {anomaly.zScore.toFixed(2)}
    </p>
  </div>
);

export default TooltipOverlay;
