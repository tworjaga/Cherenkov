import React from 'react';
import { useAppStore } from '../../stores/useAppStore';

export const Header: React.FC = () => {
  const globalStatus = useAppStore((state) => state.globalStatus);
  const connection = useAppStore((state) => state.connection);

  const getStatusColor = () => {
    if (!globalStatus) return 'bg-alert-normal';
    switch (globalStatus.level) {
      case 'CRITICAL': return 'bg-alert-critical animate-alert-pulse';
      case 'HIGH': return 'bg-alert-high';
      case 'ELEVATED': return 'bg-alert-medium';
      default: return 'bg-alert-normal';
    }
  };

  const getDefconLabel = () => {
    if (!globalStatus) return 'DEFCON 5';
    return `DEFCON ${globalStatus.defcon}`;
  };

  return (
    <header className="h-[56px] bg-bg-secondary border-b border-border-subtle flex items-center justify-between px-4 shrink-0 z-50">
      <div className="flex items-center gap-3">
        <div className={`w-3 h-3 rounded-full ${getStatusColor()}`} />
        <h1 className="text-text-primary font-semibold tracking-wider">CHERENKOV</h1>
        <span className="text-text-tertiary text-xs px-2 py-0.5 bg-bg-tertiary rounded">
          {getDefconLabel()}
        </span>
      </div>

      <div className="flex items-center gap-4">
        <div className="flex items-center gap-2 text-xs">
          <span className={`w-2 h-2 rounded-full ${
            connection === 'CONNECTED' ? 'bg-alert-normal' :
            connection === 'CONNECTING' ? 'bg-alert-medium' :
            'bg-alert-critical'
          }`} />
          <span className="text-text-secondary uppercase tracking-wider">
            {connection}
          </span>
        </div>

        <button className="p-2 hover:bg-bg-hover rounded transition-colors" title="Notifications">
          <svg className="w-5 h-5 text-text-secondary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
          </svg>
        </button>

        <button className="p-2 hover:bg-bg-hover rounded transition-colors" title="User menu">
          <svg className="w-5 h-5 text-text-secondary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
          </svg>
        </button>
      </div>
    </header>
  );
};
