# Cherenkov Project Testing & Fixing TODO

## Phase 1: Rust Backend Testing & Fixes

### 1.1 Compile All Crates
- [x] cherenkov-core
- [x] cherenkov-db
- [x] cherenkov-ingest
- [x] cherenkov-stream
- [x] cherenkov-api
- [x] cherenkov-ml
- [x] cherenkov-plume
- [x] cherenkov-observability


### 1.2 Run Unit Tests
- [ ] cherenkov-core tests
- [ ] cherenkov-db tests
- [ ] cherenkov-ingest tests
- [ ] cherenkov-stream tests
- [ ] cherenkov-api tests
- [ ] cherenkov-ml tests
- [ ] cherenkov-plume tests
- [ ] cherenkov-observability tests

### 1.3 Integration Tests
- [ ] EventBus communication
- [ ] Database operations
- [ ] Data ingestion pipeline
- [ ] Stream processing
- [ ] API endpoints

## Phase 2: Frontend Testing & Fixes

### 2.1 TypeScript Compilation
- [ ] Check all TypeScript files compile
- [ ] Fix type errors

### 2.2 Component Tests
- [ ] Run Vitest unit tests
- [ ] Fix failing tests

### 2.3 E2E Tests
- [ ] Run Playwright tests
- [ ] Fix failing scenarios

### 2.4 Build Verification
- [ ] Build Next.js application
- [ ] Verify static exports

## Phase 3: Integration Testing

### 3.1 Full Pipeline
- [ ] Test complete data flow
- [ ] Verify real-time updates

### 3.2 Cross-service Communication
- [ ] EventBus message passing
- [ ] WebSocket broadcasting

## Phase 4: Infrastructure

### 4.1 Docker
- [ ] Build all Docker images
- [ ] Test docker-compose

### 4.2 CI/CD
- [ ] Test GitHub Actions workflows

## Current Status
- Completed: Phase 1.1 - All 8 crates compiled successfully
- Completed: Phase 1.2 - cargo test --workspace executed
- Test Results: 0 tests run (no unit tests implemented in workspace)
- Warnings: 47 total (dead code, unused fields, private interfaces)
- Critical: Redis v0.24.0 future incompatibility warning
- Next: Phase 1.3 - Fix compilation warnings and add unit tests
