# Contributing to Cherenkov

Thank you for your interest in contributing to Cherenkov. This document provides guidelines for contributing to the project.

## Code of Conduct

- Be respectful and constructive in all interactions
- Focus on technical merit and project goals
- Follow security best practices for radiation monitoring systems

## Development Setup

### Prerequisites

- Rust 1.75+
- Node.js 20+
- Docker and Docker Compose
- Kubernetes cluster (for testing deployments)

### Local Development

```bash
# Clone repository
git clone https://github.com/tworjaga/cherenkov.git
cd cherenkov

# Install Rust dependencies
cargo build

# Install web dependencies
cd web && npm install

# Start local services
docker-compose up -d scylla redis

# Run tests
cargo test
cd web && npm test
```

## Project Structure

```
cherenkov/
├── crates/           # Rust workspace crates
│   ├── cherenkov-ingest/      # Data ingestion
│   ├── cherenkov-stream/      # Stream processing
│   ├── cherenkov-db/          # Database layer
│   ├── cherenkov-ml/          # ML inference
│   ├── cherenkov-plume/       # Plume modeling
│   ├── cherenkov-api/         # GraphQL API
│   └── cherenkov-observability/ # Metrics/tracing
├── web/              # SolidJS frontend
│   ├── src/          # TypeScript source
│   └── wasm/         # Rust WASM module
├── k8s/              # Kubernetes manifests
└── docs/             # Documentation
```

## Workflow

### 1. Create an Issue

Before starting work, create an issue describing:
- The problem or feature
- Expected behavior
- Proposed solution (if applicable)

### 2. Fork and Branch

```bash
# Fork on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/cherenkov.git

# Create feature branch
git checkout -b feature/your-feature-name
```

### 3. Commit Guidelines

Use [Conventional Commits](https://conventionalcommits.org/):

```
<type>: <description>

[optional body]

[optional footer]
```

**Types:**
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `style:` Code style (formatting, no logic change)
- `refactor:` Code refactoring
- `perf:` Performance improvements
- `test:` Test additions/changes
- `chore:` Build/tooling changes

**Examples:**
```
feat: add EPA RadNet data source crawler

- Implement HTTP client with rate limiting
- Add XML parsing for radiation readings
- Include unit tests for parser

fix: resolve WebSocket reconnection race condition

docs: update API schema for plume simulation
```

### 4. Code Standards

#### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Run `cargo clippy` and resolve warnings
- Maintain test coverage above 80%

```bash
cargo fmt
cargo clippy --all-targets --all-features
cargo test
```

#### TypeScript

- Follow project ESLint configuration
- Use strict TypeScript settings
- Add types for all function parameters and returns

```bash
cd web && npm run lint
cd web && npx tsc --noEmit
```

### 5. Testing

#### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anomaly_detection() {
        let detector = AnomalyDetector::new();
        let result = detector.detect(2.5, 0.1);
        assert!(result.is_anomaly);
    }
}
```

#### Integration Tests

```bash
# Run all tests
cargo test --all

# Run specific crate tests
cargo test -p cherenkov-stream

# Web tests
cd web && npm test
```

### 6. Documentation

Update relevant documentation:
- `docs/API.md` for API changes
- `docs/ARCHITECTURE.md` for structural changes
- `docs/DEPLOYMENT.md` for deployment changes
- Inline code documentation with rustdoc/jsdoc

### 7. Pull Request

1. Push branch to your fork
2. Create PR against `main` branch
3. Fill out PR template:
   - Description of changes
   - Related issue numbers
   - Testing performed
   - Screenshots (for UI changes)

4. Ensure CI passes:
   - Rust tests and clippy
   - TypeScript compilation
   - Build verification

## Review Process

- All PRs require at least one review
- Address review comments promptly
- Maintain constructive dialogue
- Squash commits before merge if requested

## Performance Considerations

Cherenkov processes 10M+ events/second. Contributions must consider:

- Memory allocation patterns (minimize allocations in hot paths)
- Lock-free data structures where possible
- Batch processing for database writes
- Efficient serialization (bincode over JSON for internal)

## Security

Radiation monitoring is safety-critical. All contributions must:

- Follow OWASP guidelines
- Never log sensitive data (API keys, PII)
- Validate all inputs
- Use parameterized queries
- Implement proper error handling (no panics in production)

## Areas for Contribution

### High Priority

- Additional data source integrations (IAEA, EURDEP)
- ML model improvements (isotope classification)
- Performance optimizations
- Documentation improvements

### Good First Issues

- Data source crawlers for new regions
- Frontend component tests
- Documentation translations
- Example code for SDKs

## Questions?

- Telegram: @al7exy
- GitHub Discussions: https://github.com/tworjaga/cherenkov/discussions

## License

By contributing, you agree that your contributions will be licensed under the AGPL-3.0 License.
