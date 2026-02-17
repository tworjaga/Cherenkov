# Rust Warning Fixes - Cherenkov Project

## Status: 98 warnings remaining

### Crates with warnings:
- cherenkov-ml: ~20 warnings
- cherenkov-plume: ~25 warnings  
- cherenkov-stream: ~30 warnings
- cherenkov-api: ~15 warnings
- cherenkov-ingest: ~8 warnings

### Fixed:
- [x] cherenkov-db: 0 warnings
- [x] cherenkov-observability: 0 warnings

### In Progress:
- [ ] cherenkov-ml - inference.rs, training.rs
- [ ] cherenkov-plume - dispersion.rs, weather.rs, particle.rs
- [ ] cherenkov-stream - window.rs, processor.rs, correlation.rs
- [ ] cherenkov-api - main.rs, rest.rs, websocket.rs
- [ ] cherenkov-ingest - pipeline.rs, sources.rs

## Warning Categories:
1. Dead code (unused fields, methods, variables)
2. Unused imports
3. Private interfaces
4. Unused async
5. Unnecessary mut
6. Unused results
