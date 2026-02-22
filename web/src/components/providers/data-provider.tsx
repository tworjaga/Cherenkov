'use client';

import { useEffect } from 'react';
import { useSensors, useFacilities, useAnomalies, useGlobalStatus } from '@/hooks/use-graphql';
import { useDataStore } from '@/stores';

export function DataProvider({ children }: { children: React.ReactNode }) {
  const { setSensors, setFacilities, setAnomalies, setGlobalStatus } = useDataStore();

  const { data: sensorsData } = useSensors();
  const { data: facilitiesData } = useFacilities();
  const { data: anomaliesData } = useAnomalies();
  const { data: globalStatusData } = useGlobalStatus();

  useEffect(() => {
    if (sensorsData) {
      setSensors(sensorsData);
    }
  }, [sensorsData, setSensors]);

  useEffect(() => {
    if (facilitiesData) {
      setFacilities(facilitiesData);
    }
  }, [facilitiesData, setFacilities]);

  useEffect(() => {
    if (anomaliesData) {
      setAnomalies(anomaliesData);
    }
  }, [anomaliesData, setAnomalies]);

  useEffect(() => {
    if (globalStatusData) {
      setGlobalStatus({
        ...globalStatusData,
        defcon: globalStatusData.defcon as 1 | 2 | 3 | 4 | 5,
        status: globalStatusData.status as 'NORMAL' | 'ELEVATED' | 'CRITICAL',
      });
    }
  }, [globalStatusData, setGlobalStatus]);



  return <>{children}</>;
}
