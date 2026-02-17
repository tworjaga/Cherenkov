'use client';

import { TimeSeriesPoint } from '@/types';
import { Card, CardHeader, CardContent } from '@/components/ui/card';
import { formatTimestamp, formatDoseRate } from '@/lib/utils';
import { Activity, TrendingUp, TrendingDown } from 'lucide-react';

interface ReadingChartProps {
  data?: TimeSeriesPoint[];
  title?: string;
  unit?: string;
  className?: string;
}


export const ReadingChart = ({ 
  data = [], 
  title = 'Dose Rate History', 
  unit = 'µSv/h',
  className 
}: ReadingChartProps) => {

  if (data.length === 0) {
    return (
      <Card className={className}>
        <CardHeader className="pb-3">
          <div className="flex items-center gap-2">
            <Activity className="w-5 h-5 text-accent-primary" />
            <h3 className="text-body-lg font-semibold text-text-primary">{title}</h3>
          </div>
        </CardHeader>
        <CardContent>
          <p className="text-body-sm text-text-tertiary">No data available</p>
        </CardContent>
      </Card>
    );
  }

  const latest = data[data.length - 1];
  const previous = data.length > 1 ? data[data.length - 2] : latest;
  const trend = latest.value > previous.value ? 'up' : latest.value < previous.value ? 'down' : 'stable';
  const maxValue = Math.max(...data.map(d => d.value));
  const minValue = Math.min(...data.map(d => d.value));
  const avgValue = data.reduce((sum, d) => sum + d.value, 0) / data.length;

  return (
    <Card className={className}>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Activity className="w-5 h-5 text-accent-primary" />
            <h3 className="text-body-lg font-semibold text-text-primary">{title}</h3>
          </div>
          <div className="flex items-center gap-1">
            {trend === 'up' && <TrendingUp className="w-4 h-4 text-alert-warning" />}
            {trend === 'down' && <TrendingDown className="w-4 h-4 text-alert-success" />}
            <span className="text-mono-sm text-text-secondary">{data.length} points</span>
          </div>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="grid grid-cols-3 gap-4">
          <div className="space-y-1">
            <span className="text-body-xs text-text-tertiary">Current</span>
            <p className="text-mono-lg text-accent-primary">{formatDoseRate(latest.value)}</p>
            <span className="text-mono-xs text-text-tertiary">{unit}</span>
          </div>
          <div className="space-y-1">
            <span className="text-body-xs text-text-tertiary">Average</span>
            <p className="text-mono-lg text-text-primary">{formatDoseRate(avgValue)}</p>
            <span className="text-mono-xs text-text-tertiary">{unit}</span>
          </div>
          <div className="space-y-1">
            <span className="text-body-xs text-text-tertiary">Range</span>
            <p className="text-mono-sm text-text-primary">
              {formatDoseRate(minValue)} - {formatDoseRate(maxValue)}
            </p>
            <span className="text-mono-xs text-text-tertiary">{unit}</span>
          </div>
        </div>

        <div className="pt-4 border-t border-border-subtle">
          <h4 className="text-body-sm font-medium text-text-primary mb-2">Recent Readings</h4>
          <div className="space-y-1 max-h-32 overflow-y-auto">
            {data.slice(-5).reverse().map((point, index) => (
              <div key={index} className="flex items-center justify-between py-1">
                <span className="text-mono-xs text-text-tertiary">
                  {formatTimestamp(point.timestamp)}
                </span>
                <span className="text-mono-sm text-text-primary">
                  {formatDoseRate(point.value)} {unit}
                </span>
              </div>
            ))}
          </div>
        </div>
      </CardContent>
    </Card>
  );
};
