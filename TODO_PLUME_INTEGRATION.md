# Plume Dispersion Integration TODO

## Step 1: Add cherenkov-plume dependency to API
- [ ] Update cherenkov-api/Cargo.toml with cherenkov-plume dependency

## Step 2: Implement real plume simulation resolver
- [ ] Replace placeholder simulate_plume resolver
- [ ] Add particle streaming subscription

## Step 3: Connect frontend to real API
- [ ] Update use-plume-simulation.ts with GraphQL mutations
- [ ] Connect plume-layer.tsx to real data

## Step 4: Implement evacuation zones
- [ ] Add dose threshold contour generation
- [ ] Integrate with dispersion output

## Step 5: Weather data integration
- [ ] Connect weather.rs to NOAA GFS/Open-Meteo
- [ ] Add weather parameter controls
