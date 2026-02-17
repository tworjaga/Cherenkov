'use client';

import React from 'react';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Slider } from '@/components/ui/slider';
import { PlumeSimulationParams } from './plume-simulator';

interface ReleaseParamsProps {
  params: PlumeSimulationParams;
  onParamsChange: (params: PlumeSimulationParams) => void;
}

export function ReleaseParams({ params, onParamsChange }: ReleaseParamsProps) {
  const updateParam = <K extends keyof PlumeSimulationParams>(
    key: K,
    value: PlumeSimulationParams[K]
  ) => {
    onParamsChange({ ...params, [key]: value });
  };

  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <div className="flex justify-between">
          <Label htmlFor="releaseRate">Release Rate (Bq/s)</Label>
          <span className="text-sm text-muted-foreground">{params.releaseRate}</span>
        </div>
        <Slider
          id="releaseRate"
          value={[params.releaseRate]}
          onValueChange={([value]) => updateParam('releaseRate', value)}
          min={1}
          max={10000}
          step={10}
        />
      </div>

      <div className="space-y-2">
        <div className="flex justify-between">
          <Label htmlFor="releaseHeight">Release Height (m)</Label>
          <span className="text-sm text-muted-foreground">{params.releaseHeight}</span>
        </div>
        <Slider
          id="releaseHeight"
          value={[params.releaseHeight]}
          onValueChange={([value]) => updateParam('releaseHeight', value)}
          min={0}
          max={500}
          step={5}
        />
      </div>

      <div className="space-y-2">
        <Label htmlFor="particleSize">Particle Size (μm)</Label>
        <Input
          id="particleSize"
          type="number"
          value={params.particleSize}
          onChange={(e) => updateParam('particleSize', Number(e.target.value))}
          min={0.1}
          max={100}
          step={0.1}
        />
      </div>
    </div>
  );
}
