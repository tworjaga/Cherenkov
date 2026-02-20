import { gql } from '@apollo/client';

export const GET_SENSORS = gql`
  query GetSensors {
    sensors {
      id
      name
      location {
        lat
        lng
      }
      type
      status
      lastReading {
        timestamp
        value
        unit
        isValid
      }
    }
  }
`;

export const GET_ALERTS = gql`
  query GetAlerts($limit: Int, $acknowledged: Boolean) {
    alerts(limit: $limit, acknowledged: $acknowledged) {
      id
      title
      message
      severity
      timestamp
      acknowledged
      sensorId
      facilityId
    }
  }
`;

export const GET_FACILITIES = gql`
  query GetFacilities {
    facilities {
      id
      name
      location {
        lat
        lng
      }
      type
      status
      reactorType
      powerOutput
    }
  }
`;

export const GET_SYSTEM_STATUS = gql`
  query GetSystemStatus {
    systemStatus {
      defconLevel
      activeAlerts
      totalSensors
      onlineSensors
      lastUpdate
    }
  }
`;

export const ALERT_SUBSCRIPTION = gql`
  subscription OnNewAlert {
    newAlert {
      id
      title
      message
      severity
      timestamp
      sensorId
      facilityId
    }
  }
`;

export const SENSOR_UPDATE_SUBSCRIPTION = gql`
  subscription OnSensorUpdate {
    sensorUpdate {
      id
      lastReading {
        timestamp
        value
        unit
        isValid
      }
    }
  }
`;
