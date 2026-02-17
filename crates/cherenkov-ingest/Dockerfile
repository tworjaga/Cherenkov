FROM rust:1.75-slim-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY crates/cherenkov-core/Cargo.toml ./crates/cherenkov-core/
COPY crates/cherenkov-db/Cargo.toml ./crates/cherenkov-db/
COPY crates/cherenkov-ingest/Cargo.toml ./crates/cherenkov-ingest/
COPY crates/cherenkov-ml/Cargo.toml ./crates/cherenkov-ml/
COPY crates/cherenkov-plume/Cargo.toml ./crates/cherenkov-plume/
COPY crates/cherenkov-observability/Cargo.toml ./crates/cherenkov-observability/

COPY crates/cherenkov-core/src ./crates/cherenkov-core/src
COPY crates/cherenkov-db/src ./crates/cherenkov-db/src
COPY crates/cherenkov-ingest/src ./crates/cherenkov-ingest/src
COPY crates/cherenkov-ml/src ./crates/cherenkov-ml/src
COPY crates/cherenkov-plume/src ./crates/cherenkov-plume/src
COPY crates/cherenkov-observability/src ./crates/cherenkov-observability/src

RUN cargo build --release --bin cherenkov-ingest

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/cherenkov-ingest /app/cherenkov-ingest

CMD ["./cherenkov-ingest"]
