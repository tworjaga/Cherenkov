# Cherenkov Testing and Validation Plan

## Project Overview
- **Backend**: Multi-crate Rust workspace (8 crates)
- **Frontend**: Next.js 14 + TypeScript + Tailwind + GraphQL + WebSocket + Deck.gl + Zustand
- **Current State**: 49 TypeScript errors in 33 files, Rust compiles with warnings, 0 tests written

## Phase 1: Rust Backend Testing (Foundation)

### 1.1 Dependency Audit
- [ ] Update Cargo.lock
- [ ] Check for security vulnerabilities with `cargo audit`
- [ ] Verify all dependencies are up to date

### 1.2 Compile Clean
- [x] Fix all warnings in cherenkov-core
- [x] Fix all warnings in cherenkov-ingest
- [x] Fix all warnings in cherenkov-stream
- [x] Fix all warnings in cherenkov-db
- [x] Fix all warnings in cherenkov-ml
- [x] Fix all warnings in cherenkov-plume
- [x] Fix all warnings in cherenkov-api
- [x] Fix all warnings in cherenkov-observability
- [x] Verify `cargo check --workspace` passes with 0 warnings

### 1.3 Unit Tests
- [ ] Write tests for cherenkov-core (EventBus, events)
- [ ] Write tests for cherenkov-db (database operations)
- [ ] Write tests for cherenkov-ml (inference, training)
- [ ] Write tests for cherenkov-plume (dispersion, particle, weather)
- [ ] Write tests for cherenkov-stream (processor, window, correlation)
- [ ] Write tests for cherenkov-ingest (pipeline, sources)
- [ ] Write tests for cherenkov-api (GraphQL, REST, WebSocket)
- [ ] Write tests for cherenkov-observability (tracing, metrics)

### 1.4 Integration Tests
- [ ] Test inter-crate communication
- [ ] Test data pipeline flow
- [ ] Test error handling

### 1.5 Performance Tests
- [ ] Benchmark stream processing
- [ ] Benchmark ML inference
- [ ] Benchmark database operations

## Phase 2: Frontend TypeScript Validation (Clean Code)

### 2.1 Error Analysis
- [x] Categorize remaining 49 errors by type
- [ ] Identify unused imports
- [ ] Identify missing props
- [ ] Identify deck.gl type issues
- [ ] Identify interface mismatches

### 2.2 Import Cleanup
- [ ] Remove unused imports from 33 affected files
- [ ] Fix import ordering
- [ ] Verify no circular dependencies

### 2.3 Type Definitions
- [ ] Fix DialogProps interface (isOpen -> open, onClose -> onOpenChange)
- [ ] Fix SwitchProps interface (checked/onCheckedChange)
- [ ] Fix Checkbox component types
- [ ] Fix layer-toggles.tsx types
- [ ] Fix SensorList prop types
- [ ] Fix collapse-button.tsx types
- [ ] Fix chart-controls.tsx types
- [ ] Fix recent-events-panel.tsx types
- [ ] Fix facility-detail.tsx types
- [ ] Fix facility-header.tsx types

### 2.4 Deck.gl Types
- [ ] Resolve deck.gl type declarations
- [ ] Update deckgl.d.ts
- [ ] Fix layer component types

### 2.5 Component Props
- [ ] Ensure all component props properly typed
- [ ] Fix GlobeProps interface
- [ ] Fix ReadingChartProps interface
- [ ] Fix AppState interface

### 2.6 GraphQL Types
- [ ] Validate GraphQL schema matches TypeScript types
- [ ] Fix fragments.ts imports
- [ ] Install missing @apollo/client dependency

## Phase 3: Integration Testing (End-to-End)

### 3.1 API Integration
- [ ] Test GraphQL endpoints
- [ ] Test REST endpoints
- [ ] Test WebSocket connections

### 3.2 Data Pipeline
- [ ] Test ingest -> stream -> db -> api flow
- [ ] Test real-time updates
- [ ] Test WebSocket subscriptions

### 3.3 UI Components
- [ ] Test all React components with proper data
- [ ] Verify Storybook stories work
- [ ] Test component interactions

### 3.4 State Management
- [ ] Test Zustand stores
- [ ] Test state updates
- [ ] Test persistence

### 3.5 Error Handling
- [ ] Test error boundaries
- [ ] Test recovery mechanisms
- [ ] Test loading states

## Phase 4: Production Readiness (Performance & Security)

### 4.1 Load Testing
- [ ] Test under realistic load
- [ ] Test with large datasets
- [ ] Test concurrent connections

### 4.2 Security Audit
- [ ] Check for vulnerabilities in dependencies
- [ ] Verify authentication flows
- [ ] Test authorization

### 4.3 Performance Optimization
- [ ] Optimize slow paths
- [ ] Verify bundle size
- [ ] Check memory usage

### 4.4 Monitoring
- [ ] Ensure observability works
- [ ] Verify tracing
- [ ] Check metrics collection

### 4.5 Deployment
- [ ] Test Docker builds
- [ ] Test Kubernetes deployment
- [ ] Verify CI/CD pipelines

### 4.6 Documentation
- [ ] Complete API documentation
- [ ] Complete user manual
- [ ] Update CHANGELOG

## Current Status

### Completed
- Phase 1.2: Compile Clean - All workspace crates compile with 0 warnings
- Partial Phase 2.1: Error Analysis - 49 TypeScript errors identified

### In Progress
- Phase 2.2: Import Cleanup - Removing unused imports

### Pending
- Phase 1.3: Unit Tests
- Phase 1.4: Integration Tests
- Phase 1.5: Performance Tests
- Phase 2.3-2.6: Type Definitions
- Phase 3: Integration Testing
- Phase 4: Production Readiness

## Error Summary

### TypeScript Errors (49 total in 33 files)
1. Unused imports (15 files)
2. Missing props (8 files)
3. Deck.gl type issues (4 files)
4. Interface mismatches (6 files)

### Rust Status
- 0 compilation errors
- 0 workspace warnings
- 0 tests written
