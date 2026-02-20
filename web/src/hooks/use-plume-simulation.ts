'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import { getWsClient } from '@/lib/graphql/client';

export interface PlumeParticle {
  id: string;
  lat: number;
  lng: number;
  altitude: number;
  concentration: number;
  doseRate: number;
  timestamp: number;
  isotope: string;
}

export interface SimulationParams {
  releaseLat: number;
  releaseLng: number;
  releaseHeight: number;
  releaseRate: number;
  duration: number;
  isotope: string;
  windSpeed: number;
  windDirection: number;
  stabilityClass: 'A' | 'B' | 'C' | 'D' | 'E' | 'F';
}

export interface SimulationState {
  particles: PlumeParticle[];
  currentTime: number;
  maxTime: number;
  isRunning: boolean;
  progress: number;
}

export function usePlumeSimulation() {
  const [state, setState] = useState<SimulationState>({
    particles: [],
    currentTime: 0,
    maxTime: 24,
    isRunning: false,
    progress: 0,
  });

  const [params, setParams] = useState<SimulationParams>({
    releaseLat: 0,
    releaseLng: 0,
    releaseHeight: 0,
    releaseRate: 1000,
    duration: 24,
    isotope: 'Cs-137',
    windSpeed: 5,
    windDirection: 0,
    stabilityClass: 'D',
  });

  const [connected, setConnected] = useState(false);
  const unsubscribeRef = useRef<(() => void) | null>(null);

  const startSimulation = useCallback(() => {
    const client = getWsClient();

    const unsubscribe = client.subscribe(
      {
        query: `
          subscription {
            plumeSimulation {
              particles {
                id
                lat
                lng
                altitude
                concentration
                doseRate
                timestamp
                isotope
              }
              currentTime
              maxTime
              isRunning
            }
          }
        `,
      },
      {
        next: (data) => {
          setConnected(true);
          if (data.data?.plumeSimulation) {
            const sim = data.data.plumeSimulation as SimulationState & { particles: PlumeParticle[] };
            setState({
              particles: sim.particles,
              currentTime: sim.currentTime,
              maxTime: sim.maxTime,
              isRunning: sim.isRunning,
              progress: (sim.currentTime / sim.maxTime) * 100,
            });
          }
        },

        error: (err) => {
          console.error('Plume simulation error:', err);
          setConnected(false);
        },
        complete: () => {
          setConnected(false);
        },
      }
    );

    unsubscribeRef.current = unsubscribe;

    // Send start command via mutation
    client.subscribe(
      {
        query: `
          mutation {
            startPlumeSimulation(
              releaseLat: ${params.releaseLat}
              releaseLng: ${params.releaseLng}
              releaseHeight: ${params.releaseHeight}
              releaseRate: ${params.releaseRate}
              duration: ${params.duration}
              isotope: "${params.isotope}"
              windSpeed: ${params.windSpeed}
              windDirection: ${params.windDirection}
              stabilityClass: "${params.stabilityClass}"
            )
          }
        `,
      },
      {
        next: () => {
          setState((prev) => ({ ...prev, isRunning: true, currentTime: 0 }));
        },
        error: (err) => {
          console.error('Failed to start simulation:', err);
        },
        complete: () => {},
      }
    );
  }, [params]);

  const stopSimulation = useCallback(() => {
    const client = getWsClient();
    
    client.subscribe(
      {
        query: `
          mutation {
            stopPlumeSimulation
          }
        `,
      },
      {
        next: () => {
          setState((prev) => ({ ...prev, isRunning: false }));
        },
        error: (err) => {
          console.error('Failed to stop simulation:', err);
        },
        complete: () => {},
      }
    );

    if (unsubscribeRef.current) {
      unsubscribeRef.current();
      unsubscribeRef.current = null;
    }
  }, []);

  const resetSimulation = useCallback(() => {
    setState({
      particles: [],
      currentTime: 0,
      maxTime: params.duration,
      isRunning: false,
      progress: 0,
    });
    
    if (unsubscribeRef.current) {
      unsubscribeRef.current();
      unsubscribeRef.current = null;
    }
  }, [params.duration]);

  const updateParams = useCallback((newParams: Partial<SimulationParams>) => {
    setParams((prev) => ({ ...prev, ...newParams }));
  }, []);

  useEffect(() => {
    return () => {
      if (unsubscribeRef.current) {
        unsubscribeRef.current();
      }
    };
  }, []);

  return {
    state,
    params,
    connected,
    startSimulation,
    stopSimulation,
    resetSimulation,
    updateParams,
  };
}
