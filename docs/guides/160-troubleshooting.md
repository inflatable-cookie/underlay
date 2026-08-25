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
| "Module not found" | Root workspace install is missing or stale | Run `effigy workspace:js:prepare` from the repository root |
| "Type error" | TypeScript config | Run `bun check` |
| "Import error" | Wrong path | Check exports |

### Frontend

| Issue | Cause | Solution |
|-------|-------|----------|
| CORS error | API not configured | Set correct CORS origins |
| 401 error | No auth token | Implement login flow |
| 404 error | Wrong route | Check API endpoints |

### Vite / SvelteKit

#### "failed to load virtual css module" errors

**Symptom**: When running `bun dev`, you see errors like:
```
[vite-plugin-svelte:load] failed to load virtual css module
/path/to/node_modules/@inflatable-cookie/underlay/patterns/FormShell.svelte?svelte&type=style&lang.css
```

The app may load but styles from underlay components are broken.

**Cause**: vite-plugin-svelte's cache can retain transformed modules after a
released Underlay update or after a workspace install has been interrupted.

**Solution**: Refresh the root install and Vite cache. Keep imports on the
explicit Underlay package subpaths; do not resolve them through a sibling
source checkout:

```typescript
effigy workspace:js:prepare
rm -rf .svelte-kit node_modules/.vite
effigy <front-package>/dev
```

If the error remains, verify that the package manifest pins the intended
Underlay release tag and that imports use explicit exports such as
`@inflatable-cookie/underlay/client/*` or
`@inflatable-cookie/underlay/runtime/*`.

See `docs/guides/code/160-troubleshooting/common-commands.txt` for a quick command checklist.

## Next Steps

- [170-checklist.md](./170-checklist.md)
