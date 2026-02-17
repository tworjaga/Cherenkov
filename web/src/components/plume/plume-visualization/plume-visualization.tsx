'use client';

import React from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Globe } from '@/components/globe/globe';

export interface PlumeVisualizationProps {
  simulationData?: {
    center: [number, number];
    radius: number;
    concentration: number[];
  };
}

export function PlumeVisualization({ simulationData }: PlumeVisualizationProps) {
  return (
    <Card className="h-full">
      <CardHeader>
        <CardTitle>Plume Visualization</CardTitle>
      </CardHeader>
      <CardContent className="h-[500px]">
        <Globe
          showPlumeLayer={true}
          selectedSensorId={null}
          onSensorSelect={() => {}}
        />
      </CardContent>
    </Card>
  );
}
