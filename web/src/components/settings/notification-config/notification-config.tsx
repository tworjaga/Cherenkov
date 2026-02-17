'use client';

import React, { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { Badge } from '@/components/ui/badge';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Bell, Mail, MessageSquare, Plus, Trash2 } from 'lucide-react';

interface NotificationChannel {
  id: string;
  name: string;
  type: 'email' | 'slack' | 'webhook' | 'sms';
  target: string;
  enabled: boolean;
  events: string[];
}

const defaultChannels: NotificationChannel[] = [
  {
    id: '1',
    name: 'Security Team Email',
    type: 'email',
    target: 'security@example.com',
    enabled: true,
    events: ['anomaly_detected', 'threshold_breach'],
  },
  {
    id: '2',
    name: 'Operations Slack',
    type: 'slack',
    target: '#radiation-alerts',
    enabled: true,
    events: ['system_alert', 'maintenance_required'],
  },
];

const eventTypes = [
  { value: 'anomaly_detected', label: 'Anomaly Detected' },
  { value: 'threshold_breach', label: 'Threshold Breach' },
  { value: 'system_alert', label: 'System Alert' },
  { value: 'maintenance_required', label: 'Maintenance Required' },
  { value: 'sensor_offline', label: 'Sensor Offline' },
];

export function NotificationConfig() {
  const [channels, setChannels] = useState<NotificationChannel[]>(defaultChannels);
  const [newChannel, setNewChannel] = useState<Partial<NotificationChannel>>({
    type: 'email',
    enabled: true,
    events: [],
  });

  const handleAdd = () => {
    if (newChannel.name && newChannel.target) {
      setChannels([
        ...channels,
        {
          ...newChannel,
          id: Date.now().toString(),
        } as NotificationChannel,
      ]);
      setNewChannel({ type: 'email', enabled: true, events: [] });
    }
  };

  const handleRemove = (id: string) => {
    setChannels(channels.filter((c) => c.id !== id));
  };

  const handleToggle = (id: string) => {
    setChannels(
      channels.map((c) => (c.id === id ? { ...c, enabled: !c.enabled } : c))
    );
  };

  const typeIcons = {
    email: Mail,
    slack: MessageSquare,
    webhook: Bell,
    sms: MessageSquare,
  };

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Bell className="h-5 w-5" />
            Notification Channels
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          {channels.map((channel) => {
            const Icon = typeIcons[channel.type];
            return (
              <div
                key={channel.id}
                className="flex items-center justify-between rounded-lg border p-4"
              >
                <div className="space-y-1">
                  <div className="flex items-center gap-2">
                    <Icon className="h-4 w-4" />
                    <span className="font-medium">{channel.name}</span>
                    <Badge variant="outline">{channel.type}</Badge>
                  </div>
                  <div className="text-sm text-muted-foreground">
                    {channel.target}
                  </div>
                  <div className="flex gap-1 flex-wrap">
                    {channel.events.map((event) => (
                      <Badge key={event} variant="secondary" className="text-xs">
                        {eventTypes.find((e) => e.value === event)?.label || event}
                      </Badge>
                    ))}
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  <Switch
                    checked={channel.enabled}
                    onCheckedChange={() => handleToggle(channel.id)}
                  />
                  <Button
                    variant="ghost"
                    size="icon"
                    onClick={() => handleRemove(channel.id)}
                  >
                    <Trash2 className="h-4 w-4" />
                  </Button>
                </div>
              </div>
            );
          })}

          <div className="rounded-lg border p-4 space-y-4">
            <div className="font-medium">Add Notification Channel</div>
            <div className="grid gap-4 md:grid-cols-2">
              <div className="space-y-2">
                <Label>Name</Label>
                <Input
                  value={newChannel.name || ''}
                  onChange={(e) =>
                    setNewChannel({ ...newChannel, name: e.target.value })
                  }
                  placeholder="Channel name"
                />
              </div>
              <div className="space-y-2">
                <Label>Type</Label>
                <Select
                  value={newChannel.type}
                  onValueChange={(v) =>
                    setNewChannel({
                      ...newChannel,
                      type: v as NotificationChannel['type'],
                    })
                  }
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="email">Email</SelectItem>
                    <SelectItem value="slack">Slack</SelectItem>
                    <SelectItem value="webhook">Webhook</SelectItem>
                    <SelectItem value="sms">SMS</SelectItem>
                  </SelectContent>
                </Select>
              </div>
              <div className="space-y-2 md:col-span-2">
                <Label>Target</Label>
                <Input
                  value={newChannel.target || ''}
                  onChange={(e) =>
                    setNewChannel({ ...newChannel, target: e.target.value })
                  }
                  placeholder="Email, webhook URL, or channel"
                />
              </div>
            </div>
            <Button onClick={handleAdd}>
              <Plus className="mr-2 h-4 w-4" />
              Add Channel
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
