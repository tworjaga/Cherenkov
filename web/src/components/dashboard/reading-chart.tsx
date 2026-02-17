'use client';

import { useMemo } from 'react';
import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  ResponsiveContainer,
  Tooltip,
} from 'recharts';
import { useDataStore } from '@/stores';
import { formatTimestamp } from '@/lib/utils';
import { Reading, TimeSeriesPoint } from '@/types';


interface ReadingChartProps {
  sensorId?: string;
}

export const ReadingChart = ({ sensorId }: ReadingChartProps) => {
  const { readings, globalTimeSeries } = useDataStore();
  
  const data = useMemo(() => {
    if (sensorId) {
      const sensorReadings: Reading[] = readings[sensorId] || [];
      return sensorReadings.slice(-24).map((r: Reading) => ({
        time: r.timestamp,
        value: r.doseRate,
      }));
    }
    // Global view - use global time series or aggregate all readings
    return globalTimeSeries.slice(-24).map((p: TimeSeriesPoint) => ({
      time: p.timestamp,
      value: p.value,
    }));

  }, [readings, sensorId, globalTimeSeries]);


  if (data.length === 0) {
    return (
      <div className="flex items-center justify-center h-full text-text-tertiary text-body-xs">
        No historical data
      </div>
    );
  }

  return (
    <ResponsiveContainer width="100%" height="100%">
      <AreaChart data={data}>
        <defs>
          <linearGradient id={`gradient-${sensorId || 'global'}`} x1="0" y1="0" x2="0" y2="1">
            <stop offset="5%" stopColor="#00d4ff" stopOpacity={0.3} />
            <stop offset="95%" stopColor="#00d4ff" stopOpacity={0} />
          </linearGradient>
        </defs>

        <XAxis
          dataKey="time"
          tickFormatter={(value: number) => {
            const date = new Date(value);
            return date.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit', hour12: false });
          }}
          stroke="#606070"
          tick={{ fontSize: 10 }}
          tickLine={false}
        />
        <YAxis
          stroke="#606070"
          tick={{ fontSize: 10 }}
          tickLine={false}
          axisLine={false}
        />
        <Tooltip
          contentStyle={{
            backgroundColor: '#12121a',
            border: '1px solid #2a2a3d',
            borderRadius: '4px',
          }}
          labelFormatter={(value: number) => formatTimestamp(value)}
          formatter={(value: number) => [`${value.toFixed(3)} μSv/h`, 'Dose Rate']}
        />
        <Area
          type="monotone"
          dataKey="value"
          stroke="#00d4ff"
          fill={`url(#gradient-${sensorId || 'global'})`}
          strokeWidth={2}
        />

      </AreaChart>
    </ResponsiveContainer>
  );
};
