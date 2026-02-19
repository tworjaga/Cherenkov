const express = require('express');
const { WebSocketServer } = require('ws');
const http = require('http');
const cors = require('cors');

const app = express();
app.use(cors());
app.use(express.json());

// Mock data
const sensors = [
  {
    id: 'sensor-001',
    name: 'Fukushima Daiichi North',
    type: 'radiation',
    location: { lat: 37.4213, lng: 141.0325 },
    status: 'active',
    reading: { value: 0.12, unit: 'uSv/h', timestamp: new Date().toISOString() },
    facility: 'Fukushima Daiichi NPP'
  },
  {
    id: 'sensor-002',
    name: 'Chernobyl Exclusion Zone',
    type: 'radiation',
    location: { lat: 51.3896, lng: 30.0991 },
    status: 'active',
    reading: { value: 2.45, unit: 'uSv/h', timestamp: new Date().toISOString() },
    facility: 'Chernobyl NPP'
  },
  {
    id: 'sensor-003',
    name: 'Hanford Site Monitor',
    type: 'radiation',
    location: { lat: 46.5507, lng: -119.4882 },
    status: 'active',
    reading: { value: 0.08, unit: 'uSv/h', timestamp: new Date().toISOString() },
    facility: 'Hanford Site'
  },
  {
    id: 'sensor-004',
    name: 'Sellafield Coast',
    type: 'radiation',
    location: { lat: 54.4205, lng: -3.4976 },
    status: 'warning',
    reading: { value: 0.35, unit: 'uSv/h', timestamp: new Date().toISOString() },
    facility: 'Sellafield'
  },
  {
    id: 'sensor-005',
    name: 'Three Mile Island',
    type: 'radiation',
    location: { lat: 40.1539, lng: -76.7247 },
    status: 'active',
    reading: { value: 0.06, unit: 'uSv/h', timestamp: new Date().toISOString() },
    facility: 'Three Mile Island'
  }
];

const facilities = [
  {
    id: 'facility-001',
    name: 'Fukushima Daiichi NPP',
    type: 'nuclear_power_plant',
    location: { lat: 37.4213, lng: 141.0325 },
    status: 'decommissioning',
    reactorCount: 6,
    description: 'Nuclear power plant undergoing decommissioning after 2011 accident'
  },
  {
    id: 'facility-002',
    name: 'Chernobyl NPP',
    type: 'nuclear_power_plant',
    location: { lat: 51.3896, lng: 30.0991 },
    status: 'decommissioned',
    reactorCount: 4,
    description: 'Site of 1986 nuclear disaster, now under confinement'
  },
  {
    id: 'facility-003',
    name: 'Hanford Site',
    type: 'nuclear_research',
    location: { lat: 46.5507, lng: -119.4882 },
    status: 'active',
    reactorCount: 0,
    description: 'Former plutonium production site, now environmental cleanup'
  }
];

const anomalies = [
  {
    id: 'anomaly-001',
    type: 'radiation_spike',
    severity: 'high',
    sensorId: 'sensor-002',
    location: { lat: 51.3896, lng: 30.0991 },
    timestamp: new Date().toISOString(),
    description: 'Radiation level 3x above baseline detected',
    status: 'investigating'
  },
  {
    id: 'anomaly-002',
    type: 'sensor_offline',
    severity: 'medium',
    sensorId: 'sensor-004',
    location: { lat: 54.4205, lng: -3.4976 },
    timestamp: new Date(Date.now() - 3600000).toISOString(),
    description: 'Sensor communication timeout',
    status: 'resolved'
  }
];

const alerts = [
  {
    id: 'alert-001',
    type: 'radiation_alert',
    severity: 'critical',
    title: 'Radiation Spike Detected',
    message: 'Chernobyl sensor reporting elevated levels',
    timestamp: new Date().toISOString(),
    acknowledged: false,
    sensorId: 'sensor-002'
  },
  {
    id: 'alert-002',
    type: 'system_alert',
    severity: 'warning',
    title: 'Sensor Offline',
    message: 'Sellafield coast sensor not responding',
    timestamp: new Date(Date.now() - 7200000).toISOString(),
    acknowledged: true,
    sensorId: 'sensor-004'
  }
];

// Health check
app.get('/health', (req, res) => {
  res.json({ status: 'healthy', timestamp: new Date().toISOString() });
});

// GraphQL endpoint
app.post('/graphql', (req, res) => {
  const { query, variables } = req.body;
  
  // Simple GraphQL mock responses
  if (query.includes('sensors')) {
    res.json({
      data: {
        sensors: {
          nodes: sensors,
          totalCount: sensors.length
        }
      }
    });
  } else if (query.includes('facilities')) {
    res.json({
      data: {
        facilities: {
          nodes: facilities,
          totalCount: facilities.length
        }
      }
    });
  } else if (query.includes('anomalies')) {
    res.json({
      data: {
        anomalies: {
          nodes: anomalies,
          totalCount: anomalies.length
        }
      }
    });
  } else if (query.includes('alerts')) {
    res.json({
      data: {
        alerts: {
          nodes: alerts,
          totalCount: alerts.length
        }
      }
    });
  } else if (query.includes('sensor')) {
    const sensorId = variables?.id || 'sensor-001';
    const sensor = sensors.find(s => s.id === sensorId) || sensors[0];
    res.json({
      data: {
        sensor
      }
    });
  } else {
    res.json({ data: {} });
  }
});

// REST API endpoints
app.get('/api/sensors', (req, res) => {
  res.json(sensors);
});

app.get('/api/sensors/:id', (req, res) => {
  const sensor = sensors.find(s => s.id === req.params.id);
  if (sensor) {
    res.json(sensor);
  } else {
    res.status(404).json({ error: 'Sensor not found' });
  }
});

app.get('/api/facilities', (req, res) => {
  res.json(facilities);
});

app.get('/api/anomalies', (req, res) => {
  res.json(anomalies);
});

app.get('/api/alerts', (req, res) => {
  res.json(alerts);
});

// Create HTTP server
const server = http.createServer(app);

// WebSocket server
const wss = new WebSocketServer({ server, path: '/ws' });

wss.on('connection', (ws) => {
  console.log('WebSocket client connected');
  
  // Send initial data
  ws.send(JSON.stringify({
    type: 'connection',
    message: 'Connected to Cherenkov Mock API',
    timestamp: new Date().toISOString()
  }));
  
  // Simulate real-time updates
  const interval = setInterval(() => {
    const randomSensor = sensors[Math.floor(Math.random() * sensors.length)];
    const variation = (Math.random() - 0.5) * 0.1;
    const newReading = {
      ...randomSensor.reading,
      value: Math.max(0.01, randomSensor.reading.value + variation),
      timestamp: new Date().toISOString()
    };
    
    ws.send(JSON.stringify({
      type: 'sensor_update',
      sensorId: randomSensor.id,
      reading: newReading
    }));
  }, 5000);
  
  ws.on('close', () => {
    console.log('WebSocket client disconnected');
    clearInterval(interval);
  });
  
  ws.on('message', (message) => {
    try {
      const data = JSON.parse(message);
      console.log('Received:', data);
      
      // Echo back with acknowledgment
      ws.send(JSON.stringify({
        type: 'ack',
        received: data,
        timestamp: new Date().toISOString()
      }));
    } catch (e) {
      console.error('Invalid message format');
    }
  });
});

const PORT = process.env.PORT || 8080;
server.listen(PORT, () => {
  console.log(`Cherenkov Mock API Server running on port ${PORT}`);
  console.log(`WebSocket endpoint: ws://localhost:${PORT}/ws`);
  console.log(`GraphQL endpoint: http://localhost:${PORT}/graphql`);
  console.log(`REST API: http://localhost:${PORT}/api`);
});
