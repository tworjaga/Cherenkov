'use client';

import React from 'react';
import { Sensor } from '@/types/models';
import { Modal } from '@/components/ui/modal';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';

interface SensorDetailModalProps {
  sensor: Sensor | null;
  isOpen: boolean;
  onClose: () => void;
}

export function SensorDetailModal({ sensor, isOpen, onClose }: SensorDetailModalProps) {
  if (!sensor) return null;

  const statusColors = {
    online: 'bg-green-500',
    offline: 'bg-red-500',
    maintenance: 'bg-yellow-500',
    error: 'bg-red-600',
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title={sensor.name}>
      <div className="space-y-4">
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2 text-lg">
              <div className={`h-3 w-3 rounded-full ${statusColors[sensor.status as keyof typeof statusColors] || 'bg-gray-500'}`} />
              Status: <span className="capitalize">{sensor.status}</span>
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-2">
            <div className="flex justify-between">
              <span className="text-muted-foreground">Type</span>
              <Badge variant="outline">{sensor.type || 'Unknown'}</Badge>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Location</span>
              <span>
                {typeof sensor.location === 'string'
                  ? sensor.location
                  : `${sensor.location.lat.toFixed(4)}, ${sensor.location.lng.toFixed(4)}`}
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground">Unit</span>
              <span>{sensor.unit || 'N/A'}</span>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="text-lg">Latest Reading</CardTitle>
          </CardHeader>
          <CardContent>
            {sensor.lastReading ? (
              <div className="space-y-2">
                <div className="flex justify-between">
                  <span className="text-muted-foreground">Value</span>
                  <span className="font-mono text-lg">
                    {sensor.lastReading.value} {sensor.unit}
                  </span>
                </div>
                <div className="flex justify-between">
                  <span className="text-muted-foreground">Timestamp</span>
                  <span>{new Date(sensor.lastReading.timestamp).toLocaleString()}</span>
                </div>
              </div>
            ) : (
              <p className="text-muted-foreground">No readings available</p>
            )}
          </CardContent>
        </Card>

        <Separator />

        <div className="flex justify-end gap-2">
          <button
            onClick={onClose}
            className="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
          >
            Close
          </button>
        </div>
      </div>
    </Modal>
  );
}
