# Cherenkov Implementation TODO

## Phase 1: Data Layer Foundation (P0) - COMPLETED
- [x] Implement RadiationDatabase struct with tiering
- [x] Add SQLite warm storage module
- [x] Add object storage cold archive module
- [x] Implement time-range queries
- [x] Implement geo-spatial queries
- [x] Add migration system
- [x] Add connection pooling with deadpool/bb8
- [x] Add retry logic with exponential backoff

## Phase 2: Ingestion Hardening (P0) - COMPLETED
- [x] Create ingestion pipeline with FuturesUnordered
- [x] Integrate database writes
- [x] Add dead letter queue
- [x] Implement circuit breaker pattern
- [x] Add backpressure handling
- [x] Fix Safecast JSON parsing
- [x] Add deduplication logic

## Phase 3: Stream Processing (P1) - IN PROGRESS
- [ ] Rewrite with async tokio streams
- [ ] Integrate anomaly detection with DB
- [ ] Implement WebSocket broadcasting
- [ ] Add cross-sensor correlation
- [ ] Add sliding window statistics

## Phase 4: API Completion (P1)
- [ ] Complete GraphQL resolvers
- [ ] Add authentication layer
- [ ] Implement WebSocket subscriptions
- [ ] Add REST endpoints
- [ ] Add rate limiting

## Phase 5: ML and Simulation (P3)
- [ ] Rule-based isotope identification
- [ ] Gaussian plume model
- [ ] Weather data integration

## Phase 6: Observability and Deployment (P2)
- [ ] Integrate metrics across components
- [ ] Complete Docker Compose
- [ ] Add health checks
- [ ] Set up Grafana dashboards
