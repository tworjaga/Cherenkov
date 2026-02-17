'use client';

import { AnomalyTimeline } from '@/components/dashboard';

export default function AnomaliesPage() {
  return (
    <div className="h-full w-full p-6 overflow-auto">
      <h1 className="text-display-md font-sans text-text-primary mb-6">Anomalies</h1>
      <AnomalyTimeline anomalies={[]} />
    </div>
  );
}
