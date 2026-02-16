# Cherenkov Deployment Guide

## Prerequisites

- Kubernetes 1.28+
- kubectl configured with cluster access
- Flux CD 2.0+ installed on cluster
- Container registry access (GHCR or private)
- TLS certificates (cert-manager recommended)

## Quick Start

### 1. Local Development (Docker Compose)

```bash
git clone https://github.com/tworjaga/cherenkov.git
cd cherenkov

# Start dependencies
docker-compose up -d scylla redis

# Run services
cargo run -p cherenkov-ingest
cargo run -p cherenkov-api

# Web
cd web && npm install && npm run dev
```

### 2. Kubernetes Deployment

#### Base Infrastructure

```bash
# Create namespace and base resources
kubectl apply -k k8s/base/

# Verify deployments
kubectl get pods -n cherenkov
```

#### Production Overlay

```bash
# Apply production configuration with scaled replicas
kubectl apply -k k8s/overlays/production/

# Check rollout status
kubectl rollout status deployment/cherenkov-api -n cherenkov
kubectl rollout status deployment/cherenkov-ingest -n cherenkov
```

### 3. Flux GitOps Setup

```bash
# Install Flux on cluster
flux install

# Create GitRepository source
flux create source git cherenkov \
  --url=https://github.com/tworjaga/cherenkov \
  --branch=main \
  --interval=1m

# Create Kustomization for automated deployment
flux create kustomization cherenkov \
  --source=cherenkov \
  --path="./k8s/overlays/production" \
  --prune=true \
  --interval=5m
```

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | info | Log level (error, warn, info, debug, trace) |
| `SCYLLA_HOSTS` | scylla:9042 | ScyllaDB cluster addresses |
| `SCYLLA_KEYSPACE` | cherenkov | Database keyspace |
| `REDIS_URL` | redis:6379 | Redis connection string |
| `JAEGER_ENDPOINT` | http://jaeger:14268 | Tracing collector |
| `API_PORT` | 8080 | GraphQL API port |
| `WS_PORT` | 8081 | WebSocket port |
| `METRICS_PORT` | 9090 | Prometheus metrics port |

### Secrets

Create required secrets before deployment:

```bash
# ScyllaDB credentials
kubectl create secret generic cherenkov-secrets \
  -n cherenkov \
  --from-literal=scylla-hosts="scylla-1:9042,scylla-2:9042" \
  --from-literal=redis-url="redis://redis:6379" \
  --from-literal=jwt-secret="$(openssl rand -base64 32)"

# Grafana admin password
kubectl create secret generic grafana-secrets \
  -n monitoring \
  --from-literal=admin-password="$(openssl rand -base64 16)"
```

## Monitoring Stack

### Prometheus

```bash
kubectl apply -f k8s/monitoring/prometheus.yaml

# Port-forward to access UI
kubectl port-forward -n monitoring svc/prometheus 9090:9090
```

### Grafana

```bash
kubectl apply -f k8s/monitoring/grafana.yaml

# Access at http://localhost:3000 (admin / secret)
kubectl port-forward -n monitoring svc/grafana 3000:3000
```

### Jaeger Tracing

```bash
kubectl apply -f k8s/monitoring/jaeger.yaml

# Access UI
kubectl port-forward -n observability svc/jaeger-query 16686:16686
```

## Multi-Region Deployment

### Cell-Based Architecture

Deploy cells in multiple regions for high availability:

```yaml
# k8s/overlays/us-east/kustomization.yaml
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization

namespace: cherenkov-us-east

resources:
- ../../base

configMapGenerator:
- name: cherenkov-config
  behavior: merge
  literals:
  - CELL_ID=us-east-1
  - SCYLLA_KEYSPACE=cherenkov_us_east
```

### Traffic Routing

Use Cloudflare or similar for anycast routing:

```yaml
# ingress with region affinity
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  annotations:
    nginx.ingress.kubernetes.io/affinity: "cookie"
    nginx.ingress.kubernetes.io/session-cookie-name: "cherenkov-region"
```

## Scaling

### Horizontal Pod Autoscaler

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: cherenkov-api
  namespace: cherenkov
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: cherenkov-api
  minReplicas: 3
  maxReplicas: 50
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Pods
    pods:
      metric:
        name: http_requests_per_second
      target:
        type: AverageValue
        averageValue: "1000"
```

## Troubleshooting

### Check Pod Status

```bash
kubectl get pods -n cherenkov -o wide
kubectl describe pod <pod-name> -n cherenkov
kubectl logs <pod-name> -n cherenkov --tail=100
```

### Database Connectivity

```bash
# Test ScyllaDB connection
kubectl run -it --rm cqlsh --image=scylladb/scylla-cqlsh \
  --restart=Never -- scylla-1:9042 -k cherenkov

# Check Redis
kubectl run -it --rm redis-cli --image=redis:alpine \
  --restart=Never -- redis-cli -h redis ping
```

### Performance Tuning

| Issue | Solution |
|-------|----------|
| High latency | Increase ScyllaDB nodes, add Redis cache |
| Memory pressure | Reduce batch sizes, increase pod limits |
| CPU throttling | Add HPA, optimize queries |
| Connection errors | Check network policies, increase pool size |

## Security

### Network Policies

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: cherenkov-api
  namespace: cherenkov
spec:
  podSelector:
    matchLabels:
      app: cherenkov-api
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: ingress-nginx
    ports:
    - protocol: TCP
      port: 8080
```

### Pod Security Standards

All manifests enforce restricted PSS:

```yaml
securityContext:
  runAsNonRoot: true
  runAsUser: 1000
  seccompProfile:
    type: RuntimeDefault
  capabilities:
    drop: ["ALL"]
  readOnlyRootFilesystem: true
  allowPrivilegeEscalation: false
```

## Backup and Disaster Recovery

### ScyllaDB Backup

```bash
# Create snapshot
kubectl exec -it scylla-0 -n cherenkov -- nodetool snapshot cherenkov

# Upload to S3
aws s3 sync /var/lib/scylla/data/cherenkov/ s3://cherenkov-backups/$(date +%Y%m%d)/
```

### Point-in-Time Recovery

Configure ScyllaDB incremental backups and restore from specific timestamps using the `restore_from` procedure.

## Support

- Issues: https://github.com/tworjaga/cherenkov/issues
- Telegram: @al7exy
