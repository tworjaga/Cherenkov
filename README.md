# Cherenkov

> Real-time global radiological intelligence platform

![Build](https://github.com/tworjaga/cherenkov/workflows/ci/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)


The blue glow of nuclear reactors, made visible.

## What

Cherenkov aggregates 50,000+ radiation sensors worldwide, detects anomalies 
in milliseconds, predicts fallout dispersion, and provides situational 
awareness for nuclear events — from medical isotope spills to reactor 
meltdowns.

## Architecture

- **Ingestion:** Rust (tokio) — 10M events/sec
- **Stream Processing:** Rust (timely-dataflow) — sub-10ms anomaly detection
- **Storage:** ScyllaDB time-series — petabyte scale
- **ML:** Rust (candle) — GPU-native inference, no Python
- **API:** GraphQL + WebSocket — 1M concurrent subscribers
- **Globe:** WebGL2 + WASM (wgpu) — 100k points, 60fps

## Quick Start

```bash
git clone https://github.com/tworjaga/cherenkov.git
cd cherenkov

# Local development (Docker)
docker-compose up -d scylla redis
cargo run -p cherenkov-ingest
cargo run -p cherenkov-api

# Web
cd web && npm install && npm run dev
```

## Data Sources

- Safecast (5,000+ sensors)
- uRADMonitor (10,000+ sensors)
- EPA RadNet (USA)
- EURDEP (EU)
- IAEA PRIS (440 power plants)
- USGS Seismic
- NASA FIRMS (satellite thermal)
- NOAA GFS (weather)

See DATA_SOURCES.md for complete list.

## License

MIT — See [LICENSE](LICENSE) for details.
