'use client';

import React, { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Switch } from '@/components/ui/switch';
import { Badge } from '@/components/ui/badge';
import { Database, Plus, Trash2, TestTube } from 'lucide-react';

interface DataSource {
  id: string;
  name: string;
  type: 'scylladb' | 'postgresql' | 'influxdb' | 'kafka';
  url: string;
  enabled: boolean;
  status: 'connected' | 'disconnected' | 'error';
}

const defaultSources: DataSource[] = [
  {
    id: '1',
    name: 'Primary ScyllaDB',
    type: 'scylladb',
    url: 'scylla://localhost:9042',
    enabled: true,
    status: 'connected',
  },
  {
    id: '2',
    name: 'Time Series DB',
    type: 'influxdb',
    url: 'http://localhost:8086',
    enabled: true,
    status: 'connected',
  },
];

export function DataSourceConfig() {
  const [sources, setSources] = useState<DataSource[]>(defaultSources);
  const [newSource, setNewSource] = useState<Partial<DataSource>>({
    type: 'scylladb',
    enabled: true,
  });

  const handleAdd = () => {
    if (newSource.name && newSource.url) {
      setSources([
        ...sources,
        {
          ...newSource,
          id: Date.now().toString(),
          status: 'disconnected',
        } as DataSource,
      ]);
      setNewSource({ type: 'scylladb', enabled: true });
    }
  };

  const handleRemove = (id: string) => {
    setSources(sources.filter((s) => s.id !== id));
  };

  const handleToggle = (id: string) => {
    setSources(
      sources.map((s) => (s.id === id ? { ...s, enabled: !s.enabled } : s))
    );
  };

  const statusColors = {
    connected: 'bg-green-500',
    disconnected: 'bg-gray-500',
    error: 'bg-red-500',
  };

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Database className="h-5 w-5" />
            Data Sources
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          {sources.map((source) => (
            <div
              key={source.id}
              className="flex items-center justify-between rounded-lg border p-4"
            >
              <div className="space-y-1">
                <div className="flex items-center gap-2">
                  <span className="font-medium">{source.name}</span>
                  <Badge variant="outline">{source.type}</Badge>
                  <div
                    className={`h-2 w-2 rounded-full ${statusColors[source.status]}`}
                  />
                </div>
                <div className="text-sm text-muted-foreground">{source.url}</div>
              </div>
              <div className="flex items-center gap-2">
                <Switch
                  checked={source.enabled}
                  onCheckedChange={() => handleToggle(source.id)}
                />
                <Button
                  variant="ghost"
                  size="icon"
                  onClick={() => handleRemove(source.id)}
                >
                  <Trash2 className="h-4 w-4" />
                </Button>
              </div>
            </div>
          ))}

          <div className="rounded-lg border p-4 space-y-4">
            <div className="font-medium">Add New Data Source</div>
            <div className="grid gap-4 md:grid-cols-2">
              <div className="space-y-2">
                <Label>Name</Label>
                <Input
                  value={newSource.name || ''}
                  onChange={(e) =>
                    setNewSource({ ...newSource, name: e.target.value })
                  }
                  placeholder="Data source name"
                />
              </div>
              <div className="space-y-2">
                <Label>Type</Label>
                <Select
                  value={newSource.type}
                  onValueChange={(v) =>
                    setNewSource({ ...newSource, type: v as DataSource['type'] })
                  }
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="scylladb">ScyllaDB</SelectItem>
                    <SelectItem value="postgresql">PostgreSQL</SelectItem>
                    <SelectItem value="influxdb">InfluxDB</SelectItem>
                    <SelectItem value="kafka">Kafka</SelectItem>
                  </SelectContent>
                </Select>
              </div>
              <div className="space-y-2 md:col-span-2">
                <Label>Connection URL</Label>
                <Input
                  value={newSource.url || ''}
                  onChange={(e) =>
                    setNewSource({ ...newSource, url: e.target.value })
                  }
                  placeholder="Connection string"
                />
              </div>
            </div>
            <div className="flex gap-2">
              <Button onClick={handleAdd}>
                <Plus className="mr-2 h-4 w-4" />
                Add Source
              </Button>
              <Button variant="outline">
                <TestTube className="mr-2 h-4 w-4" />
                Test Connection
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
