'use client';

import { Globe } from '@/components/globe';
import { useGlobeStore } from '@/stores';

export default function GlobePage() {
  const { viewport, setViewport } = useGlobeStore();

  return (
    <div className="h-full w-full">
      <Globe
        sensors={[]}
        facilities={[]}
        anomalies={[]}
        plumes={[]}
        selectedSensorId={null}
        viewport={viewport}
        layers={{
          sensors: true,
          facilities: true,
          anomalies: true,
          plumes: false,
          heatmap: true,
        }}
        onViewportChange={setViewport}
        onSensorSelect={() => {}}
      />
    </div>
  );
}
