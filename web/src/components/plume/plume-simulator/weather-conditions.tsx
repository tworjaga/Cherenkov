'use client';

import React from 'react';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Slider } from '@/components/ui/slider';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { PlumeSimulationParams } from './plume-simulator';

interface WeatherConditionsProps {
  params: PlumeSimulationParams;
  onParamsChange: (params: PlumeSimulationParams) => void;
}

export function WeatherConditions({ params, onParamsChange }: WeatherConditionsProps) {
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
          <Label htmlFor="windSpeed">Wind Speed (m/s)</Label>
          <span className="text-sm text-muted-foreground">{params.windSpeed}</span>
        </div>
        <Slider
          id="windSpeed"
          value={[params.windSpeed]}
          onValueChange={([value]) => updateParam('windSpeed', value)}
          min={0}
          max={30}
          step={0.5}
        />
      </div>

      <div className="space-y-2">
        <div className="flex justify-between">
          <Label htmlFor="windDirection">Wind Direction (degrees)</Label>
          <span className="text-sm text-muted-foreground">{params.windDirection}</span>
        </div>
        <Slider
          id="windDirection"
          value={[params.windDirection]}
          onValueChange={([value]) => updateParam('windDirection', value)}
          min={0}
          max={360}
          step={5}
        />
      </div>

      <div className="space-y-2">
        <Label htmlFor="temperature">Temperature (C)</Label>
        <Input
          id="temperature"
          type="number"
          value={params.temperature}
          onChange={(e) => updateParam('temperature', Number(e.target.value))}
          min={-50}
          max={50}
          step={1}
        />
      </div>

      <div className="space-y-2">
        <Label htmlFor="stabilityClass">Stability Class</Label>
        <Select
          value={params.stabilityClass}
          onValueChange={(value) => updateParam('stabilityClass', value)}
        >
          <SelectTrigger id="stabilityClass">
            <SelectValue placeholder="Select stability class" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="A">A - Very Unstable</SelectItem>
            <SelectItem value="B">B - Unstable</SelectItem>
            <SelectItem value="C">C - Slightly Unstable</SelectItem>
            <SelectItem value="D">D - Neutral</SelectItem>
            <SelectItem value="E">E - Slightly Stable</SelectItem>
            <SelectItem value="F">F - Stable</SelectItem>
          </SelectContent>
        </Select>
      </div>
    </div>
  );
}
