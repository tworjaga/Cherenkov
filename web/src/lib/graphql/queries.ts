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
