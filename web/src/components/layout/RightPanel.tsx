import React from 'react';
import { useAppStore } from '../../stores/useAppStore';
import { AlertFeed } from '../dashboard/AlertFeed';


export const RightPanel: React.FC = () => {
  const open = useAppStore((state) => state.rightPanelOpen);
  const toggle = useAppStore((state) => state.toggleRightPanel);
  const selectedSensor = useAppStore((state) => state.globe.selectedSensor);
  const selectedFacility = useAppStore((state) => state.globe.selectedFacility);

  if (!open) {
    return (
      <button
        onClick={toggle}
        className="absolute right-4 top-20 z-40 p-2 bg-bg-secondary border border-border-subtle rounded-lg hover:bg-bg-hover transition-colors hidden lg:block"
        title="Open panel"
      >
        <svg className="w-5 h-5 text-text-secondary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M11 19l-7-7 7-7m8 14l-7-7 7-7" />
        </svg>
      </button>
    );
  }

  return (
    <>
      {/* Mobile backdrop */}
      <div 
        className="lg:hidden fixed inset-0 bg-black/50 z-40"
        onClick={toggle}
      />
      <aside className={`
        fixed lg:static inset-y-0 right-0 z-50
        w-[280px] sm:w-[320px] 
        bg-bg-secondary border-l border-border-subtle 
        flex flex-col shrink-0
        transform transition-transform duration-300 ease-out
        ${open ? 'translate-x-0' : 'translate-x-full lg:translate-x-0'}
      `}>

      <div className="flex items-center justify-between p-4 border-b border-border-subtle">
        <h2 className="text-text-primary font-semibold">Details</h2>
        <button
          onClick={toggle}
          className="p-1 hover:bg-bg-hover rounded transition-colors"
          title="Close panel"
        >
          <svg className="w-5 h-5 text-text-secondary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <div className="flex-1 overflow-hidden flex flex-col">
        {selectedSensor ? (
          <div className="p-4 space-y-4">
            <div>
              <h3 className="text-text-secondary text-xs uppercase tracking-wider mb-1">Sensor</h3>
              <p className="text-text-primary font-mono">{selectedSensor}</p>
            </div>
            <div className="h-px bg-border-subtle" />
            <p className="text-text-tertiary text-sm">Sensor data loading...</p>
          </div>
        ) : selectedFacility ? (
          <div className="p-4 space-y-4">
            <div>
              <h3 className="text-text-secondary text-xs uppercase tracking-wider mb-1">Facility</h3>
              <p className="text-text-primary font-mono">{selectedFacility}</p>
            </div>
            <div className="h-px bg-border-subtle" />
            <p className="text-text-tertiary text-sm">Facility data loading...</p>
          </div>
        ) : (
          <AlertFeed />
        )}
      </div>

    </aside>
    </>
  );
};
