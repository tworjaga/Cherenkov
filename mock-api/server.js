const express = require('express');
const { WebSocketServer } = require('ws');
const http = require('http');
const cors = require('cors');

const app = express();
app.use(cors());
app.use(express.json());

// Mock data - formatted to match frontend GraphQL schema
const sensors = [
  {
    id: 'sensor-001',
    name: 'Fukushima Daiichi North',
    latitude: 37.4213,
    longitude: 141.0325,
    status: 'active',
    lastReading: { value: 0.12, unit: 'uSv/h', timestamp: new Date().toISOString() }
  },
  {
    id: 'sensor-002',
    name: 'Chernobyl Exclusion Zone',
    latitude: 51.3896,
    longitude: 30.0991,
    status: 'active',
    lastReading: { value: 2.45, unit: 'uSv/h', timestamp: new Date().toISOString() }
  },
  {
    id: 'sensor-003',
    name: 'Hanford Site Monitor',
    latitude: 46.5507,
    longitude: -119.4882,
    status: 'active',
    lastReading: { value: 0.08, unit: 'uSv/h', timestamp: new Date().toISOString() }
  },
  {
    id: 'sensor-004',
    name: 'Sellafield Coast',
    latitude: 54.4205,
    longitude: -3.4976,
    status: 'warning',
    lastReading: { value: 0.35, unit: 'uSv/h', timestamp: new Date().toISOString() }
  },
  {
    id: 'sensor-005',
    name: 'Three Mile Island',
    latitude: 40.1539,
    longitude: -76.7247,
    status: 'active',
    lastReading: { value: 0.06, unit: 'uSv/h', timestamp: new Date().toISOString() }
  }
];

const facilities = [
  {
    id: 'facility-001',
    name: 'Fukushima Daiichi NPP',
    facilityType: 'nuclear_power_plant',
    latitude: 37.4213,
    longitude: 141.0325,
    status: 'decommissioning'
  },
  {
    id: 'facility-002',
    name: 'Chernobyl NPP',
    facilityType: 'nuclear_power_plant',
    latitude: 51.3896,
    longitude: 30.0991,
    status: 'decommissioned'
  },
  {
    id: 'facility-003',
    name: 'Hanford Site',
    facilityType: 'nuclear_research',
    latitude: 46.5507,
    longitude: -119.4882,
    status: 'active'
  }
];

const anomalies = [
  {
    id: 'anomaly-001',
    sensorId: 'sensor-002',
    severity: 'high',
    zScore: 3.5,
    detectedAt: new Date().toISOString()
  },
  {
    id: 'anomaly-002',
    sensorId: 'sensor-004',
    severity: 'medium',
    zScore: 2.1,
    detectedAt: new Date(Date.now() - 3600000).toISOString()
  }
];

const globalStatus = {
  level: 3,
  defcon: 3,
  status: 'MONITORING',
  activeAlerts: 2,
  activeSensors: 5,
  lastUpdate: Date.now()
};



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

// Root route
app.get('/', (req, res) => {
  res.json({
    name: 'Cherenkov Mock API',
    version: '1.0.0',
    endpoints: {
      health: '/health',
      graphql: '/graphql',
      websocket: '/ws',
      api: {
        sensors: '/api/sensors',
        facilities: '/api/facilities',
        anomalies: '/api/anomalies',
        alerts: '/api/alerts'
      }
    },
    timestamp: new Date().toISOString()
  });
});

// Health check
app.get('/health', (req, res) => {
  res.json({ status: 'healthy', timestamp: new Date().toISOString() });
});

// GraphQL endpoint - GET for introspection/playground
app.get('/graphql', (req, res) => {
  res.json({
    data: {
      __schema: {
        queryType: { name: 'Query' },
        mutationType: { name: 'Mutation' },
        subscriptionType: { name: 'Subscription' },
        types: []
      }
    }
  });
});

// GraphQL endpoint - POST for queries
app.post('/graphql', (req, res) => {
  const { query, variables } = req.body;
  
  // Simple GraphQL mock responses matching frontend schema
  if (query.includes('sensors') && !query.includes('sensor(')) {
    res.json({
      data: {
        sensors: sensors
      }
    });
  } else if (query.includes('facilities')) {
    res.json({
      data: {
        facilities: facilities
      }
    });
  } else if (query.includes('anomalies')) {
    res.json({
      data: {
        anomalies: anomalies
      }
    });
  } else if (query.includes('alerts')) {
    res.json({
      data: {
        alerts: alerts
      }
    });
  } else if (query.includes('globalStatus')) {
    res.json({
      data: {
        globalStatus: globalStatus
      }
    });
  } else if (query.includes('sensor(')) {
    const sensorId = variables?.id || 'sensor-001';
    const sensor = sensors.find(s => s.id === sensorId) || sensors[0];
    res.json({
      data: {
        sensor
      }
    });
  } else if (query.includes('readings')) {
    // Mock readings data
    const mockReadings = sensors.map(s => ({
      id: `reading-${s.id}`,
      sensorId: s.id,
      timestamp: new Date().toISOString(),
      doseRate: s.lastReading.value,
      unit: s.lastReading.unit
    }));
    res.json({
      data: {
        readings: mockReadings
      }
    });
  } else if (query.includes('runPlumeSimulation') || query.includes('PlumeSimulation')) {
    res.json({
      data: {
        runPlumeSimulation: {
          id: 'sim-001',
          status: 'completed',
          particles: [
            { id: 'p1', latitude: 37.4213, longitude: 141.0325, altitude: 100, concentration: 0.5, timestamp: new Date().toISOString() }
          ],
          evacuationZones: [
            { id: 'z1', level: 1, boundary: [{ latitude: 37.4213, longitude: 141.0325 }], maxDoseRate: 1.0, timeToEvacuate: 3600 }
          ],
          weatherConditions: {
            windSpeed: 5.5,
            windDirection: 180,
            temperature: 20,
            pressure: 1013,
            stabilityClass: 'D'
          },
          createdAt: new Date().toISOString(),
          completedAt: new Date().toISOString()
        }
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
  
  let interval = null;
  
  ws.on('close', () => {
    console.log('WebSocket client disconnected');
    if (interval) clearInterval(interval);
  });
  
  ws.on('message', (message) => {
    try {
      const data = JSON.parse(message);
      console.log('Received:', data);
      
      // Handle GraphQL subscription protocol
      if (data.type === 'connection_init') {
        ws.send(JSON.stringify({
          type: 'connection_ack',
          payload: {}
        }));
      } else if (data.type === 'subscribe') {
        const subscriptionId = data.id;
        const query = data.payload?.query || '';
        
        console.log('Subscription started:', subscriptionId, query.substring(0, 50));
        
        // Send initial data for the subscription
        if (query.includes('allSensorUpdates')) {
          // Send initial sensor data
          sensors.forEach(sensor => {
            ws.send(JSON.stringify({
              type: 'data',
              id: subscriptionId,
              payload: {
                data: {
                  allSensorUpdates: {
                    sensorId: sensor.id,
                    timestamp: new Date().toISOString(),
                    doseRate: sensor.lastReading.value,
                    latitude: sensor.latitude,
                    longitude: sensor.longitude
                  }
                }
              }
            }));
          });
          
          // Start sending real-time updates
          interval = setInterval(() => {
            const randomSensor = sensors[Math.floor(Math.random() * sensors.length)];
            const variation = (Math.random() - 0.5) * 0.1;
            const newDoseRate = Math.max(0.01, randomSensor.lastReading.value + variation);
            
            // Update the sensor's last reading
            randomSensor.lastReading.value = newDoseRate;
            randomSensor.lastReading.timestamp = new Date().toISOString();
            
            // Send as GraphQL subscription data matching allSensorUpdates schema
            ws.send(JSON.stringify({
              type: 'data',
              id: subscriptionId,
              payload: {
                data: {
                  allSensorUpdates: {
                    sensorId: randomSensor.id,
                    timestamp: new Date().toISOString(),
                    doseRate: newDoseRate,
                    latitude: randomSensor.latitude,
                    longitude: randomSensor.longitude
                  }
                }
              }
            }));
          }, 3000);
        } else {
          // Generic subscription acknowledgment
          ws.send(JSON.stringify({
            type: 'data',
            id: subscriptionId,
            payload: { data: {} }
          }));
        }
      } else if (data.type === 'ping') {
        ws.send(JSON.stringify({ type: 'pong' }));
      } else if (data.type === 'complete') {
        // Client requested to stop subscription
        if (interval) {
          clearInterval(interval);
          interval = null;
        }
      }
    } catch (e) {
      console.error('Invalid message format:', e);
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
