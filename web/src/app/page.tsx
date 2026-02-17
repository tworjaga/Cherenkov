'use client';

import { Globe } from '@/components/globe';
import { useDataStore } from '@/stores';
import { useKeyboardShortcuts } from '@/hooks';

export default function Home() {
  const { sensors, anomalies } = useDataStore();
  useKeyboardShortcuts();

  return (
    <div className="w-full h-full">
      <Globe sensors={sensors} anomalies={anomalies} />
    </div>
  );
}
