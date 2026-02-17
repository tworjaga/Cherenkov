'use client';

import { BarChart, Bar, XAxis, YAxis, ResponsiveContainer, Cell } from 'recharts';

import { RegionalStat } from '@/types';


interface RegionalStatsProps {
  stats: RegionalStat[];
}

export const RegionalStats = ({ stats }: RegionalStatsProps) => {
  const sortedStats = [...stats]
    .sort((a, b) => b.averageDose - a.averageDose)
    .slice(0, 5);

  const getBarColor = (dose: number) => {
    if (dose > 10) return '#ff3366';
    if (dose > 5) return '#ff6b35';
    if (dose > 2) return '#ffb800';
    if (dose > 1) return '#00d4ff';
    return '#00ff88';
  };

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between p-3 border-b border-border-subtle">
        <span className="text-heading-xs text-text-secondary">TOP REGIONS</span>
        <span className="text-body-xs text-text-tertiary">By avg dose</span>
      </div>
      
      <div className="flex-1 p-2">
        {sortedStats.length === 0 ? (
          <div className="flex items-center justify-center h-full text-text-tertiary text-body-xs">
            No regional data
          </div>
        ) : (
          <ResponsiveContainer width="100%" height="100%">
            <BarChart
              data={sortedStats}
              layout="vertical"
              margin={{ top: 5, right: 30, left: 40, bottom: 5 }}
            >
              <XAxis type="number" hide />
              <YAxis
                dataKey="region"
                type="category"
                tick={{ fill: '#a0a0b0', fontSize: 11 }}
                width={80}
                axisLine={false}
                tickLine={false}
              />
              <Bar dataKey="averageDose" radius={[0, 4, 4, 0]}>
                {sortedStats.map((entry, index) => (
                  <Cell
                    key={`cell-${index}`}
                    fill={getBarColor(entry.averageDose)}
                  />
                ))}
              </Bar>
            </BarChart>
          </ResponsiveContainer>
        )}
      </div>
    </div>
  );
};
