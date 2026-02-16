# Cherenkov Security Documentation

## Overview

Cherenkov is a safety-critical radiation monitoring platform. Security is paramount to ensure data integrity, system availability, and protection against malicious actors who could disrupt public safety operations.

## Threat Model

### Assets

| Asset | Value | Risk Level |
|-------|-------|------------|
| Sensor data integrity | Critical | High |
| Alert system availability | Critical | Critical |
| User credentials | High | High |
| ML models | Medium | Medium |
| Source code | Medium | Low |

### Threat Actors

- **Nation-state actors:** Targeting infrastructure disruption
- **Hacktivists:** Defacement or data manipulation
- **Criminals:** Ransomware or data theft
- **Insiders:** Unauthorized data access

### Attack Vectors

1. **Data injection:** False sensor readings
2. **DoS:** Overwhelming API or WebSocket endpoints
3. **Supply chain:** Compromised dependencies
4. **Credential theft:** API key or JWT compromise
5. **Infrastructure:** Kubernetes cluster compromise

## Security Architecture

### Defense in Depth

```
┌─────────────────────────────────────────┐
│  Edge: Cloudflare (DDoS, WAF, Bot Mgmt)  │
├─────────────────────────────────────────┤
│  Ingress: TLS 1.3, mTLS, Rate Limiting  │
├─────────────────────────────────────────┤
│  Auth: JWT + ABAC (OPA/Rego)            │
├─────────────────────────────────────────┤
│  API: Input validation, Parameterized   │
│       queries, Request signing          │
├─────────────────────────────────────────┤
│  Services: Network policies, Seccomp,   │
│            Non-root containers          │
├─────────────────────────────────────────┤
│  Data: Encryption at rest (AES-256),    │
│        TLS in transit, Field-level      │
│        encryption for PII               │
└─────────────────────────────────────────┘
```

## Authentication

### JWT Implementation

```rust
// crates/cherenkov-api/src/auth.rs
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

pub struct Claims {
    pub sub: String,          // User ID
    pub roles: Vec<String>,   // Role assignments
    pub iat: i64,             // Issued at
    pub exp: i64,             // Expiration
    pub jti: String,          // JWT ID for revocation
}

pub fn verify_token(token: &str, jwks: &JwksClient) -> Result<Claims, AuthError> {
    let header = decode_header(token)?;
    let key = jwks.get_key(&header.kid.ok_or(AuthError::MissingKeyId)?).await?;
    
    let validation = Validation::new(Algorithm::ES256);
    let token_data = decode::<Claims>(token, &DecodingKey::from_ec_pem(&key)?, &validation)?;
    
    // Check revocation list
    if is_revoked(&token_data.claims.jti).await? {
        return Err(AuthError::Revoked);
    }
    
    Ok(token_data.claims)
}
```

### Token Security

- **Algorithm:** ES256 (ECDSA with P-256)
- **Key rotation:** Every 30 days
- **Expiration:** 1 hour access tokens, 7 day refresh tokens
- **Revocation:** Redis-backed revocation list with 24h TTL

## Authorization

### Attribute-Based Access Control (ABAC)

```rust
// Policy evaluation
pub async fn authorize(
    &self,
    subject: &Claims,
    resource: &Resource,
    action: Action,
    context: &RequestContext,
) -> Result<bool, PolicyError> {
    // Load policy from OPA
    let policy = self.opa.get_policy("cherenkov/main").await?;
    
    let input = json!({
        "subject": {
            "id": subject.sub,
            "roles": subject.roles,
            "mfa_verified": context.mfa_verified,
        },
        "resource": {
            "type": resource.type_name(),
            "id": resource.id(),
            "owner": resource.owner_id(),
            "classification": resource.classification(),
        },
        "action": action.to_string(),
        "context": {
            "time": context.timestamp,
            "ip": context.client_ip,
            "user_agent": context.user_agent,
        }
    });
    
    policy.evaluate(input).await
}
```

### Default Policies

| Role | Permissions |
|------|-------------|
| `admin` | Full system access |
| `operator` | Read all, acknowledge alerts, run simulations |
| `analyst` | Read sensors, readings, anomalies |
| `viewer` | Read public sensor data only |

## Data Protection

### Encryption

**At Rest:**
- ScyllaDB: Transparent data encryption (TDE)
- S3 backups: AES-256-GCM
- Secrets: Sealed Secrets + Vault

**In Transit:**
- TLS 1.3 mandatory
- Certificate pinning for mobile apps
- mTLS for service-to-service communication

### Field-Level Encryption

PII fields encrypted with per-user keys:

```rust
#[derive(Debug)]
struct EncryptedField {
    ciphertext: Vec<u8>,
    nonce: [u8; 12],
    key_id: String,  // Reference to KMS
}

impl EncryptedField {
    pub fn encrypt(plaintext: &str, key: &DataKey) -> Result<Self, CryptoError> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let cipher = Aes256Gcm::new(key);
        let ciphertext = cipher.encrypt(&nonce, plaintext.as_bytes())?;
        
        Ok(Self {
            ciphertext,
            nonce: nonce.into(),
            key_id: key.id.clone(),
        })
    }
}
```

## API Security

### Rate Limiting

```rust
// Tiered rate limits per endpoint
pub struct RateLimiter {
    // Anonymous: 10 req/min
    anonymous: Arc<RateLimit>,
    // Authenticated: 1000 req/min
    authenticated: Arc<RateLimit>,
    // Premium: 10000 req/min
    premium: Arc<RateLimit>,
}

impl RateLimiter {
    pub async fn check(&self, key: &str, tier: Tier) -> Result<(), RateLimitError> {
        let limiter = match tier {
            Tier::Anonymous => &self.anonymous,
            Tier::Authenticated => &self.authenticated,
            Tier::Premium => &self.premium,
        };
        
        if !limiter.check_key(key).await? {
            return Err(RateLimitError::Exceeded);
        }
        Ok(())
    }
}
```

### Input Validation

```rust
pub struct ValidatedInput<T> {
    inner: T,
    _marker: PhantomData<()>,
}

impl ValidatedInput<SensorQuery> {
    pub fn new(input: SensorQuery) -> Result<Self, ValidationError> {
        // Validate sensor IDs are valid UUIDs
        for id in &input.sensor_ids {
            Uuid::parse_str(id).map_err(|_| ValidationError::InvalidUuid)?;
        }
        
        // Validate time range (max 30 days)
        let duration = input.to - input.from;
        if duration > Duration::days(30) {
            return Err(ValidationError::TimeRangeTooLarge);
        }
        
        // Validate coordinate bounds
        if let Some(ref region) = input.region {
            if region.lat < -90.0 || region.lat > 90.0 {
                return Err(ValidationError::InvalidLatitude);
            }
        }
        
        Ok(Self {
            inner: input,
            _marker: PhantomData,
        })
    }
}
```

## Infrastructure Security

### Kubernetes Hardening

**Pod Security Standards (Restricted):**

```yaml
apiVersion: v1
kind: Pod
spec:
  securityContext:
    runAsNonRoot: true
    runAsUser: 1000
    seccompProfile:
      type: RuntimeDefault
  containers:
  - name: cherenkov-api
    securityContext:
      allowPrivilegeEscalation: false
      readOnlyRootFilesystem: true
      capabilities:
        drop: ["ALL"]
    resources:
      limits:
        memory: "512Mi"
        cpu: "1000m"
```

**Network Policies:**

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: api-isolation
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
  egress:
  - to:
    - podSelector:
        matchLabels:
          app: scylla
    ports:
    - protocol: TCP
      port: 9042
```

### Secrets Management

```bash
# Sealed Secrets for GitOps
kubeseal --format=yaml < secret.yaml > sealed-secret.yaml

# Vault integration for dynamic secrets
vault write cherenkov/database/creds/readonly \
    ttl=1h
```

## Audit Logging

All security-relevant events logged with structured format:

```json
{
  "timestamp": "2024-01-15T09:23:47.123Z",
  "event_type": "authentication_failure",
  "severity": "warning",
  "actor": {
    "ip": "192.0.2.1",
    "user_agent": "Mozilla/5.0...",
    "user_id": null
  },
  "resource": {
    "type": "api_endpoint",
    "path": "/graphql",
    "method": "POST"
  },
  "details": {
    "reason": "invalid_token",
    "token_fingerprint": "a1b2c3d4..."
  },
  "trace_id": "4bf92f3577b34da6a3ce929d0e0e4736"
}
```

**Logged Events:**
- Authentication attempts (success/failure)
- Authorization denials
- Data access (sensitive resources)
- Configuration changes
- Alert acknowledgments

## Incident Response

### Severity Levels

| Level | Criteria | Response Time |
|-------|----------|---------------|
| P0 | Data integrity compromise, alert system down | 15 minutes |
| P1 | Unauthorized data access, API abuse | 1 hour |
| P2 | Suspicious activity, minor policy violations | 4 hours |
| P3 | Security scan findings, compliance issues | 24 hours |

### Response Playbook

1. **Detection:** Automated alerts from SIEM, monitoring, or manual report
2. **Containment:** Isolate affected systems, revoke compromised credentials
3. **Investigation:** Preserve logs, identify scope and root cause
4. **Recovery:** Restore from verified backups, rotate all secrets
5. **Post-Incident:** Update runbooks, implement preventive measures

## Compliance

### Standards

- **SOC 2 Type II:** Security, availability, confidentiality
- **ISO 27001:** Information security management
- **GDPR:** Data protection for EU users
- **NIST 800-53:** Security controls for federal systems

### Data Retention

| Data Type | Retention | Encryption |
|-----------|-----------|------------|
| Sensor readings | 7 years (hot: 7d, warm: 90d, cold: 2y, archive: 5y) | AES-256 |
| User data | Account lifetime + 1 year | AES-256 |
| Audit logs | 10 years | AES-256 |
| Session tokens | 7 days (refresh), 1 hour (access) | N/A |

## Vulnerability Disclosure

**Contact:** security@cherenkov.io  
**PGP Key:** [0xA1B2C3D4...](https://cherenkov.io/security/pgp.asc)

**Scope:**
- *.cherenkov.io
- GitHub repository: tworjaga/cherenkov
- Mobile applications

**Safe Harbor:** We support responsible disclosure and will not pursue legal action against researchers who:
- Report vulnerabilities promptly
- Do not access or modify data beyond what is necessary
- Do not degrade service availability
- Keep findings confidential until fixed

## Security Checklist

### For Developers

- [ ] No secrets in code (use environment variables)
- [ ] Input validation on all endpoints
- [ ] Parameterized queries (no SQL injection)
- [ ] Proper error handling (no information leakage)
- [ ] Rate limiting implemented
- [ ] Audit logging for sensitive operations
- [ ] Dependencies updated (cargo audit, npm audit)

### For Operators

- [ ] TLS certificates valid and auto-renewing
- [ ] Secrets rotated every 90 days
- [ ] Backups encrypted and tested
- [ ] Network policies active
- [ ] Monitoring alerts configured
- [ ] Incident response plan tested quarterly

## Contact

- **Security Team:** security@cherenkov.io
- **Telegram:** @al7exy (encrypted)
- **Bug Bounty:** https://cherenkov.io/security/bounty
