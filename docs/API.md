# Cherenkov API Documentation

## Overview

Cherenkov provides a GraphQL API for querying radiation data, managing alerts, and running simulations. WebSocket subscriptions enable real-time updates.

**Endpoint:** `https://api.cherenkov.io/graphql`  
**WebSocket:** `wss://api.cherenkov.io/ws`

## Authentication

All requests require a JWT bearer token:

```bash
Authorization: Bearer <token>
```

Obtain tokens via the OAuth2 flow at `https://cherenkov.io/auth`.

## Queries

### Get Sensors

```graphql
query Sensors($region: GeoPolygon, $activeOnly: Boolean) {
  sensorsInRegion(region: $region, activeOnly: $activeOnly) {
    id
    name
    location {
      lat
      lon
    }
    currentReading {
      doseRate
      timestamp
    }
    status
  }
}
```

### Get Readings

```graphql
query Readings($sensorIds: [UUID!]!, $from: Timestamp!, $to: Timestamp!) {
  readings(sensorIds: $sensorIds, from: $from, to: $to, aggregation: HOURLY) {
    sensorId
    points {
      timestamp
      doseRate
      uncertainty
    }
  }
}
```

### Get Anomalies

```graphql
query Anomalies($severity: [Severity!], $since: Timestamp) {
  anomalies(severity: $severity, since: $since, first: 100) {
    edges {
      node {
        id
        severity
        message
        timestamp
        location {
          lat
          lon
        }
        metadata {
          sensorId
          doseRate
          baseline
          zScore
        }
      }
    }
  }
}
```

### Get Facilities

```graphql
query Facilities($type: FacilityType, $status: OperationalStatus) {
  facilities(type: $type, status: $status) {
    id
    name
    type
    location {
      lat
      lon
    }
    status
    reactorCount
    currentOutput
  }
}
```

### Search

```graphql
query Search($query: String!, $filters: SearchFilters) {
  search(query: $query, filters: $filters) {
    results {
      id
      title
      content
      location {
        lat
        lon
      }
      timestamp
      severity
    }
    totalCount
  }
}
```

## Mutations

### Acknowledge Alert

```graphql
mutation AcknowledgeAlert($alertId: UUID!) {
  acknowledgeAlert(alertId: $alertId) {
    id
    acknowledged
    acknowledgedAt
  }
}
```

### Create Alert Rule

```graphql
mutation CreateAlertRule($rule: AlertRuleInput!) {
  createAlertRule(rule: $rule) {
    id
    name
    enabled
    createdAt
  }
}
```

**Input:**
```json
{
  "name": "High Radiation Alert",
  "severityThreshold": "high",
  "doseRateThreshold": 2.5,
  "emailNotifications": true,
  "pushNotifications": true
}
```

### Run Plume Simulation

```graphql
mutation SimulatePlume($release: ReleaseParameters!, $duration: Int) {
  simulatePlume(release: $release, duration: $duration) {
    id
    status
    estimatedArrival
    concentrationGrid {
      lat
      lon
      concentration
    }
  }
}
```

**Input:**
```json
{
  "release": {
    "location": { "lat": 35.6762, "lon": 139.6503 },
    "height": 100,
    "rate": 1000000000,
    "isotope": "Cs-137",
    "duration": 3600
  },
  "duration": 72
}
```

## Subscriptions

### Real-time Readings

```graphql
subscription OnReading($sensorId: UUID!) {
  reading(sensorId: $sensorId) {
    sensorId
    doseRate
    timestamp
    qualityFlag
  }
}
```

### Anomaly Alerts

```graphql
subscription OnAnomaly($region: GeoPolygon!) {
  anomaliesInRegion(region: $region) {
    id
    severity
    message
    location {
      lat
      lon
    }
    timestamp
  }
}
```

### Global Alerts

```graphql
subscription OnGlobalAlert {
  globalAlerts {
    id
    type
    severity
    message
    affectedRegion {
      lat
      lon
      radius
    }
  }
}
```

## Error Handling

GraphQL errors return with HTTP 200 and the following structure:

```json
{
  "errors": [
    {
      "message": "Sensor not found",
      "path": ["readings", 0],
      "extensions": {
        "code": "NOT_FOUND",
        "sensorId": "550e8400-e29b-41d4-a716-446655440000"
      }
    }
  ]
}
```

**Error Codes:**
- `UNAUTHENTICATED` - Invalid or missing token
- `FORBIDDEN` - Insufficient permissions
- `NOT_FOUND` - Resource does not exist
- `BAD_REQUEST` - Invalid input parameters
- `RATE_LIMITED` - Too many requests

## Rate Limits

| Operation | Limit |
|-----------|-------|
| Queries | 1000/minute |
| Mutations | 100/minute |
| Subscriptions | 10 concurrent |

## SDKs

- **Rust:** `cherenkov-client` crate
- **TypeScript:** `@cherenkov/sdk` npm package
- **Python:** `cherenkov-py` PyPI package

## Examples

### cURL

```bash
curl -X POST https://api.cherenkov.io/graphql \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"query": "query { sensorsInRegion { id name } }"}'
```

### JavaScript

```javascript
import { CherenkovClient } from '@cherenkov/sdk';

const client = new CherenkovClient({ token: process.env.API_TOKEN });

const sensors = await client.query(`
  query Sensors($lat: Float!, $lon: Float!, $radius: Float!) {
    sensorsInRegion(region: {
      center: { lat: $lat, lon: $lon },
      radiusKm: $radius
    }) {
      id
      name
      currentReading { doseRate }
    }
  }
`, { lat: 35.6762, lon: 139.6503, radius: 100 });
