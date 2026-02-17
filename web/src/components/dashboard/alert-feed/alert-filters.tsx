'use client';

import { Filter } from 'lucide-react';
import { cn } from '@/lib/utils';


type SeverityFilter = 'all' | 'critical' | 'high' | 'medium' | 'low';

interface AlertFiltersProps {
  activeFilter: SeverityFilter;
  onFilterChange: (filter: SeverityFilter) => void;
  counts: Record<SeverityFilter, number>;
}

const filters: { value: SeverityFilter; label: string; color: string }[] = [
  { value: 'all', label: 'All', color: '#a0a0b0' },
  { value: 'critical', label: 'Critical', color: '#ff3366' },
  { value: 'high', label: 'High', color: '#ff6b35' },
  { value: 'medium', label: 'Medium', color: '#ffb800' },
  { value: 'low', label: 'Low', color: '#00d4ff' },
];

export function AlertFilters({
  activeFilter,
  onFilterChange,
  counts,
}: AlertFiltersProps) {
  return (
    <div className="flex items-center gap-2 p-3 border-b border-[#1f1f2e]">
      <Filter size={14} className="text-[#606070]" />
      <div className="flex gap-1">
        {filters.map((filter) => (
          <button
            key={filter.value}
            onClick={() => onFilterChange(filter.value)}
            className={cn(
              'px-2 py-1 text-xs rounded-md transition-colors',
              activeFilter === filter.value
                ? 'bg-[#1a1a25] text-white'
                : 'text-[#606070] hover:text-[#a0a0b0]'
            )}
          >
            {filter.label}
            {counts[filter.value] > 0 && (
              <span
                className="ml-1.5 inline-flex h-4 min-w-[1rem] items-center justify-center rounded px-1 text-[10px] font-medium text-white"
                style={{ backgroundColor: filter.color }}
              >
                {counts[filter.value]}
              </span>
            )}

          </button>
        ))}
      </div>
    </div>
  );
}
