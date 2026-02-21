# Plume Integration Implementation TODO

## Tasks

### Task 1: Create usePlumeParticles Hook
- [x] Create `web/src/hooks/use-plume-particles.ts`
- [x] Subscribe to PLUME_PARTICLES GraphQL subscription
- [x] Transform particle data for deck.gl format
- [x] Handle WebSocket connection lifecycle
- [x] Commit: `feat: Add usePlumeParticles hook for real-time particle streaming`

### Task 2: Create useEvacuationZones Hook
- [x] Create `web/src/hooks/use-evacuation-zones.ts`
- [x] Subscribe to EVACUATION_ZONES GraphQL subscription
- [x] Transform zone data for deck.gl PolygonLayer
- [x] Parse dose contours from subscription data
- [x] Commit: `feat: Add useEvacuationZones hook for evacuation zone updates`

### Task 3: Update plume-layer.tsx
- [x] Replace mock data with real subscription data
- [x] Integrate usePlumeParticles hook
- [x] Integrate useEvacuationZones hook
- [x] Add loading states and error handling
- [x] Commit: `feat: Connect plume-layer to real dispersion data`

### Task 4: Backend Integration
- [x] Update `crates/cherenkov-api/src/graphql/subscription.rs`
- [x] Connect to cherenkov-plume dispersion calculations
- [x] Real-time particle position updates
- [x] Dose contour generation from dispersion output
- [x] Commit: `feat: Integrate dispersion calculations with GraphQL subscriptions`

### Task 5: Testing and Validation
- [ ] Test WebSocket connections
- [ ] Verify particle rendering performance
- [ ] Validate dose contour accuracy
- [ ] End-to-end integration testing
- [ ] Commit: `test: Add plume integration tests`

## Progress
- Started: Task 1
- Current: Creating hooks and updating components
