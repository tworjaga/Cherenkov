'use client';

import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { TimeSeriesPoint } from '@/types/models';

interface SensorChartProps {
  data?: TimeSeriesPoint[];
  title?: string;
}

export function SensorChart({ data = [], title = 'Dose Rate History' }: SensorChartProps) {
  const maxValue = Math.max(...data.map(d => d.value), 0);
  const minValue = Math.min(...data.map(d => d.value), 0);
  const range = maxValue - minValue || 1;

  return (
    <Card className="m-4">
      <CardHeader>
        <CardTitle className="text-sm">{title}</CardTitle>
      </CardHeader>
      <CardContent>
        {data.length === 0 ? (
          <div className="h-32 flex items-center justify-center text-muted-foreground text-sm">
            No historical data available
          </div>
        ) : (
          <div className="h-32 flex items-end gap-1">
            {data.map((point, index) => {
              const height = ((point.value - minValue) / range) * 100;
              return (
                <div
                  key={index}
                  className="flex-1 bg-primary/20 hover:bg-primary/40 transition-colors rounded-sm relative group"
                  style={{ height: `${Math.max(height, 5)}%` }}
                >
                  <div className="absolute bottom-full left-1/2 -translate-x-1/2 mb-1 opacity-0 group-hover:opacity-100 transition-opacity bg-popover text-popover-foreground text-xs px-2 py-1 rounded shadow-lg whitespace-nowrap z-10">
                    {point.value.toFixed(3)} at {new Date(point.timestamp).toLocaleTimeString()}
                  </div>
                </div>
              );
            })}
          </div>
        )}
        <div className="flex justify-between text-xs text-muted-foreground mt-2">
          <span>Min: {minValue.toFixed(3)}</span>
          <span>Max: {maxValue.toFixed(3)}</span>
        </div>
      </CardContent>
    </Card>
  );
}
