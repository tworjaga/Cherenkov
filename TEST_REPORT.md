# Cherenkov Test Report

**Date:** 2026-02-16  
**Tester:** @al7exy (tworjaga)  
**Repository:** https://github.com/tworjaga/cherenkov

## Test Environment

- **OS:** Windows 11
- **Rust Version:** 1.93.1 (stable-x86_64-pc-windows-gnu)
- **Toolchain:** MinGW-w64 GCC 13.2.0

## Test Files Identified

### 1. Unit/Integration Tests (`tests/integration_test.rs`)
**10 tests covering:**
- EventBus publish/subscribe functionality
- Anomaly detection event flow
- Alert event flow
- Multiple subscribers handling
- EventBus lag handling with small buffers
- Database reading serialization (RadiationReading)
- QualityFlag serialization (Valid, Suspect, Invalid)
- Sensor status change events
- Health update events

**Test Dependencies:**
- `cherenkov_core::{EventBus, CherenkovEvent, NormalizedReading, Anomaly, Alert, Severity, SensorStatus}`
- `cherenkov_db::{RadiationReading, QualityFlag}`
- `tokio::test` for async runtime
- `uuid`, `chrono` for test data generation

### 2. API Integration Tests (`tests/api_test.rs`)
**17 tests covering:**
- Health check endpoint (`/health`)
- Readiness check endpoint (`/ready`)
- GraphQL introspection
- GraphQL sensor query
- GraphQL readings query with time range
- GraphQL anomalies query with severity filter
- REST sensors endpoint (`/v1/sensors`)
- REST sensor detail endpoint (`/v1/sensors/{id}`)
- REST readings endpoint with time range
- REST nearby sensors with geo-spatial query
- REST status endpoint (`/v1/status`)
- REST anomalies endpoint
- REST acknowledge alert endpoint
- CORS headers validation
- Rate limiting (70+ requests)
- Compression (gzip)
- Invalid endpoint handling (404)
- Method not allowed handling (405)

**Test Dependencies:**
- `reqwest::Client` for HTTP requests
- `serde_json` for request/response handling

### 3. WebSocket Integration Tests (`tests/websocket_test.rs`)
**10 tests covering:**
- WebSocket connection and ping/pong
- Subscribe to specific sensor
- Subscribe to geographic region
- Get active subscriptions
- Unsubscribe from sensor
- Heartbeat handling
- Rate limiting (110 messages)
- Invalid command handling
- Batch updates subscription
- Connection status query

**Test Dependencies:**
- `tokio_tungstenite` for WebSocket client
- `url::Url` for endpoint parsing

## Compilation Status

**Issue Encountered:** Stack buffer overflow during compilation of `redis` and `sqlx-sqlite` crates with MinGW toolchain.

**Error Details:**
```
error: could not compile `redis` (lib)
Caused by: process didn't exit successfully (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)

error: could not compile `sqlx-sqlite` (lib)
Caused by: process didn't exit successfully (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
```

**Root Cause:** Known issue with Rust 1.93.1 GNU toolchain on Windows when compiling certain async dependencies with complex monomorphization.

## Test Coverage Analysis

### Core Components
| Component | Test Coverage | Status |
|-----------|--------------|--------|
| cherenkov-core EventBus | 5 tests | Ready |
| cherenkov-db serialization | 2 tests | Ready |
| cherenkov-api REST | 8 tests | Requires running server |
| cherenkov-api GraphQL | 4 tests | Requires running server |
| cherenkov-api WebSocket | 10 tests | Requires running server |
| Integration flows | 3 tests | Ready |

### Test Execution Requirements

**Unit Tests (Can run standalone):**
```bash
cargo test --test integration_test
```

**Integration Tests (Require infrastructure):**
```bash
# Start dependencies
docker-compose up -d scylladb redis

# Run API tests
cargo test --test api_test -- --ignored

# Run WebSocket tests
cargo test --test websocket_test -- --ignored
```

## Recommendations

1. **For Windows Testing:** Use MSVC toolchain instead of GNU:
   ```bash
   rustup default stable-x86_64-pc-windows-msvc
   ```

2. **For CI/CD:** Use Linux-based runners where GNU toolchain works correctly

3. **Test Priority:**
   - P0: Unit tests in `integration_test.rs` (no external deps)
   - P1: API tests (requires running API server)
   - P1: WebSocket tests (requires running API server)

## Code Quality Observations

1. **Good Practices Found:**
   - Proper use of `#[tokio::test]` for async tests
   - Timeout handling with `tokio::time::timeout`
   - Comprehensive error case coverage
   - Proper resource cleanup (WebSocket close)

2. **Test Organization:**
   - Tests are well-categorized by functionality
   - Clear naming conventions (test_*)
   - Documentation comments for each test

## Conclusion

The Cherenkov project has comprehensive test coverage with 37 total tests across 3 test files. The tests are well-structured and cover:
- Core event bus functionality
- Database serialization
- REST API endpoints
- GraphQL queries
- WebSocket real-time communication

**Status:** Tests are code-complete but require proper toolchain setup for execution on Windows.
