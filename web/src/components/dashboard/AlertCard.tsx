import React, { useCallback } from 'react';
import { useAppStore, Alert } from '../../stores/useAppStore';
import { formatDistanceToNow } from 'date-fns';

interface AlertCardProps {
  alert: Alert;
  isSelected?: boolean;
}

export const AlertCard: React.FC<AlertCardProps> = ({ alert, isSelected = false }) => {
  const acknowledgeAlert = useAppStore((state) => state.acknowledgeAlert);
  const selectSensor = useAppStore((state) => state.selectSensor);
  const selectFacility = useAppStore((state) => state.selectFacility);
  const setGlobeViewport = useAppStore((state) => state.setGlobeViewport);

  const handleAcknowledge = useCallback((e: React.MouseEvent) => {
    e.stopPropagation();
    acknowledgeAlert(alert.id);
  }, [alert.id, acknowledgeAlert]);

  const handleClick = useCallback(() => {
    // Center globe on alert location
    setGlobeViewport({
      latitude: alert.location.lat,
      longitude: alert.location.lon,
      zoom: 6,
    });

    // Select associated entity
    if (alert.sensorId) {
      selectSensor(alert.sensorId);
    } else if (alert.facilityId) {
      selectFacility(alert.facilityId);
    }
  }, [alert, setGlobeViewport, selectSensor, selectFacility]);

  const severityConfig = {
    CRITICAL: {
      border: 'border-alert-critical',
      bg: 'bg-alert-critical/10',
      glow: 'shadow-[0_0_12px_rgba(255,51,102,0.3)]',
      dot: 'bg-alert-critical',
      pulse: 'animate-pulse',
    },
    HIGH: {
      border: 'border-alert-high',
      bg: 'bg-alert-high/10',
      glow: '',
      dot: 'bg-alert-high',
      pulse: '',
    },
    MEDIUM: {
      border: 'border-alert-medium',
      bg: 'bg-alert-medium/10',
      glow: '',
      dot: 'bg-alert-medium',
      pulse: '',
    },
    LOW: {
      border: 'border-alert-low',
      bg: 'bg-alert-low/10',
      glow: '',
      dot: 'bg-alert-low',
      pulse: '',
    },
  };

  const config = severityConfig[alert.severity];

  return (
    <div
      onClick={handleClick}
      className={`
        relative p-3 rounded-lg border cursor-pointer transition-all duration-200
        ${config.border} ${config.bg} ${config.glow}
        ${alert.acknowledged ? 'opacity-50' : 'opacity-100'}
        ${isSelected ? 'ring-2 ring-accent-primary' : ''}
        hover:scale-[1.02] hover:shadow-lg
      `}
      role="listitem"
      aria-label={`${alert.severity} alert: ${alert.title}`}
      tabIndex={0}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          handleClick();
        } else if (e.key === 'a' || e.key === 'A') {
          handleAcknowledge(e as unknown as React.MouseEvent);
        }
      }}
    >
      {/* Severity indicator */}
      <div className={`absolute left-0 top-3 bottom-3 w-1 rounded-full ${config.dot} ${config.pulse}`} />

      <div className="pl-3">
        <div className="flex items-start justify-between gap-2">
          <div className="flex-1 min-w-0">
            <h4 className={`text-sm font-medium truncate ${alert.acknowledged ? 'line-through' : ''}`}>
              {alert.title}
            </h4>
            <p className="text-xs text-text-secondary mt-1 line-clamp-2">
              {alert.description}
            </p>
          </div>

          {!alert.acknowledged && (
            <button
              onClick={handleAcknowledge}
              className="p-1.5 text-text-tertiary hover:text-text-primary hover:bg-bg-hover rounded transition-colors shrink-0"
              title="Acknowledge (A)"
              aria-label="Acknowledge alert"
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
              </svg>
            </button>
          )}
        </div>

        <div className="flex items-center gap-3 mt-2 text-xs text-text-tertiary">
          <span className="font-mono">
            {alert.location.lat.toFixed(4)}, {alert.location.lon.toFixed(4)}
          </span>
          <span>•</span>
          <span>{formatDistanceToNow(alert.timestamp, { addSuffix: true })}</span>
          {alert.sensorId && (
            <>
              <span>•</span>
              <span className="font-mono text-accent-primary">{alert.sensorId}</span>
            </>
          )}
          {alert.facilityId && (
            <>
              <span>•</span>
              <span className="font-mono text-accent-secondary">{alert.facilityId}</span>
            </>
          )}
        </div>
      </div>
    </div>
  );
};
