# Changelog

All notable changes to the Cherenkov project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-01-15

### Added
- Initial release of Cherenkov radiological intelligence platform
- Real-time radiation monitoring from multiple data sources (SAFECAST, EURDEP, NRC)
- 3D WebGL globe visualization with deck.gl
- Machine learning anomaly detection using candle_nn
- Atmospheric dispersion modeling (Gaussian and Lagrangian particle models)
- GraphQL API with WebSocket subscriptions
- React/Next.js frontend with TypeScript strict mode
- Kubernetes deployment manifests
- Comprehensive test suite (unit + E2E with Playwright)

### Backend
- cherenkov-core: Event bus, circuit breaker, deduplication
- cherenkov-db: SQLite and ScyllaDB storage with time-series optimization
- cherenkov-api: GraphQL API, REST endpoints, WebSocket server
- cherenkov-ingest: Data ingestion pipeline with multiple sources
- cherenkov-stream: Real-time stream processing with anomaly detection
- cherenkov-ml: Neural network inference and training
- cherenkov-plume: Atmospheric dispersion simulation
- cherenkov-observability: Metrics, tracing, logging

### Frontend
- Next.js 14 with App Router
- TypeScript strict mode (0 errors)
- Tailwind CSS with custom Cherenkov design system
- deck.gl 3D globe with sensor/facility/anomaly layers
- Zustand state management
- Framer Motion animations (60fps)
- Recharts data visualization
- Keyboard shortcuts and accessibility (WCAG 2.1 AA)
- 29 unit tests + 7 E2E tests passing

### Infrastructure
- Docker containers for all services
- Kubernetes manifests with Kustomize
- GitHub Actions CI/CD workflows
- Prometheus/Grafana monitoring stack
- Jaeger distributed tracing

## [0.9.0] - 2024-01-01

### Added
- Beta release with core functionality
- Basic data ingestion from SAFECAST
- SQLite storage backend
- Simple REST API

### Fixed
- Various compilation issues on Windows
- Database connection pooling
