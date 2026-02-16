# Cherenkov Implementation TODO

## Phase 0: Core Infrastructure (P0) - COMPLETED
- [x] Create cherenkov-core crate with EventBus
- [x] Implement shared event types (NewReading, AnomalyDetected, AlertTriggered)
- [x] Add configuration system with YAML and env var support
- [x] Wire cherenkov-core dependency to ingest, stream, api crates
- [x] Integrate EventBus into ingest main.rs to publish NewReading events
- [x] Integrate EventBus into stream main.rs to subscribe and publish AnomalyDetected
- [x] Integrate EventBus into api main.rs for WebSocket broadcasting


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

## Phase 3: Stream Processing (P1) - COMPLETED
- [x] Rewrite with async tokio streams
- [x] Integrate anomaly detection with DB
- [x] Implement WebSocket broadcasting
- [x] Add cross-sensor correlation
- [x] Add sliding window statistics

## Phase 4: API Completion (P1) - COMPLETED
- [x] Complete GraphQL resolvers
- [x] Add authentication layer
- [x] Implement WebSocket subscriptions
- [x] Add REST endpoints
- [x] Add rate limiting

## Phase 5: ML and Simulation (P3) - COMPLETED
- [x] Rule-based isotope identification
- [x] Gaussian plume model
- [x] Weather data integration

## Phase 6: Observability and Deployment (P2) - COMPLETED
- [x] Integrate metrics across components
- [x] Complete Docker Compose
- [x] Add health checks
- [x] Set up Grafana dashboards


---

## Implementation Summary

### Components Implemented

| Component | Status | Key Features |
|-----------|--------|--------------|
| cherenkov-core | COMPLETE | EventBus, shared events, configuration system |
| cherenkov-db | COMPLETE | Hot/warm/cold tiering, ScyllaDB/SQLite/Redis, migrations |
| cherenkov-ingest | COMPLETE | Concurrent sources, circuit breaker, DLQ, deduplication |
| cherenkov-stream | COMPLETE | Async streams, anomaly detection, correlation engine |
| cherenkov-api | COMPLETE | GraphQL, REST, auth, rate limiting, WebSocket |
| cherenkov-ml | COMPLETE | Rule-based isotope classifier |
| cherenkov-plume | COMPLETE | Gaussian plume model, dose calculations |
| cherenkov-observability | COMPLETE | Metrics, tracing, logging |


### Git Commits
- 5e28469: feat(core): add EventBus, configuration, and shared event types for inter-crate communication
- 6e4e4dd: chore(deps): add cherenkov-core dependency to ingest, stream, and api crates
- 030505f: feat(config): add default configuration file with all service settings
- 7521251: feat(db): implement tiered storage with hot/warm/cold architecture
- 302b665: feat(ingest): implement concurrent ingestion pipeline with resilience patterns
- 6243d5f: feat(stream): implement async stream processor with anomaly detection
- dd379cd: feat(api): add authentication module and database integration
- ff26dbc: feat(api): add REST endpoints and rate limiting
- bc9fb07: feat(ml): implement rule-based isotope classifier
- 3fa3ebe: feat(plume): implement Gaussian plume model with dose calculations
- 13e9d23: feat(deploy): add Dockerfiles for ingest, api, and stream services


### Next Steps (User to Complete)
- [x] Add OpenAQ data source integration


- [x] Add Open-Meteo weather integration

- [x] Implement EPA RadNet CSV parsing

- [x] Add EURDEP XML SOAP parsing

- [x] Complete GraphQL subscription resolvers

- [x] Add WebSocket real-time streaming

- [x] Implement ML training pipeline

- [x] Add GPU acceleration for plume simulation

- [x] Set up CI/CD with GitHub Actions

- [x] Deploy to production environment
