'use client';

import React, { useMemo } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { GlobeLazy } from '@/components/globe/globe-lazy';
import { Badge } from '@/components/ui/badge';
import { formatDoseRate } from '@/lib/utils/formatters';


export interface PlumePoint {
  lat: number;
  lng: number;
  concentration: number;
  doseRate: number;
}

export interface PlumeVisualizationProps {
  simulationData?: {
    center: [number, number];
    radius: number;
    concentration: number[];
    windDirection: number;
    windSpeed: number;
    stabilityClass: string;
    timeStep: number;
    maxTime: number;
  };
  plumeData?: PlumePoint[];
  isAnimating?: boolean;
  currentTime?: number;
  onTimeChange?: (time: number) => void;
}

const CONCENTRATION_LEVELS = [
  { threshold: 50, color: '#7f1d1d', label: 'Critical', description: '> 50 mSv/h' },
  { threshold: 10, color: '#dc2626', label: 'High', description: '10-50 mSv/h' },
  { threshold: 1, color: '#ea580c', label: 'Medium', description: '1-10 mSv/h' },
  { threshold: 0.1, color: '#f59e0b', label: 'Low', description: '0.1-1 mSv/h' },
  { threshold: 0, color: '#22c55e', label: 'Safe', description: '< 0.1 mSv/h' },
];

export const PlumeVisualization = React.memo(function PlumeVisualization({
  simulationData,
  plumeData = [],
  isAnimating = false,
  currentTime = 0,

}: PlumeVisualizationProps) {
  const layers = useMemo(() => ({
    sensors: true,
    facilities: false,
    anomalies: false,
    plumes: true,
    heatmap: false,
  }), []);



  const concentrationStats = useMemo(() => {
    if (plumeData.length === 0) return null;
    
    const concentrations = plumeData.map(p => p.concentration);
    const maxConcentration = Math.max(...concentrations);
    const avgConcentration = concentrations.reduce((a, b) => a + b, 0) / concentrations.length;
    const maxDoseRate = Math.max(...plumeData.map(p => p.doseRate));
    
    return {
      maxConcentration,
      avgConcentration,
      maxDoseRate,
      pointCount: plumeData.length,
    };
  }, [plumeData]);



  return (
    <Card className="h-full flex flex-col">
      <CardHeader className="pb-2">
        <div className="flex items-center justify-between">
          <CardTitle>Plume Visualization</CardTitle>
          {isAnimating && (
            <Badge variant="default" className="animate-pulse">
              Animating
            </Badge>
          )}
        </div>
        {simulationData && (
          <div className="flex items-center gap-4 text-sm text-muted-foreground mt-2">
            <span>Wind: {simulationData.windSpeed} m/s</span>
            <span>Direction: {simulationData.windDirection}°</span>
            <span>Stability: {simulationData.stabilityClass}</span>
            <span>Time: {currentTime.toFixed(1)}h / {simulationData.maxTime}h</span>
          </div>
        )}
      </CardHeader>
      
      <CardContent className="flex-1 min-h-0 relative">
        <div className="absolute inset-0">
          <GlobeLazy
            layers={layers}
            onSensorSelect={() => {}}
          />
        </div>

        
        {/* Concentration Legend */}
        <div className="absolute bottom-4 left-4 bg-background/90 backdrop-blur-sm rounded-lg border p-3 shadow-lg">
          <h4 className="text-xs font-semibold mb-2">Dose Rate Levels</h4>
          <div className="space-y-1">
            {CONCENTRATION_LEVELS.map((level) => (
              <div key={level.label} className="flex items-center gap-2 text-xs">
                <div
                  className="w-3 h-3 rounded-full"
                  style={{ backgroundColor: level.color }}
                />
                <span className="font-medium">{level.label}</span>
                <span className="text-muted-foreground">({level.description})</span>
              </div>
            ))}
          </div>
        </div>

        {/* Stats Panel */}
        {concentrationStats && (
          <div className="absolute top-4 right-4 bg-background/90 backdrop-blur-sm rounded-lg border p-3 shadow-lg min-w-[180px]">
            <h4 className="text-xs font-semibold mb-2">Concentration Stats</h4>
            <div className="space-y-1 text-xs">
              <div className="flex justify-between">
                <span className="text-muted-foreground">Max Dose:</span>
                <span className="font-medium">
                  {formatDoseRate(concentrationStats.maxDoseRate)}
                </span>

              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Avg Conc:</span>
                <span className="font-medium">
                  {concentrationStats.avgConcentration.toExponential(2)} Bq/m³
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Data Points:</span>
                <span className="font-medium">{concentrationStats.pointCount}</span>
              </div>
            </div>
          </div>
        )}

        {/* Wind Direction Indicator */}
        {simulationData && (
          <div className="absolute bottom-4 right-4 bg-background/90 backdrop-blur-sm rounded-lg border p-3 shadow-lg">
            <div className="text-xs font-semibold mb-1">Wind Direction</div>
            <div className="relative w-16 h-16">
              <div className="absolute inset-0 border-2 border-muted rounded-full" />
              <div
                className="absolute top-1/2 left-1/2 w-6 h-0.5 bg-primary origin-left -translate-y-1/2"
                style={{
                  transform: `translate(-50%, -50%) rotate(${simulationData.windDirection}deg)`,
                }}
              >
                <div className="absolute right-0 -top-1 w-0 h-0 border-l-4 border-l-primary border-y-2 border-y-transparent" />
              </div>
              <div className="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-1 text-[8px] text-muted-foreground">N</div>
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
});
