'use client';

import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { graphqlClient } from '@/lib/graphql/client';
import { Sensor, Reading, Anomaly, Facility } from '@/types/models';
import {
  GET_SENSORS,
  GET_READINGS,
  GET_ANOMALIES,
  GET_FACILITIES,
  GET_GLOBAL_STATUS,
} from '@/lib/graphql/queries';
import { ACKNOWLEDGE_ALERT } from '@/lib/graphql/mutations';

interface SensorsResponse {
  sensors: Sensor[];
}

interface ReadingsResponse {
  readings: Reading[];
}

interface AnomaliesResponse {
  anomalies: Anomaly[];
}

interface FacilitiesResponse {
  facilities: Facility[];
}

export function useSensors() {
  return useQuery({
    queryKey: ['sensors'],
    queryFn: async () => {
      const data = await graphqlClient.request<SensorsResponse>(GET_SENSORS);
      return data.sensors;
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
      return data.readings;
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
      return data.anomalies;
    },
    staleTime: 15000,
  });
}

export function useFacilities() {
  return useQuery({
    queryKey: ['facilities'],
    queryFn: async () => {
      const data = await graphqlClient.request<FacilitiesResponse>(GET_FACILITIES);
      return data.facilities;
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
