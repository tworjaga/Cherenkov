# Cherenkov Full Project Testing Plan

## Project Structure
- **Rust Backend**: 8 crates (cherenkov-core, cherenkov-ingest, cherenkov-stream, cherenkov-db, cherenkov-ml, cherenkov-plume, cherenkov-api, cherenkov-observability)
- **Web Frontend**: Next.js 14 + TypeScript + Tailwind CSS
- **Mock API**: Node.js server on port 8080

## Testing Phases

### Phase 1: Rust Backend Testing
- [ ] Test cherenkov-core (event bus, core types)
- [ ] Test cherenkov-db (SQLite, Scylla, Redis, cache)
- [ ] Test cherenkov-ingest (data sources, pipeline, normalizer)
- [ ] Test cherenkov-stream (windowing, anomaly detection, correlation)
- [ ] Test cherenkov-ml (inference, training)
- [ ] Test cherenkov-plume (dispersion, weather, particle)
- [ ] Test cherenkov-api (GraphQL, REST, WebSocket, auth)
- [ ] Test cherenkov-observability (metrics, tracing, logging)

### Phase 2: Web Frontend Testing
- [ ] TypeScript compilation check
- [ ] ESLint validation
- [ ] Unit tests (Vitest) - 253 tests
- [ ] E2E tests (Playwright) - 100 tests across 5 browsers
- [ ] Component tests
- [ ] Integration tests

### Phase 3: Integration Testing
- [ ] Web + Mock API integration
- [ ] WebSocket connectivity
- [ ] GraphQL queries/mutations/subscriptions
- [ ] REST API endpoints

### Phase 4: Performance & Quality
- [ ] Lighthouse audit
- [ ] Bundle analysis
- [ ] Memory leak detection
- [ ] Accessibility audit (WCAG)

## Current Status
- Web GUI optimization: COMPLETED (collapsible panels, responsive design, accessibility)
- Unit tests: 253 tests (99.6% pass rate)
- E2E tests: 100 tests passing
- Rust backend: Build pending (MinGW issue)

## Known Issues
1. Rust build fails due to missing MinGW dlltool.exe
2. 3 pre-existing test failures in web frontend (unrelated to optimization)
