# Cherenkov Full Testing Plan

## Current State Assessment

### Unit Tests (Vitest)
- **Total**: 253 tests
- **Passing**: 233 tests
- **Failing**: 20 tests
- **Status**: 92% pass rate

### E2E Tests (Playwright)
- **Browsers**: Chromium, Firefox, WebKit
- **Status**: Running (50+ tests executed)
- **Passing**: Globe, Dashboard, Auth, Sensors tests
- **Issues**: Some timeout issues on Firefox

### Mock API Server
- **Status**: Running on port 8080
- **WebSocket**: Active with frequent connections/disconnections
- **GraphQL/REST**: Functional

### Rust Backend
- **Status**: Build failed (MinGW dlltool.exe missing)
- **Workaround**: Mock API server in use

## Test Failure Categories

### 1. Component Test Failures
- [ ] `dates.test.ts` - Timezone issue (expected 15, got 16)
- [ ] `sensor-list.test.tsx` - Text matching issue (/safecast/i not found)
- [ ] `dropdown.test.tsx` - Dropdown not opening in test
- [ ] `select.test.tsx` - Select options not found
- [ ] `textarea.test.tsx` - Error state class mismatch
- [ ] `switch.test.tsx` - data-state attribute not set
- [ ] `checkbox.test.tsx` - Children prop error on input
- [ ] `date-picker.test.tsx` - Placeholder text not found
- [ ] `search.test.tsx` - searchbox role not found
- [ ] `radio.test.tsx` - Custom class not applied
- [ ] `layer-toggles.test.tsx` - Checkbox role not found

### 2. E2E Test Issues
- [ ] Firefox timeout issues (30s+ per test)
- [ ] WebSocket connection instability
- [ ] Strict mode violations (duplicate elements)

## Action Plan

### Phase 1: Fix Unit Tests
1. Fix date utility tests (timezone handling)
2. Fix component prop issues (checkbox, switch, radio)
3. Fix test selectors (dropdown, select, search)
4. Fix styling assertions (textarea, date-picker)

### Phase 2: Stabilize E2E Tests
1. Configure Playwright timeouts
2. Fix strict mode violations
3. Stabilize WebSocket connections
4. Add proper test data setup

### Phase 3: Integration Testing
1. Test full user flows
2. Verify API endpoints
3. Test WebSocket subscriptions
4. Validate data consistency

### Phase 4: Documentation
1. Update test documentation
2. Create troubleshooting guide
3. Document known issues
