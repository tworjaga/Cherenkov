import React from 'react';
import { useAppStore } from '../../stores/useAppStore';
import { AlertFeed } from '../dashboard/AlertFeed';

export const BottomPanel: React.FC = () => {
  const open = useAppStore((state) => state.bottomPanelOpen);
  const toggle = useAppStore((state) => state.toggleBottomPanel);
  const alerts = useAppStore((state) => state.alerts);

  const unacknowledgedAlerts = alerts.filter((a) => !a.acknowledged);

  return (
    <div className={`${open ? 'h-[240px]' : 'h-[40px]'} bg-bg-secondary border-t border-border-subtle shrink-0 transition-all duration-300`}>
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
        <div className="h-[200px]">
          <AlertFeed />
        </div>
      )}
    </div>
  );
};
