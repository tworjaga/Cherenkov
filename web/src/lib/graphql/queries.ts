import { gql } from 'graphql-request';

export const GET_SENSORS = gql`
  query Sensors {
    sensors {
      id
      name
      latitude
      longitude
      status
      lastReading
    }
  }
`;

export const GET_SENSOR = gql`
  query Sensor($id: ID!) {
    sensor(id: $id) {
      id
      name
      latitude
      longitude
      status
      lastReading
    }
  }
`;

export const GET_READINGS = gql`
  query Readings($sensorIds: [ID!]!, $from: Timestamp!, $to: Timestamp!, $aggregation: String) {
    readings(sensorIds: $sensorIds, from: $from, to: $to, aggregation: $aggregation) {
      id
      sensorId
      timestamp
      doseRate
      unit
    }
  }
`;

export const GET_ANOMALIES = gql`
  query Anomalies($severity: [String!], $since: Timestamp!, $limit: Int) {
    anomalies(severity: $severity, since: $since, limit: $limit) {
      id
      sensorId
      severity
      zScore
      detectedAt
    }
  }
`;

export const GET_FACILITIES = gql`
  query Facilities {
    facilities {
      id
      name
      facilityType
      latitude
      longitude
      status
    }
  }
`;

export const GET_GLOBAL_STATUS = gql`
  query GlobalStatus {
    globalStatus {
      defconLevel
      status
      activeAnomalies
      lastUpdated
    }
  }
`;

export const RUN_PLUME_SIMULATION = gql`
  query RunPlumeSimulation($input: PlumeSimulationInput!) {
    runPlumeSimulation(input: $input) {
      id
      status
      particles {
        id
        latitude
        longitude
        altitude
        concentration
        timestamp
      }
      evacuationZones {
        id
        level
        boundary {
          latitude
          longitude
        }
        maxDoseRate
        timeToEvacuate
      }
      weatherConditions {
        windSpeed
        windDirection
        temperature
        pressure
        stabilityClass
      }
      createdAt
      completedAt
    }
  }
`;

export const GET_PLUME_SIMULATION = gql`
  query PlumeSimulation($id: ID!) {
    plumeSimulation(id: $id) {
      id
      status
      particles {
        id
        latitude
        longitude
        altitude
        concentration
        timestamp
      }
      evacuationZones {
        id
        level
        boundary {
          latitude
          longitude
        }
        maxDoseRate
        timeToEvacuate
      }
      weatherConditions {
        windSpeed
        windDirection
        temperature
        pressure
        stabilityClass
      }
      createdAt
      completedAt
    }
  }
`;

export const GET_EVACUATION_ZONES = gql`
  query EvacuationZones($simulationId: ID!) {
    evacuationZones(simulationId: $simulationId) {
      id
      level
      boundary {
        latitude
        longitude
      }
      maxDoseRate
      timeToEvacuate
      populationAffected
    }
  }
`;

export const GET_WEATHER_FOR_LOCATION = gql`
  query WeatherForLocation($latitude: Float!, $longitude: Float!, $timestamp: Timestamp) {
    weatherForLocation(latitude: $latitude, longitude: $longitude, timestamp: $timestamp) {
      windSpeed
      windDirection
      temperature
      pressure
      humidity
      stabilityClass
      timestamp
    }
  }
`;
