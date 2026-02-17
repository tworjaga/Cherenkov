'use client';

import { useState } from 'react';
import { motion } from 'framer-motion';
import { ChevronDown, ChevronUp, Activity, MapPin, Clock } from 'lucide-react';
import { useAppStore, useDataStore } from '@/stores';
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
  globalTimeSeries: TimeSeriesPoint[];
  regionalStats: RegionalStat[];
  recentEvents: Alert[];
}

export const BottomPanel = ({
  globalTimeSeries,
  regionalStats,
  recentEvents,
}: BottomPanelProps) => {
  const { bottomPanelOpen, toggleBottomPanel } = useAppStore();
  const [activeTab, setActiveTab] = useState<'chart' | 'regions' | 'events'>('chart');

  if (!bottomPanelOpen) {
    return (
      <motion.button
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        onClick={toggleBottomPanel}
        className="absolute bottom-4 left-1/2 -translate-x-1/2 flex items-center gap-2 px-4 py-2 bg-bg-secondary/90 backdrop-blur-md rounded-full border border-border-subtle text-text-secondary hover:text-text-primary transition-colors z-10"
      >
        <ChevronUp size={16} />
        <span className="text-body-xs uppercase tracking-wider">Show Analytics</span>
      </motion.button>
    );
  }

  return (
    <motion.div
      initial={{ height: 0, opacity: 0 }}
      animate={{ height: 200, opacity: 1 }}
      exit={{ height: 0, opacity: 0 }}
      transition={{ duration: 0.3, ease: [0.4, 0, 0.2, 1] }}
      className="flex flex-col h-bottom-panel bg-bg-secondary border-t border-border-subtle overflow-hidden"
    >
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-2 border-b border-border-subtle">
        <div className="flex items-center gap-4">
          <button
            onClick={() => setActiveTab('chart')}
            className={`text-heading-xs transition-colors ${
              activeTab === 'chart' ? 'text-accent-primary' : 'text-text-secondary hover:text-text-primary'
            }`}
          >
            GLOBAL RADIATION
          </button>
          <button
            onClick={() => setActiveTab('regions')}
            className={`text-heading-xs transition-colors ${
              activeTab === 'regions' ? 'text-accent-primary' : 'text-text-secondary hover:text-text-primary'
            }`}
          >
            REGIONAL STATS
          </button>
          <button
            onClick={() => setActiveTab('events')}
            className={`text-heading-xs transition-colors ${
              activeTab === 'events' ? 'text-accent-primary' : 'text-text-secondary hover:text-text-primary'
            }`}
          >
            RECENT EVENTS
          </button>
        </div>
        <button
          onClick={toggleBottomPanel}
          className="p-1 rounded hover:bg-bg-hover text-text-tertiary hover:text-text-primary transition-colors"
        >
          <ChevronDown size={18} />
        </button>
      </div>

      {/* Content */}
      <div className="flex-1 p-4 overflow-hidden">
        {activeTab === 'chart' && (
          <div className="w-full h-full">
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
                  tickFormatter={(value) => {
                    const date = new Date(value * 1000);
                    return `${date.getHours()}:${date.getMinutes().toString().padStart(2, '0')}`;
                  }}
                  stroke="#606070"
                  tick={{ fill: '#606070', fontSize: 11 }}
                />
                <YAxis
                  stroke="#606070"
                  tick={{ fill: '#606070', fontSize: 11 }}
                  tickFormatter={(value) => `${value.toFixed(2)} μSv/h`}
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
                  labelFormatter={(label) => formatTimestamp(label as number)}
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
          <div className="w-full h-full">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={regionalStats} layout="vertical">
                <CartesianGrid strokeDasharray="3 3" stroke="#1f1f2e" horizontal={false} />
                <XAxis
                  type="number"
                  stroke="#606070"
                  tick={{ fill: '#606070', fontSize: 11 }}
                  tickFormatter={(value) => `${value.toFixed(1)} μSv/h`}
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
          <div className="h-full overflow-y-auto scrollbar-thin scrollbar-thumb-border-active scrollbar-track-transparent">
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
  );
};
