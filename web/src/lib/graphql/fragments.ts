import { gql } from '@apollo/client';

export const SENSOR_FIELDS = gql`
  fragment SensorFields on Sensor {
    id
    name
    type
    location {
      lat
      lng
      elevation
    }
    status
    lastReading
    metadata {
      manufacturer
      model
      installDate
    }
  }
`;

export const FACILITY_FIELDS = gql`
  fragment FacilityFields on Facility {
    id
    name
    type
    location {
      lat
      lng
    }
    status
    reactorCount
    capacity
    operator
  }
`;

export const ANOMALY_FIELDS = gql`
  fragment AnomalyFields on Anomaly {
    id
    sensorId
    type
    severity
    detectedAt
    acknowledgedAt
    resolvedAt
    description
    confidence
  }
`;

export const READING_FIELDS = gql`
  fragment ReadingFields on Reading {
    id
    sensorId
    timestamp
    value
    unit
    quality
    isAnomalous
  }
`;

export const PLUME_FIELDS = gql`
  fragment PlumeFields on Plume {
    id
    facilityId
    releaseRate
    releaseHeight
    particleSize
    windSpeed
    windDirection
    stabilityClass
    calculatedAt
    geometry {
      type
      coordinates
    }
  }
`;

export const USER_FIELDS = gql`
  fragment UserFields on User {
    id
    email
    name
    role
    preferences {
      timezone
      units
      notifications
    }
    createdAt
    lastLoginAt
  }
`;
