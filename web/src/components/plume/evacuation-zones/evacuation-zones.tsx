'use client';

import React, { useMemo } from 'react';
import { Card, CardContent, CardHeader } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { AlertTriangle, Circle, MapPin, Activity } from 'lucide-react';

// Dose thresholds in μSv/h (microsieverts per hour)
// Based on radiation protection standards
const DOSE_THRESHOLDS = {
  CRITICAL: 1000,    // 1 mSv/h - Immediate evacuation
  HIGH: 100,         // 0.1 mSv/h - Shelter in place
  MEDIUM: 10,        // 0.01 mSv/h - Monitoring zone
  LOW: 1,            // 0.001 mSv/h - Precautionary zone
} as const;

interface PlumeParticle {
  x: number;
  y: number;
  z: number;
  concentration: number;
}

interface EvacuationZone {
  id: string;
  name: string;
  radius: number;
  severity: 'critical' | 'high' | 'medium' | 'low';
  population: number;
  instructions: string;
  doseRate: number;
  contourPoints?: Array<{ x: number; y: number }>;
}

interface EvacuationZonesProps {
  zones?: EvacuationZone[];
  particles?: PlumeParticle[];
  releaseLat?: number;
  releaseLng?: number;
  isotope?: string;
}

// Calculate dose rate from particle concentration
// Simplified model: dose rate proportional to concentration
function calculateDoseRate(concentration: number, isotope: string = 'Cs-137'): number {
  // Dose conversion factors (μSv/h per unit concentration)
  const conversionFactors: Record<string, number> = {
    'Cs-137': 0.0001,
    'I-131': 0.0002,
    'Co-60': 0.0003,
    'Sr-90': 0.00015,
  };
  
  const factor = conversionFactors[isotope] || 0.0001;
  return concentration * factor;
}

// Generate contour points for a given dose threshold
function generateContourPoints(
  particles: PlumeParticle[],
  threshold: number,
  isotope: string
): Array<{ x: number; y: number }> {
  if (!particles.length) return [];
  
  // Find particles at or above threshold
  const thresholdParticles = particles.filter(p => {
    const doseRate = calculateDoseRate(p.concentration, isotope);
    return doseRate >= threshold;
  });
  
  if (thresholdParticles.length < 3) return [];
  
  // Sort by angle around centroid for convex hull approximation
  const centroid = thresholdParticles.reduce(
    (acc, p) => ({ x: acc.x + p.x / thresholdParticles.length, y: acc.y + p.y / thresholdParticles.length }),
    { x: 0, y: 0 }
  );
  
  return thresholdParticles
    .map(p => ({
      x: p.x,
      y: p.y,
      angle: Math.atan2(p.y - centroid.y, p.x - centroid.x),
    }))
    .sort((a, b) => a.angle - b.angle)
    .map(({ x, y }) => ({ x, y }));
}

// Calculate evacuation zones from particle data
function calculateZonesFromParticles(
  particles: PlumeParticle[],
  releaseLat: number,
  releaseLng: number,
  isotope: string = 'Cs-137'
): EvacuationZone[] {
  if (!particles.length) {
    return getDefaultZones();
  }
  
  const zones: EvacuationZone[] = [];
  
  // Calculate max radius for each threshold
  const thresholds = [
    { level: 'critical', threshold: DOSE_THRESHOLDS.CRITICAL, name: 'Immediate Evacuation Zone' },
    { level: 'high', threshold: DOSE_THRESHOLDS.HIGH, name: 'Shelter in Place Zone' },
    { level: 'medium', threshold: DOSE_THRESHOLDS.MEDIUM, name: 'Monitoring Zone' },
    { level: 'low', threshold: DOSE_THRESHOLDS.LOW, name: 'Precautionary Zone' },
  ] as const;
  
  for (const { level, threshold, name } of thresholds) {
    const exceedingParticles = particles.filter(p => {
      const doseRate = calculateDoseRate(p.concentration, isotope);
      return doseRate >= threshold;
    });
    
    if (exceedingParticles.length === 0) continue;
    
    // Calculate max distance from release point
    const maxRadius = Math.max(
      ...exceedingParticles.map(p => 
        Math.sqrt(Math.pow(p.x - releaseLng, 2) + Math.pow(p.y - releaseLat, 2))
      )
    );
    
    // Estimate population (simplified model: 1000 people per km² in rural, 5000 in urban)
    const area = Math.PI * Math.pow(maxRadius, 2);
    const population = Math.round(area * 2000);
    
    // Generate contour points
    const contourPoints = generateContourPoints(particles, threshold, isotope);
    
    // Calculate average dose rate in zone
    const avgDoseRate = exceedingParticles.reduce((sum, p) => 
      sum + calculateDoseRate(p.concentration, isotope), 0
    ) / exceedingParticles.length;
    
    zones.push({
      id: level,
      name,
      radius: Math.round(maxRadius * 10) / 10,
      severity: level,
      population,
      doseRate: Math.round(avgDoseRate * 100) / 100,
      contourPoints,
      instructions: getInstructionsForLevel(level),
    });
  }
  
  return zones.length > 0 ? zones : getDefaultZones();
}

function getDefaultZones(): EvacuationZone[] {
  return [
    {
      id: '1',
      name: 'Immediate Evacuation Zone',
      radius: 2,
      severity: 'critical',
      population: 1500,
      doseRate: 1000,
      instructions: 'Evacuate immediately. Move perpendicular to wind direction.',
    },
    {
      id: '2',
      name: 'Shelter in Place Zone',
      radius: 5,
      severity: 'high',
      population: 8500,
      doseRate: 100,
      instructions: 'Close all windows and doors. Turn off ventilation.',
    },
    {
      id: '3',
      name: 'Monitoring Zone',
      radius: 10,
      severity: 'medium',
      population: 25000,
      doseRate: 10,
      instructions: 'Stay alert for updates. Prepare for potential evacuation.',
    },
  ];
}

function getInstructionsForLevel(level: string): string {
  const instructions: Record<string, string> = {
    critical: 'Evacuate immediately. Move perpendicular to wind direction. Seek medical attention if exposed.',
    high: 'Close all windows and doors. Turn off ventilation. Move to interior rooms.',
    medium: 'Stay alert for updates. Prepare for potential evacuation. Limit outdoor activities.',
    low: 'Monitor local news. Follow official guidance. No immediate action required.',
  };
  return instructions[level] || 'Follow official guidance.';
}

const severityColors = {
  critical: 'bg-red-500',
  high: 'bg-orange-500',
  medium: 'bg-yellow-500',
  low: 'bg-blue-500',
};

const severityLabels = {
  critical: 'Critical',
  high: 'High',
  medium: 'Medium',
  low: 'Low',
};

export function EvacuationZones({ 
  zones: propZones, 
  particles = [],
  releaseLat = 0,
  releaseLng = 0,
  isotope = 'Cs-137',
}: EvacuationZonesProps) {
  const zones = useMemo(() => {
    if (propZones) return propZones;
    if (particles.length > 0) {
      return calculateZonesFromParticles(particles, releaseLat, releaseLng, isotope);
    }
    return getDefaultZones();
  }, [propZones, particles, releaseLat, releaseLng, isotope]);

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-2 text-amber-500">
        <AlertTriangle className="h-5 w-5" />
        <span className="font-semibold">Emergency Response Zones</span>
      </div>

      {particles.length > 0 && (
        <div className="flex items-center gap-2 text-sm text-muted-foreground">
          <Activity className="h-4 w-4" />
          <span>Calculated from {particles.length} particle measurements</span>
        </div>
      )}
      
      <div className="grid gap-4">
        {zones.map((zone) => (
          <Card key={zone.id}>
            <CardHeader className="pb-2">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <Circle className={`h-4 w-4 ${severityColors[zone.severity]}`} />
                  <span className="font-semibold">{zone.name}</span>
                </div>
                <Badge variant={zone.severity === 'critical' ? 'danger' : 'outline'}>
                  {severityLabels[zone.severity]}
                </Badge>
              </div>
            </CardHeader>
            <CardContent className="space-y-2">
              <div className="flex items-center gap-4 text-sm text-muted-foreground">
                <div className="flex items-center gap-1">
                  <MapPin className="h-4 w-4" />
                  <span>{zone.radius} km radius</span>
                </div>
                <div>Population: {zone.population.toLocaleString()}</div>
                <div className="flex items-center gap-1">
                  <Activity className="h-4 w-4" />
                  <span>{zone.doseRate} μSv/h</span>
                </div>
              </div>
              <p className="text-sm">{zone.instructions}</p>
              {zone.contourPoints && zone.contourPoints.length > 0 && (
                <div className="text-xs text-muted-foreground">
                  Contour: {zone.contourPoints.length} boundary points defined
                </div>
              )}
            </CardContent>
          </Card>
        ))}
      </div>

    </div>
  );
}
