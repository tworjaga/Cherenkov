# Cherenkov Bug Fix TODO

## Phase 1: Fix Core Compilation Issues
- [ ] Fix cherenkov-core/src/bus.rs - Make publish method async
- [ ] Fix cherenkov-core/src/bus.rs - Add proper async handling

## Phase 2: Fix Database Code Issues
- [ ] Fix cherenkov-db/src/sqlite.rs - Move AggregatedRow struct definition
- [ ] Fix cherenkov-db/src/sqlite.rs - Resolve visibility issues

## Phase 3: Toolchain Configuration
- [ ] Add cargo configuration for Windows builds
- [ ] Configure to mitigate stack overflow issues

## Phase 4: Testing and Validation
- [ ] Run cargo check
- [ ] Run cargo test --test integration_test
- [ ] Push changes to GitHub
