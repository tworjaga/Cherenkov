# Cherenkov Project TODO

## Completed

- [x] Initialize Rust workspace with 8 crates
- [x] Set up project structure (crates, web, docs, k8s)
- [x] Create ingestion daemon architecture
- [x] Implement stream processing with timely-dataflow
- [x] Add ScyllaDB time-series schema
- [x] Create WebGL2 globe renderer (WASM)
- [x] Build GraphQL API foundation
- [x] Set up SolidJS web frontend
- [x] Fix 605 TypeScript compilation errors
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

## In Progress

## Pending

### Backend
- [ ] Add distributed tracing (OpenTelemetry/Jaeger)
- [ ] Implement structured logging (JSON format)
- [ ] Add ML model training pipeline
- [ ] Implement weather data ingestion for plume modeling
- [ ] Add facility status inference
- [ ] Implement seismic-radiation correlation

### Frontend
- [ ] Connect to real GraphQL API
- [ ] Implement WebSocket subscriptions
- [ ] Add real sensor data visualization
- [ ] Implement globe interaction (zoom, pan, select)
- [ ] Add plume simulation UI
- [ ] Implement alert management
- [ ] Add user preferences and settings

### DevOps
- [ ] Set up Kubernetes manifests
- [ ] Configure Flux GitOps
- [ ] Add monitoring stack (Prometheus, Grafana)
- [ ] Set up distributed tracing (Jaeger)
- [ ] Configure multi-region deployment

### Documentation
- [ ] Add API documentation
- [ ] Create deployment guide
- [ ] Write contribution guidelines
- [ ] Add security documentation

## Notes

- GitHub: tworjaga
- Telegram: @al7exy
- License: AGPL-3.0
