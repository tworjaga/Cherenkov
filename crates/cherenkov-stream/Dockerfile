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
COPY crates/cherenkov-stream/Cargo.toml ./crates/cherenkov-stream/
COPY crates/cherenkov-observability/Cargo.toml ./crates/cherenkov-observability/

# Create dummy main files for dependency caching
RUN mkdir -p crates/cherenkov-db/src crates/cherenkov-stream/src crates/cherenkov-observability/src && \
    echo "fn main() {}" > crates/cherenkov-db/src/lib.rs && \
    echo "fn main() {}" > crates/cherenkov-stream/src/main.rs && \
    echo "fn main() {}" > crates/cherenkov-observability/src/lib.rs

# Build dependencies
RUN cargo build --release -p cherenkov-stream 2>/dev/null || true

# Copy actual source code
COPY crates/cherenkov-db/src ./crates/cherenkov-db/src
COPY crates/cherenkov-stream/src ./crates/cherenkov-stream/src
COPY crates/cherenkov-observability/src ./crates/cherenkov-observability/src
COPY crates/cherenkov-db/migrations ./crates/cherenkov-db/migrations

# Build the application
RUN cargo build --release -p cherenkov-stream

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/cherenkov-stream /usr/local/bin/

# Run the application
CMD ["cherenkov-stream"]
