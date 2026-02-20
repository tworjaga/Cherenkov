'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import { getWsClient } from '@/lib/graphql/client';
import { PLUME_PARTICLES } from '@/lib/graphql/subscriptions';


export interface PlumeParticle {
  id: string;
  x: number;
  y: number;
  z: number;
  concentration: number;
  timestamp: number;
}

export interface PlumeParticleBatch {
  simulationId: string;
  particles: PlumeParticle[];
  timestamp: number;
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
  simulationId: string | null;
}


export function usePlumeSimulation() {
  const [state, setState] = useState<SimulationState>({
    particles: [],
    currentTime: 0,
    maxTime: 24,
    isRunning: false,
    progress: 0,
    simulationId: null,
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
    const simulationId = `sim-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;

    setState((prev) => ({ ...prev, simulationId }));

    const unsubscribe = client.subscribe(
      {
        query: PLUME_PARTICLES,
        variables: {
          simulationId,
          batchSize: 100,
        },
      },
      {
        next: (data) => {
          setConnected(true);
          if (data.data?.plumeParticles) {
            const batch = data.data.plumeParticles as PlumeParticleBatch;
            setState((prev) => ({
              ...prev,
              particles: [...prev.particles, ...batch.particles],
              currentTime: batch.timestamp,
              progress: (batch.timestamp / params.duration) * 100,
            }));
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
      simulationId: null,
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
