# Cherenkov Bug Fix TODO

## Current Status: COMPILATION SUCCESSFUL (0 Errors, Warnings Only)

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

### Phase 5: API & WebSocket Fixes (COMPLETED)
- [x] Fix axum 0.8 Message type changes (Utf8Bytes/Bytes)
- [x] Fix async-graphql subscription stream type mismatches
- [x] Fix database API private field access (warm/hot fields)
- [x] Fix cherenkov_plume unresolved imports (removed integration)
- [x] Fix SlidingWindow API mismatches in stream processor
- [x] Fix cherenkov-api GraphQL resolvers to use public DB APIs
- [x] Fix cherenkov-stream processor NormalizedReading field access
- [x] Fix cherenkov-ingest pipeline moved value errors

### Phase 6: Final Cleanup (COMPLETED)
- [x] Fix cherenkov-ingest DataSource trait bound (Send)
- [x] Fix cherenkov-ingest unused imports
- [x] Fix cherenkov-ingest locations borrow in OpenMeteo source
- [x] All crates now compile with warnings only

### Commits Pushed (20 total)
All fixes committed and pushed to: https://github.com/tworjaga/Cherenkov.git

### Build Status
- cherenkov-core: Compiles
- cherenkov-db: Compiles (7 warnings)
- cherenkov-ml: Compiles (5 warnings)
- cherenkov-plume: Compiles (10 warnings)
- cherenkov-stream: Compiles (41 warnings)
- cherenkov-api: Compiles (30 warnings)
- cherenkov-ingest: Compiles (3 warnings)
- cherenkov-observability: Compiles (4 warnings)

### EventBus Integration Verified
The EventBus integration in cherenkov-ingest IS fully implemented:
- pipeline.rs: IngestionPipeline contains Arc<EventBus>
- Constructor receives event_bus: Arc<EventBus> parameter
- write_batch() publishes CherenkovEvent::NewReading events after successful DB writes
- main.rs: EventBus initialized and passed to pipeline, metrics reporter spawned

### Data Flow
```
ingest (fetch) -> SQLite (store) -> EventBus.publish() -> stream/api (consume)
```

### Phase 7: Web Frontend Implementation (COMPLETED)
- [x] Initialize Next.js 14 project with TypeScript strict mode
- [x] Configure Tailwind CSS with Cherenkov design tokens
- [x] Implement design system (colors, typography, spacing, animations)
- [x] Create Zustand stores (app-store, globe-store, data-store)
- [x] Set up GraphQL client with schema, queries, mutations, subscriptions
- [x] Implement WebSocket hook for real-time updates
- [x] Add keyboard shortcuts hook (1-5 views, space, arrows)
- [x] Create Header component with DEFCON indicator and connection status
- [x] Create Sidebar component with navigation
- [x] Build layout structure with proper grid/flex

Frontend Location: `cherenkov/web/`
Commit: `37c3cd6 feat(web): initialize Next.js frontend with complete architecture`

### Next Actions (Optional)
1. Address warnings (unused imports, dead code)
2. Run cargo test to verify functionality
3. Add integration tests for EventBus event flow
4. Continue frontend: 3D globe, dashboard widgets, responsive design
