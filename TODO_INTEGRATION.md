# Cherenkov Backend-Frontend Integration Plan

## Phase 1: Verify Core Integration
- [x] Test all GraphQL queries (sensors, facilities, anomalies, alerts, globalStatus)
- [x] Verify WebSocket subscription functionality
- [x] Confirm data flows correctly from GraphQL to Zustand stores
- [x] Test connection status indicators and error states


## Phase 2: Fix Integration Issues
- [x] Address any GraphQL query/response mismatches
- [x] Fix WebSocket connection/reconnection issues
- [x] Ensure proper error handling and loading states
- [x] Validate type consistency between backend schema and frontend types


## Phase 3: Comprehensive Testing
- [x] Run all unit tests for components, hooks, and utilities
- [x] Execute integration tests for API and WebSocket functionality
- [x] Run end-to-end tests for critical user flows
- [x] Test browser compatibility and performance


## Phase 4: Browser Testing & Validation
- [x] Manual testing of all dashboard features
- [x] Verify real-time updates work correctly
- [x] Test navigation and routing
- [x] Validate responsive design and accessibility


## Phase 5: Final Verification
- [x] Ensure all backend features connect properly to frontend
- [x] Verify all pipelines (data flow, WebSocket, GraphQL) work correctly
- [x] Confirm no errors in browser console or terminal output
- [x] Validate all tests pass


## Core Files to Verify
- `web/src/lib/graphql/client.ts` - GraphQL and WebSocket client configuration
- `web/src/hooks/use-graphql.ts` - GraphQL query hooks
- `web/src/hooks/use-websocket.ts` - WebSocket subscription management
- `web/src/components/providers/data-provider.tsx` - Data synchronization
- `web/src/stores/app-store.ts` - Connection status management
- `web/src/stores/data-store.ts` - Data state management
- `mock-api/server.js` - Mock API server

## Test Commands
```bash
# Unit tests
cd cherenkov/web && npm run test:unit

# Integration tests
cd cherenkov/web && npm run test:integration

# E2E tests
cd cherenkov/web && npm run test:e2e

# All tests
cd cherenkov/web && npm test
