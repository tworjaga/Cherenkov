'use client';

import { SensorList } from '@/components/dashboard';

export default function SensorsPage() {
  return (
    <div className="h-full w-full p-6 overflow-auto">
      <h1 className="text-display-md font-sans text-text-primary mb-6">Sensors</h1>
      <SensorList sensors={[]} onSensorClick={() => {}} />

    </div>
  );
}
