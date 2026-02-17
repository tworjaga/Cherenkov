.PHONY: all build test clean dev docker-up docker-down lint fmt

# Default target
all: build

# Build all Rust binaries
build:
	cargo build --release

# Build debug version
build-debug:
	cargo build

# Run all tests
test:
	cargo test --all-features
	cd web && npm test

# Run E2E tests
test-e2e:
	cd web && npx playwright test

# Clean build artifacts
clean:
	cargo clean
	cd web && rm -rf .next dist node_modules

# Run development servers
dev:
	@echo "Starting development environment..."
	@make docker-up &

# Run API server locally
dev-api:
	cargo run --bin cherenkov-api

# Run ingest service locally
dev-ingest:
	cargo run --bin cherenkov-ingest

# Run stream processor locally
dev-stream:
	cargo run --bin cherenkov-stream

# Run web frontend locally
dev-web:
	cd web && npm run dev

# Docker operations
docker-up:
	docker-compose up -d

docker-down:
	docker-compose down

docker-build:
	docker-compose build

# Linting and formatting
lint:
	cargo clippy --all-features -- -D warnings
	cd web && npm run lint

fmt:
	cargo fmt
	cd web && npx prettier --write .

fmt-check:
	cargo fmt -- --check
	cd web && npx prettier --check .

# Generate code (GraphQL types)
codegen:
	cd web && npm run codegen

# Install dependencies
install:
	cd web && npm ci

# Build web for production
build-web:
	cd web && npm run build

# Security audit
audit:
	cargo audit
	cd web && npm audit

# Update dependencies
update:
	cargo update
	cd web && npm update

# Help
help:
	@echo "Available targets:"
	@echo "  build        - Build all Rust binaries (release)"
	@echo "  build-debug  - Build all Rust binaries (debug)"
	@echo "  test         - Run all tests"
	@echo "  test-e2e     - Run E2E tests with Playwright"
	@echo "  clean        - Clean build artifacts"
	@echo "  dev          - Start development environment"
	@echo "  dev-api      - Run API server locally"
	@echo "  dev-ingest   - Run ingest service locally"
	@echo "  dev-stream   - Run stream processor locally"
	@echo "  dev-web      - Run web frontend locally"
	@echo "  docker-up    - Start Docker services"
	@echo "  docker-down  - Stop Docker services"
	@echo "  docker-build - Build Docker images"
	@echo "  lint         - Run linters"
	@echo "  fmt          - Format code"
	@echo "  fmt-check    - Check code formatting"
	@echo "  codegen      - Generate GraphQL types"
	@echo "  install      - Install dependencies"
	@echo "  build-web    - Build web for production"
	@echo "  audit        - Security audit"
	@echo "  update       - Update dependencies"
