# Plume Dispersion Integration TODO

## Step 1: Add cherenkov-plume dependency to API
- [x] Update cherenkov-api/Cargo.toml with cherenkov-plume dependency
  - Completed: 41a817e feat(deps): add cherenkov-plume dependency to API crate

## Step 2: Implement real plume simulation resolver
- [x] Replace placeholder simulate_plume resolver
  - Completed: 42127c4 feat(api): integrate GaussianPlumeModel into simulate_plume resolver
  - Completed: 717d0f7 feat(api): integrate cherenkov-plume crate for dispersion simulation
- [ ] Add particle streaming subscription


## Step 3: Connect frontend to real API
- [x] Update use-plume-simulation.ts with GraphQL mutations
  - Completed: 6d3a420 feat(graphql): add plume simulation queries
- [ ] Connect plume-layer.tsx to real data


## Step 4: Implement evacuation zones
- [ ] Add dose threshold contour generation
- [ ] Integrate with dispersion output

## Step 5: Weather data integration
- [x] Connect weather.rs to NOAA GFS/Open-Meteo
  - Completed: 76958ad fix(plume): correct WeatherConditions field names for LocalWeather integration
- [x] Add weather parameter controls
  - Completed: 76958ad fix(plume): correct WeatherConditions field names for LocalWeather integration
