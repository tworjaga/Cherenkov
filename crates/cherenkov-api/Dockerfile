# Build stage
FROM rust:1.75-slim-bookworm as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/cherenkov-db/Cargo.toml ./crates/cherenkov-db/
COPY crates/cherenkov-api/Cargo.toml ./crates/cherenkov-api/
COPY crates/cherenkov-observability/Cargo.toml ./crates/cherenkov-observability/
COPY crates/cherenkov-ml/Cargo.toml ./crates/cherenkov-ml/
COPY crates/cherenkov-plume/Cargo.toml ./crates/cherenkov-plume/

# Create dummy main files for dependency caching
RUN mkdir -p crates/cherenkov-db/src crates/cherenkov-api/src crates/cherenkov-observability/src crates/cherenkov-ml/src crates/cherenkov-plume/src && \
    echo "fn main() {}" > crates/cherenkov-db/src/lib.rs && \
    echo "fn main() {}" > crates/cherenkov-api/src/main.rs && \
    echo "fn main() {}" > crates/cherenkov-observability/src/lib.rs && \
    echo "fn main() {}" > crates/cherenkov-ml/src/lib.rs && \
    echo "fn main() {}" > crates/cherenkov-plume/src/lib.rs

# Build dependencies
RUN cargo build --release -p cherenkov-api 2>/dev/null || true

# Copy actual source code
COPY crates/cherenkov-db/src ./crates/cherenkov-db/src
COPY crates/cherenkov-api/src ./crates/cherenkov-api/src
COPY crates/cherenkov-observability/src ./crates/cherenkov-observability/src
COPY crates/cherenkov-ml/src ./crates/cherenkov-ml/src
COPY crates/cherenkov-plume/src ./crates/cherenkov-plume/src
COPY crates/cherenkov-db/migrations ./crates/cherenkov-db/migrations

# Build the application
RUN cargo build --release -p cherenkov-api

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/cherenkov-api /usr/local/bin/

# Create data directory
RUN mkdir -p /data

# Expose API port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the application
CMD ["cherenkov-api"]
