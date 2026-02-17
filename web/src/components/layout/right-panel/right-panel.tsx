'use client';

import { useState, useRef, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { AlertTriangle, Check, X, Radio, MapPin, Clock } from 'lucide-react';
import { useAppStore, useDataStore } from '@/stores';
import { Alert, Sensor } from '@/types';
import { formatTimestamp, formatDoseRate } from '@/lib/utils/formatters';
import { getSeverityColor } from '@/lib/utils/calculations';

interface AlertCardProps {
  alert: Alert;
  onAcknowledge: (id: string) => void;
  onClick: () => void;
}

const AlertCard = ({ alert, onAcknowledge, onClick }: AlertCardProps) => {
  const severityColor = getSeverityColor(alert.severity);
  
  return (
    <motion.div
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: alert.acknowledged ? 0.6 : 1, x: 0 }}
      exit={{ opacity: 0, x: -20 }}
      className={`relative flex flex-col gap-2 p-3 rounded-md border-l-2 bg-bg-tertiary cursor-pointer transition-all hover:bg-bg-hover ${
        alert.acknowledged ? 'opacity-60' : ''
      }`}
      style={{ borderLeftColor: severityColor }}
      onClick={onClick}
    >
      <div className="flex items-start justify-between gap-2">
        <div className="flex items-center gap-2">
          <AlertTriangle 
            size={16} 
            style={{ color: severityColor }}
            className={alert.severity === 'critical' ? 'animate-pulse' : ''}
          />
          <span className="text-body-sm font-medium text-text-primary">
            {alert.type.toUpperCase()}
          </span>
        </div>
        <span className="text-mono-xs text-text-tertiary">
          {formatTimestamp(alert.timestamp)}
        </span>
      </div>
      
      <p className={`text-body-sm ${alert.acknowledged ? 'line-through text-text-tertiary' : 'text-text-secondary'}`}>
        {alert.message}
      </p>
      
      {!alert.acknowledged && (
        <button
          onClick={(e) => {
            e.stopPropagation();
            onAcknowledge(alert.id);
          }}
          className="self-end flex items-center gap-1 px-2 py-1 rounded text-body-xs text-accent-primary hover:bg-accent-primary/10 transition-colors"
        >
          <Check size={12} />
          Acknowledge
        </button>
      )}
    </motion.div>
  );
};

interface SensorDetailProps {
  sensor: Sensor;
  onClose: () => void;
}

const SensorDetail = ({ sensor, onClose }: SensorDetailProps) => {
  const reading = sensor.lastReading;
  const doseColor = reading ? getSeverityColor(
    reading.doseRate > 10 ? 'critical' :
    reading.doseRate > 5 ? 'high' :
    reading.doseRate > 2 ? 'medium' :
    reading.doseRate > 0.5 ? 'low' : 'normal'
  ) : '#606070';

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className="flex flex-col gap-4 p-4 bg-bg-tertiary rounded-lg border border-border-subtle"
    >
      <div className="flex items-start justify-between">
        <div className="flex items-center gap-2">
          <Radio size={20} className="text-accent-primary" />
          <div>
            <h3 className="text-body-sm font-semibold text-text-primary">{sensor.name}</h3>
            <span className={`text-mono-xs px-2 py-0.5 rounded-full ${
              sensor.status === 'active' ? 'bg-alert-normal/20 text-alert-normal' :
              sensor.status === 'inactive' ? 'bg-text-tertiary/20 text-text-tertiary' :
              'bg-alert-medium/20 text-alert-medium'
            }`}>
              {sensor.status.toUpperCase()}
            </span>
          </div>
        </div>
        <button 
          onClick={onClose}
          className="p-1 rounded hover:bg-bg-hover text-text-tertiary hover:text-text-primary transition-colors"
        >
          <X size={16} />
        </button>
      </div>

      {reading && (
        <div className="flex flex-col gap-1 p-3 bg-bg-secondary rounded-md">
          <span className="text-heading-xs text-text-secondary">CURRENT READING</span>
          <div className="flex items-baseline gap-2">
            <span 
              className="text-display-md font-mono font-bold"
              style={{ color: doseColor }}
            >
              {formatDoseRate(reading.doseRate)}
            </span>
            <span className="text-body-xs text-text-tertiary">{reading.unit}</span>
          </div>
          <div className="flex items-center gap-1 text-mono-xs text-text-tertiary">
            <Clock size={12} />
            {formatTimestamp(reading.timestamp)}
          </div>
        </div>
      )}

      <div className="flex flex-col gap-2">
        <div className="flex items-center gap-2 text-body-xs text-text-secondary">
          <MapPin size={14} />
          <span>{sensor.location.lat.toFixed(4)}°N, {sensor.location.lon.toFixed(4)}°E</span>
        </div>
        <div className="text-body-xs text-text-tertiary">
          Source: {sensor.source}
        </div>
      </div>
    </motion.div>
  );
};

export const RightPanel = () => {
  const { selectedSensorId, selectSensor } = useAppStore();
  const { alerts, sensors, acknowledgeAlert } = useDataStore();
  const [isPaused, setIsPaused] = useState(false);
  const scrollRef = useRef<HTMLDivElement>(null);

  const selectedSensor = selectedSensorId 
    ? sensors.find(s => s.id === selectedSensorId) 
    : null;

  const sortedAlerts = [...alerts].sort((a, b) => b.timestamp - a.timestamp);
  const unacknowledgedCount = alerts.filter(a => !a.acknowledged).length;

  useEffect(() => {
    if (!isPaused && scrollRef.current) {
      scrollRef.current.scrollTop = 0;
    }
  }, [alerts, isPaused]);

  return (
    <div className="flex flex-col w-right-panel h-full bg-bg-secondary border-l border-border-subtle">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-3 border-b border-border-subtle">
        <div className="flex items-center gap-2">
          <span className="text-heading-xs text-text-secondary">ALERTS</span>
          {unacknowledgedCount > 0 && (
            <span className="flex items-center justify-center w-5 h-5 rounded-full bg-alert-critical text-white text-mono-xs font-bold">
              {unacknowledgedCount}
            </span>
          )}
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={() => setIsPaused(!isPaused)}
            className={`text-body-xs px-2 py-1 rounded transition-colors ${
              isPaused ? 'bg-alert-medium/20 text-alert-medium' : 'text-text-tertiary hover:text-text-primary'
            }`}
          >
            {isPaused ? 'RESUME' : 'PAUSE'}
          </button>
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-3 space-y-3 scrollbar-thin scrollbar-thumb-border-active scrollbar-track-transparent">
        <AnimatePresence mode="popLayout">
          {selectedSensor ? (
            <SensorDetail 
              key="sensor-detail"
              sensor={selectedSensor} 
              onClose={() => selectSensor(null)} 
            />
          ) : null}
          
          {sortedAlerts.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-8 text-text-tertiary">
              <Check size={32} className="mb-2 opacity-50" />
              <span className="text-body-sm">No active alerts</span>
            </div>
          ) : (
            sortedAlerts.map((alert) => (
              <AlertCard
                key={alert.id}
                alert={alert}
                onAcknowledge={acknowledgeAlert}
                onClick={() => {
                  if (alert.location) {
                    // Would trigger globe fly-to in real implementation
                  }
                }}
              />
            ))
          )}
        </AnimatePresence>
      </div>

      {/* Footer */}
      <div className="px-4 py-2 border-t border-border-subtle bg-bg-tertiary">
        <div className="flex items-center justify-between text-mono-xs text-text-tertiary">
          <span>Total: {alerts.length}</span>
          <span>Active: {unacknowledgedCount}</span>
        </div>
      </div>
    </div>
  );
};
