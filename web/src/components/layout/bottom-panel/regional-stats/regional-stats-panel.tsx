'use client';

import { motion } from 'framer-motion';
import { useDataStore } from '@/stores';
import { animations } from '@/styles/theme';
import { RegionalStat } from '@/types';

export const RegionalStatsPanel = () => {
  const { sensors, alerts } = useDataStore();

  // Calculate regional statistics from sensor data
  // Group by approximate regions based on latitude
  const regionalData = sensors.reduce((acc, sensor) => {
    // Determine region based on latitude
    let region = 'Unknown';
    const lat = sensor.location.lat;
    if (lat > 60) region = 'Arctic';
    else if (lat > 35) region = 'North Temperate';
    else if (lat > 0) region = 'Tropical';
    else if (lat > -35) region = 'South Temperate';
    else region = 'Antarctic';

    if (!acc[region]) {
      acc[region] = { total: 0, count: 0, max: 0, alertCount: 0 };
    }
    if (sensor.lastReading) {
      acc[region].total += sensor.lastReading.doseRate;
      acc[region].count += 1;
      acc[region].max = Math.max(acc[region].max, sensor.lastReading.doseRate);
    }
    return acc;
  }, {} as Record<string, { total: number; count: number; max: number; alertCount: number }>);

  // Count alerts per region
  alerts.forEach((alert) => {
    if (alert.location) {
      const lat = alert.location.lat;
      let region = 'Unknown';
      if (lat > 60) region = 'Arctic';
      else if (lat > 35) region = 'North Temperate';
      else if (lat > 0) region = 'Tropical';
      else if (lat > -35) region = 'South Temperate';
      else region = 'Antarctic';
      
      if (regionalData[region]) {
        regionalData[region].alertCount += 1;
      }
    }
  });

  const stats: RegionalStat[] = Object.entries(regionalData)
    .map(([region, data]) => ({
      region,
      averageDose: data.count > 0 ? data.total / data.count : 0,
      sensorCount: data.count,
      alertCount: data.alertCount,
    }))
    .sort((a, b) => b.averageDose - a.averageDose)
    .slice(0, 5);

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={animations.slideIn.transition}
      className="h-full flex flex-col"
    >
      <div className="flex items-center justify-between mb-3">
        <h3 className="text-heading-xs">Regional Statistics</h3>
        <span className="text-body-xs text-text-tertiary">
          Top 5 by average dose
        </span>
      </div>

      <div className="flex-1 overflow-hidden space-y-2">
        {stats.map((stat, index) => (
          <div
            key={stat.region}
            className="flex items-center justify-between p-2 bg-bg-tertiary rounded"
          >
            <div className="flex items-center gap-2">
              <span className="text-body-xs text-text-tertiary w-4">{index + 1}</span>
              <span className="text-body-sm text-text-primary">{stat.region}</span>
            </div>
            <div className="flex items-center gap-4">
              <span className="text-mono-xs text-accent-primary">
                {stat.averageDose.toFixed(3)} μSv/h
              </span>
              <span className="text-body-xs text-text-tertiary">
                {stat.sensorCount} sensors
              </span>
              {stat.alertCount > 0 && (
                <span className="text-body-xs text-alert-critical">
                  {stat.alertCount} alerts
                </span>
              )}
            </div>
          </div>
        ))}
        {stats.length === 0 && (
          <div className="flex items-center justify-center h-full text-text-tertiary text-body-xs">
            No regional data available
          </div>
        )}
      </div>
    </motion.div>
  );
};
