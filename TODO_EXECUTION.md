# Cherenkov Project Execution TODO

## Phase 1: Backend Fixes and Implementation

### Database Layer
- [ ] Add anomaly query methods to cherenkov-db/src/lib.rs
- [ ] Implement anomaly storage in cherenkov-stream/src/processor.rs
- [ ] Add sensor/facility data population methods

### GraphQL API
- [ ] Complete resolver implementations in cherenkov-api/src/graphql/resolvers.rs
- [ ] Fix empty sensor/anomaly/facility list returns
- [ ] Implement proper data fetching from database

### REST API
- [ ] Implement DEFCON calculation logic in cherenkov-api/src/rest.rs
- [ ] Add actual anomaly count queries

### WebSocket
- [ ] Verify EventBus broadcasting to WebSocket clients
- [ ] Test real-time sensor updates and anomaly alerts

## Phase 2: Frontend-Backend Integration

### GraphQL Client
- [ ] Verify GraphQL client setup in web/src/lib/graphql/client.ts
- [ ] Test all queries against backend
- [ ] Validate subscription connections

### WebSocket Frontend
- [ ] Verify WebSocket connection in web/src/hooks/use-websocket.ts
- [ ] Test real-time data flow

### Plume Integration
- [ ] Test plume simulation GraphQL endpoint
- [ ] Verify frontend visualization with backend data

## Phase 3: Testing and Quality Assurance

### Test Fixes
- [ ] Replace unwrap() calls in tests with proper error handling
- [ ] Fix failing unit tests
- [ ] Add integration tests for API endpoints

### E2E Testing
- [ ] Test complete data flow: ingest -> database -> API -> frontend
- [ ] Test WebSocket real-time updates
- [ ] Test plume simulation workflow

### Browser Testing
- [ ] Launch web application
- [ ] Test all features in browser
- [ ] Verify real-time updates

## Phase 4: Feature Completion

### Alert System
- [ ] Implement alert acknowledgment mutations
- [ ] Add alert rule management

### ML Pipeline
- [ ] Complete model training pipeline
- [ ] Add model management UI

### Performance
- [ ] Add caching where needed
- [ ] Optimize database queries

## Git Commit Strategy

Every change must be committed with conventional commits:
- `feat:` - New features
- `fix:` - Bug fixes
- `refactor:` - Code refactoring
- `docs:` - Documentation
- `test:` - Test updates
- `style:` - Code style changes

Format: `<type>(<scope>): <description>`
