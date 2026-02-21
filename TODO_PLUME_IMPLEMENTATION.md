# Plume Integration Implementation TODO

## Tasks

### Task 1: Create usePlumeParticles Hook
- [ ] Create `web/src/hooks/use-plume-particles.ts`
- [ ] Subscribe to PLUME_PARTICLES GraphQL subscription
- [ ] Transform particle data for deck.gl format
- [ ] Handle WebSocket connection lifecycle
- [ ] Commit: `feat: Add usePlumeParticles hook for real-time particle streaming`

### Task 2: Create useEvacuationZones Hook
- [ ] Create `web/src/hooks/use-evacuation-zones.ts`
- [ ] Subscribe to EVACUATION_ZONES GraphQL subscription
- [ ] Transform zone data for deck.gl PolygonLayer
- [ ] Parse dose contours from subscription data
- [ ] Commit: `feat: Add useEvacuationZones hook for evacuation zone updates`

### Task 3: Update plume-layer.tsx
- [ ] Replace mock data with real subscription data
- [ ] Integrate usePlumeParticles hook
- [ ] Integrate useEvacuationZones hook
- [ ] Add loading states and error handling
- [ ] Commit: `feat: Connect plume-layer to real dispersion data`

### Task 4: Backend Integration
- [ ] Update `crates/cherenkov-api/src/graphql/subscription.rs`
- [ ] Connect to cherenkov-plume dispersion calculations
- [ ] Real-time particle position updates
- [ ] Dose contour generation from dispersion output
- [ ] Commit: `feat: Integrate dispersion calculations with GraphQL subscriptions`

### Task 5: Testing and Validation
- [ ] Test WebSocket connections
- [ ] Verify particle rendering performance
- [ ] Validate dose contour accuracy
- [ ] End-to-end integration testing
- [ ] Commit: `test: Add plume integration tests`

## Progress
- Started: Task 1
- Current: Creating hooks and updating components
