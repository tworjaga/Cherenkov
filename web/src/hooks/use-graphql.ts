'use client';

import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { graphqlClient } from '@/lib/graphql/client';
import { Sensor, Reading, Anomaly, Facility, GeoLocation } from '@/types/models';
import {
  GET_SENSORS,
  GET_READINGS,
  GET_ANOMALIES,
  GET_FACILITIES,
  GET_GLOBAL_STATUS,
} from '@/lib/graphql/queries';
import { ACKNOWLEDGE_ALERT } from '@/lib/graphql/mutations';

// API response types (raw from GraphQL)
interface ApiSensor {
  id: string;
  name: string;
  latitude: number;
  longitude: number;
  status: string;
  type?: string;
  source?: string;
  unit?: string;
  lastReading?: {
    value: number;
    unit: string;
    timestamp: string;
  } | null;
}

interface ApiFacility {
  id: string;
  name: string;
  facilityType: string;
  latitude: number;
  longitude: number;
  status: string;
}

interface ApiAnomaly {
  id: string;
  sensorId: string;
  severity: string;
  detectedAt: string;
  description?: string;
  type?: string;
  zScore?: number;
  doseRate?: number;
  baseline?: number;
  algorithm?: string;
  acknowledged?: boolean;
  latitude?: number;
  longitude?: number;
  message?: string;
}

interface ApiReading {
  timestamp: string;
  value?: number;
  doseRate?: number;
  unit: string;
  qualityFlag?: string;
  uncertainty?: number;
}

interface SensorsResponse {
  sensors: ApiSensor[];
}

interface ReadingsResponse {
  readings: ApiReading[];
}

interface AnomaliesResponse {
  anomalies: ApiAnomaly[];
}

interface FacilitiesResponse {
  facilities: ApiFacility[];
}

// Transformation functions
function transformSensor(apiSensor: ApiSensor): Sensor {
  return {
    id: apiSensor.id,
    name: apiSensor.name,
    type: apiSensor.type || 'unknown',
    location: {
      lat: apiSensor.latitude,
      lon: apiSensor.longitude,
      latitude: apiSensor.latitude,
      longitude: apiSensor.longitude,
    },
    longitude: apiSensor.longitude,
    latitude: apiSensor.latitude,
    status: apiSensor.status as Sensor['status'],
    source: apiSensor.source || 'mock',
    unit: apiSensor.unit,
    lastReading: apiSensor.lastReading ? {
      timestamp: new Date(apiSensor.lastReading.timestamp).getTime(),
      doseRate: apiSensor.lastReading.value,
      value: apiSensor.lastReading.value,
      unit: apiSensor.lastReading.unit,
      qualityFlag: 'good',
    } : null,
  };
}

function transformFacility(apiFacility: ApiFacility): Facility {
  return {
    id: apiFacility.id,
    name: apiFacility.name,
    type: (apiFacility.facilityType?.includes('power') ? 'nuclear' : 
           apiFacility.facilityType?.includes('research') ? 'research' : 'industrial') as Facility['type'],
    location: {
      lat: apiFacility.latitude,
      lon: apiFacility.longitude,
      latitude: apiFacility.latitude,
      longitude: apiFacility.longitude,
    },
    longitude: apiFacility.longitude,
    latitude: apiFacility.latitude,
    status: apiFacility.status as Facility['status'],
  };
}

function transformAnomaly(apiAnomaly: ApiAnomaly): Anomaly {
  return {
    id: apiAnomaly.id,
    type: apiAnomaly.type || 'unknown',
    description: apiAnomaly.description || apiAnomaly.message || 'Anomaly detected',
    sensorId: apiAnomaly.sensorId,
    severity: apiAnomaly.severity as Anomaly['severity'],
    zScore: apiAnomaly.zScore || 0,
    detectedAt: new Date(apiAnomaly.detectedAt).getTime(),
    doseRate: apiAnomaly.doseRate || 0,
    baseline: apiAnomaly.baseline || 0,
    algorithm: apiAnomaly.algorithm || 'statistical',
    acknowledged: apiAnomaly.acknowledged || false,
    location: {
      lat: apiAnomaly.latitude || 0,
      lon: apiAnomaly.longitude || 0,
      latitude: apiAnomaly.latitude || 0,
      longitude: apiAnomaly.longitude || 0,
    },
    longitude: apiAnomaly.longitude || 0,
    latitude: apiAnomaly.latitude || 0,
    message: apiAnomaly.message || apiAnomaly.description || 'Anomaly detected',
  };
}

function transformReading(apiReading: ApiReading): Reading {
  return {
    timestamp: new Date(apiReading.timestamp).getTime(),
    doseRate: apiReading.doseRate || apiReading.value || 0,
    value: apiReading.value || apiReading.doseRate || 0,
    unit: apiReading.unit,
    qualityFlag: (apiReading.qualityFlag as Reading['qualityFlag']) || 'good',
    uncertainty: apiReading.uncertainty,
  };
}


export function useSensors() {
  return useQuery({
    queryKey: ['sensors'],
    queryFn: async () => {
      const data = await graphqlClient.request<SensorsResponse>(GET_SENSORS);
      return data.sensors.map(transformSensor);
    },
    staleTime: 30000,
  });
}


export function useReadings(
  sensorIds: string[],
  from: Date,
  to: Date,
  aggregation: string = 'raw'
) {
  return useQuery({
    queryKey: ['readings', sensorIds, from, to, aggregation],
    queryFn: async () => {
      const data = await graphqlClient.request<ReadingsResponse>(GET_READINGS, {
        sensorIds,
        from: from.toISOString(),
        to: to.toISOString(),
        aggregation,
      });
      return data.readings.map(transformReading);
    },
    enabled: sensorIds.length > 0,
    staleTime: 60000,
  });
}


export function useAnomalies(severities?: string[]) {
  return useQuery({
    queryKey: ['anomalies', severities],
    queryFn: async () => {
      const data = await graphqlClient.request<AnomaliesResponse>(GET_ANOMALIES, {
        severity: severities,
        since: new Date(Date.now() - 86400000).toISOString(),
        limit: 100,
      });
      return data.anomalies.map(transformAnomaly);
    },
    staleTime: 15000,
  });
}


export function useFacilities() {
  return useQuery({
    queryKey: ['facilities'],
    queryFn: async () => {
      const data = await graphqlClient.request<FacilitiesResponse>(GET_FACILITIES);
      return data.facilities.map(transformFacility);
    },
    staleTime: 300000,
  });
}


interface GlobalStatusResponse {
  globalStatus: {
    level: number;
    defcon: number;
    status: string;
    activeAlerts: number;
    activeSensors: number;
    lastUpdate: number;
  };
}


interface AcknowledgeAlertResponse {
  acknowledgeAlert: {
    id: string;
    acknowledged: boolean;
    acknowledgedAt: string;
  };
}

export function useGlobalStatus() {
  return useQuery({
    queryKey: ['globalStatus'],
    queryFn: async () => {
      const data = await graphqlClient.request<GlobalStatusResponse>(GET_GLOBAL_STATUS);
      return data.globalStatus;
    },
    staleTime: 5000,
    refetchInterval: 30000,
  });
}

export function useAcknowledgeAlert() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (alertId: string) => {
      const data = await graphqlClient.request<AcknowledgeAlertResponse>(ACKNOWLEDGE_ALERT, { alertId });
      return data.acknowledgeAlert;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['anomalies'] });
      queryClient.invalidateQueries({ queryKey: ['alerts'] });
    },
  });
}
