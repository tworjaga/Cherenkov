# Cherenkov Project TODO

## Completed

- [x] Initialize Rust workspace with 8 crates
- [x] Set up project structure (crates, web, docs, k8s)
- [x] Create ingestion daemon architecture
- [x] Implement stream processing with timely-dataflow
- [x] Add ScyllaDB time-series schema
- [x] Create WebGL2 globe renderer (WASM)
- [x] Build GraphQL API foundation
- [x] Set up React web frontend
- [x] Fix TypeScript compilation errors
- [x] Configure router, WASM types, component lifecycle
- [x] Add Docker Compose for local development
- [x] Set up CI/CD with GitHub Actions
- [x] Create architecture documentation
- [x] Implement data source crawlers (Safecast, uRADMonitor, EPA RadNet)
- [x] Add WebSocket server for real-time updates
- [x] Implement anomaly detection algorithms (Welford, Isolation Forest)
- [x] Set up ScyllaDB cluster configuration with connection pooling
- [x] Add ML inference service with candle and batch processing
- [x] Implement plume dispersion modeling with physics simulation
- [x] Add JWT authentication and ABAC authorization
- [x] Set up observability metrics collection
- [x] Add distributed tracing (OpenTelemetry/Jaeger)
- [x] Implement structured logging (JSON format)
- [x] Add ML model training pipeline
- [x] Implement weather data ingestion for plume modeling
- [x] Add facility status inference
- [x] Implement seismic-radiation correlation
- [x] Connect to real GraphQL API
- [x] Implement WebSocket subscriptions
- [x] Add real sensor data visualization
- [x] Implement globe interaction (zoom, pan, select)
- [x] Add plume simulation UI
- [x] Implement alert management
- [x] Add user preferences and settings
- [x] Integrate WebGL2 WASM globe renderer with custom layers
- [x] Create TimeSlider component with LIVE/PAUSED/REPLAY modes
- [x] Build AlertCard and AlertFeed with filtering
- [x] Add GlobalChart for radiation trends
- [x] Implement useWebSocket hook for real-time data
- [x] Fix WASM TypeScript definitions (setView, resize, addFacility, updatePlume, setLayerVisibility)
- [x] Add ErrorBoundary component for WASM/WebGL error handling
- [x] Verify Tailwind CSS custom color configuration
- [x] Confirm date-fns dependency in package.json

## In Progress


## Pending

### Critical Fixes Needed
- [ ] Fix ARIA accessibility warnings in components
- [ ] Implement actual WebGL shader programs for globe rendering
- [ ] Add facility data to WASM renderer
- [ ] Implement plume particle system in WebGL


### UI/UX Improvements
- [ ] Add loading states for async operations
- [ ] Implement toast notifications for alerts
- [ ] Add responsive design for mobile/tablet
- [ ] Create dark/light theme toggle
- [ ] Add keyboard navigation shortcuts help panel
- [ ] Implement data export functionality
- [ ] Add print-friendly styles for reports

### Performance Optimizations
- [ ] Implement sensor clustering for large datasets
- [ ] Add virtual scrolling for alert feed
- [ ] Optimize WebGL rendering with instancing
- [ ] Add service worker for offline support
- [ ] Implement requestAnimationFrame throttling
- [ ] Add memory leak detection and cleanup

### Testing
- [ ] Add unit tests for React components
- [ ] Create integration tests for WebSocket
- [ ] Add E2E tests with Playwright
- [ ] Implement WASM test suite
- [ ] Add performance benchmarks
- [ ] Create visual regression tests

### Backend Integration
- [ ] Connect WebSocket to actual backend endpoint
- [ ] Implement GraphQL queries for historical data
- [ ] Add authentication flow to web app
- [ ] Implement rate limiting for API calls
- [ ] Add request caching layer

### DevOps
- [x] Set up Kubernetes manifests
- [x] Configure Flux GitOps
- [x] Add monitoring stack (Prometheus, Grafana)
- [x] Set up distributed tracing (Jaeger)
- [x] Configure multi-region deployment
- [ ] Add WASM build to CI pipeline
- [ ] Configure CDN for static assets
- [ ] Implement blue-green deployments

### Documentation
- [x] Add API documentation
- [x] Create deployment guide
- [x] Write contribution guidelines
- [x] Add security documentation
- [x] Add frontend component storybook
- [x] Create user manual
- [x] Add troubleshooting guide


## Notes

- GitHub: tworjaga
- Telegram: @al7exy
- License: MIT
