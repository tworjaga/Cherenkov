'use client';

import { useState, useRef, useCallback, useMemo } from 'react';
import { Sensor } from '@/types';
import { Card, CardHeader, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { formatDoseRate, formatTimestamp } from '@/lib/utils';
import { Activity, MapPin, Clock } from 'lucide-react';
import { useIntersectionObserver } from '@/hooks/use-intersection-observer';

const ITEM_HEIGHT = 88; // Height of each sensor item in pixels
const BUFFER_SIZE = 5; // Number of items to render above/below viewport
const CONTAINER_HEIGHT = 400; // Max height of scrollable container

interface SensorListProps {
  sensors: Sensor[];
  onSensorClick?: (sensor: Sensor) => void;
  className?: string;
}

export const SensorList = ({ sensors, onSensorClick, className }: SensorListProps) => {
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [scrollTop, setScrollTop] = useState(0);

  // Use intersection observer for lazy loading off-screen content
  const [loadMoreRef, isIntersecting] = useIntersectionObserver<HTMLDivElement>({
    threshold: 0,
    rootMargin: '100px',
  });



  const handleClick = useCallback((sensor: Sensor) => {
    setSelectedId(sensor.id);
    onSensorClick?.(sensor);
  }, [onSensorClick]);

  const getStatusVariant = (status: Sensor['status']) => {
    switch (status) {
      case 'active':
        return 'success';
      case 'inactive':
        return 'warning';
      case 'maintenance':
        return 'default';
      case 'offline':
        return 'danger';
      default:
        return 'default';
    }
  };

  // Calculate visible range for virtual scrolling
  const { virtualItems, totalHeight, startIndex } = useMemo(() => {
    const totalItems = sensors.length;
    const totalHeight = totalItems * ITEM_HEIGHT;
    
    const startIndex = Math.max(0, Math.floor(scrollTop / ITEM_HEIGHT) - BUFFER_SIZE);
    const visibleCount = Math.ceil(CONTAINER_HEIGHT / ITEM_HEIGHT) + 2 * BUFFER_SIZE;
    const endIndex = Math.min(totalItems, startIndex + visibleCount);
    
    const virtualItems = sensors.slice(startIndex, endIndex).map((sensor, index) => ({
      sensor,
      index: startIndex + index,
      style: {
        position: 'absolute' as const,
        top: (startIndex + index) * ITEM_HEIGHT,
        height: ITEM_HEIGHT,
        left: 0,
        right: 0,
      },
    }));

    return { virtualItems, totalHeight, startIndex };
  }, [sensors, scrollTop]);

  // Handle scroll with throttling
  const handleScroll = useCallback((e: React.UIEvent<HTMLDivElement>) => {
    setScrollTop(e.currentTarget.scrollTop);
  }, []);

  // Memoized sensor item renderer
  const SensorItem = useMemo(() => {
    return ({ sensor, style, index }: { sensor: Sensor; style: React.CSSProperties; index: number }) => (
      <div
        key={sensor.id}
        data-testid={`sensor-item-${sensor.id}`}
        onClick={() => handleClick(sensor)}
        style={style}
        className={`absolute px-3 py-2 rounded-lg border cursor-pointer transition-all hover:shadow-md ${
          selectedId === sensor.id
            ? 'border-accent-primary bg-accent-primary/5'
            : 'border-border-subtle bg-bg-secondary'
        }`}
        role="listitem"
        aria-posinset={index + 1}
        aria-setsize={sensors.length}
        tabIndex={0}
        onKeyDown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            handleClick(sensor);
          }
        }}
      >
        <div className="flex items-start justify-between gap-2">
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2">
              <Activity className="w-4 h-4 text-accent-primary" aria-hidden="true" />
              <span className="text-body-sm font-medium text-text-primary truncate">
                {sensor.name}
              </span>
            </div>
            <div className="mt-1 flex items-center gap-1 text-text-tertiary">
              <MapPin className="w-3 h-3" aria-hidden="true" />
              <span className="text-mono-xs">
                {sensor.location.lat.toFixed(4)}, {sensor.location.lon.toFixed(4)}
              </span>
            </div>
          </div>
          <Badge variant={getStatusVariant(sensor.status)}>{sensor.status}</Badge>
        </div>
        {sensor.lastReading && (
          <div className="mt-2 pt-2 border-t border-border-subtle flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-mono-sm text-accent-primary">
                {formatDoseRate(sensor.lastReading.doseRate)}
              </span>
              <span className="text-mono-xs text-text-tertiary">
                {sensor.lastReading.unit}
              </span>
            </div>
            <div className="flex items-center gap-1 text-text-tertiary">
              <Clock className="w-3 h-3" aria-hidden="true" />
              <span className="text-mono-xs">
                {formatTimestamp(sensor.lastReading.timestamp)}
              </span>
            </div>
          </div>
        )}
      </div>
    );
  }, [selectedId, sensors.length, handleClick]);

  return (
    <Card className={className}>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <h3 className="text-body-lg font-semibold text-text-primary" id="sensor-list-heading">
            Sensors
          </h3>
          <Badge variant="default" aria-label={`${sensors.length} sensors total`}>
            {sensors.length} total
          </Badge>
        </div>
      </CardHeader>
      <CardContent className="p-0">
        <div
          ref={containerRef}
          onScroll={handleScroll}
          className="overflow-y-auto px-3 pb-3"
          style={{ height: CONTAINER_HEIGHT }}
          role="list"
          aria-labelledby="sensor-list-heading"
          aria-label="Sensor list"
          tabIndex={0}
        >
          <div style={{ position: 'relative', height: totalHeight }}>
            {virtualItems.map(({ sensor, index, style }) => (
              <SensorItem key={sensor.id} sensor={sensor} style={style} index={index} />
            ))}
          </div>
          {/* Intersection observer target for lazy loading */}
          <div ref={loadMoreRef as React.RefObject<HTMLDivElement>} className="h-4" aria-hidden="true" />

        </div>
      </CardContent>
    </Card>
  );
};
