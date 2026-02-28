# 140 - Local Development

> **Reference Implementation**: This guide includes patterns from a production application built with Underlay. These serve as working examples of best practices.

This document covers running and debugging the application locally.

Paths below use monorepo-style logical paths (e.g. `apps/api/...`). In multi-repo mode, run the same commands from the relevant repo root.

## Vite Configuration for Local Dependencies

When using Underlay as a local `file:` dependency (symlinked), Vite's dependency caching can cause stale module errors and hydration mismatches. Configure Vite to handle this properly.

### vite.config.ts

```typescript
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [sveltekit()],
  optimizeDeps: {
    // Local file: dependencies change frequently - exclude to avoid stale cache.
    // List each subpath export you import from.
    exclude: [
      "@decodelabs/underlay",
      "@decodelabs/underlay/components",
      "@decodelabs/underlay/nightfire",
      "@decodelabs/underlay/patterns",
      "@decodelabs/underlay/styles",
      "@decodelabs/underlay/client"
    ]
  },
  ssr: {
    // Force these packages through Vite's transform pipeline for SSR
    noExternal: ["bits-ui", "runed", "svelte-toolbelt", "lucide-svelte"]
  },
  server: {
    port: 5173,
    watch: {
      // Watch changes in symlinked local dependencies.
      // The `!` prefix means "don't ignore" (inverts the pattern).
      ignored: [
        "!**/node_modules/@decodelabs/underlay/**",
        "!**/node_modules/@myapp/shared/**"  // Add your other local deps
      ]
    }
  }
});
```

### package.json Scripts

Add helper scripts for cache management:

```json
{
  "scripts": {
    "dev": "vite dev",
    "dev:clean": "bun run clean && vite dev --force",
    "dev:force": "vite dev --force",
    "clean": "rm -rf .svelte-kit node_modules/.vite"
  }
}
```

| Script | When to Use |
|--------|-------------|
| `bun dev` | Normal development |
| `bun dev:force` | After updating Underlay (quick refresh) |
| `bun dev:clean` | If hydration errors persist (thorough refresh) |
| `bun clean` | Just clear cache without starting dev |

### Common Issues

**Hydration Mismatch After Updating Underlay**

Vite caches prebundled dependencies in `node_modules/.vite`. When you change Underlay, this cache becomes stale.

```bash
# Quick fix
bun dev:force

# Thorough fix
bun dev:clean
```

**"Cannot set properties of null" Hydration Errors**

This usually means the server-rendered HTML doesn't match the client hydration. Common causes:

1. Stale Vite cache (use `bun dev:clean`)
2. Browser APIs used at module scope (use guardrails to detect)
3. Conditional rendering that differs between server and client

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
cd apps/api/crates/db
sqlx database create
sqlx migrate run
```

### 3. Start Backend

```bash
cd apps/api
cargo run -p myapp-api
```

### 4. Start Frontends

```bash
# Terminal 1: Web frontend
cd apps/web
bun dev

# Terminal 2: Admin frontend
cd apps/admin
bun dev
```

## Access Points

| Service | URL | Purpose |
|---------|-----|---------|
| API | http://localhost:3000 | Backend API |
| Web | http://localhost:5173 | User UI |
| Admin | http://localhost:5174 | Admin UI |
| Health | http://localhost:3000/health | Health check |

## Debugging

See code examples in `docs/guides/code/140-local-development/docker-compose.yml` and `docs/guides/code/140-local-development/run-local.txt`

## Architectural Guardrails

As your application grows, it becomes important to enforce architectural rules and best practices automatically. Underlay provides a `guardrails` CLI tool that scans source code for anti-patterns and enforces project-specific conventions.

### Overview

Guardrails is a lightweight linting tool that scans your TypeScript and Svelte code for:

1. **Banned Patterns** - APIs or patterns you want to discourage (e.g., `window.alert`, `navigator.clipboard`)
2. **Module-Scope Browser APIs** - Browser globals used outside functions (breaks SSR)

**Why Guardrails?**

- **SSR Safety**: SvelteKit and other SSR frameworks crash when browser-only code runs at module scope. Guardrails catches these issues at build time.
- **Architectural Enforcement**: Ban deprecated APIs, enforce better UX patterns, and prevent common mistakes.
- **Fast & Focused**: Scans only for patterns you care about. Works alongside ESLint/TypeScript without overlap.

### Quick Start

#### 1. Create Configuration

Create `.guardrailsrc.json` in your project root:

```json
{
  "srcDir": "./src",
  "extensions": [".ts", ".svelte"],
  "bannedPatterns": [
    {
      "name": "window.alert",
      "regex": "\\bwindow\\.alert\\s*\\(",
      "message": "Use a toast or dialog component instead of window.alert()."
    }
  ],
  "moduleScopeChecks": "@decodelabs/underlay/tools/templates/sveltekit-ssr"
}
```

#### 2. Add to package.json

```json
{
  "scripts": {
    "lint:guardrails": "bun ../underlay/ts/src/tools/guardrails.ts"
  }
}
```

#### 3. Run

```bash
bun lint:guardrails
```

### Configuration Options

```json
{
  "srcDir": "./src",              // Directory to scan
  "extensions": [".ts", ".svelte"], // File extensions
  "bannedPatterns": [...],        // Patterns to ban
  "moduleScopeChecks": [...],     // Module-scope rules
  "suppressionPrefix": "guardrails-disable" // Comment prefix
}
```

### Using Templates

Instead of defining rules inline, use pre-built templates:

```json
{
  "moduleScopeChecks": "@decodelabs/underlay/tools/templates/sveltekit-ssr",
  "bannedPatterns": "@decodelabs/underlay/tools/templates/banned-apis"
}
```

**Available Templates**:

| Template | Path | Purpose |
|----------|------|---------|
| **SvelteKit SSR** | `sveltekit-ssr` | Prevents module-scope browser APIs |
| **Banned APIs** | `banned-apis` | Common problematic APIs (alert, confirm, etc.) |

### Custom Rules

#### Banned Patterns

```json
{
  "bannedPatterns": [
    {
      "name": "console.log",
      "regex": "\\bconsole\\.log\\s*\\(",
      "message": "Use a proper logger instead of console.log"
    }
  ]
}
```

**Fields**:
- `name` - Rule identifier (for suppressions)
- `regex` - Pattern to match (use `\\` for escaping in JSON)
- `message` - Error message shown to developers

#### Module-Scope Checks

```json
{
  "moduleScopeChecks": [
    {
      "name": "window.innerWidth",
      "kind": "prefix",
      "value": "window.",
      "message": "Use onMount() or typeof guard"
    },
    {
      "name": "localStorage",
      "kind": "identifier",
      "value": "localStorage",
      "message": "Use onMount() for storage access"
    },
    {
      "name": "matchMedia(...)",
      "kind": "call",
      "value": "matchMedia",
      "message": "Use onMount() for media queries"
    }
  ]
}
```

**Kinds**:
- `prefix` - Matches `value` followed by anything (e.g., `window.` matches `window.alert`, `window.innerWidth`)
- `identifier` - Matches exact identifier (e.g., `localStorage`)
- `call` - Matches function call (e.g., `matchMedia(...)`)

### Suppression

Use comments to suppress specific rules:

#### Suppress Single Line

```typescript
// guardrails-disable-next-line window.alert
window.alert("This is OK");
```

```typescript
const width = window.innerWidth; // guardrails-disable-line module-scope-browser-api
```

#### Suppress Multiple Rules

```typescript
// guardrails-disable-next-line window.alert, navigator.clipboard
window.alert(navigator.clipboard.readText());
```

#### Suppress All Rules

```typescript
// guardrails-disable-next-line all
someProblematicCode();
```

**Rule IDs**:
- `banned` - All banned patterns
- `module-scope` - All module-scope checks
- `module-scope-browser-api` - Browser API checks specifically
- `[rule-name]` - Specific rule (e.g., `window.alert`, `navigator.clipboard`)

### CLI Usage

```bash
# Use default config (.guardrailsrc.json)
bun underlay/ts/src/tools/guardrails.ts

# Custom config file
bun underlay/ts/src/tools/guardrails.ts --config custom-config.json

# Custom source directory
bun underlay/ts/src/tools/guardrails.ts --src ./app

# Show help
bun underlay/ts/src/tools/guardrails.ts --help
```

### Common Patterns

#### SvelteKit SSR Safety

**Problem**: Module-scope browser APIs crash SSR.

```typescript
// ❌ BAD: Module-scope browser API
const width = window.innerWidth;

export function MyComponent() {
  return <div>Width: {width}</div>;
}
```

**Solutions**:

```typescript
// ✅ GOOD: Inside onMount
let width = 0;
onMount(() => {
  width = window.innerWidth;
});

// ✅ GOOD: Type guard
const width = typeof window !== "undefined" ? window.innerWidth : 0;

// ✅ GOOD: Dynamic import
onMount(async () => {
  const { setupClient} = await import('./client-only');
  setupClient();
});
```

#### Banned Patterns

**Problem**: `window.alert` blocks the UI and provides poor UX.

```typescript
// ❌ BAD: Blocking alert
window.alert("Saved!");
```

**Solutions**:

```typescript
// ✅ GOOD: Toast notification
import { showToast } from '@decodelabs/underlay/patterns';
showToast({ message: "Saved!", type: "success" });

// ✅ GOOD: Dialog component
<AlertDialog title="Success" message="Saved!" />
```

### How It Works

The scanner:

1. **File Walker**: Recursively scans directories for matching extensions
2. **Pattern Matcher**: Uses regex to find banned patterns
3. **Module-Scope Analyzer**: Custom JavaScript parser that:
   - Tracks comments, strings, templates (doesn't scan inside)
   - Tracks function depth (module scope vs function scope)
   - Detects type guards (`typeof window !== "undefined"`)
   - Handles Svelte `<script>` blocks
4. **Suppression Parser**: Checks for disable comments
5. **Reporter**: Outputs file:line errors

### Module-Scope Detection

The scanner distinguishes module scope from function scope:

```typescript
// Module scope (DETECTED)
const x = window.innerWidth;

function foo() {
  // Function scope (NOT DETECTED)
  const y = window.innerWidth;
}

// Module scope with guard (NOT DETECTED)
const z = typeof window !== "undefined" ? window.innerWidth : 0;
```

**Function bodies are ignored** because they only run on the client (after hydration).

### CI Integration

#### GitHub Actions

```yaml
- name: Run Guardrails
  run: bun lint:guardrails
```

#### Pre-commit Hook

```json
{
  "husky": {
    "hooks": {
      "pre-commit": "bun lint:guardrails"
    }
  }
}
```

### Troubleshooting

#### "Module not found" when using templates

Make sure the template path is correct. Templates are located at:

```
underlay/ts/src/tools/templates/sveltekit-ssr.ts
underlay/ts/src/tools/templates/banned-apis.ts
```

Use the reference in config:

```json
{
  "moduleScopeChecks": "@decodelabs/underlay/tools/templates/sveltekit-ssr"
}
```

#### False positives in comments

Guardrails is comment-aware and won't flag patterns inside comments:

```typescript
// This won't trigger: window.alert("test")
/* window.confirm() is fine here too */
```

#### Type guard not recognized

Guardrails recognizes:

```typescript
typeof window !== "undefined"
typeof document !== "undefined"
typeof navigator !== "undefined"
```

But **not**:

```typescript
if (browser) { ... }  // ❌ Not recognized (too generic)
```

Use explicit type guards or suppress the warning.

### Limitations

1. **Not a JavaScript parser**: Guardrails uses pattern matching and heuristics, not a full AST. Complex code may produce false positives.

2. **Svelte reactivity**: Can't distinguish Svelte's reactive `$:` statements from module scope. Use suppressions if needed.

3. **Template literals**: Complex template expressions may confuse the scanner. Use suppressions if needed.

### Source Files

- `ts/src/tools/guardrails.ts` - Main scanner (~550 lines)
- `ts/src/tools/guardrails-config.ts` - Config loader
- `ts/src/tools/templates/sveltekit-ssr.ts` - SSR safety rules
- `ts/src/tools/templates/banned-apis.ts` - Banned API patterns

**Battle-tested**: Extracted from a production admin app (production use since 2025).

## Next Steps

- [150-ci-cd.md](./150-ci-cd.md)
