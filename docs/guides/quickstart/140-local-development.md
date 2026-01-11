# 140 - Local Development

> **Reference Implementation**: This guide includes patterns from Acowtancy, a production application built with Underlay. These serve as working examples of best practices.

This document covers running and debugging the application locally.

Paths below use monorepo-style logical paths (e.g. `apps/nursery/...`). In multi-repo mode, run the same commands from the relevant repo root.

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

See code examples in `docs/guides/quickstart/code/140-local-development/docker-compose.yml` and `docs/guides/quickstart/code/140-local-development/run-local.txt`

## Architectural Guardrails

As your application grows, it becomes important to enforce architectural rules and best practices automatically. Underlay applications can implement custom "guardrails" scripts that scan source code for anti-patterns and enforce project-specific conventions.

### What Are Guardrails?

Guardrails are automated checks that:

- Prevent anti-patterns (e.g., using `window.alert()` instead of proper UI components)
- Enforce component library usage (e.g., requiring Froyo components instead of raw HTML)
- Catch common mistakes (e.g., using browser globals at module scope in SvelteKit)
- Maintain code quality and consistency across the team

Unlike linters, guardrails are **project-specific rules** tailored to your application's architecture. They run in CI/CD pipelines and fail the build if violations are found.

### Example: Enforcing Component Library Usage

One common use case is preventing developers from using raw HTML input elements (`<input>`, `<select>`, etc.) and instead requiring them to use your component library's wrapper components. This ensures:

- Consistent styling across the application
- Proper validation integration
- Accessibility standards
- Easier maintenance when design changes

Here's an example from the Acowtancy Dairy admin app, which enforces the use of Froyo components:

```javascript
// guardrails.mjs
import { readdir, readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = new URL("./src/", import.meta.url);

// Define banned patterns
const bannedPatterns = [
  {
    name: "window.alert",
    regex: /\bwindow\.alert\s*\(/g,
    message: "Use a toast or dialog component instead of window.alert()."
  },
  {
    name: "window.confirm",
    regex: /\bwindow\.confirm\s*\(/g,
    message: "Use AlertDialog/ConfirmAction instead of window.confirm()."
  },
  {
    name: "navigator.clipboard",
    regex: /\bnavigator\.clipboard\b/g,
    message: "Use @decodelabs/underlay/patterns copyToClipboard() instead of navigator.clipboard."
  }
];

// Define module-scope browser API checks (for SvelteKit SSR safety)
const moduleScopeBrowserApiChecks = [
  {
    name: "window.*",
    kind: "prefix",
    value: "window.",
    message: "No browser globals at module scope. Use onMount(), a typeof guard, or a client-only dynamic import."
  },
  {
    name: "document.*",
    kind: "prefix",
    value: "document.",
    message: "No browser globals at module scope. Use onMount(), a typeof guard, or a client-only dynamic import."
  },
  // ... more checks
];

// Walk directory and scan files
let failures = 0;

for await (const fileUrl of walk(ROOT)) {
  const filePath = fileURLToPath(fileUrl);
  const text = await readFile(filePath, "utf8");
  const lineStarts = getLineStarts(text);

  // Check banned patterns
  for (const pattern of bannedPatterns) {
    pattern.regex.lastIndex = 0;
    
    let match;
    while ((match = pattern.regex.exec(text))) {
      if (isSuppressed(text, lineStarts, match.index, ["banned", pattern.name])) {
        continue;
      }
      
      failures++;
      const line = getLineNumberFromIndex(lineStarts, match.index);
      console.error(`${toRelative(fileUrl)}:${line}: banned ${pattern.name}. ${pattern.message}`);
    }
  }
}

if (failures > 0) {
  console.error(`\nGuardrails failed: ${failures} issue(s) found.`);
  process.exit(1);
}
```

**Key features:**

1. **Pattern matching**: Uses regular expressions to find violations
2. **Suppression support**: Allow developers to suppress specific rules when necessary using `// guardrails-disable-line` or `// guardrails-disable-next-line` comments
3. **Clear error messages**: Provides actionable guidance on what to use instead
4. **Exit codes**: Returns non-zero exit code to fail CI builds

### Running Guardrails

Add to your `package.json`:

```json
{
  "scripts": {
    "dev": "vite dev",
    "build": "vite build",
    "guardrails": "node guardrails.mjs",
    "check": "pnpm guardrails && svelte-kit sync && svelte-check --tsconfig ./tsconfig.json"
  }
}
```

Run locally:

```bash
pnpm guardrails
```

### Suppressing Violations

Sometimes you need to violate a rule (e.g., in a polyfill or wrapper). Add suppression comments:

```typescript
// guardrails-disable-next-line window.*
const hasClipboard = typeof window !== "undefined" && window.navigator.clipboard;

// Or suppress multiple rules:
// guardrails-disable-next-line banned navigator.clipboard
navigator.clipboard.writeText(text);

// Or suppress the entire line:
const userAgent = window.navigator.userAgent; // guardrails-disable-line window.*
```

### Integration with CI/CD

Add guardrails to your CI pipeline:

```yaml
# .github/workflows/ci.yml
- name: Run guardrails
  run: pnpm guardrails

- name: Type check
  run: pnpm check
```

This ensures that any pull request violating architectural rules will be rejected.

### Common Guardrail Rules

Here are some common rules you might want to enforce:

1. **Component library usage**: Prevent raw `<input>`, `<button>`, `<select>` tags
2. **SSR safety**: Prevent browser globals at module scope in SvelteKit
3. **API patterns**: Enforce using your API client instead of raw `fetch()`
4. **Navigation**: Require router navigation instead of `window.location`
5. **Dialogs**: Require component library dialogs instead of `alert/confirm/prompt`
6. **Error handling**: Enforce using your error boundary instead of `try/catch` in certain contexts

### Starter Template

Here's a minimal guardrails template to get started:

```javascript
// guardrails.mjs
import { readdir, readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = new URL("./src/", import.meta.url);

const bannedPatterns = [
  {
    name: "raw-input",
    regex: /<input\b/g,
    message: "Use Field + TextInput from your UI kit instead of raw <input>."
  },
  {
    name: "raw-button",
    regex: /<button\b/g,
    message: "Use Button component from your UI kit instead of raw <button>."
  },
  {
    name: "window.alert",
    regex: /\bwindow\.alert\s*\(/g,
    message: "Use a dialog component instead of window.alert()."
  }
];

async function* walk(dirUrl) {
  const dirPath = fileURLToPath(dirUrl);
  const entries = await readdir(dirPath, { withFileTypes: true });

  for (const entry of entries) {
    if (entry.name.startsWith(".")) continue;

    const entryUrl = new URL(entry.name + (entry.isDirectory() ? "/" : ""), dirUrl);

    if (entry.isDirectory()) {
      yield* walk(entryUrl);
    } else {
      const ext = path.extname(entry.name);
      if (ext === ".ts" || ext === ".svelte") {
        yield entryUrl;
      }
    }
  }
}

function getLineNumberFromIndex(text, index) {
  let line = 1;
  for (let i = 0; i < index; i++) {
    if (text.charCodeAt(i) === 10) line++;
  }
  return line;
}

let failures = 0;

for await (const fileUrl of walk(ROOT)) {
  const filePath = fileURLToPath(fileUrl);
  const text = await readFile(filePath, "utf8");

  for (const pattern of bannedPatterns) {
    pattern.regex.lastIndex = 0;
    
    let match;
    while ((match = pattern.regex.exec(text))) {
      failures++;
      const line = getLineNumberFromIndex(text, match.index);
      const relativePath = path.relative(process.cwd(), filePath);
      console.error(`${relativePath}:${line}: ${pattern.name} - ${pattern.message}`);
    }
  }
}

if (failures > 0) {
  console.error(`\nGuardrails failed: ${failures} issue(s) found.`);
  process.exit(1);
}

console.log("Guardrails passed!");
```

### When to Use Guardrails

Guardrails are most valuable when:

- **Your team is growing**: More developers = more need for consistency
- **You have architectural opinions**: Specific patterns you want enforced
- **You're building a component library**: Want to ensure it's actually used
- **You've been bitten by SSR bugs**: Want to prevent browser globals at module scope
- **You want to prevent regressions**: Had issues and want to prevent them happening again

Start small with 1-2 rules, then add more as patterns emerge.

## Next Steps

- [150-ci-cd.md](./150-ci-cd.md)
