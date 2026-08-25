# 140 - Local Development

> **Reference Implementation**: This guide includes patterns from a production application built with Underlay. These serve as working examples of best practices.

This document covers running and debugging the application locally.

Paths use the supported single-workspace shape: `apps/*`, `packages/*`, and a
root `docs/`. Underlay is a released dependency; sibling checkouts may be
mounted for QA tooling but are not part of the committed application graph.

## Vite Configuration

Use the released Underlay package through its explicit exports. Keep Vite
configuration focused on the app's SSR boundary; do not add aliases to a
sibling Underlay checkout.

### vite.config.ts

```typescript
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [sveltekit()],
  ssr: {
    noExternal: ["bits-ui", "runed", "svelte-toolbelt", "lucide-svelte"]
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

Vite caches prebundled dependencies in `node_modules/.vite`. After bumping the
Underlay release tag, this cache may become stale.

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

### 1. Prepare the workspace and local state

```bash
effigy container up
effigy state plan
effigy state apply local --yes
effigy health
```

### 2. Run schema tasks when needed

```bash
effigy migration:reset
```

The API package owns the `migration:*` implementation. Use its routed task
from the workspace root; do not add root or package `db:*` aliases.

### 3. Start the workspace

```bash
effigy dev
```

For one surface, use a catalog-qualified root task such as
`effigy <front-package>/dev` or `effigy <admin-package>/dev`.

The root catalog owns cross-package startup; package-level commands remain
implementation details.

## Access Points

| Service | URL | Purpose |
|---------|-----|---------|
| API | http://localhost:3000 | Backend API |
| Front | http://localhost:5173 | User UI |
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
  "moduleScopeChecks": "@inflatable-cookie/underlay/tools/templates/sveltekit-ssr"
}
```

#### 2. Run

```bash
effigy check:guardrails
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
  "moduleScopeChecks": "@inflatable-cookie/underlay/tools/templates/sveltekit-ssr",
  "bannedPatterns": "@inflatable-cookie/underlay/tools/templates/banned-apis"
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
# Use the repo-owned guardrail task and its checked-in configuration.
effigy check:guardrails
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
// ✅ GOOD: Use the app's configured toast store
import { useToasts } from '@inflatable-cookie/underlay/runtime/feedback';

const toastStore = useToasts();
toastStore.push({ variant: "success", message: "Saved!" });

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
  run: effigy check:guardrails
```

#### Pre-commit Hook

```json
{
  "husky": {
    "hooks": {
      "pre-commit": "effigy check:guardrails"
    }
  }
}
```

### Troubleshooting

#### "Module not found" when using templates

Make sure the exported template name is correct. Templates are consumed from:

```
@inflatable-cookie/underlay/tools/templates/sveltekit-ssr
@inflatable-cookie/underlay/tools/templates/banned-apis
```

Use the reference in config:

```json
{
  "moduleScopeChecks": "@inflatable-cookie/underlay/tools/templates/sveltekit-ssr"
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
