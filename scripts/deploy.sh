#!/bin/bash

# Cherenkov Deployment Script

set -e

ENVIRONMENT=${1:-production}
NAMESPACE=${2:-cherenkov}

echo "Deploying Cherenkov to $ENVIRONMENT environment..."

# Build Docker images
echo "Building Docker images..."
docker-compose build

# Push to registry (if configured)
if [ -n "$DOCKER_REGISTRY" ]; then
    echo "Pushing images to registry..."
    docker-compose push
fi

# Apply Kubernetes manifests
echo "Applying Kubernetes manifests..."
kubectl apply -k k8s/overlays/$ENVIRONMENT/

# Wait for deployments
echo "Waiting for deployments to be ready..."
kubectl wait --for=condition=available --timeout=300s deployment/cherenkov-api -n $NAMESPACE
kubectl wait --for=condition=available --timeout=300s deployment/cherenkov-ingest -n $NAMESPACE
kubectl wait --for=condition=available --timeout=300s deployment/cherenkov-stream -n $NAMESPACE
kubectl wait --for=condition=available --timeout=300s deployment/cherenkov-web -n $NAMESPACE

echo "Deployment complete!"
echo ""
echo "Services:"
kubectl get services -n $NAMESPACE
echo ""
echo "Pods:"
kubectl get pods -n $NAMESPACE
