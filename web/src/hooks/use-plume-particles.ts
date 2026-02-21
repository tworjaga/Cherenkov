'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import { useWebSocket } from './use-websocket';

export interface PlumeParticle {
  id: string;
  x: number;
  y: number;
  z: number;
  concentration: number;
  timestamp: string;
}

export interface PlumeParticlesBatch {
  simulationId: string;
  particles: PlumeParticle[];
  timestamp: string;
}

export interface UsePlumeParticlesOptions {
  simulationId: string;
  batchSize?: number;
  enabled?: boolean;
  onBatchReceived?: (batch: PlumeParticlesBatch) => void;
}

export interface UsePlumeParticlesReturn {
  particles: PlumeParticle[];
  isConnected: boolean;
  error: Error | null;
  lastBatchTime: Date | null;
  totalParticles: number;
  clearParticles: () => void;
}

export function usePlumeParticles(options: UsePlumeParticlesOptions): UsePlumeParticlesReturn {
  const { simulationId, batchSize = 100, enabled = true, onBatchReceived } = options;
  
  const [particles, setParticles] = useState<PlumeParticle[]>([]);
  const [lastBatchTime, setLastBatchTime] = useState<Date | null>(null);
  const [error, setError] = useState<Error | null>(null);
  
  const particlesRef = useRef<PlumeParticle[]>([]);
  const maxParticlesRef = useRef(10000); // Limit to prevent memory issues
  
  // Update ref when particles change
  useEffect(() => {
    particlesRef.current = particles;
  }, [particles]);
  
  const handleMessage = useCallback((data: unknown) => {
    try {
      const batch = data as PlumeParticlesBatch;
      
      if (!batch.particles || !Array.isArray(batch.particles)) {
        return;
      }
      
      // Add new particles
      const newParticles = batch.particles;
      
      setParticles(prev => {
        const combined = [...prev, ...newParticles];
        // Keep only the most recent particles to prevent memory issues
        if (combined.length > maxParticlesRef.current) {
          return combined.slice(-maxParticlesRef.current);
        }
        return combined;
      });
      
      setLastBatchTime(new Date());
      
      // Call optional callback
      if (onBatchReceived) {
        onBatchReceived(batch);
      }
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Failed to process particle batch'));
    }
  }, [onBatchReceived]);
  
  const { isConnected } = useWebSocket({
    url: enabled ? `/ws/plume/${simulationId}?batchSize=${batchSize}` : null,
    onMessage: handleMessage,
    reconnectInterval: 5000,
    maxReconnectAttempts: 10,
  });
  
  const clearParticles = useCallback(() => {
    setParticles([]);
    setLastBatchTime(null);
    setError(null);
  }, []);
  
  // Clear particles when simulation changes
  useEffect(() => {
    clearParticles();
  }, [simulationId, clearParticles]);
  
  return {
    particles,
    isConnected,
    error,
    lastBatchTime,
    totalParticles: particles.length,
    clearParticles,
  };
}

// Hook for managing particle animation state
export function useParticleAnimation(
  particles: PlumeParticle[],
  options: {
    animationSpeed?: number;
    fadeOutTime?: number;
    maxAge?: number;
  } = {}
) {
  const { animationSpeed = 1, fadeOutTime = 5000, maxAge = 30000 } = options;
  
  const [animatedParticles, setAnimatedParticles] = useState<PlumeParticle[]>([]);
  const animationRef = useRef<number | null>(null);
  const startTimeRef = useRef<number>(Date.now());
  
  useEffect(() => {
    startTimeRef.current = Date.now();
  }, [particles]);
  
  useEffect(() => {
    const animate = () => {
      const now = Date.now();
      const elapsed = now - startTimeRef.current;
      
      // Filter particles based on age and animation progress
      const visibleParticles = particles.filter(p => {
        const particleTime = new Date(p.timestamp).getTime();
        const age = now - particleTime;
        return age < maxAge;
      });
      
      setAnimatedParticles(visibleParticles);
      
      animationRef.current = requestAnimationFrame(animate);
    };
    
    animationRef.current = requestAnimationFrame(animate);
    
    return () => {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
    };
  }, [particles, maxAge]);
  
  return animatedParticles;
}

// Utility to convert particles to deck.gl format
export function particlesToDeckGlFormat(
  particles: PlumeParticle[],
  options: {
    getPosition?: (p: PlumeParticle) => [number, number, number];
    getColor?: (p: PlumeParticle) => [number, number, number, number];
    getRadius?: (p: PlumeParticle) => number;
  } = {}
) {
  const {
    getPosition = (p) => [p.x, p.y, p.z],
    getColor = (p) => {
      // Color based on concentration (red = high, yellow = medium, green = low)
      const intensity = Math.min(1, Math.max(0, p.concentration / 100));
      return [
        Math.floor(255 * intensity),
        Math.floor(255 * (1 - intensity)),
        0,
        180,
      ];
    },
    getRadius = (p) => Math.max(5, Math.min(50, p.concentration / 10)),
  } = options;
  
  return particles.map(p => ({
    position: getPosition(p),
    color: getColor(p),
    radius: getRadius(p),
    concentration: p.concentration,
    id: p.id,
  }));
}
