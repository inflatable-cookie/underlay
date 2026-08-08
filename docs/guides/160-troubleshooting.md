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
| "Module not found" | bun install not run | Run `bun install` |
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
/path/to/node_modules/@inflatable-cookie/underlay/ts/src/patterns/FormShell.svelte?svelte&type=style&lang.css
```

The app may load but styles from underlay components are broken.

**Cause**: vite-plugin-svelte creates virtual CSS modules for Svelte component styles. When underlay is installed via `file:` protocol, the plugin may fail to resolve these virtual modules correctly.

**Solution**: Add a resolve alias in `vite.config.ts` to resolve underlay directly from source:

```typescript
import { fileURLToPath } from "node:url";
import path from "node:path";

const __dirname = fileURLToPath(new URL(".", import.meta.url));
const underlayPath = path.resolve(__dirname, "../underlay/ts/src");

export default defineConfig({
  resolve: {
    alias: [
      // Resolve underlay directly from source to avoid path issues
      // with virtual CSS modules. Uses regex to match all subpaths.
      {
        find: /^@inflatable-cookie\/underlay(\/.*)?$/,
        replacement: `${underlayPath}$1`
      }
    ],
    dedupe: ["@inflatable-cookie/underlay"]
  },
  optimizeDeps: {
    exclude: ["@inflatable-cookie/underlay"]
  },
  // ... rest of config
});
```

This resolves `@inflatable-cookie/underlay` and all subpaths (e.g., `/components`,
`/patterns`) directly to the source directory via the project's underlay
symlink.

See `docs/guides/code/160-troubleshooting/common-commands.txt` for a quick command checklist.

## Next Steps

- [170-checklist.md](./170-checklist.md)
