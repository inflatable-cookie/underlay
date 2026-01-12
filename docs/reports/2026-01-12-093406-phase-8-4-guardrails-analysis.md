# Phase 8.4: Guardrails Analysis & Extraction Plan

**Created**: 2026-01-12  
**Status**: Analysis Complete, Ready for Implementation

## Executive Summary

Dairy's guardrails system is a ~520-line Node.js script that enforces architectural rules by scanning TypeScript and Svelte files for banned patterns and module-scope browser API usage. **The core engine is highly reusable** and can be extracted to Underlay with minimal changes.

## What Exists in Dairy

**File**: `dairy/guardrails.mjs` (520 lines)

### Core Components

1. **Pattern Matching Engine**
   - Regex-based pattern detection
   - Line number tracking for error reporting
   - Suppression comment support (`guardrails-disable-line`, `guardrails-disable-next-line`)
   - Multi-rule suppression (rule IDs, `all` keyword)

2. **Module-Scope Browser API Scanner**
   - Sophisticated JavaScript/TypeScript parser (comment-aware, string-literal-aware)
   - Tracks function depth to distinguish module-scope from function-scope
   - Detects browser globals: `window.*`, `document.*`, `navigator.*`, `localStorage`, etc.
   - Recognizes type guards (`typeof window !== "undefined"`)
   - Handles Svelte `<script>` blocks

3. **File Walker**
   - Recursive directory traversal
   - Filters by extension (`.ts`, `.svelte`)
   - Skips hidden files/directories

4. **Rules Configuration**
   - Banned patterns (e.g., `window.alert`, `navigator.clipboard`)
   - Module-scope checks (browser API detection)
   - Custom error messages per rule

### Current Rules in Dairy

**Banned Patterns**:
- `window.alert` → "Use toast or dialog"
- `window.confirm` → "Use AlertDialog/ConfirmAction"
- `navigator.clipboard` → "Use @decodelabs/underlay/patterns copyToClipboard()"

**Module-Scope Browser APIs**:
- `window.*`, `document.*`, `navigator.*`, `location.*`, `history.*`
- `localStorage`, `sessionStorage`, `matchMedia()`
- Error message: "Use onMount(), typeof guard, or client-only dynamic import"

## What's Reusable vs App-Specific

### ✅ Highly Reusable (Extract to Underlay)

| Component | Why Reusable | Extraction Approach |
|-----------|--------------|---------------------|
| **Core Engine** | Pattern matching, file walking, line tracking | Extract as-is with configurability |
| **Suppression System** | Comment-based rule disabling | 100% reusable |
| **Module-Scope Scanner** | SSR-safety enforcement for any SvelteKit/SSR app | Reusable with configurable API list |
| **Svelte Script Parser** | Handles `<script>` blocks | Reusable for any Svelte project |
| **Line Number Tracking** | Error reporting infrastructure | 100% reusable |

### ⚠️ App-Specific (Provide as Templates)

| Component | Why App-Specific | How to Handle |
|-----------|------------------|---------------|
| **Banned Pattern Rules** | Dairy-specific choices (e.g., "use AlertDialog") | Provide as template/example config |
| **Error Messages** | Reference Dairy components | Make messages configurable |
| **Source Directory** | Hardcoded `./src/` | Make configurable via CLI args |

## Extraction Strategy

### Option 1: CLI Tool (Recommended)

Extract as a standalone CLI tool in Underlay's TypeScript package.

**Location**: `underlay/ts/src/tools/guardrails.ts`  
**Package**: Part of `@decodelabs/underlay` (no new package needed)  
**Invocation**: `node --import tsx underlay/ts/src/tools/guardrails.ts [options]`

**Pros**:
- Single-package-per-language architecture (aligns with Underlay)
- Easy to iterate and evolve
- Can be published as bin entry later if needed
- Zero new package overhead

**Cons**:
- Slightly less convenient than `npx @underlay/guardrails`
- Requires tsx or compilation step

### Option 2: Standalone npm Package (Not Recommended)

Create `@underlay/guardrails` as separate package.

**Pros**:
- Convenient CLI: `npx @underlay/guardrails`
- Standalone versioning

**Cons**:
- Violates Underlay's single-package-per-language rule
- More maintenance overhead
- Less discoverability (buried in packages vs part of main package)

**Recommendation**: Use Option 1. If CLI convenience becomes important, add bin entry to main Underlay package later.

## Implementation Plan

### Phase 8.4.1: Extract Core Engine

**Create**: `underlay/ts/src/tools/guardrails.ts` (~600 lines)

**Components**:
1. File walker (reuse Dairy's implementation)
2. Line number tracking (reuse Dairy's implementation)
3. Suppression parser (reuse Dairy's implementation)
4. Module-scope scanner (reuse Dairy's implementation)
5. Svelte script parser (reuse Dairy's implementation)

**Enhancements**:
- Add TypeScript types for all functions
- Make configurable via options object
- Extract hardcoded paths to config

### Phase 8.4.2: Configuration System

**Create**: `underlay/ts/src/tools/guardrails-config.ts`

**Interface**:
```typescript
interface GuardrailsConfig {
  srcDir: string;                    // Default: './src'
  extensions: string[];              // Default: ['.ts', '.svelte']
  bannedPatterns: BannedPattern[];
  moduleScopeChecks: ModuleScopeCheck[];
  suppressionPrefix: string;         // Default: 'guardrails-disable'
}

interface BannedPattern {
  name: string;
  regex: RegExp;
  message: string;
}

interface ModuleScopeCheck {
  name: string;
  kind: 'prefix' | 'identifier' | 'call';
  value: string;
  message: string;
}
```

**Load from**:
1. `.guardrailsrc.json` (if exists)
2. `package.json` `"guardrails"` field
3. Built-in defaults

### Phase 8.4.3: Template Configurations

**Create**: `underlay/ts/src/tools/guardrails-templates/`

**Templates**:
1. `sveltekit-ssr.ts` - SSR-safety rules (Dairy's current module-scope checks)
2. `banned-apis.ts` - Common banned patterns (alert, confirm, etc.)
3. `react-ssr.ts` - React SSR rules (hydration-safe checks)

**Usage**:
```typescript
// In consumer project
import { ssrSafetyRules } from '@decodelabs/underlay/tools/guardrails-templates/sveltekit-ssr';

export default {
  srcDir: './src',
  moduleScopeChecks: ssrSafetyRules,
  bannedPatterns: [
    { name: 'window.alert', regex: /\bwindow\.alert\s*\(/g, message: 'Use toast' }
  ]
};
```

### Phase 8.4.4: CLI Interface

**Features**:
- `--config <path>` - Custom config file
- `--src <dir>` - Source directory override
- `--fix` - Auto-fix (future)
- Exit code 1 on failures (CI-friendly)

**Example**:
```bash
# Use default config
node --import tsx underlay/ts/src/tools/guardrails.ts

# Custom config
node --import tsx underlay/ts/src/tools/guardrails.ts --config .guardrailsrc.json

# Custom source dir
node --import tsx underlay/ts/src/tools/guardrails.ts --src ./app
```

### Phase 8.4.5: Documentation

**Create**: `underlay/docs/guides/tools/guardrails.md`

**Sections**:
1. Overview & motivation (SSR safety, architectural enforcement)
2. Setup & configuration
3. Built-in templates
4. Custom rules
5. Suppression comments
6. CI integration
7. Migration from Dairy

**Update**: `underlay/docs/guides/quickstart/140-local-development.md`
- Add section on guardrails for frontend projects
- Reference tools/guardrails.md

### Phase 8.4.6: Testing

**Create**: `underlay/ts/tests/tools/guardrails.test.ts`

**Test Coverage**:
1. Pattern matching (banned patterns)
2. Module-scope detection (browser APIs)
3. Suppression parsing (line, next-line, all, specific rules)
4. Svelte script extraction
5. Function depth tracking (module vs function scope)
6. Type guard detection (`typeof window`)
7. Line number accuracy

**Test Fixtures**:
- Sample `.ts` files with violations
- Sample `.svelte` files with script blocks
- Config files

## Migration Path for Dairy

### Before (Dairy-specific)
```json
// package.json
{
  "scripts": {
    "lint:guardrails": "node guardrails.mjs"
  }
}
```

### After (Using Underlay)
```json
// package.json
{
  "scripts": {
    "lint:guardrails": "node --import tsx ../underlay/ts/src/tools/guardrails.ts"
  }
}
```

### Config File (`.guardrailsrc.json`)
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
      "message": "Use @decodelabs/underlay/patterns copyToClipboard() instead of navigator.clipboard."
    }
  ],
  "moduleScopeChecks": "@decodelabs/underlay/tools/guardrails-templates/sveltekit-ssr"
}
```

## Success Criteria

- [ ] Core engine extracted to `underlay/ts/src/tools/guardrails.ts`
- [ ] Configuration system supports `.guardrailsrc.json`
- [ ] Template for SvelteKit SSR rules
- [ ] CLI accepts `--config` and `--src` flags
- [ ] All tests passing (pattern matching, suppression, module-scope detection)
- [ ] Documentation complete with examples
- [ ] Dairy migrated to use Underlay guardrails
- [ ] Zero regressions in Dairy's existing guardrails checks

## Timeline

**Estimated Effort**: 1-2 days (revised from 5-6 days)

| Task | Duration | Status |
|------|----------|--------|
| 8.4.1: Extract core engine | 3-4 hours | Pending |
| 8.4.2: Configuration system | 1-2 hours | Pending |
| 8.4.3: Templates | 1 hour | Pending |
| 8.4.4: CLI interface | 1 hour | Pending |
| 8.4.5: Documentation | 2 hours | Pending |
| 8.4.6: Testing | 2-3 hours | Pending |
| **Total** | **1-2 days** | **0% complete** |

## Future Enhancements (Post-Phase 8)

1. **Auto-fix Support** - Automatically add suppression comments or fix simple violations
2. **Watch Mode** - Run guardrails on file changes during dev
3. **IDE Integration** - VSCode extension for inline warnings
4. **Performance Optimization** - Parallel file processing for large codebases
5. **More Templates** - React SSR, Vue SSR, accessibility rules
6. **Configurable Severity** - Warning vs error levels per rule

## Recommendation

**Proceed with extraction** using Option 1 (CLI tool in main Underlay package). The core engine is battle-tested, highly reusable, and addresses a real pain point (SSR safety) for any SvelteKit or SSR-focused project.

The extraction is straightforward since the code is already well-structured and focused. Main work is adding TypeScript types, configuration loading, and documentation.

## Next Steps

1. Create `underlay/ts/src/tools/` directory
2. Port `guardrails.mjs` to `guardrails.ts` with types
3. Add configuration system
4. Create SvelteKit SSR template
5. Add tests
6. Document usage
7. Migrate Dairy to use Underlay version
