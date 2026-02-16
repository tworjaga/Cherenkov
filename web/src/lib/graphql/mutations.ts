import { gql } from 'graphql-request';

export const ACKNOWLEDGE_ALERT = gql`
  mutation AcknowledgeAlert($alertId: ID!) {
    acknowledgeAlert(alertId: $alertId) {
      id
      acknowledged
      acknowledgedAt
    }
  }
`;

export const CREATE_ALERT_RULE = gql`
  mutation CreateAlertRule($rule: AlertRuleInput!) {
    createAlertRule(rule: $rule) {
      id
      name
      enabled
      createdAt
    }
  }
`;
