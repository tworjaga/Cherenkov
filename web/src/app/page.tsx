'use client';

import { Globe } from '@/components/globe';
import { useDataStore } from '@/stores';

export default function Home() {
  const { sensors, anomalies } = useDataStore();

  return (
    <div className="w-full h-full">
      <Globe sensors={sensors} anomalies={anomalies} />
    </div>
  );
}
