# Cherenkov Bug Fix TODO

## Current Status: ~60 Compilation Errors Remaining (cherenkov-ml FIXED)

### Phase 1: Core Fixes (COMPLETED)
- [x] Fix cherenkov-core/src/bus.rs - Make publish method async
- [x] Fix cherenkov-db/src/sqlite.rs - Move AggregatedRow struct definition
- [x] Fix cherenkov-db/src/scylla.rs - Consistency enum variants
- [x] Add .cargo/config.toml for Windows builds
- [x] Switch to GNU toolchain (stable-x86_64-pc-windows-gnu)

### Phase 2: Type System Fixes (COMPLETED)
- [x] Add Algorithm enum variants (Welford, ZScore, IQR, Grubbs)
- [x] Add AggregationLevel variants (OneMinute, FiveMinutes)
- [x] Fix AggregationLevel match arms in sqlite.rs
- [x] Add CorrelatedEventDetected event variant

### Phase 3: Dependency Injection & Clone Fixes (COMPLETED)
- [x] Add Clone impls for CircuitBreaker, DeadLetterQueue, Deduplicator
- [x] Fix pipeline lifetime issues with Arc<Self>
- [x] Add missing dependencies (uuid, regex, rand, futures-util, tokio-stream, metrics)

### Phase 4: candle_nn API Fixes (COMPLETED)
- [x] Fix VarMap::get signature - changed to (shape, name, init, dtype, device) pattern
- [x] Fix Init::Zeros -> Init::Const(0.0) migration
- [x] Fix E0502 borrow checker error in training.rs (checkpoint resume)
- [x] Add uuid v5 feature for Uuid::new_v5 support
- [x] cherenkov-ml now compiles successfully

### Phase 5: Remaining Errors (IN PROGRESS)

#### cherenkov-api (56 errors - axum router type issues)
- [ ] Fix Router state type mismatches (expected tuple, found single Arc)
- [ ] Fix websocket router signature - create_websocket_router expects tuple state
- [ ] Fix rest::create_router return type to match Router with state
- [ ] Fix Handler trait bounds for route handlers

#### cherenkov-ingest (6 errors - field/method issues)
- [ ] Fix NormalizedReading field mismatches (cell_id, uncertainty)
- [ ] Fix EventBus::publish async call in pipeline
- [ ] Fix Uuid::new_v5 usage (now fixed with v5 feature)

#### cherenkov-stream (Import/struct issues)
- [ ] Fix Anomaly struct field mismatches (anomaly_id, detected_at)
- [ ] Fix Algorithm -> String conversion
- [ ] Fix CorrelationEngine constructor signature

### Commits Pushed (19 total)
All fixes committed and pushed to: https://github.com/tworjaga/Cherenkov.git

### Build Status
- cherenkov-core: Compiles
- cherenkov-db: Compiles  
- cherenkov-ml: Compiles (FIXED)
- cherenkov-plume: Compiles with warnings
- cherenkov-stream: Has errors
- cherenkov-api: Has 56 errors (axum type system)
- cherenkov-ingest: Has 6 errors

### Next Actions Required
1. Fix axum Router state type consistency in cherenkov-api
2. Fix cherenkov-stream Anomaly struct conversions
3. Fix cherenkov-ingest NormalizedReading field issues
