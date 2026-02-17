# Build stage
FROM rust:1.75-slim-bookworm AS builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY crates/cherenkov-core/Cargo.toml ./crates/cherenkov-core/
COPY crates/cherenkov-db/Cargo.toml ./crates/cherenkov-db/
COPY crates/cherenkov-api/Cargo.toml ./crates/cherenkov-api/
COPY crates/cherenkov-ml/Cargo.toml ./crates/cherenkov-ml/
COPY crates/cherenkov-plume/Cargo.toml ./crates/cherenkov-plume/
COPY crates/cherenkov-stream/Cargo.toml ./crates/cherenkov-stream/
COPY crates/cherenkov-ingest/Cargo.toml ./crates/cherenkov-ingest/
COPY crates/cherenkov-observability/Cargo.toml ./crates/cherenkov-observability/

# Copy source code
COPY crates/cherenkov-core/src ./crates/cherenkov-core/src
COPY crates/cherenkov-db/src ./crates/cherenkov-db/src
COPY crates/cherenkov-api/src ./crates/cherenkov-api/src
COPY crates/cherenkov-ml/src ./crates/cherenkov-ml/src
COPY crates/cherenkov-plume/src ./crates/cherenkov-plume/src
COPY crates/cherenkov-stream/src ./crates/cherenkov-stream/src
COPY crates/cherenkov-ingest/src ./crates/cherenkov-ingest/src
COPY crates/cherenkov-observability/src ./crates/cherenkov-observability/src

# Build release binary
RUN cargo build --release --bin cherenkov-api

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/cherenkov-api /app/cherenkov-api

# Expose ports
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the binary
CMD ["./cherenkov-api"]
