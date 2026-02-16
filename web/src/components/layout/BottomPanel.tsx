import React from 'react';
import { useAppStore } from '../../stores/useAppStore';

export const BottomPanel: React.FC = () => {
  const open = useAppStore((state) => state.bottomPanelOpen);
  const toggle = useAppStore((state) => state.toggleBottomPanel);
  const alerts = useAppStore((state) => state.alerts);

  const unacknowledgedAlerts = alerts.filter((a) => !a.acknowledged);

  return (
    <div className={`${open ? 'h-[200px]' : 'h-[40px]'} bg-bg-secondary border-t border-border-subtle shrink-0 transition-all duration-300`}>
      <div className="flex items-center justify-between px-4 h-[40px] border-b border-border-subtle">
        <div className="flex items-center gap-4">
          <button
            onClick={toggle}
            className="flex items-center gap-2 text-text-secondary hover:text-text-primary transition-colors"
            title={open ? 'Collapse panel' : 'Expand panel'}
          >
            <svg
              className={`w-4 h-4 transition-transform ${open ? '' : 'rotate-180'}`}
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M19 9l-7 7-7-7" />
            </svg>
            <span className="text-xs uppercase tracking-wider">Alerts</span>
          </button>
          
          {unacknowledgedAlerts.length > 0 && (
            <span className="px-2 py-0.5 bg-alert-critical/20 text-alert-critical text-xs rounded-full">
              {unacknowledgedAlerts.length}
            </span>
          )}
        </div>

        <div className="flex items-center gap-2 text-xs text-text-tertiary">
          <span>Live</span>
          <span className="w-2 h-2 rounded-full bg-alert-normal animate-pulse" />
        </div>
      </div>

      {open && (
        <div className="h-[160px] overflow-y-auto p-4">
          {alerts.length === 0 ? (
            <p className="text-text-tertiary text-sm">No active alerts</p>
          ) : (
            <div className="space-y-2">
              {alerts.slice(0, 10).map((alert) => (
                <div
                  key={alert.id}
                  className={`flex items-center gap-3 p-2 rounded-lg ${
                    alert.acknowledged ? 'opacity-50' : ''
                  } ${
                    alert.severity === 'CRITICAL'
                      ? 'bg-alert-critical/10 border border-alert-critical/30'
                      : alert.severity === 'HIGH'
                      ? 'bg-alert-high/10 border border-alert-high/30'
                      : 'bg-bg-tertiary border border-border-subtle'
                  }`}
                >
                  <span
                    className={`w-2 h-2 rounded-full ${
                      alert.severity === 'CRITICAL'
                        ? 'bg-alert-critical'
                        : alert.severity === 'HIGH'
                        ? 'bg-alert-high'
                        : alert.severity === 'MEDIUM'
                        ? 'bg-alert-medium'
                        : 'bg-alert-low'
                    }`}
                  />
                  <span className="text-text-primary text-sm flex-1">{alert.title}</span>
                  <span className="text-text-tertiary text-xs">
                    {new Date(alert.timestamp).toLocaleTimeString()}
                  </span>
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
};
