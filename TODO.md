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
- [x] Check all TypeScript files compile
- [x] Fix type errors

### 2.2 Component Tests
- [x] Run Vitest unit tests
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
- Completed: Phase 2.1 - TypeScript compilation passed (0 errors)
- Completed: Phase 2.2 - Vitest unit tests executed
- Test Results: 230 passed, 24 failed, 1 skipped (255 total)
- Failed Suites: 16 files | Passed: 31 files (47 total)
- Critical Issues:
  - Playwright E2E config conflict with Vitest (4 suites)
  - UI component test expectations mismatched (16 tests)
  - Date utility timezone issue (1 test)
  - Missing accessibility attributes in components
- Warnings: 47 Rust compilation warnings
- Next: Phase 2.2 fixes - Update component implementations to match test expectations
