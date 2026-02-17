'use client';

import { useState } from 'react';
import { Input } from '@/components/ui/input';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Badge } from '@/components/ui/badge';

export type AlertFilterSeverity = 'all' | 'critical' | 'high' | 'medium' | 'low' | 'info';
export type AlertFilterStatus = 'all' | 'active' | 'acknowledged' | 'resolved';

interface AlertFilterProps {
  onFilterChange?: (filters: {
    severity: AlertFilterSeverity;
    status: AlertFilterStatus;
    search: string;
  }) => void;
}

export function AlertFilter({ onFilterChange }: AlertFilterProps) {
  const [severity, setSeverity] = useState<AlertFilterSeverity>('all');
  const [status, setStatus] = useState<AlertFilterStatus>('all');
  const [search, setSearch] = useState('');

  const handleChange = () => {
    onFilterChange?.({ severity, status, search });
  };

  return (
    <div className="space-y-3 p-3 border-b">
      <Input
        placeholder="Search alerts..."
        value={search}
        onChange={(e) => {
          setSearch(e.target.value);
          handleChange();
        }}
        className="h-8"
      />
      
      <div className="flex gap-2">
        <Select
          value={severity}
          onValueChange={(value: AlertFilterSeverity) => {
            setSeverity(value);
            handleChange();
          }}
        >
          <SelectTrigger className="h-8 flex-1">
            <SelectValue placeholder="Severity" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All Severities</SelectItem>
            <SelectItem value="critical">
              <Badge variant="outline" className="bg-red-500/10 text-red-500 border-red-500/20">
                Critical
              </Badge>
            </SelectItem>
            <SelectItem value="high">
              <Badge variant="outline" className="bg-orange-500/10 text-orange-500 border-orange-500/20">
                High
              </Badge>
            </SelectItem>
            <SelectItem value="medium">
              <Badge variant="outline" className="bg-yellow-500/10 text-yellow-500 border-yellow-500/20">
                Medium
              </Badge>
            </SelectItem>
            <SelectItem value="low">
              <Badge variant="outline" className="bg-blue-500/10 text-blue-500 border-blue-500/20">
                Low
              </Badge>
            </SelectItem>
            <SelectItem value="info">
              <Badge variant="outline" className="bg-slate-500/10 text-slate-500 border-slate-500/20">
                Info
              </Badge>
            </SelectItem>
          </SelectContent>
        </Select>

        <Select
          value={status}
          onValueChange={(value: AlertFilterStatus) => {
            setStatus(value);
            handleChange();
          }}
        >
          <SelectTrigger className="h-8 flex-1">
            <SelectValue placeholder="Status" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">All Status</SelectItem>
            <SelectItem value="active">Active</SelectItem>
            <SelectItem value="acknowledged">Acknowledged</SelectItem>
            <SelectItem value="resolved">Resolved</SelectItem>
          </SelectContent>
        </Select>
      </div>
    </div>
  );
}
