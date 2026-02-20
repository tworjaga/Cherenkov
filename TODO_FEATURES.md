# Cherenkov Feature Implementation TODO

## Phase 1: Plume Dispersion Modeling Enhancement

### Tasks
- [x] Integrate real-time plume visualization on the globe
  - [x] Update plume-layer.tsx with DeckGL visualization
  - [x] Connect to dispersion simulation API
  - [x] Add particle concentration heatmap

- [x] Connect weather data sources (NOAA GFS, Open-Meteo) to dispersion calculations
  - [x] Update weather.rs with real-time data fetching
  - [x] Integrate with dispersion.rs

- [x] Add plume simulation controls (release parameters, weather conditions)
  - [x] Enhance plume-simulator.tsx with usePlumeSimulation hook
  - [x] Add time-slider integration with play/pause/seek controls


### Completed
- [x] Implement evacuation zone calculations based on dose thresholds
  - [x] Update evacuation-zones.tsx component
  - [x] Add contour generation for dose thresholds
  - [x] Fix evacuation-zones.test.tsx - use getAllByText for population/radius assertions (8 tests passing)



### Completed
- [x] Connect weather data sources (NOAA GFS, Open-Meteo) to dispersion calculations
  - [x] weather.rs: WeatherDataProvider trait with GfsWeatherProvider, OpenMeteoWeatherProvider, CompositeWeatherProvider
  - [x] dispersion.rs: LagrangianDispersion with weather_provider integration, new_with_weather() constructor

- [x] Add plume simulation controls with temporal simulation
  - [x] use-plume-simulation.ts: Created hook with play/pause/seek/speed/loop controls
  - [x] plume-simulator.tsx: Integrated time-slider with simulation state management

- [x] Implement evacuation zone calculations based on dose thresholds
  - [x] evacuation-zones.tsx: Component with Critical/High/Medium zones
  - [x] evacuation-zones.test.tsx: All 8 tests passing

- [x] Analysis of current codebase state
- [x] Plan approval



## Phase 2: ML-based Anomaly Classification

### Completed
- [x] Fix ONNX model loading (update to current candle-onnx API)
  - [x] inference.rs: OnnxModel using candle-onnx 0.3 with ModelProto::from_file()
  - [x] Proper input/output name extraction from graph
  - [x] Device parameter threading through load path

- [x] Implement training pipeline for radiation anomaly detection
  - [x] training.rs: TrainingPipeline with TrainingConfig, LrScheduler, AugmentationConfig
  - [x] Checkpointing with resume capability
  - [x] Per-class accuracy and confusion matrix calculation

- [x] Add model versioning and hot-swapping
  - [x] ModelVersion struct with metadata (git commit, tags, metrics)
  - [x] hot_swap_model() in InferenceService for zero-downtime updates
  - [x] ModelManager with rollback capability

- [x] Integrate classification results into anomaly detection workflow
  - [x] integration.rs: MlAnomalyIntegration with process_reading() and process_batch()
  - [x] RecommendedAction enum (Monitor, Investigate, Alert, Evacuate, Critical)
  - [x] TrainingScheduler for automated retraining


## Phase 3: Alert Notification System
- [ ] Implement email notification service using SMTP
- [ ] Add SMS gateway integration (Twilio or similar)
- [ ] Create webhook delivery system
- [ ] Add Telegram bot notifications

## Phase 4: Mobile Application Foundation
- [ ] Set up React Native project structure
- [ ] Implement basic mobile dashboard
- [ ] Add push notification support
- [ ] Create offline data caching
