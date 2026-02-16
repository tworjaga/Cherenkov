import { Sensor } from '../stores/useAppStore';

export type ExportFormat = 'csv' | 'json';

export interface ExportOptions {
  filename?: string;
  format: ExportFormat;
  includeMetadata?: boolean;
}

const formatDate = (date: Date): string => {
  return date.toISOString().replace(/[:.]/g, '-').slice(0, 19);
};

const generateFilename = (prefix: string, format: ExportFormat): string => {
  const timestamp = formatDate(new Date());
  return `${prefix}_${timestamp}.${format}`;
};

const sensorsToCSV = (sensors: Sensor[], includeMetadata: boolean): string => {
  const headers = ['id', 'latitude', 'longitude', 'doseRate', 'unit', 'lastReading', 'status'];
  
  if (includeMetadata) {
    headers.push('exportedAt');
  }

  const rows = Object.values(sensors).map((sensor) => {
    const base = [
      sensor.id,
      sensor.location.lat,
      sensor.location.lon,
      sensor.doseRate,
      sensor.unit,
      sensor.lastReading.toISOString(),
      sensor.status,
    ];
    
    if (includeMetadata) {
      base.push(new Date().toISOString());
    }
    
    return base.join(',');
  });

  return [headers.join(','), ...rows].join('\n');
};

const sensorsToJSON = (sensors: Sensor[], includeMetadata: boolean): string => {
  const data = {
    sensors: Object.values(sensors),
    ...(includeMetadata && {
      metadata: {
        exportedAt: new Date().toISOString(),
        count: Object.keys(sensors).length,
        version: '1.0',
      },
    }),
  };

  return JSON.stringify(data, null, 2);
};

export const exportSensors = (
  sensors: Record<string, Sensor>,
  options: ExportOptions
): void => {
  const { format, includeMetadata = true, filename } = options;
  
  const content = format === 'csv'
    ? sensorsToCSV(sensors, includeMetadata)
    : sensorsToJSON(sensors, includeMetadata);

  const blob = new Blob(
    [content],
    { type: format === 'csv' ? 'text/csv;charset=utf-8;' : 'application/json' }
  );

  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = filename || generateFilename('sensors', format);
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  URL.revokeObjectURL(url);
};

export const exportChartData = (
  data: Array<{ timestamp: Date; value: number }>,
  options: ExportOptions
): void => {
  const { format, filename } = options;
  
  const content = format === 'csv'
    ? ['timestamp,value', ...data.map(d => `${d.timestamp.toISOString()},${d.value}`)].join('\n')
    : JSON.stringify({ data }, null, 2);

  const blob = new Blob(
    [content],
    { type: format === 'csv' ? 'text/csv;charset=utf-8;' : 'application/json' }
  );

  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = filename || generateFilename('chart_data', format);
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  URL.revokeObjectURL(url);
};
