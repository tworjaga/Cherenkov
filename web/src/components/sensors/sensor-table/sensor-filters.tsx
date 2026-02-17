'use client';

import React from 'react';
import { Input } from '@/components/ui/input';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';

interface SensorFiltersState {
  status: string;
  type: string;
  search: string;
}

interface SensorFiltersProps {
  filters: SensorFiltersState;
  onFiltersChange: (filters: SensorFiltersState) => void;
}

export function SensorFilters({ filters, onFiltersChange }: SensorFiltersProps) {
  const handleStatusChange = (value: string) => {
    onFiltersChange({ ...filters, status: value });
  };

  const handleTypeChange = (value: string) => {
    onFiltersChange({ ...filters, type: value });
  };

  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onFiltersChange({ ...filters, search: e.target.value });
  };

  return (
    <div className="flex flex-col gap-4 p-4 border-b sm:flex-row sm:items-center">
      <div className="flex-1">
        <Input
          placeholder="Search sensors..."
          value={filters.search}
          onChange={handleSearchChange}
          className="max-w-sm"
        />
      </div>
      
      <div className="flex gap-2">
        <Select value={filters.status} onValueChange={handleStatusChange}>
          <SelectTrigger className="w-[140px]">
            <SelectValue placeholder="Status" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All Status</SelectItem>
            <SelectItem value="online">Online</SelectItem>
            <SelectItem value="offline">Offline</SelectItem>
            <SelectItem value="maintenance">Maintenance</SelectItem>
            <SelectItem value="error">Error</SelectItem>
          </SelectContent>
        </Select>

        <Select value={filters.type} onValueChange={handleTypeChange}>
          <SelectTrigger className="w-[140px]">
            <SelectValue placeholder="Type" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All Types</SelectItem>
            <SelectItem value="radiation">Radiation</SelectItem>
            <SelectItem value="temperature">Temperature</SelectItem>
            <SelectItem value="pressure">Pressure</SelectItem>
            <SelectItem value="flow">Flow</SelectItem>
          </SelectContent>
        </Select>
      </div>
    </div>
  );
}
