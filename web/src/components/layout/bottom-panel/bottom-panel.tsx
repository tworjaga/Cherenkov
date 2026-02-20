'use client';

import { useState, useCallback, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { ChevronDown, ChevronUp, Activity, MapPin, Clock, X } from 'lucide-react';
import { useAppStore } from '@/stores';
import { useLayout } from '@/components/providers';
import { TimeSeriesPoint, RegionalStat, Alert } from '@/types';
import { formatTimestamp, formatDoseRate } from '@/lib/utils/formatters';
import { getSeverityColor } from '@/lib/utils/calculations';
import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  BarChart,
  Bar,
  Cell,
} from 'recharts';

interface BottomPanelProps {
  globalTimeSeries?: TimeSeriesPoint[];
  regionalStats?: RegionalStat[];
  recentEvents?: Alert[];
}

export const BottomPanel = ({
  globalTimeSeries = [],
  regionalStats = [],
  recentEvents = [],
}: BottomPanelProps) => {
  const { bottomPanelOpen, toggleBottomPanel } = useAppStore();
  const { isMobile } = useLayout();
  const [activeTab, setActiveTab] = useState<'chart' | 'regions' | 'events'>('chart');

  // Handle escape key to close panel on mobile
  const handleKeyDown = useCallback((e: KeyboardEvent) => {
    if (e.key === 'Escape' && isMobile && bottomPanelOpen) {
      toggleBottomPanel();
    }
  }, [isMobile, bottomPanelOpen, toggleBottomPanel]);

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);

  if (!bottomPanelOpen) {
    return (
      <motion.button
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        onClick={toggleBottomPanel}
        className="absolute bottom-4 left-1/2 -translate-x-1/2 flex items-center gap-2 px-4 py-2 bg-bg-secondary/90 backdrop-blur-md rounded-full border border-border-subtle text-text-secondary hover:text-text-primary transition-colors z-10 focus:outline-none focus:ring-2 focus:ring-accent-primary/50"
        aria-label="Show analytics panel"
        aria-expanded="false"
      >
        <ChevronUp size={16} aria-hidden="true" />
        <span className="text-body-xs uppercase tracking-wider">Show Analytics</span>
      </motion.button>
    );
  }

  // Mobile overlay backdrop
  const MobileOverlay = () => (
    <AnimatePresence>
      {isMobile && bottomPanelOpen && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.2 }}
          className="fixed inset-0 bg-black/50 z-30 md:hidden"
          onClick={toggleBottomPanel}
          aria-hidden="true"
        />
      )}
    </AnimatePresence>
  );

  return (
    <>
      <MobileOverlay />
      <motion.div
        data-testid="bottom-panel"
        initial={{ height: 0, opacity: 0, y: 100 }}
        animate={{ 
          height: isMobile ? 'auto' : 200, 
          opacity: 1, 
          y: 0 
        }}
        exit={{ height: 0, opacity: 0, y: 100 }}
        transition={{ duration: 0.3, ease: [0.4, 0, 0.2, 1] }}
        className={`
          flex flex-col bg-bg-secondary border-t border-border-subtle overflow-hidden z-40
          ${isMobile ? 'fixed bottom-0 left-0 right-0 max-h-[60vh] rounded-t-lg shadow-2xl' : 'h-bottom-panel relative'}
        `}
        role="complementary"
        aria-label="Analytics panel"
      >

        {/* Header */}
        <div className="flex items-center justify-between px-4 py-2 border-b border-border-subtle flex-shrink-0">
          <div className="flex items-center gap-2 md:gap-4 overflow-x-auto scrollbar-hide" role="tablist" aria-label="Analytics tabs">
            <button
              data-testid="tab-chart"
              onClick={() => setActiveTab('chart')}
              className={`text-heading-xs transition-colors whitespace-nowrap px-2 py-1 rounded focus:outline-none focus:ring-2 focus:ring-accent-primary/50 ${
                activeTab === 'chart' ? 'text-accent-primary bg-accent-primary/10' : 'text-text-secondary hover:text-text-primary'
              }`}
              role="tab"
              aria-selected={activeTab === 'chart'}
              aria-controls="tab-panel-chart"
              tabIndex={activeTab === 'chart' ? 0 : -1}
            >
              GLOBAL RADIATION
            </button>
            <button
              data-testid="tab-regions"
              onClick={() => setActiveTab('regions')}
              className={`text-heading-xs transition-colors whitespace-nowrap px-2 py-1 rounded focus:outline-none focus:ring-2 focus:ring-accent-primary/50 ${
                activeTab === 'regions' ? 'text-accent-primary bg-accent-primary/10' : 'text-text-secondary hover:text-text-primary'
              }`}
              role="tab"
              aria-selected={activeTab === 'regions'}
              aria-controls="tab-panel-regions"
              tabIndex={activeTab === 'regions' ? 0 : -1}
            >
              REGIONAL STATS
            </button>
            <button
              data-testid="tab-events"
              onClick={() => setActiveTab('events')}
              className={`text-heading-xs transition-colors whitespace-nowrap px-2 py-1 rounded focus:outline-none focus:ring-2 focus:ring-accent-primary/50 ${
                activeTab === 'events' ? 'text-accent-primary bg-accent-primary/10' : 'text-text-secondary hover:text-text-primary'
              }`}
              role="tab"
              aria-selected={activeTab === 'events'}
              aria-controls="tab-panel-events"
              tabIndex={activeTab === 'events' ? 0 : -1}
            >
              RECENT EVENTS
            </button>
          </div>
          <button
            onClick={toggleBottomPanel}
            className="p-1 rounded hover:bg-bg-hover text-text-tertiary hover:text-text-primary transition-colors focus:outline-none focus:ring-2 focus:ring-accent-primary/50 flex-shrink-0"
            aria-label="Hide analytics panel"
            aria-expanded={bottomPanelOpen}
          >
            {isMobile ? <X size={18} aria-hidden="true" /> : <ChevronDown size={18} aria-hidden="true" />}
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 p-4 overflow-hidden min-h-0">
          {activeTab === 'chart' && (
            <div 
              id="tab-panel-chart"
              className="w-full h-full"
              role="tabpanel"
              aria-labelledby="tab-chart"
            >
              <ResponsiveContainer width="100%" height="100%">
                <AreaChart data={globalTimeSeries}>
                  <defs>
                    <linearGradient id="doseGradient" x1="0" y1="0" x2="0" y2="1">
                      <stop offset="5%" stopColor="#00d4ff" stopOpacity={0.3} />
                      <stop offset="95%" stopColor="#00d4ff" stopOpacity={0} />
                    </linearGradient>
                  </defs>
                  <CartesianGrid strokeDasharray="3 3" stroke="#1f1f2e" />
                  <XAxis
                    dataKey="timestamp"
                    tickFormatter={(value: number) => {
                      const date = new Date(value * 1000);
                      return `${date.getHours()}:${date.getMinutes().toString().padStart(2, '0')}`;
                    }}
                    stroke="#606070"
                    tick={{ fill: '#606070', fontSize: 11 }}
                  />
                  <YAxis
                    stroke="#606070"
                    tick={{ fill: '#606070', fontSize: 11 }}
                    tickFormatter={(value: number) => `${value.toFixed(2)} μSv/h`}
                  />
                  <Tooltip
                    contentStyle={{
                      backgroundColor: '#12121a',
                      border: '1px solid #2a2a3d',
                      borderRadius: '4px',
                    }}
                    labelStyle={{ color: '#a0a0b0' }}
                    itemStyle={{ color: '#00d4ff' }}
                    formatter={(value: number) => [formatDoseRate(value), 'Dose Rate']}
                    labelFormatter={(label: number) => formatTimestamp(label)}
                  />
                  <Area
                    type="monotone"
                    dataKey="value"
                    stroke="#00d4ff"
                    strokeWidth={2}
                    fillOpacity={1}
                    fill="url(#doseGradient)"
                  />
                </AreaChart>
              </ResponsiveContainer>
            </div>
          )}

          {activeTab === 'regions' && (
            <div 
              id="tab-panel-regions"
              className="w-full h-full"
              role="tabpanel"
              aria-labelledby="tab-regions"
            >
              <ResponsiveContainer width="100%" height="100%">
                <BarChart data={regionalStats} layout="vertical">
                  <CartesianGrid strokeDasharray="3 3" stroke="#1f1f2e" horizontal={false} />
                  <XAxis
                    type="number"
                    stroke="#606070"
                    tick={{ fill: '#606070', fontSize: 11 }}
                    tickFormatter={(value: number) => `${value.toFixed(1)} μSv/h`}
                  />
                  <YAxis
                    type="category"
                    dataKey="region"
                    stroke="#606070"
                    tick={{ fill: '#a0a0b0', fontSize: 11 }}
                    width={100}
                  />
                  <Tooltip
                    contentStyle={{
                      backgroundColor: '#12121a',
                      border: '1px solid #2a2a3d',
                      borderRadius: '4px',
                    }}
                    cursor={{ fill: '#1a1a25' }}
                  />
                  <Bar dataKey="averageDose" radius={[0, 4, 4, 0]}>
                    {regionalStats.map((entry, index) => (
                      <Cell
                        key={`cell-${index}`}
                        fill={getSeverityColor(
                          entry.averageDose > 10 ? 'critical' :
                          entry.averageDose > 5 ? 'high' :
                          entry.averageDose > 2 ? 'medium' :
                          entry.averageDose > 0.5 ? 'low' : 'normal'
                        )}
                      />
                    ))}
                  </Bar>
                </BarChart>
              </ResponsiveContainer>
            </div>
          )}

          {activeTab === 'events' && (
            <div 
              id="tab-panel-events"
              className="h-full overflow-y-auto scrollbar-thin scrollbar-thumb-border-active scrollbar-track-transparent"
              role="tabpanel"
              aria-labelledby="tab-events"
            >
              <div className="space-y-2">
                {recentEvents.length === 0 ? (
                  <div className="flex flex-col items-center justify-center h-full text-text-tertiary">
                    <Activity size={24} className="mb-2 opacity-50" />
                    <span className="text-body-sm">No recent events</span>
                  </div>
                ) : (
                  recentEvents.slice(0, 10).map((event) => (
                    <div
                      key={event.id}
                      className="flex items-center gap-3 p-2 rounded-md bg-bg-tertiary hover:bg-bg-hover transition-colors cursor-pointer"
                    >
                      <div
                        className="w-2 h-2 rounded-full"
                        style={{ backgroundColor: getSeverityColor(event.severity) }}
                      />
                      <div className="flex-1 min-w-0">
                        <p className="text-body-sm text-text-primary truncate">{event.message}</p>
                        <div className="flex items-center gap-3 text-mono-xs text-text-tertiary">
                          <span className="flex items-center gap-1">
                            <Clock size={10} />
                            {formatTimestamp(event.timestamp)}
                          </span>
                          {event.location && (
                            <span className="flex items-center gap-1">
                              <MapPin size={10} />
                              {event.location.lat.toFixed(2)}°N, {event.location.lon.toFixed(2)}°E
                            </span>
                          )}
                        </div>
                      </div>
                    </div>
                  ))
                )}
              </div>
            </div>
          )}
        </div>
      </motion.div>
    </>
  );
};
