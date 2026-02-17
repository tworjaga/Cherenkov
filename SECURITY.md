# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| 0.9.x   | :x:                |
| < 0.9   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability within Cherenkov, please send an email to security@cherenkov.io. All security vulnerabilities will be promptly addressed.

Please do not open public issues for security vulnerabilities.

## Security Measures

Cherenkov implements the following security measures:

- All data is encrypted in transit using TLS 1.3
- Database connections use encrypted channels
- API authentication via JWT tokens with short expiration
- Rate limiting on all endpoints
- Input validation and sanitization
- Regular dependency updates

## Security Best Practices

When deploying Cherenkov:

1. Use strong, unique passwords for all services
2. Enable 2FA for all administrative accounts
3. Keep all dependencies up to date
4. Monitor logs for suspicious activity
5. Use network segmentation for microservices
6. Regularly backup data and test restoration
