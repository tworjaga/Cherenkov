# Cherenkov Architecture

## Overview

Cherenkov is a real-time global radiological intelligence platform built on a cell-based distributed architecture. The system processes 10M+ events per second from 50,000+ sensors worldwide.

## Core Components

### Data Plane (Hot Path)

| Component | Technology | Throughput | Latency |
|-----------|-----------|------------|---------|
| Ingest Daemon | Rust (tokio) | 10M evt/s | <1ms |
| Stream Processor | Rust (timely-dataflow) | 10M evt/s | <5ms |
| Hot Storage | ScyllaDB | 10M writes/s | <2ms |
| Hot Read | ScyllaDB | 1M QPS | <1ms |

### Processing Plane

| Component | Technology | Purpose |
|-----------|-----------|---------|
| ML Inference | Candle (ONNX) | Isotope classification |
| Plume Engine | CUDA/Rust | Atmospheric dispersion |
| Correlator | timely-dataflow | Cross-sensor fusion |

### Serving Plane

| Component | Protocol | Scale |
|-----------|----------|-------|
| GraphQL API | HTTP/WebSocket | 1M concurrent |
| REST API | HTTP | Legacy integrations |
| gRPC | HTTP/2 | Internal mesh |

## Data Flow

```
┌─────────┐    ┌──────────┐    ┌─────────┐    ┌────────┐
│ Sensors │───▶│  Ingest  │───▶│  Stream │───▶│  Hot   │
│(50k+)   │    │ (tokio)  │    │(timely) │    │Storage │
└─────────┘    └──────────┘    └────┬────┘    └────────┘
                                    │
                              ┌─────┴─────┐
                              ▼           ▼
                         ┌────────┐   ┌────────┐
                         │Anomaly │   │  ML    │
                         │Detect  │   │Inference
                         └────────┘   └────────┘
```

## Storage Architecture

### Time-Series (ScyllaDB)

- Hot tier: NVMe, 7 days
- Warm tier: SSD, 90 days
- Cold tier: S3, 2 years
- Archive: Glacier, 10 years

### Schema

```rust
struct RadiationReading {
    sensor_id: Uuid,      // Partition key
    bucket: i64,          // Hour bucket
    timestamp: i64,       // Clustering key (nanos)
    location: GeoPoint,
    dose_rate: f64,
    quality: QualityFlag,
}
```

## Cell-Based Distribution

```
┌─────────────────────────────────────────┐
│           Global Anycast                │
│    (Cloudflare Magic Transit)           │
└─────────────────┬───────────────────────┘
                  │
    ┌─────────────┼─────────────┐
    ▼             ▼             ▼
┌────────┐   ┌────────┐   ┌────────┐
│us-east │   │eu-west │   │ap-north│
│   -1   │   │   -1   │   │  east  │
└────────┘   └────────┘   └────────┘
```

Each cell contains:
- Ingest workers (100 pods)
- Stream processors (50 pods)
- Query services (200 pods)
- Hot storage (ScyllaDB cluster)

## Observability

- Metrics: Prometheus + Grafana
- Tracing: Jaeger (OpenTelemetry)
- Logging: Structured JSON (Loki)

## Security

- Zero Trust architecture
- JWT-based authentication
- ABAC (Attribute-Based Access Control)
- mTLS for service mesh
