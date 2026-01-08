# 160 - Troubleshooting

This document covers common issues and their solutions.

## Common Issues

### Rust

| Issue | Cause | Solution |
|-------|-------|----------|
| "cannot find `sqlx`" | Migration not run | Run `sqlx migrate run` |
| "connection refused" | DB not running | Start PostgreSQL |
| "uuid parse error" | Invalid UUID format | Use valid UUID v7 |

### TypeScript

| Issue | Cause | Solution |
|-------|-------|----------|
| "Module not found" | pnpm install not run | Run `pnpm install` |
| "Type error" | TypeScript config | Run `pnpm check` |
| "Import error" | Wrong path | Check exports |

### Frontend

| Issue | Cause | Solution |
|-------|-------|----------|
| CORS error | API not configured | Set correct CORS origins |
| 401 error | No auth token | Implement login flow |
| 404 error | Wrong route | Check API endpoints |

See full troubleshooting guide in `/code/160-troubleshooting/`

## Next Steps

- [170-checklist.md](./170-checklist.md)
