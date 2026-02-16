# Cherenkov Bug Fix TODO

## Phase 1: Fix Core Compilation Issues
- [x] Fix cherenkov-core/src/bus.rs - Make publish method async
- [x] Fix cherenkov-core/src/bus.rs - Add proper async handling

## Phase 2: Fix Database Code Issues
- [x] Fix cherenkov-db/src/sqlite.rs - Move AggregatedRow struct definition
- [x] Fix cherenkov-db/src/sqlite.rs - Resolve visibility issues

## Phase 3: Toolchain Configuration
- [x] Add cargo configuration for Windows builds
- [x] Configure to mitigate stack overflow issues

## Phase 4: Testing and Validation
- [ ] Run cargo check
- [ ] Run cargo test --test integration_test
- [x] Push changes to GitHub
