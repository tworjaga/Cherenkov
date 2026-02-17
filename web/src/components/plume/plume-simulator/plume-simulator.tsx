'use client';

import React, { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { ReleaseParams } from './release-params';
import { WeatherConditions } from './weather-conditions';

export interface PlumeSimulationParams {
  releaseRate: number;
  releaseHeight: number;
  particleSize: number;
  windSpeed: number;
  windDirection: number;
  temperature: number;
  stabilityClass: string;
}

export function PlumeSimulator() {
  const [params, setParams] = useState<PlumeSimulationParams>({
    releaseRate: 100,
    releaseHeight: 50,
    particleSize: 10,
    windSpeed: 5,
    windDirection: 270,
    temperature: 20,
    stabilityClass: 'D',
  });

  const [isSimulating, setIsSimulating] = useState(false);

  const handleRunSimulation = () => {
    setIsSimulating(true);
    setTimeout(() => setIsSimulating(false), 2000);
  };

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card>
          <CardHeader>
            <CardTitle>Release Parameters</CardTitle>
          </CardHeader>
          <CardContent>
            <ReleaseParams params={params} onParamsChange={setParams} />
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Weather Conditions</CardTitle>
          </CardHeader>
          <CardContent>
            <WeatherConditions params={params} onParamsChange={setParams} />
          </CardContent>
        </Card>
      </div>

      <div className="flex justify-end gap-4">
        <Button variant="outline" onClick={() => setIsSimulating(false)}>
          Reset
        </Button>
        <Button onClick={handleRunSimulation} disabled={isSimulating}>
          {isSimulating ? 'Running...' : 'Run Simulation'}
        </Button>
      </div>
    </div>
  );
}
