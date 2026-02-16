import React, { useMemo, useCallback, useEffect, useState } from 'react';
import { useAppStore } from '../../stores/useAppStore';
import { AlertCard } from './AlertCard';

export const AlertFeed: React.FC = () => {
  const alerts = useAppStore((state) => state.alerts);
  const [filter, setFilter] = useState<'ALL' | 'UNACKNOWLEDGED' | 'CRITICAL' | 'HIGH'>('ALL');
  const [selectedId, setSelectedId] = useState<string | null>(null);

  // Auto-scroll to new alerts
  const feedRef = React.useRef<HTMLDivElement>(null);
  const prevAlertCount = React.useRef(alerts.length);

  useEffect(() => {
    if (alerts.length > prevAlertCount.current && feedRef.current) {
      feedRef.current.scrollTop = 0;
    }
    prevAlertCount.current = alerts.length;
  }, [alerts.length]);

  // Filter alerts
  const filteredAlerts = useMemo(() => {
    let result = [...alerts];
    
    switch (filter) {
      case 'UNACKNOWLEDGED':
        result = result.filter(a => !a.acknowledged);
        break;
      case 'CRITICAL':
        result = result.filter(a => a.severity === 'CRITICAL');
        break;
      case 'HIGH':
        result = result.filter(a => a.severity === 'HIGH' || a.severity === 'CRITICAL');
        break;
    }
    
    return result;
  }, [alerts, filter]);

  // Statistics
  const stats = useMemo(() => ({
    total: alerts.length,
    unacknowledged: alerts.filter(a => !a.acknowledged).length,
    critical: alerts.filter(a => a.severity === 'CRITICAL' && !a.acknowledged).length,
    high: alerts.filter(a => a.severity === 'HIGH' && !a.acknowledged).length,
  }), [alerts]);

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'a' || e.key === 'A') {
        // Acknowledge selected alert
        if (selectedId) {
          useAppStore.getState().acknowledgeAlert(selectedId);
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [selectedId]);

  const handleAlertClick = useCallback((id: string) => {
    setSelectedId(id);
  }, []);

  return (
    <div className="flex flex-col h-full">
      {/* Filter tabs */}
      <div className="flex items-center gap-1 p-2 border-b border-border-subtle">
        {[
          { key: 'ALL', label: 'All', count: stats.total },
          { key: 'UNACKNOWLEDGED', label: 'Unread', count: stats.unacknowledged },
          { key: 'CRITICAL', label: 'Critical', count: stats.critical },
          { key: 'HIGH', label: 'High', count: stats.high },
        ].map(({ key, label, count }) => (
          <button
            key={key}
            onClick={() => setFilter(key as typeof filter)}
            className={`
              px-3 py-1.5 text-xs font-medium rounded-lg transition-colors
              ${filter === key 
                ? 'bg-accent-primary/20 text-accent-primary border border-accent-primary/50' 
                : 'text-text-secondary hover:text-text-primary hover:bg-bg-hover'}
            `}
            aria-pressed={filter === key ? 'true' : 'false'}
          >
            {label}
            {count > 0 && (
              <span className={`
                ml-1.5 px-1.5 py-0.5 rounded-full text-[10px]
                ${key === 'CRITICAL' ? 'bg-alert-critical text-white' : 
                  key === 'HIGH' ? 'bg-alert-high text-white' : 
                  'bg-bg-tertiary text-text-secondary'}
              `}>
                {count}
              </span>
            )}
          </button>
        ))}
      </div>

      {/* Alert list */}
      <div 
        ref={feedRef}
        className="flex-1 overflow-y-auto p-3 space-y-2"
        role="list"
        aria-label="Alert feed"
      >
        {filteredAlerts.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-32 text-text-tertiary">
            <svg className="w-8 h-8 mb-2 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <span className="text-sm">No alerts</span>
          </div>
        ) : (
          filteredAlerts.map((alert) => (
            <AlertCard
              key={alert.id}
              alert={alert}
              isSelected={selectedId === alert.id}
            />
          ))
        )}
      </div>

      {/* Footer info */}
      <div className="p-2 border-t border-border-subtle text-xs text-text-tertiary flex justify-between">
        <span>Press 'A' to acknowledge selected</span>
        <span>{filteredAlerts.length} of {alerts.length}</span>
      </div>
    </div>
  );
};
