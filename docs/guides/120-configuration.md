# 120 - Configuration

This document covers configuration management for all layers of the application.

## Environment Variables

### API (.env)

```bash
# Server
PORT=3000
HOST=0.0.0.0
ENVIRONMENT=development

# Database
DATABASE_URL=postgres://user:pass@localhost:5432/myapp

# Auth
MYAPP_API_DEV_AUTH=false

# Ed25519 / EdDSA (used by `underlay-auth-jwt`)
# - AUTH_JWT_PRIVATE_KEY: base64 PKCS#8 DER private key
# - AUTH_JWT_PUBLIC_KEY: base64url (or base64) raw 32-byte public key
# Generator: `docs/guides/code/060-authentication/generate-jwt-env.rs`
AUTH_JWT_PRIVATE_KEY=...
AUTH_JWT_PUBLIC_KEY=...

AUTH_JWT_ISSUER=myapp
AUTH_JWT_AUDIENCE=myapp-api
AUTH_ACCESS_TOKEN_LIFETIME_MINUTES=15
AUTH_REFRESH_TOKEN_LIFETIME_DAYS=30
AUTH_JWT_LEEWAY_SECONDS=30

# Tracing
RUST_LOG=info
RUST_LOG_FORMAT=json
```

### Web (.env)

```bash
PUBLIC_API_URL=http://localhost:3000
# Sent as `X-Api-Version` header
PUBLIC_API_VERSION=2025-01-01
PUBLIC_APP_NAME=MyApp
```

### Admin (.env)

```bash
PUBLIC_API_URL=http://localhost:3000
# Sent as `X-Api-Version` header
PUBLIC_API_VERSION=2025-01-01
PUBLIC_APP_NAME=MyApp Admin
```

## Configuration Validation

See code examples in `/code/120-configuration/`

## Next Steps

- [130-testing.md](./130-testing.md)
