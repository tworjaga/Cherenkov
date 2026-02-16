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

## In Progress

## Pending

### Backend
- [ ] Implement actual data source crawlers (Safecast, uRADMonitor, EPA RadNet)
- [ ] Add WebSocket server for real-time updates
- [ ] Implement anomaly detection algorithms (Welford, Isolation Forest)
- [ ] Set up ScyllaDB cluster configuration
- [ ] Add ML inference service with candle
- [ ] Implement plume dispersion modeling
- [ ] Add authentication and authorization
- [ ] Set up observability (tracing, metrics, logging)

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
