import { bench, describe } from 'vitest';
import { clusterSensors } from '../lib/clustering';

// Generate test data
const generateSensors = (count: number) => {
  return Array.from({ length: count }, (_, i) => ({
    id: `sensor-${i}`,
    lat: (Math.random() - 0.5) * 180,
    lon: (Math.random() - 0.5) * 360,
    value: Math.random() * 10,
  }));
};

describe('Clustering Performance', () => {
  const sensors100 = generateSensors(100);
  const sensors1000 = generateSensors(1000);
  const sensors10000 = generateSensors(10000);

  bench('cluster 100 sensors', () => {
    clusterSensors(sensors100, { zoom: 5 });
  });

  bench('cluster 1000 sensors', () => {
    clusterSensors(sensors1000, { zoom: 5 });
  });

  bench('cluster 10000 sensors', () => {
    clusterSensors(sensors10000, { zoom: 5 });
  });

  bench('cluster with maxClusters limit', () => {
    clusterSensors(sensors1000, { zoom: 5, maxClusters: 50 });
  });
});

describe('Data Processing Performance', () => {
  const readings = Array.from({ length: 10000 }, (_, i) => ({
    timestamp: Date.now() - i * 60000,
    value: Math.random() * 10,
    sensorId: `sensor-${i % 100}`,
  }));

  bench('sort 10000 readings', () => {
    [...readings].sort((a, b) => a.timestamp - b.timestamp);
  });

  bench('group readings by sensor', () => {
    const grouped = new Map<string, typeof readings>();
    for (const reading of readings) {
      if (!grouped.has(reading.sensorId)) {
        grouped.set(reading.sensorId, []);
      }
      grouped.get(reading.sensorId)!.push(reading);
    }
  });

  bench('calculate moving average', () => {
    const window = 60;
    const averages: number[] = [];
    for (let i = window; i < readings.length; i++) {
      const sum = readings.slice(i - window, i).reduce((a, b) => a + b.value, 0);
      averages.push(sum / window);
    }
  });
});

describe('Memory Operations', () => {
  bench('create large array', () => {
    new Float64Array(1000000);
  });

  bench('copy large array', () => {
    const arr = new Float64Array(1000000);
    new Float64Array(arr);
  });

  bench('iterate large array', () => {
    const arr = new Float64Array(1000000);
    let sum = 0;
    for (let i = 0; i < arr.length; i++) {
      sum += arr[i];
    }
  });
});
