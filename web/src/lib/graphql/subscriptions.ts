import { gql } from 'graphql-request';

export const SENSOR_UPDATES = gql`
  subscription SensorUpdates($sensorId: ID!) {
    sensorUpdates(sensorId: $sensorId) {
      sensorId
      timestamp
      doseRate
      latitude
      longitude
    }
  }
`;

export const ANOMALY_ALERTS = gql`
  subscription AnomalyAlerts($minLat: Float, $maxLat: Float, $minLon: Float, $maxLon: Float) {
    anomalyAlerts(minLat: $minLat, maxLat: $maxLat, minLon: $minLon, maxLon: $maxLon) {
      anomalyId
      sensorId
      severity
      zScore
      detectedAt
      message
    }
  }
`;

export const ALL_SENSOR_UPDATES = gql`
  subscription AllSensorUpdates {
    allSensorUpdates {
      sensorId
      timestamp
      doseRate
      latitude
      longitude
    }
  }
`;

export const PLUME_PARTICLES = gql`
  subscription PlumeParticles($simulationId: ID!, $batchSize: Int) {
    plumeParticles(simulationId: $simulationId, batchSize: $batchSize) {
      simulationId
      particles {
        id
        x
        y
        z
        concentration
        timestamp
      }
      timestamp
    }
  }
`;

export const EVACUATION_ZONES = gql`
  subscription EvacuationZones($simulationId: ID!) {
    evacuationZones(simulationId: $simulationId) {
      simulationId
      zones {
        id
        name
        level
        doseThreshold
        polygon {
          coordinates
        }
        center {
          latitude
          longitude
        }
        radius
        population
        timestamp
      }
      timestamp
    }
  }
`;
