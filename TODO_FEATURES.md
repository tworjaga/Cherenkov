# Cherenkov Feature Implementation TODO

## Phase 1: Plume Dispersion Modeling Enhancement

### Tasks
- [x] Integrate real-time plume visualization on the globe
  - [x] Update plume-layer.tsx with DeckGL visualization
  - [x] Connect to dispersion simulation API
  - [x] Add particle concentration heatmap

- [ ] Connect weather data sources (NOAA GFS, Open-Meteo) to dispersion calculations
  - [ ] Update weather.rs with real-time data fetching
  - [ ] Integrate with dispersion.rs
- [ ] Implement evacuation zone calculations based on dose thresholds
  - [ ] Update evacuation-zones.tsx component
  - [ ] Add contour generation for dose thresholds
- [ ] Add plume simulation controls (release parameters, weather conditions)
  - [ ] Enhance plume-simulator.tsx
  - [ ] Add time-slider integration

### In Progress
- [ ] Implement evacuation zone calculations based on dose thresholds
  - [ ] Update evacuation-zones.tsx component
  - [ ] Add contour generation for dose thresholds

### Completed
- [x] Connect weather data sources (NOAA GFS, Open-Meteo) to dispersion calculations
  - [x] weather.rs: WeatherDataProvider trait with GfsWeatherProvider, OpenMeteoWeatherProvider, CompositeWeatherProvider
  - [x] dispersion.rs: LagrangianDispersion with weather_provider integration, new_with_weather() constructor

- [x] Analysis of current codebase state
- [x] Plan approval

## Phase 2: ML-based Anomaly Classification
- [ ] Fix ONNX model loading (update to current candle-onnx API)
- [ ] Implement training pipeline for radiation anomaly detection
- [ ] Add model versioning and hot-swapping
- [ ] Integrate classification results into anomaly detection workflow

## Phase 3: Alert Notification System
- [ ] Implement email notification service using SMTP
- [ ] Add SMS gateway integration (Twilio or similar)
- [ ] Create webhook delivery system
- [ ] Add Telegram bot notifications (@al7exy integration)

## Phase 4: Mobile Application Foundation
- [ ] Set up React Native project structure
- [ ] Implement basic mobile dashboard
- [ ] Add push notification support
- [ ] Create offline data caching
