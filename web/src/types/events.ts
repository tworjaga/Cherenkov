/**
 * Event type definitions
 * TypeScript interfaces for system events and notifications
 */

export type EventType = 
  | 'sensor_reading'
  | 'anomaly_detected'
  | 'alert_triggered'
  | 'facility_status_change'
  | 'plume_simulation_complete'
  | 'system_error'
  | 'user_action';

export type EventSeverity = 'info' | 'warning' | 'critical' | 'emergency';

export interface SystemEvent {
  id: string;
  type: EventType;
  severity: EventSeverity;
  title: string;
  description: string;
  timestamp: string;
  source: string;
  metadata?: Record<string, unknown>;
  acknowledged: boolean;
  acknowledgedBy?: string;
  acknowledgedAt?: string;
}

export interface AlertEvent extends SystemEvent {
  type: 'alert_triggered';
  sensorId?: string;
  facilityId?: string;
  threshold: number;
  value: number;
  unit: string;
}

export interface AnomalyEvent extends SystemEvent {
  type: 'anomaly_detected';
  sensorId: string;
  anomalyType: string;
  confidence: number;
  expectedRange: { min: number; max: number };
  actualValue: number;
}

export interface EventFilter {
  types?: EventType[];
  severities?: EventSeverity[];
  startDate?: string;
  endDate?: string;
  sources?: string[];
  acknowledged?: boolean;
}

export interface EventSubscription {
  id: string;
  name: string;
  filters: EventFilter;
  channels: ('email' | 'sms' | 'push' | 'webhook')[];
  active: boolean;
}
