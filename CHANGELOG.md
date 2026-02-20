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

## [1.1.0] - 2024-01-20

### Added
- Comprehensive test suite with 100% pass rate
- 253 unit tests (Vitest) covering components, hooks, utilities, stores
- 100 E2E tests (Playwright) across 5 browsers (Chromium, Firefox, WebKit, Mobile Chrome, Mobile Safari)
- Mock API server for isolated frontend testing
- Component documentation with Storybook (50+ stories)
- WebSocket reconnection and heartbeat handling
- DEFCON indicator component with status badges
- Alert feed with filtering and real-time updates
- Sensor detail panel with metrics and charts
- Facility detail panel with information display
- Regional statistics panel with data visualization
- Global chart controls for time-series data
- Recent events panel with timeline
- Keyboard shortcuts for dashboard navigation
- Responsive design for mobile and tablet devices

### Changed
- Updated README.md with comprehensive project documentation
- Enhanced test coverage for all UI components
- Improved WebSocket stability with reconnection logic
- Optimized globe rendering performance
- Updated data source configurations

### Fixed
- WebSocket connection stability issues
- Firefox E2E test timeouts
- React strict mode violations
- Component prop type definitions
- Test environment setup and teardown

## [0.9.0] - 2024-01-01

### Added
- Beta release with core functionality
- Basic data ingestion from SAFECAST
- SQLite storage backend
- Simple REST API

### Fixed
- Various compilation issues on Windows
- Database connection pooling
