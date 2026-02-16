# Cherenkov Implementation TODO

## Phase 1: Data Layer Foundation (P0)
- [ ] Implement RadiationDatabase struct with tiering
- [ ] Add SQLite warm storage module
- [ ] Add object storage cold archive module
- [ ] Implement time-range queries
- [ ] Implement geo-spatial queries
- [ ] Add migration system
- [ ] Add connection pooling with deadpool/bb8
- [ ] Add retry logic with exponential backoff

## Phase 2: Ingestion Hardening (P0)
- [ ] Create ingestion pipeline with FuturesUnordered
- [ ] Integrate database writes
- [ ] Add dead letter queue
- [ ] Implement circuit breaker pattern
- [ ] Add backpressure handling
- [ ] Fix Safecast JSON parsing
- [ ] Add deduplication logic

## Phase 3: Stream Processing (P1)
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
