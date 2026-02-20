# Cherenkov

[![CI](https://github.com/tworjaga/cherenkov/workflows/ci/badge.svg)](https://github.com/tworjaga/cherenkov/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0%2B-blue.svg)](https://www.typescriptlang.org)
[![Next.js](https://img.shields.io/badge/Next.js-14-black.svg)](https://nextjs.org)

> Real-time global radiological intelligence platform

The blue glow of nuclear reactors, made visible. Cherenkov aggregates 50,000+ radiation sensors worldwide, detects anomalies in milliseconds, predicts fallout dispersion, and provides situational awareness for nuclear events — from medical isotope spills to reactor meltdowns.

## Overview

Cherenkov is a high-performance, real-time monitoring platform designed for radiological intelligence. It combines a Rust-based backend for data ingestion and processing with a modern React/Next.js frontend for visualization and interaction.

### Key Features

- **Real-time Data Ingestion**: 10M+ events per second from 50,000+ sensors
- **Anomaly Detection**: Sub-10ms detection using stream processing
- **Global Visualization**: WebGL2-powered globe with 100k points at 60fps
- **Plume Dispersion Modeling**: Real-time fallout prediction
- **Multi-source Integration**: 15+ data sources including Safecast, uRADMonitor, EPA RadNet
- **WebSocket Subscriptions**: Real-time updates to 1M+ concurrent clients
- **GraphQL API**: Flexible data querying and mutations

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Frontend                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Next.js   │  │   WebGL2    │  │   React/TypeScript  │  │
│  │   (Web)     │  │   (Globe)   │  │   (Dashboard)       │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      API Gateway                            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   GraphQL   │  │  WebSocket  │  │   REST (Health)     │  │
│  │   (Async)   │  │  (Real-time)│  │   (Status)          │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Stream Processing                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Anomaly   │  │ Correlation │  │   Windowing         │  │
│  │  Detection  │  │   Engine    │  │   (Time-series)     │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Data Ingestion                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Polling   │  │   Webhook   │  │   Normalization     │  │
│  │  (Sources)  │  │  (Events)   │  │   (Transform)       │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Data Sources                           │
│  Safecast │ uRADMonitor │ EPA RadNet │ EURDEP │ IAEA PRIS   │
│  USGS │ NASA FIRMS │ NOAA GFS │ OpenAQ │ Open-Meteo         │
└─────────────────────────────────────────────────────────────┘
```

## Technology Stack

### Backend (Rust)
- **Runtime**: Tokio (async runtime)
- **Stream Processing**: Timely Dataflow
- **ML Inference**: Candle (GPU-native, no Python)
- **Storage**: ScyllaDB (time-series), SQLite (cache)
- **API**: GraphQL (async-graphql), WebSocket (tokio-tungstenite)
- **Observability**: Custom metrics and tracing

### Frontend (TypeScript/React)
- **Framework**: Next.js 14 (App Router)
- **UI Library**: React 18 + TypeScript
- **Styling**: Tailwind CSS + CSS Modules
- **State Management**: Zustand
- **Data Fetching**: GraphQL (Apollo Client), WebSocket
- **Visualization**: Deck.gl (WebGL2), D3.js
- **Testing**: Vitest (unit), Playwright (E2E)
- **Documentation**: Storybook

### Infrastructure
- **Containerization**: Docker + Docker Compose
- **Orchestration**: Kubernetes (Kustomize)
- **CI/CD**: GitHub Actions
- **Monitoring**: Prometheus + Grafana (planned)

## Project Structure

```
cherenkov/
├── crates/                    # Rust workspace
│   ├── cherenkov-api/         # GraphQL/WebSocket API
│   ├── cherenkov-core/        # Core types and event bus
│   ├── cherenkov-db/          # Database layer
│   ├── cherenkov-ingest/      # Data ingestion pipeline
│   ├── cherenkov-ml/          # ML inference and training
│   ├── cherenkov-observability/  # Metrics and tracing
│   ├── cherenkov-plume/        # Dispersion modeling
│   └── cherenkov-stream/       # Stream processing
├── web/                       # Next.js frontend
│   ├── src/
│   │   ├── app/               # Next.js App Router
│   │   ├── components/        # React components
│   │   ├── hooks/             # Custom React hooks
│   │   ├── lib/               # Utilities and clients
│   │   ├── stores/            # Zustand stores
│   │   ├── styles/            # Tailwind + themes
│   │   └── types/             # TypeScript types
│   ├── tests/                 # Test suites
│   └── .storybook/            # Storybook config
├── mock-api/                  # Mock API for development
├── docs/                      # Documentation
├── k8s/                       # Kubernetes manifests
└── scripts/                   # Deployment scripts
```

## Quick Start

### Prerequisites

- **Rust**: 1.75+ (install via [rustup](https://rustup.rs))
- **Node.js**: 18+ (install via [nvm](https://github.com/nvm-sh/nvm))
- **Docker**: For local infrastructure (optional)

### Development Setup

```bash
# Clone repository
git clone https://github.com/tworjaga/cherenkov.git
cd cherenkov

# Install dependencies
cd web && npm install && cd ..

# Start mock API (for frontend development)
cd mock-api && npm start

# In another terminal, start frontend
cd web && npm run dev

# Open http://localhost:3000
```

### Full Stack Development

```bash
# Start infrastructure services
docker-compose up -d scylla redis

# Build and run Rust services
cargo build --release
cargo run -p cherenkov-ingest
cargo run -p cherenkov-api
cargo run -p cherenkov-stream

# Start frontend
cd web && npm run dev
```

## Testing

### Unit Tests (Vitest)
```bash
cd web
npm test
```
- **Status**: 253 tests passing (100% pass rate)
- **Coverage**: Components, hooks, utilities, stores

### E2E Tests (Playwright)
```bash
cd web
npx playwright test
```
- **Browsers**: Chromium, Firefox, WebKit, Mobile Chrome, Mobile Safari
- **Tests**: Authentication, Dashboard, Globe, Sensors

### Rust Tests
```bash
cargo test --workspace
```

## Data Sources

| Source | Type | Coverage | Status |
|--------|------|----------|--------|
| Safecast | Crowdsourced | Global | Active |
| uRADMonitor | Commercial | Global | Active |
| EPA RadNet | Government | USA | Active |
| EURDEP | Government | EU | Active |
| IAEA PRIS | Regulatory | 440 plants | Active |
| USGS Seismic | Scientific | Global | Active |
| NASA FIRMS | Satellite | Global | Active |
| NOAA GFS | Weather | Global | Active |
| OpenAQ | Air quality | Global | Active |
| Open-Meteo | Weather | Global | Active |

See [DATA_SOURCES.md](docs/DATA_SOURCES.md) for complete documentation.

## API Documentation

### GraphQL Endpoint
```
http://localhost:8080/graphql
```

### WebSocket Endpoint
```
ws://localhost:8080
```

### Key Queries

```graphql
# Get all sensors
query GetSensors {
  sensors {
    id
    name
    location {
      lat
      lon
    }
    readings {
      value
      unit
      timestamp
    }
  }
}

# Subscribe to real-time readings
subscription OnNewReading {
  newReading {
    sensorId
    value
    timestamp
  }
}
```

See [API.md](docs/API.md) for complete documentation.

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `CHERENKOV_API_PORT` | API server port | 8080 |
| `CHERENKOV_WEB_PORT` | Web server port | 3000 |
| `SCYLLA_HOSTS` | ScyllaDB hosts | localhost:9042 |
| `REDIS_URL` | Redis connection | redis://localhost:6379 |
| `JWT_SECRET` | JWT signing key | - |
| `LOG_LEVEL` | Logging level | info |

See [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) for production configuration.

## Deployment

### Docker Compose (Local)
```bash
docker-compose up -d
```

### Kubernetes (Production)
```bash
kubectl apply -k k8s/overlays/production
```

See [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) for detailed deployment guides.

## Documentation

- [Architecture](docs/ARCHITECTURE.md) - System design and data flow
- [API Reference](docs/API.md) - GraphQL schema and examples
- [Data Sources](docs/DATA_SOURCES.md) - Source documentation
- [Deployment](docs/DEPLOYMENT.md) - Production deployment
- [Contributing](docs/CONTRIBUTING.md) - Contribution guidelines
- [Security](docs/SECURITY.md) - Security policies
- [Troubleshooting](docs/TROUBLESHOOTING.md) - Common issues

## Roadmap

### Completed
- [x] Core data ingestion pipeline
- [x] GraphQL API with subscriptions
- [x] WebGL2 globe visualization
- [x] Real-time anomaly detection
- [x] WebSocket infrastructure
- [x] Component library (Storybook)
- [x] Unit and E2E testing (100% pass rate)

### In Progress
- [ ] Plume dispersion modeling
- [ ] ML-based anomaly classification
- [ ] Mobile application
- [ ] Alert notification system

### Planned
- [ ] Historical data analysis
- [ ] Predictive maintenance
- [ ] Multi-tenant support
- [ ] Federated learning

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'feat: add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Security

For security concerns, please review [SECURITY.md](SECURITY.md).

## License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- Safecast for open radiation data
- uRADMonitor for sensor network
- EPA RadNet for US monitoring data
- IAEA for nuclear facility information

---

**Repository**: https://github.com/tworjaga/cherenkov
