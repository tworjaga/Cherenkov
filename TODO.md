# Cherenkov Bug Fix TODO

## Current Status: 119 Compilation Errors Remaining

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

### Phase 4: Remaining 119 Errors (IN PROGRESS)

#### cherenkov-ml (candle_nn API issues)
- [ ] Fix candle_onnx::onnx::OnnxModel import
- [ ] Fix VarMap::get signature (Shape trait bound errors)
- [ ] Fix Init::Zeros -> Init::Const(0.0) migration
- [ ] Fix type mismatches in training.rs

#### cherenkov-api (WebSocket/GraphQL issues)
- [ ] Fix tokio_stream::wrappers::BroadcastStream import
- [ ] Fix axum::extract::ws import
- [ ] Fix async-graphql InputObject derive macro
- [ ] Add missing warn! macro imports

#### cherenkov-stream (Import issues)
- [ ] Fix crate::window::Reading struct
- [ ] Fix SlidingWindow::is_stale method
- [ ] Fix cherenkov_plume integration

#### cherenkov-ingest (Field issues)
- [ ] Add cell_id field to NormalizedReading
- [ ] Fix borrow checker issues in pipeline

### Commits Pushed (17 total)
All fixes committed and pushed to: https://github.com/tworjaga/Cherenkov.git

### Next Actions Required
1. Fix candle_nn API breaking changes (major effort)
2. Add missing WebSocket/GraphQL dependencies
3. Resolve cherenkov_plume module imports
4. Fix remaining borrow checker issues
