'use client';

import React from 'react';
import { Input } from '@/components/ui/input';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';

interface AnomalyFiltersProps {
  searchQuery: string;
  onSearchChange: (value: string) => void;
  severityFilter: string;
  onSeverityChange: (value: string) => void;
  statusFilter: string;
  onStatusChange: (value: string) => void;
}

export function AnomalyFilters({
  searchQuery,
  onSearchChange,
  severityFilter,
  onSeverityChange,
  statusFilter,
  onStatusChange,
}: AnomalyFiltersProps) {
  return (
    <div className="flex flex-col sm:flex-row gap-4 p-4 border-b">
      <Input
        placeholder="Search anomalies..."
        value={searchQuery}
        onChange={(e) => onSearchChange(e.target.value)}
        className="sm:w-64"
      />
      
      <Select value={severityFilter} onValueChange={onSeverityChange}>
        <SelectTrigger className="sm:w-40">
          <SelectValue placeholder="Severity" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="all">All Severities</SelectItem>
          <SelectItem value="low">Low</SelectItem>
          <SelectItem value="medium">Medium</SelectItem>
          <SelectItem value="high">High</SelectItem>
          <SelectItem value="critical">Critical</SelectItem>
        </SelectContent>
      </Select>

      <Select value={statusFilter} onValueChange={onStatusChange}>
        <SelectTrigger className="sm:w-40">
          <SelectValue placeholder="Status" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="all">All Statuses</SelectItem>
          <SelectItem value="active">Active</SelectItem>
          <SelectItem value="investigating">Investigating</SelectItem>
          <SelectItem value="resolved">Resolved</SelectItem>
          <SelectItem value="false_positive">False Positive</SelectItem>
        </SelectContent>
      </Select>
    </div>
  );
}
