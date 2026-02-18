# Cherenkov Project Testing Status

## Current Status Summary

### Web Frontend (Next.js + TypeScript)
- **Unit Tests**: 253 passing, 1 skipped (43 test files)
- **TypeScript Errors**: 40+ errors (see details below)
- **Linting**: In progress

### Rust Backend
- **Unit Tests**: 0 tests across all 8 crates (no tests written)
- **Build**: Successful with warnings
- **Warnings**: Unused imports, dead code, unused variables

## TypeScript Errors to Fix

### Globe Component Errors
1. `src/components/globe/globe.tsx` - Type mismatches with deck.gl
2. `src/components/globe/layers/*.tsx` - Layer type errors
3. Missing type declarations for deck.gl

### UI Component Errors
1. `src/components/ui/chart/chart.test.tsx` - Missing color property
2. `src/components/ui/date-picker/date-picker.stories.tsx` - Invalid props
3. Various unused imports in test files

### Store/State Errors
1. `src/stores/settings-store.ts` - Unused defaultSettings
2. `src/components/layout/right-panel/sensor-detail/sensor-detail-panel.tsx` - Property 'sensors' does not exist

### Test File Errors
1. Missing imports (beforeEach)
2. Type mismatches in test assertions

## Rust Backend Issues

### Missing Unit Tests
All 8 crates have 0 unit tests:
- cherenkov-api
- cherenkov-core
- cherenkov-db
- cherenkov-ingest
- cherenkov-ml
- cherenkov-observability
- cherenkov-plume
- cherenkov-stream

### Warnings to Fix
1. Unused imports (NaiveDateTime, QualityFlag, error, AsyncCommands)
2. Unused variables (`reading`)
3. Dead code

## Next Steps

1. Fix TypeScript type errors in web frontend
2. Add missing type declarations for deck.gl
3. Fix Rust compiler warnings
4. Write unit tests for Rust crates
5. Run integration tests
6. Run E2E tests with Playwright
7. Verify Docker builds
8. Test CI/CD pipeline

## Test Commands

```bash
# Web tests
cd cherenkov/web && npm test -- --run

# Type checking
cd cherenkov/web && npm run type-check

# Linting
cd cherenkov/web && npm run lint

# Rust tests
cd cherenkov && cargo test --workspace

# Rust build
cd cherenkov && cargo build --workspace

# E2E tests
cd cherenkov/web && npx playwright test
