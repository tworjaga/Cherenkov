import React, { useMemo } from 'react';
import { useAppStore } from '../../stores/useAppStore';
import { format } from 'date-fns';

interface DataPoint {
  timestamp: Date;
  value: number;
  region: string;
}

export const GlobalChart: React.FC = () => {
  const sensors = useAppStore((state) => state.sensors);
  const alerts = useAppStore((state) => state.alerts);

  // Generate mock historical data (in production, this comes from API)
  const chartData = useMemo(() => {
    const data: DataPoint[] = [];
    const now = new Date();
    const regions = ['North America', 'Europe', 'Asia', 'South America', 'Africa'];
    
    // Generate 24h of 5-minute buckets
    for (let i = 0; i < 288; i++) {
      const timestamp = new Date(now.getTime() - (287 - i) * 5 * 60 * 1000);
      const baseValue = 0.1 + Math.random() * 0.05;
      const spike = i > 250 && i < 260 ? 0.3 : 0; // Simulate an event
      
      data.push({
        timestamp,
        value: baseValue + spike + Math.sin(i / 20) * 0.02,
        region: regions[i % regions.length],
      });
    }
    
    return data;
  }, [sensors]);

  // Calculate statistics
  const stats = useMemo(() => {
    const values = chartData.map(d => d.value);
    const current = values[values.length - 1];
    const avg = values.reduce((a, b) => a + b, 0) / values.length;
    const max = Math.max(...values);
    const min = Math.min(...values);
    
    return { current, avg, max, min };
  }, [chartData]);

  // Chart dimensions
  const width = 600;
  const height = 120;
  const padding = { top: 10, right: 10, bottom: 30, left: 50 };
  const chartWidth = width - padding.left - padding.right;
  const chartHeight = height - padding.top - padding.bottom;

  // Scales
  const xScale = (index: number) => (index / (chartData.length - 1)) * chartWidth;
  const yScale = (value: number) => {
    const range = stats.max - stats.min || 1;
    return chartHeight - ((value - stats.min) / range) * chartHeight;
  };

  // Generate path
  const pathData = useMemo(() => {
    if (chartData.length === 0) return '';
    
    return chartData.map((d, i) => {
      const x = xScale(i);
      const y = yScale(d.value);
      return `${i === 0 ? 'M' : 'L'} ${x} ${y}`;
    }).join(' ');
  }, [chartData]);

  // Generate area path
  const areaPath = useMemo(() => {
    if (chartData.length === 0) return '';
    
    const line = chartData.map((d, i) => {
      const x = xScale(i);
      const y = yScale(d.value);
      return `${i === 0 ? 'M' : 'L'} ${x} ${y}`;
    }).join(' ');
    
    return `${line} L ${chartWidth} ${chartHeight} L 0 ${chartHeight} Z`;
  }, [chartData]);

  // Event markers from alerts
  const eventMarkers = useMemo(() => {
    return alerts
      .filter(a => a.severity === 'CRITICAL' || a.severity === 'HIGH')
      .slice(0, 5)
      .map((alert, i) => ({
        x: chartWidth - (i + 1) * 50,
        severity: alert.severity,
        title: alert.title,
      }));
  }, [alerts, chartWidth]);

  return (
    <div className="flex flex-col h-full p-4">
      <div className="flex items-center justify-between mb-2">
        <div>
          <h3 className="text-xs uppercase tracking-wider text-text-secondary">Global Radiation</h3>
          <div className="flex items-baseline gap-2">
            <span className="text-2xl font-mono font-semibold text-text-primary">
              {stats.current.toFixed(3)}
            </span>
            <span className="text-xs text-text-tertiary">μSv/h</span>
          </div>
        </div>
        
        <div className="flex gap-4 text-xs">
          <div>
            <span className="text-text-tertiary">Avg:</span>
            <span className="ml-1 font-mono text-text-secondary">{stats.avg.toFixed(3)}</span>
          </div>
          <div>
            <span className="text-text-tertiary">Max:</span>
            <span className="ml-1 font-mono text-alert-high">{stats.max.toFixed(3)}</span>
          </div>
          <div>
            <span className="text-text-tertiary">Min:</span>
            <span className="ml-1 font-mono text-alert-normal">{stats.min.toFixed(3)}</span>
          </div>
        </div>
      </div>

      <div className="flex-1 relative">
        <svg
          width="100%"
          height="100%"
          viewBox={`0 0 ${width} ${height}`}
          preserveAspectRatio="none"
          className="overflow-visible"
        >
          <defs>
            <linearGradient id="chartGradient" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor="#00d4ff" stopOpacity="0.3" />
              <stop offset="100%" stopColor="#00d4ff" stopOpacity="0" />
            </linearGradient>
          </defs>

          {/* Grid lines */}
          {[0, 0.25, 0.5, 0.75, 1].map((t) => (
            <line
              key={t}
              x1={0}
              y1={padding.top + t * chartHeight}
              x2={width}
              y2={padding.top + t * chartHeight}
              stroke="rgba(255,255,255,0.05)"
              strokeWidth={1}
            />
          ))}

          {/* Area fill */}
          <path
            d={areaPath}
            fill="url(#chartGradient)"
            transform={`translate(${padding.left}, ${padding.top})`}
          />

          {/* Line */}
          <path
            d={pathData}
            fill="none"
            stroke="#00d4ff"
            strokeWidth={2}
            transform={`translate(${padding.left}, ${padding.top})`}
          />

          {/* Event markers */}
          {eventMarkers.map((marker, i) => (
            <g key={i} transform={`translate(${padding.left + marker.x}, ${padding.top})`}>
              <line
                y1={0}
                y2={chartHeight}
                stroke={marker.severity === 'CRITICAL' ? '#ff3366' : '#ff6b35'}
                strokeWidth={2}
                strokeDasharray="4 2"
              />
              <circle
                cy={chartHeight / 2}
                r={4}
                fill={marker.severity === 'CRITICAL' ? '#ff3366' : '#ff6b35'}
              />
              <title>{marker.title}</title>
            </g>
          ))}

          {/* X-axis labels */}
          <text x={padding.left} y={height - 5} fill="#606070" fontSize={10} fontFamily="JetBrains Mono">
            {format(chartData[0]?.timestamp || new Date(), 'HH:mm')}
          </text>
          <text x={width - padding.right - 30} y={height - 5} fill="#606070" fontSize={10} fontFamily="JetBrains Mono">
            {format(chartData[chartData.length - 1]?.timestamp || new Date(), 'HH:mm')}
          </text>
        </svg>
      </div>
    </div>
  );
};
