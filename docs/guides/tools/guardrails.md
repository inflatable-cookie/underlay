# Guardrails - Architectural Rule Enforcement

**Location**: `ts/src/tools/guardrails.ts`  
**Type**: CLI Tool  
**Purpose**: Enforce architectural rules and SSR safety in TypeScript and Svelte projects

## Overview

Guardrails is a lightweight linting tool that scans your TypeScript and Svelte code for:

1. **Banned Patterns** - APIs or patterns you want to discourage (e.g., `window.alert`, `navigator.clipboard`)
2. **Module-Scope Browser APIs** - Browser globals used outside functions (breaks SSR)

### Why Guardrails?

**SSR Safety**: SvelteKit and other SSR frameworks crash when browser-only code runs at module scope. Guardrails catches these issues at build time.

**Architectural Enforcement**: Ban deprecated APIs, enforce better UX patterns, and prevent common mistakes.

**Fast & Focused**: Scans only for patterns you care about. Works alongside ESLint/TypeScript without overlap.

## Quick Start

### 1. Create Configuration

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

### 2. Add to package.json

```json
{
  "scripts": {
    "lint:guardrails": "node --import tsx ../underlay/ts/src/tools/guardrails.ts"
  }
}
```

### 3. Run

```bash
pnpm lint:guardrails
```

## Configuration

### Full Configuration Options

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

## Suppression

Use comments to suppress specific rules:

### Suppress Single Line

```typescript
// guardrails-disable-next-line window.alert
window.alert("This is OK");
```

```typescript
const width = window.innerWidth; // guardrails-disable-line module-scope-browser-api
```

### Suppress Multiple Rules

```typescript
// guardrails-disable-next-line window.alert, navigator.clipboard
window.alert(navigator.clipboard.readText());
```

### Suppress All Rules

```typescript
// guardrails-disable-next-line all
someProblematicCode();
```

**Rule IDs**:
- `banned` - All banned patterns
- `module-scope` - All module-scope checks
- `module-scope-browser-api` - Browser API checks specifically
- `[rule-name]` - Specific rule (e.g., `window.alert`, `navigator.clipboard`)

## CLI Usage

```bash
# Use default config (.guardrailsrc.json)
node --import tsx underlay/ts/src/tools/guardrails.ts

# Custom config file
node --import tsx underlay/ts/src/tools/guardrails.ts --config custom-config.json

# Custom source directory
node --import tsx underlay/ts/src/tools/guardrails.ts --src ./app

# Show help
node --import tsx underlay/ts/src/tools/guardrails.ts --help
```

## Common Patterns

### SvelteKit SSR Safety

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

### Banned Patterns

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

## How It Works

### Scanner Architecture

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

## CI Integration

### GitHub Actions

```yaml
- name: Run Guardrails
  run: pnpm lint:guardrails
```

### Pre-commit Hook

```json
{
  "husky": {
    "hooks": {
      "pre-commit": "pnpm lint:guardrails"
    }
  }
}
```

## Migration from Dairy

If you're migrating from Dairy's standalone `guardrails.mjs`:

### Before

```json
// package.json
{
  "scripts": {
    "lint:guardrails": "node guardrails.mjs"
  }
}
```

### After

```json
// package.json
{
  "scripts": {
    "lint:guardrails": "node --import tsx ../underlay/ts/src/tools/guardrails.ts"
  }
}
```

### Create Config File

Extract rules from `guardrails.mjs` into `.guardrailsrc.json`:

```json
{
  "srcDir": "./src",
  "bannedPatterns": [
    {
      "name": "window.alert",
      "regex": "\\bwindow\\.alert\\s*\\(",
      "message": "Use a toast or dialog component instead of window.alert()."
    },
    {
      "name": "window.confirm",
      "regex": "\\bwindow\\.confirm\\s*\\(",
      "message": "Use AlertDialog/ConfirmAction instead of window.confirm()."
    },
    {
      "name": "navigator.clipboard",
      "regex": "\\bnavigator\\.clipboard\\b",
      "message": "Use @decodelabs/underlay/patterns copyToClipboard() instead."
    }
  ],
  "moduleScopeChecks": "@decodelabs/underlay/tools/templates/sveltekit-ssr"
}
```

## Troubleshooting

### "Module not found" when using templates

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

### False positives in comments

Guardrails is comment-aware and won't flag patterns inside comments:

```typescript
// This won't trigger: window.alert("test")
/* window.confirm() is fine here too */
```

### Type guard not recognized

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

## Limitations

1. **Not a JavaScript parser**: Guardrails uses pattern matching and heuristics, not a full AST. Complex code may produce false positives.

2. **Svelte reactivity**: Can't distinguish Svelte's reactive `$:` statements from module scope. Use suppressions if needed.

3. **Template literals**: Complex template expressions may confuse the scanner. Use suppressions if needed.

## Future Enhancements

- **Auto-fix**: Automatically add suppressions or refactor simple violations
- **Watch mode**: Run on file changes during development
- **IDE integration**: VSCode extension for inline warnings
- **Performance**: Parallel file processing for large codebases
- **More templates**: React SSR, Vue SSR, accessibility rules

## See Also

- **[140-local-development.md](../quickstart/140-local-development.md)** - Dev workflow integration
- **[065-session-management.md](../quickstart/065-session-management.md)** - SvelteKit SSR patterns
- **Phase 8.4 Analysis** - `docs/roadmap/PHASE-8.4-GUARDRAILS-ANALYSIS.md`

## Reference

**Source Files**:
- `ts/src/tools/guardrails.ts` - Main scanner (~550 lines)
- `ts/src/tools/guardrails-config.ts` - Config loader
- `ts/src/tools/templates/sveltekit-ssr.ts` - SSR safety rules
- `ts/src/tools/templates/banned-apis.ts` - Banned API patterns

**Battle-tested**: Extracted from Acowtancy's Dairy admin app (production use since 2025).
