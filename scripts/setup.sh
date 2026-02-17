#!/bin/bash

# Cherenkov Development Environment Setup Script

set -e

echo "Setting up Cherenkov development environment..."

# Check prerequisites
command -v rustc >/dev/null 2>&1 || { echo "Rust not installed. Please install Rust: https://rustup.rs/"; exit 1; }
command -v node >/dev/null 2>&1 || { echo "Node.js not installed. Please install Node.js v20+"; exit 1; }
command -v docker >/dev/null 2>&1 || { echo "Docker not installed. Please install Docker"; exit 1; }

echo "Prerequisites check passed."

# Install Rust components
echo "Installing Rust components..."
rustup component add rustfmt clippy

# Setup web dependencies
echo "Installing web dependencies..."
cd web
npm install
cd ..

# Create data directory
echo "Creating data directory..."
mkdir -p data

# Copy environment file
if [ ! -f .env ]; then
    echo "Creating .env file from template..."
    cp .env.example .env
    echo "Please edit .env file with your configuration."
fi

echo "Setup complete!"
echo ""
echo "Next steps:"
echo "1. Edit .env file with your configuration"
echo "2. Run 'make dev' to start development environment"
echo "3. Or run 'make docker-up' to start with Docker"
