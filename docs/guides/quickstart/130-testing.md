# 130 - Testing

This document covers testing strategies for all layers of your Underlay application, from unit tests to end-to-end integration tests.

## Testing Philosophy

**Test pyramid approach:**
- **Many** unit tests (fast, focused, cheap to maintain)
- **Some** integration tests (test interactions between components)
- **Few** E2E tests (slow, brittle, expensive to maintain)

**Key principles:**
1. **Test behavior, not implementation** - Focus on what the code does, not how
2. **Fast feedback** - Tests should run quickly to enable rapid iteration
3. **Isolated tests** - Each test should be independent and repeatable
4. **Clear failures** - When a test fails, it should be obvious why

---

## Rust Backend Testing

### Unit Tests

Place unit tests in the same file as the code being tested:

```rust
// apps/nursery/crates/core/src/user.rs

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
}

impl User {
    pub fn is_valid_email(email: &str) -> bool {
        email.contains('@') && email.len() >= 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_email_returns_true() {
        assert!(User::is_valid_email("user@example.com"));
    }

    #[test]
    fn invalid_email_returns_false() {
        assert!(!User::is_valid_email("invalid"));
        assert!(!User::is_valid_email("@"));
        assert!(!User::is_valid_email("a@"));
    }
}
```

### Integration Tests with Database

Create a `tests/` directory in your crate for integration tests:

```
apps/nursery/crates/db/
├── src/
│   ├── lib.rs
│   └── learning.rs
├── tests/
│   ├── learning_soft_delete.rs
│   └── auth_repository.rs
└── Cargo.toml
```

**Test database setup pattern:**

```rust
// apps/nursery/crates/db/tests/learning_soft_delete.rs

use farmyard_db::{create_pool, run_migrations, DbPool};
use sqlx::Row;
use uuid::Uuid;

/// Get test database URL from environment
fn test_database_url() -> Option<String> {
    std::env::var("FARMYARD_DATABASE_URL").ok()
}

/// Set up test database with migrations
async fn setup_db() -> DbPool {
    let Some(db_url) = test_database_url() else {
        unreachable!("setup_db called without FARMYARD_DATABASE_URL")
    };

    let pool = create_pool(&db_url)
        .await
        .expect("failed to create test DB pool");

    run_migrations(&pool)
        .await
        .expect("failed to run migrations for test DB");

    pool
}

/// Seed test data for a learning graph
async fn seed_learning_graph(pool: &DbPool) -> (Uuid, Uuid, Uuid) {
    let module_id = Uuid::now_v7();
    let section_id = Uuid::now_v7();
    let area_id = Uuid::now_v7();

    // Use unique slugs to avoid conflicts between test runs
    let unique = Uuid::now_v7().to_string();
    let module_slug = format!("test-module-{}", unique);

    sqlx::query(
        r#"
        INSERT INTO learning.module (id, pathway_id, slug, code, title)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(module_id)
    .bind(pathway_id)
    .bind(module_slug)
    .bind("TST")
    .bind("Test Module")
    .execute(pool)
    .await
    .expect("failed to insert module");

    // ... more inserts ...

    (module_id, section_id, area_id)
}

#[tokio::test]
async fn soft_delete_module_cascades() {
    // Skip if no test database is configured
    if test_database_url().is_none() {
        return;
    }

    let pool = setup_db().await;
    let (module_id, section_id, area_id) = seed_learning_graph(&pool).await;

    // Verify setup
    let modules = list_modules(&pool, pathway_id)
        .await
        .expect("list_modules failed");
    assert_eq!(modules.len(), 1);

    // Perform soft delete
    let delete_batch_id = Uuid::now_v7();
    soft_delete_module_cascade(&pool, module_id, delete_batch_id)
        .await
        .expect("soft_delete_module_cascade failed");

    // Verify cascade
    let modules_after = list_modules(&pool, pathway_id)
        .await
        .expect("list_modules failed");
    assert_eq!(modules_after.len(), 0, "module should be hidden after soft delete");
}
```

**Test database configuration:**

```bash
# .env.test
FARMYARD_DATABASE_URL=postgres://user:pass@localhost/myapp_test
```

**Run tests with test database:**

```bash
# Multi-repo
cd myapp-api
cargo test -- --test-threads=1

# Monorepo
cd apps/api
cargo test -- --test-threads=1
```

**Note:** Use `--test-threads=1` to avoid concurrent database access issues in integration tests.

### Test Helpers and Assertions

Create reusable test helpers:

```rust
// apps/nursery/crates/db/tests/helpers.rs

use sqlx::Row;
use uuid::Uuid;

pub async fn assert_row_marked_deleted(
    pool: &DbPool,
    table: &str,
    id: Uuid,
    delete_batch_id: Uuid,
) {
    let sql = format!(
        "SELECT deleted_at, delete_batch_id FROM {} WHERE id = $1",
        table
    );
    let row = sqlx::query(&sql)
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap_or_else(|_| panic!("failed to fetch row from {}", table));

    let deleted_at: Option<chrono::DateTime<chrono::Utc>> = row.get("deleted_at");
    let batch: Option<Uuid> = row.get("delete_batch_id");

    assert!(
        deleted_at.is_some(),
        "expected {} row to have deleted_at set",
        table
    );
    assert_eq!(
        batch,
        Some(delete_batch_id),
        "expected {} row delete_batch_id to match",
        table
    );
}

pub async fn assert_row_restored(pool: &DbPool, table: &str, id: Uuid) {
    let sql = format!(
        "SELECT deleted_at, delete_batch_id FROM {} WHERE id = $1",
        table
    );
    let row = sqlx::query(&sql)
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap_or_else(|_| panic!("failed to fetch row from {}", table));

    let deleted_at: Option<chrono::DateTime<chrono::Utc>> = row.get("deleted_at");
    let batch: Option<Uuid> = row.get("delete_batch_id");

    assert!(
        deleted_at.is_none(),
        "expected {} row deleted_at cleared",
        table
    );
    assert!(batch.is_none(), "expected {} row batch cleared", table);
}
```

Use in tests:

```rust
#[tokio::test]
async fn test_soft_delete_and_restore() {
    let pool = setup_db().await;
    let id = seed_module(&pool).await;
    let batch_id = Uuid::now_v7();

    soft_delete(&pool, id, batch_id).await;
    assert_row_marked_deleted(&pool, "learning.module", id, batch_id).await;

    restore(&pool, batch_id).await;
    assert_row_restored(&pool, "learning.module", id).await;
}
```

---

## TypeScript/JavaScript Testing

### Vitest Configuration

Install dependencies:

```bash
# Multi-repo
cd myapp-client
pnpm add -D vitest @vitest/ui

# Monorepo
cd libs/client
pnpm add -D vitest @vitest/ui
```

Create `vitest.config.ts`:

```typescript
// libs/client/vitest.config.ts
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    globals: true,
    environment: "node",
    coverage: {
      provider: "v8",
      reporter: ["text", "html"],
      exclude: ["node_modules/", "dist/", "**/*.test.ts"],
    },
  },
});
```

### Unit Tests for API Commands

Test API client commands using fake HTTP clients:

```typescript
// libs/client/tests/learning-commands.test.ts

import { describe, expect, it } from "vitest";
import type { HttpClient } from "../src/utils/http-client";
import type { LearningModule } from "../src/types/learning-types";
import type { ListResponse, SingleResponse } from "../src/types/common-types";
import { createLearningCommands } from "../src/commands/learning-commands";

/** Fake HTTP client for testing */
class FakeHttpClient implements Pick<HttpClient, "get" | "post"> {
  public calls: { method: string; path: string; body?: unknown }[] = [];
  public nextResponse: unknown;

  async get<T>(path: string): Promise<T> {
    this.calls.push({ method: "GET", path });
    return this.nextResponse as T;
  }

  async post<T>(path: string, body: unknown): Promise<T> {
    this.calls.push({ method: "POST", path, body });
    return this.nextResponse as T;
  }
}

describe("learning commands", () => {
  it("getModules calls the correct path and returns the data", async () => {
    const http = new FakeHttpClient();
    const expected: ListResponse<LearningModule> = {
      data: [
        {
          moduleId: "mod-1",
          slug: "f3-financial-accounting",
          code: "F3",
          title: "Financial Accounting",
          description: "Core financial accounting module.",
        },
      ],
    };
    http.nextResponse = expected;

    const learning = createLearningCommands(http as unknown as HttpClient);

    const result = await learning.getModules();

    expect(http.calls).toEqual([{ method: "GET", path: "/v1/learning/modules" }]);
    expect(result).toEqual(expected);
  });

  it("getModule calls the correct path with encoded moduleId", async () => {
    const http = new FakeHttpClient();
    const expected: SingleResponse<LearningModule> = {
      data: {
        moduleId: "mod/with special",
        slug: "ma-management-accounting",
        code: "MA",
        title: "Management Accounting",
        description: null,
      },
    };
    http.nextResponse = expected;

    const learning = createLearningCommands(http as unknown as HttpClient);

    const moduleId = "mod/with special";
    const result = await learning.getModule(moduleId);

    expect(http.calls).toEqual([
      {
        method: "GET",
        path: `/v1/learning/modules/${encodeURIComponent(moduleId)}`,
      },
    ]);
    expect(result).toEqual(expected);
  });
});
```

**Key patterns:**
- Use fake/mock implementations for external dependencies
- Test that the correct HTTP paths are called
- Test that parameters are properly encoded
- Test that responses are correctly typed

### Integration Tests Against Real API

For integration tests, use a real HTTP client against a test server:

```typescript
// libs/client/tests/integration-learning.test.ts

import { describe, expect, it, beforeAll, afterAll } from "vitest";
import { createClient } from "../src/client";

let apiUrl: string;

beforeAll(async () => {
  // Start test server or use existing one
  apiUrl = process.env.API_URL || "http://localhost:3001";
});

afterAll(async () => {
  // Cleanup if needed
});

describe("learning integration", () => {
  it("can fetch modules from real API", async () => {
    const client = createClient(apiUrl);
    const response = await client.learning.getModules();

    expect(response.data).toBeDefined();
    expect(Array.isArray(response.data)).toBe(true);
  });
});
```

**Run integration tests separately:**

```json
{
  "scripts": {
    "test": "vitest run",
    "test:unit": "vitest run --exclude tests/integration-*.test.ts",
    "test:integration": "vitest run tests/integration-*.test.ts"
  }
}
```

---

## Frontend Testing (SvelteKit)

### Component Testing

Test Svelte components using `@testing-library/svelte`:

```bash
pnpm add -D @testing-library/svelte @testing-library/jest-dom
```

```typescript
// apps/web/tests/Button.test.ts

import { describe, it, expect } from "vitest";
import { render, fireEvent } from "@testing-library/svelte";
import Button from "$lib/components/Button.svelte";

describe("Button", () => {
  it("renders with label", () => {
    const { getByText } = render(Button, { props: { label: "Click me" } });
    expect(getByText("Click me")).toBeInTheDocument();
  });

  it("calls onClick when clicked", async () => {
    let clicked = false;
    const { getByRole } = render(Button, {
      props: {
        label: "Click me",
        onClick: () => {
          clicked = true;
        },
      },
    });

    await fireEvent.click(getByRole("button"));
    expect(clicked).toBe(true);
  });

  it("is disabled when disabled prop is true", () => {
    const { getByRole } = render(Button, {
      props: { label: "Click me", disabled: true },
    });
    expect(getByRole("button")).toBeDisabled();
  });
});
```

### Load Function Testing

Test SvelteKit load functions:

```typescript
// apps/web/tests/load-functions.test.ts

import { describe, it, expect } from "vitest";
import { load } from "../src/routes/modules/+page.server";

describe("modules load function", () => {
  it("fetches modules from API", async () => {
    const mockFetch = async (url: string) => {
      if (url.includes("/v1/learning/modules")) {
        return new Response(
          JSON.stringify({
            data: [{ moduleId: "1", title: "Test Module" }],
          }),
          { status: 200 }
        );
      }
      throw new Error(`Unexpected fetch: ${url}`);
    };

    const result = await load({
      fetch: mockFetch,
      locals: { authToken: "test-token" },
    } as any);

    expect(result.modules).toEqual([
      { moduleId: "1", title: "Test Module" },
    ]);
  });
});
```

---

## End-to-End Testing (Playwright)

### Setup

Install Playwright:

```bash
# Multi-repo
cd myapp-web
pnpm create playwright

# Monorepo
cd apps/web
pnpm create playwright
```

### E2E Test Example

```typescript
// apps/web/tests/e2e/login.spec.ts

import { test, expect } from "@playwright/test";

test.describe("Login flow", () => {
  test("user can log in with valid credentials", async ({ page }) => {
    await page.goto("/login");

    await page.fill('input[name="email"]', "test@example.com");
    await page.fill('input[name="password"]', "password123");
    await page.click('button[type="submit"]');

    await expect(page).toHaveURL("/dashboard");
    await expect(page.locator("h1")).toContainText("Dashboard");
  });

  test("shows error for invalid credentials", async ({ page }) => {
    await page.goto("/login");

    await page.fill('input[name="email"]', "wrong@example.com");
    await page.fill('input[name="password"]', "wrongpassword");
    await page.click('button[type="submit"]');

    await expect(page.locator(".error")).toContainText(
      "Invalid email or password"
    );
  });
});
```

**Run E2E tests:**

```bash
pnpm playwright test
pnpm playwright test --ui  # Interactive mode
pnpm playwright test --debug  # Debug mode
```

---

## Test Organization

### Directory Structure

```
apps/web/
├── src/
│   ├── routes/
│   │   ├── +page.svelte
│   │   └── +page.server.ts
│   └── lib/
│       └── components/
│           └── Button.svelte
├── tests/
│   ├── unit/
│   │   ├── Button.test.ts
│   │   └── utils.test.ts
│   ├── integration/
│   │   └── api-client.test.ts
│   └── e2e/
│       ├── login.spec.ts
│       └── dashboard.spec.ts
└── vitest.config.ts
```

### Naming Conventions

- **Unit tests**: `<filename>.test.ts` or `<ComponentName>.test.ts`
- **Integration tests**: `integration-<feature>.test.ts`
- **E2E tests**: `<feature>.spec.ts`

---

## CI Integration

### GitHub Actions Example

```yaml
# .github/workflows/test.yml
name: Tests

on:
  push:
    branches: [main]
  pull_request:

jobs:
  rust-tests:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_USER: test
          POSTGRES_PASSWORD: test
          POSTGRES_DB: myapp_test
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run Rust tests
        run: |
          cd apps/api
          cargo test
        env:
          FARMYARD_DATABASE_URL: postgres://test:test@localhost/myapp_test

  typescript-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
        with:
          version: 9
      - uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: "pnpm"
      - name: Install dependencies
        run: pnpm install
      - name: Run unit tests
        run: |
          cd libs/client
          pnpm test:unit
      - name: Run integration tests
        run: |
          cd libs/client
          pnpm test:integration
        env:
          API_URL: http://localhost:3000

  e2e-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
        with:
          version: 9
      - uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: "pnpm"
      - name: Install dependencies
        run: pnpm install
      - name: Install Playwright browsers
        run: pnpm playwright install --with-deps
      - name: Run E2E tests
        run: |
          cd apps/web
          pnpm playwright test
```

---

## Test Coverage

### Rust Coverage with cargo-tarpaulin

```bash
cargo install cargo-tarpaulin

cd apps/api
cargo tarpaulin --out Html --output-dir coverage
```

### TypeScript Coverage with Vitest

```bash
cd libs/client
pnpm vitest run --coverage
```

**View coverage reports:**
- Rust: Open `coverage/index.html`
- TypeScript: Open `coverage/index.html`

---

## Performance Testing

### Basic Load Testing with k6

```javascript
// tests/load/modules.js
import http from "k6/http";
import { check, sleep } from "k6";

export const options = {
  vus: 10, // 10 virtual users
  duration: "30s",
};

export default function () {
  const res = http.get("http://localhost:3000/v1/learning/modules", {
    headers: { Authorization: "Bearer test-token" },
  });

  check(res, {
    "status is 200": (r) => r.status === 200,
    "response time < 200ms": (r) => r.timings.duration < 200,
  });

  sleep(1);
}
```

**Run load test:**

```bash
k6 run tests/load/modules.js
```

---

## Best Practices

### Do's
- ✅ **Test behavior**, not implementation details
- ✅ **Use descriptive test names** that explain what is being tested
- ✅ **Keep tests focused** - one assertion per logical behavior
- ✅ **Use test fixtures** for reusable test data
- ✅ **Mock external dependencies** in unit tests
- ✅ **Run tests in CI/CD** on every commit
- ✅ **Test error cases** as well as happy paths

### Don'ts
- ❌ **Don't test framework code** - trust that SvelteKit/Axum work
- ❌ **Don't make tests depend on each other** - each test should be independent
- ❌ **Don't skip test cleanup** - always clean up test data
- ❌ **Don't test implementation details** - test public APIs
- ❌ **Don't commit flaky tests** - fix or remove them

---

## See Also

**Related Guides:**
- **[050-database.md](./050-database.md)** - Database setup for testing
- **[070-api-handlers.md](./070-api-handlers.md)** - Testing API endpoints
- **[100-frontend-bloom.md](./100-frontend-bloom.md)** - Frontend testing patterns
- **[150-ci-cd.md](./150-ci-cd.md)** - CI/CD integration

**Key Topics:**
- Test database setup with migrations
- Fake HTTP clients for API command testing
- Playwright for E2E testing
- Coverage reporting with tarpaulin and vitest

## Next Steps

- [140-local-development.md](./140-local-development.md)
