# 140 - Local Development

This document covers running and debugging the application locally.

## Development Workflow

### 1. Start Database

```bash
# Using Docker
docker run -d \
  --name myapp-db \
  -e POSTGRES_USER=user \
  -e POSTGRES_PASSWORD=pass \
  -e POSTGRES_DB=myapp \
  -p 5432:5432 \
  postgres:15
```

### 2. Run Migrations

```bash
cd apps/nursery/crates/db
sqlx database create
sqlx migrate run
```

### 3. Start Backend

```bash
cd apps/nursery
cargo run -p myapp-api
```

### 4. Start Frontends

```bash
# Terminal 1: Bloom
cd apps/bloom
pnpm dev

# Terminal 2: Greenhouse
cd apps/greenhouse
pnpm dev
```

## Access Points

| Service | URL | Purpose |
|---------|-----|---------|
| API | http://localhost:3000 | Backend API |
| Bloom | http://localhost:5173 | Artist UI |
| Greenhouse | http://localhost:5174 | Admin UI |
| Health | http://localhost:3000/health | Health check |

## Debugging

See code examples in `/code/140-local-development/`

## Next Steps

- [150-ci-cd.md](./150-ci-cd.md)
