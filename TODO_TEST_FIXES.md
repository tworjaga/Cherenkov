# Cherenkov Test Fixes - Comprehensive Plan

## Test Results Summary
- Total Tests: 255
- Passed: 212
- Failed: 42
- Skipped: 1

## Critical Issues to Fix

### 1. Date Utility (1 failure)
- File: `src/lib/utils/dates.test.ts`
- Issue: Timezone offset causing hour mismatch in `addTime` test
- Expected: 15, Received: 16

### 2. UI Component Test Failures

#### Switch Component (4 failures)
- File: `src/components/ui/switch/switch.test.tsx`
- Issues:
  - `data-state="checked"` attribute not found
  - Custom className not applied
  - Size variants (w-8, w-14) not applied

#### Textarea Component (1 failure)
- File: `src/components/ui/textarea/textarea.test.tsx`
- Issue: Error state class `border-alert-critical` not applied
- Current: Uses `border-[#ff3366]` instead of semantic class

#### Dropdown Component (3 failures)
- File: `src/components/ui/dropdown/dropdown.test.tsx`
- Issue: Items not found when dropdown is clicked
- Root cause: Radix UI dropdown content not rendering in test environment

#### Select Component (2 failures)
- File: `src/components/ui/select/select.test.tsx`
- Issue: Options not found when select is clicked
- Root cause: Radix UI select content not rendering in test environment

#### Checkbox Component (2 failures)
- File: `src/components/ui/checkbox/checkbox.test.tsx`
- Issues:
  - Label rendering with children causes React error
  - Custom className not applied

#### LayerToggles Component (1 failure)
- File: `src/components/globe/controls/layer-toggles.test.tsx`
- Issue: Cannot find checkbox role elements
- Root cause: Switch components use role="switch" not "checkbox"

#### SensorList Component (1 failure)
- File: `src/components/dashboard/sensor-list.test.tsx`
- Issue: Cannot find text "safecast"
- Root cause: Test expects specific data source label

### 3. Accessibility Issues

#### Modal Component
- Warning: Missing DialogTitle for accessibility
- Warning: Missing Description or aria-describedby

## Fix Priority Order

### Phase 1: Core Utilities
1. Fix dates.test.ts timezone issue

### Phase 2: UI Components
2. Fix Switch component implementation
3. Fix Textarea error state styling
4. Fix Checkbox label rendering
5. Fix Dropdown/Select test setup (add proper test wrappers)

### Phase 3: Dashboard Components
6. Fix LayerToggles test assertions
7. Fix SensorList test data

### Phase 4: Accessibility
8. Add DialogTitle to Modal component
9. Add proper aria attributes

## Implementation Strategy

Each fix will:
1. Read the failing test to understand expected behavior
2. Update component implementation to match test expectations
3. Run tests to verify fix
4. Commit with conventional commit format
5. Push to GitHub
