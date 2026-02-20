'use client';

import React, { useState } from 'react';
import { Play, Pause, RotateCcw, Clock } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Slider } from '@/components/ui/slider';
import { ReleaseParams } from './release-params';
import { WeatherConditions } from './weather-conditions';
import { usePlumeSimulation } from '@/hooks/use-plume-simulation';


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

  const [state, controls] = usePlumeSimulation();


  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  const handleRunSimulation = () => {
    if (state.isPlaying) {
      controls.pause();
    } else {
      controls.play();
    }
  };

  const handleReset = () => {
    controls.reset();
    setParams({
      releaseRate: 100,
      releaseHeight: 50,
      particleSize: 10,
      windSpeed: 5,
      windDirection: 270,
      temperature: 20,
      stabilityClass: 'D',
    });
  };

  const handleSliderChange = (value: number[]) => {
    controls.seekToProgress(value[0]);
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

      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Clock className="w-5 h-5" />
            Temporal Simulation Control
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            <div className="flex items-center gap-4">
              <Button
                variant="outline"
                size="icon"
                onClick={handleRunSimulation}
                className="h-10 w-10"
              >
                {state.isPlaying ? (
                  <Pause className="w-4 h-4" />
                ) : (
                  <Play className="w-4 h-4" />
                )}
              </Button>

              <Button
                variant="outline"
                size="icon"
                onClick={handleReset}
                className="h-10 w-10"
              >
                <RotateCcw className="w-4 h-4" />
              </Button>

              <div className="flex-1 px-4">
                <Slider
                  value={[state.progress]}
                  max={100}
                  step={0.1}
                  onValueChange={handleSliderChange}
                />
              </div>

              <div className="flex items-center gap-2 min-w-[120px] justify-end">
                <span className="text-sm font-mono text-text-primary">
                  {formatTime(state.currentTime)}
                </span>
                <span className="text-xs text-text-tertiary">
                  / {formatTime(state.duration)}
                </span>
              </div>
            </div>

            <div className="flex items-center justify-between text-xs text-text-tertiary">
              <div className="flex items-center gap-2">
                <span>Speed:</span>
                <div className="flex gap-1">
                  {[0.5, 1, 2, 5, 10].map((speed) => (
                    <button
                      key={speed}
                      onClick={() => controls.setSpeed(speed)}
                      className={`px-2 py-0.5 rounded ${
                        state.speed === speed
                          ? 'bg-accent-primary/20 text-accent-primary'
                          : 'hover:bg-bg-hover'
                      }`}
                    >
                      {speed}x
                    </button>
                  ))}
                </div>
              </div>
              <div className="flex items-center gap-2">
                <label className="flex items-center gap-1 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={state.isLooping}
                    onChange={controls.toggleLoop}
                    className="w-3 h-3"
                  />
                  Loop
                </label>
                <span>Progress: {state.progress.toFixed(1)}%</span>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      <div className="flex justify-end gap-4">
        <Button variant="outline" onClick={handleReset}>
          Reset All
        </Button>
        <Button
          onClick={handleRunSimulation}
          variant={state.isPlaying ? 'secondary' : 'default'}
        >
          {state.isPlaying ? 'Pause Simulation' : 'Run Simulation'}
        </Button>
      </div>
    </div>
  );
}
