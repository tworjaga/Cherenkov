'use client';

import { Activity } from 'lucide-react';
import { ReadingChart } from '@/components/dashboard';

export function GlobalChartPanel(): JSX.Element {
  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center gap-2 px-4 py-2 border-b border-[#1f1f2e]">
        <Activity className="w-4 h-4 text-[#00d4ff]" />
        <span className="text-xs font-semibold text-[#a0a0b0] uppercase tracking-wider">
          Global Radiation
        </span>
      </div>
      <div className="flex-1 p-4">
        <ReadingChart />
      </div>
    </div>
  );
}
