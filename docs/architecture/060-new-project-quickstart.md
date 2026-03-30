# New Project Quickstart Guide

This guide provides step-by-step instructions for initializing a new project that follows the Songsprout/Acowtancy architecture. It is primarily written for LLMs tasked with bootstrapping new projects, but also serves as documentation for human developers.

**Reference implementations:** Songsprout (`apps/songsprout/`) and Acowtancy (`apps/acowtancy/`).

---

## 1. Architecture Overview

The architecture follows a **monorepo pattern** with clear separation of concerns:

```
new-project/
├── apps/
│   ├── bloom/          # Artist-facing SvelteKit frontend
│   ├── greenhouse/     # Admin/author SvelteKit frontend
│   └── nursery/        # Rust API backend
├── libs/
│   ├── petal/          # Shared Svelte UI kit and design system
│   ├── stem/           # Shared TypeScript API client
│   └── underlay/       # Shared Rust foundation (external or sibling)
└── trellis/            # Documentation (architecture, domain, process)
```

### Key Principles

1. **Apps own domain logic** - Underlay provides primitives; apps define their nouns and rules.
2. **Shared boundaries are stable** - Small, well-typed interfaces that apps compose.
3. **No forced stack choices** - Underlay provides defaults (axum/sqlx/SvelteKit) but they are optional.
4. **Consistent conventions** - Follow the naming and structure patterns from reference apps.

---

## 2. Prerequisites

### System Requirements

- **Rust:** 1.75+ (`rustc --version`)
- **Node.js:** 20+ (`node --version`)
- **bun:** 9+ (`bun --version`)
- **PostgreSQL:** 14+ (for local development)
- **sqlx-cli:** For running migrations (`cargo install sqlx-cli`)

### Verify Installation

```bash
rustc --version      # Should be 1.75+
node --version       # Should be 20+
bun --version       # Should be 9+
psql --version       # Should be 14+
cargo install sqlx-cli --no-default-features --features postgres
```

---

## 3. Create Project Structure

### 3.1 Initialize the Monorepo Root

```bash
# Create project directory
mkdir -p new-project/apps new-project/libs new-project/trellis
cd new-project

# Initialize Git
git init
cat > .gitignore << 'EOF'
.env
.env.local
*.log
.DS_Store
node_modules
.pnp
.pnp.js
build
dist
.svelte-kit
target
Cargo.lock
EOF
git add .
git commit -m "Initial project structure"
```

### 3.2 Create Root AGENTS.md

Create `AGENTS.md` in the project root (critical for LLM interactions):

```markdown
# Repository Guidelines

This monorepo contains several related projects:

- `apps/bloom/` – artist-facing SvelteKit frontend.
- `apps/greenhouse/` – admin/author SvelteKit frontend.
- `apps/nursery/` – Rust API backend.
- `libs/petal/` – shared Svelte UI kit and design system.
- `libs/stem/` – shared TypeScript API client for Nursery.
- `trellis/` – system, domain, and process documentation.

> Root-scope rule for agents:
> Do **not** create or modify files directly in the repository root **except** this `AGENTS.md`.
> All new code, docs, and configuration must live inside the appropriate subdirectory.

## Project Structure & Module Organization

- App frontends: `apps/bloom/` and `apps/greenhouse/` (routes, Svelte components, assets).
- Backend: `apps/nursery/` (Rust crates, domain modules, HTTP handlers, integrations).
- Shared libraries: `libs/petal/` (UI components, design tokens) and `libs/stem/` (HTTP client, commands, typed models).
- Documentation: `trellis/` (vision, architecture, domain, processes, roadmaps, logs).

## Build, Test, and Development Commands

- Bloom dev server: `cd apps/bloom && bun install && bun dev`.
- Greenhouse dev server: `cd apps/greenhouse && bun install && bun dev`.
- Nursery backend: `cd apps/nursery && cargo test` (tests) and `cargo run` (local API).
- Libraries: `cd libs/stem && bun test`, `cd libs/petal && bun test`.

## Coding Style & Naming Conventions

- TypeScript/JavaScript (bloom/greenhouse/petal/stem): 2-space indentation; components `PascalCase.svelte`; helpers `kebab-case.ts` with `camelCase` identifiers.
- Rust (nursery): use `rustfmt` defaults; modules and files `snake_case`, types and enums `PascalCase`.
- Docs: Markdown with `kebab-case` filenames; keep sections short and skimmable.
```

---

## 4. Set Up Underlay Integration

### 4.1 Option A: Use Underlay as a Sibling Directory (Recommended for Active Development)

```bash
# Assuming Underlay is cloned at the same level as your new project
cd new-project
ln -s ../legacy/libraries/underlay libs/underlay
```

### 4.2 Option B: Use Underlay as a Git Submodule (For Stable Dependencies)

```bash
git submodule add https://github.com/your-org/underlay.git libs/underlay
git submodule update --init --recursive
```

### 4.3 Configure Underlay Paths

Create `libs/underlay/rust/crates/underlay-Cargo.toml` references in your Nursery workspace, or use path overrides in `apps/nursery/Cargo.toml`:

```toml
[workspace]
members = [
  "crates/core",
  "crates/api",
  "crates/auth",
  # ... other crates
]

[workspace.dependencies]
# Underlay (local dev via relative paths)
underlay-core = { path = "../../../libs/underlay/rust/crates/underlay-core" }
underlay-http = { path = "../../../libs/underlay/rust/crates/underlay-http" }
underlay-auth = { path = "../../../libs/underlay/rust/crates/underlay-auth" }
underlay-db = { path = "../../../libs/underlay/rust/crates/underlay-db" }
underlay-observability = { path = "../../../libs/underlay/rust/crates/underlay-observability" }
underlay-metrics = { path = "../../../libs/underlay/rust/crates/underlay-metrics" }
```

---

## 5. Create the Rust Backend (Nursery Pattern)

### 5.1 Initialize Nursery Workspace

```bash
mkdir -p apps/nursery/crates/{core,api,auth,db,infra}
cd apps/nursery
```

Create `Cargo.toml`:

```toml
[workspace]
members = [
  "crates/core",
  "crates/infra",
  "crates/db",
  "crates/api",
  "crates/auth",
]

[workspace.package]
version = "0.1.0"
edition = "2021"

[workspace.dependencies]
anyhow = "1"
async-trait = "0.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["fmt", "env-filter", "json"] }
uuid = { version = "1", features = ["v7", "serde"] }
axum = "0.7"
tower-http = { version = "0.6", features = ["trace", "request-id", "propagate-header", "cors"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread", "signal"] }
chrono = { version = "0.4", features = ["clock", "serde"] }
sqlx = { version = "0.8.6", features = ["runtime-tokio-rustls", "postgres", "uuid", "migrate", "chrono"] }
dotenvy = "0.15"

# Underlay
underlay-core = { path = "../../../libs/underlay/rust/crates/underlay-core" }
underlay-http = { path = "../../../libs/underlay/rust/crates/underlay-http" }
underlay-auth = { path = "../../../libs/underlay/rust/crates/underlay-auth" }
underlay-db = { path = "../../../libs/underlay/rust/crates/underlay-db" }
underlay-observability = { path = "../../../libs/underlay/rust/crates/underlay-observability" }
underlay-metrics = { path = "../../../libs/underlay/rust/crates/underlay-metrics" }
```

### 5.2 Create Core Crate

Create `crates/core/src/lib.rs`:

```rust
//! Core domain types and identifiers.

use underlay_core::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtistId(pub Uuid);

impl ArtistId {
    pub fn new() -> Self {
        Self(Uuid::new_v7())
    }
}

impl From<Uuid> for ArtistId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<ArtistId> for Uuid {
    fn from(value: ArtistId) -> Self {
        value.0
    }
}

impl std::fmt::Display for ArtistId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

### 5.3 Create Auth Crate

Create `crates/auth/src/lib.rs` (see `apps/songsprout/nursery/crates/auth/src/lib.rs` for full pattern):

```rust
//! Auth boundary crate.

mod jwt;
mod provider;
mod underlay;

pub use jwt::{JwtAuthProvider, JwtConfig};
pub use provider::{AuthError, AuthProvider, AuthResult};
pub use underlay::{DevBearerUuidAuthProvider, UnderlayJwtAuthProvider};
```

Create `crates/auth/src/underlay.rs`:

```rust
//! Underlay auth integration.

use async_trait::async_trait;
use underlay_auth::{AuthProvider, AuthResult, Principal};

/// Dev auth provider for local development only.
/// WARNING: This accepts ANY UUID as a valid token and grants all roles.
/// NEVER enable in production.
#[derive(Debug, Default, Clone, Copy)]
pub struct DevBearerUuidAuthProvider;

#[async_trait]
impl AuthProvider for DevBearerUuidAuthProvider {
    async fn authenticate_bearer(&self, bearer_token: &str) -> AuthResult<Principal> {
        let user_id = underlay_core::Uuid::parse_str(bearer_token)
            .map_err(|_| underlay_auth::AuthError::InvalidToken)?;

        Ok(Principal {
            user_id,
            roles: underlay_auth::RoleSet::new(["admin", "artist"]),
        })
    }
}

/// Production JWT auth provider using Underlay.
#[derive(Clone)]
pub struct UnderlayJwtAuthProvider {
    inner: JwtAuthProvider,
}

impl UnderlayJwtAuthProvider {
    pub fn from_env() -> Option<Self> {
        let cfg = JwtConfig::from_env()?;
        Some(Self {
            inner: JwtAuthProvider::new(cfg),
        })
    }
}

#[async_trait]
impl AuthProvider for UnderlayJwtAuthProvider {
    async fn authenticate_bearer(&self, bearer_token: &str) -> AuthResult<Principal> {
        // Implementation from reference app
        todo!()
    }
}
```

### 5.4 Create DB Crate

Create `crates/db/src/lib.rs`:

```rust
//! Database utilities.

pub use sqlx::PgPool;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("migrations").run(pool).await
}
```

### 5.5 Create API Crate

Create `crates/api/src/main.rs`:

```rust
use std::{net::SocketAddr, sync::Arc};
use axum::{routing::get, Router, Extension};
use tracing::{info, error};

mod handlers;
mod middleware;

use crate::handlers::{list_artists, get_artist};
use crate::middleware::AuthLayer;

#[derive(Clone)]
pub struct AppState {
    // Add repositories here
    pub auth_provider: Arc<dyn underlay_auth::AuthProvider>,
}

impl underlay_auth::HasAuthProvider for AppState {
    fn auth_provider(&self) -> &dyn underlay_auth::AuthProvider {
        self.auth_provider.as_ref()
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create database pool
    let db_pool = nursery_db::create_pool(&config.database_url).await?;

    // Run migrations
    nursery_db::run_migrations(&db_pool).await?;
    info!("Database migrations completed");

    // Create auth provider
    let auth_provider: Arc<dyn underlay_auth::AuthProvider> =
        match nursery_auth::UnderlayJwtAuthProvider::from_env() {
            Some(provider) => Arc::new(provider),
            None => {
                // For local dev, require explicit opt-in
                if std::env::var("NURSERY_DEV_AUTH").as_deref() == Ok("true") {
                    Arc::new(nursery_auth::DevBearerUuidAuthProvider)
                } else {
                    error!("Auth not configured. Set JWT env vars or NURSERY_DEV_AUTH=true");
                    std::process::exit(1);
                }
            }
        };

    // Create app state
    let state = AppState { auth_provider };

    // Build router
    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/api/v1/artists", get(list_artists))
        .route("/api/v1/artists/:id", get(get_artist))
        .layer(AuthLayer)
        .layer(Extension(state));

    // Start server
    let addr = SocketAddr::from(([0.0.0.0], config.port));
    info!("Starting server on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

struct AppConfig {
    database_url: String,
    port: u16,
}

impl AppConfig {
    fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a number"),
        }
    }
}
```

Create `crates/api/src/handlers.rs`:

```rust
use axum::{Extension, Json, extract::Path};
use underlay_http::SingleResponse;
use crate::AppState;

pub async fn list_artists(
    state: Extension<AppState>,
) -> Json<SingleResponse<Vec<ArtistDto>>> {
    // Implementation
    todo!()
}

pub async fn get_artist(
    state: Extension<AppState>,
    Path(id): Path<Uuid>,
) -> Json<SingleResponse<ArtistDto>> {
    // Implementation
    todo!()
}

#[derive(serde::Serialize)]
pub struct ArtistDto {
    id: Uuid,
    name: String,
}
```

Create `crates/api/src/middleware.rs`:

```rust
use axum::{
    body::Body,
    response::Response,
    extract::Request,
    middleware::{self, Next},
};
use underlay_auth::Authenticated;

pub async fn auth_layer(request: Request, next: Next) -> Response {
    // Use the standard Underlay extractor
    // This is handled by the Authenticated extractor in handlers
    next.run(request).await
}
```

### 5.6 Create Migrations Directory

```bash
mkdir -p apps/nursery/crates/db/migrations
cd apps/nursery/crates/db
# Create initial migration
sqlx migrate add init
```

Edit `migrations/XXXX_init.sql`:

```sql
-- Your initial schema here
-- Follow underlay-auth database schema patterns
```

---

## 6. Create the TypeScript API Client (Stem Pattern)

### 6.1 Initialize Stem

```bash
mkdir -p libs/stem/src
cd libs/stem
```

Create `package.json`:

```json
{
  "name": "@new-project/stem",
  "version": "0.0.1",
  "private": true,
  "type": "module",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "scripts": {
    "build": "tsc -p tsconfig.json",
    "lint": "eslint src --ext .ts",
    "check": "tsc -p tsconfig.json --noEmit",
    "test": "vitest"
  },
  "dependencies": {
    "@decodelabs/underlay": "file:../../libs/underlay"
  },
  "devDependencies": {
    "@types/node": "^22.0.0",
    "@typescript-eslint/eslint-plugin": "^8.0.0",
    "@typescript-eslint/parser": "^8.0.0",
    "eslint": "^9.0.0",
    "typescript": "^5.0.0",
    "vitest": "^2.0.0"
  }
}
```

Create `tsconfig.json`:

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "NodeNext",
    "moduleResolution": "NodeNext",
    "outDir": "dist",
    "rootDir": "src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist"]
}
```

### 6.2 Create HTTP Client Pattern

Create `libs/stem/src/http.ts`:

```typescript
import type { ErrorEnvelope } from '@decodelabs/underlay/client/types';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

export class HttpClient {
  private baseUrl: string;
  private defaultHeaders: HeadersInit;

  constructor(baseUrl: string = API_URL) {
    this.baseUrl = baseUrl;
    this.defaultHeaders = {
      'Content-Type': 'application/json',
    };
  }

  async request<T>(
    method: string,
    path: string,
    options: RequestInit = {}
  ): Promise<T> {
    const url = `${this.baseUrl}${path}`;
    const headers = { ...this.defaultHeaders, ...options.headers };

    const response = await fetch(url, {
      method,
      headers,
      body: options.body ? JSON.stringify(options.body) : undefined,
      credentials: 'include',
    });

    if (!response.ok) {
      const error: ErrorEnvelope = await response.json();
      throw new ApiError(error);
    }

    return response.json();
  }

  get<T>(path: string) {
    return this.request<T>('GET', path);
  }

  post<T>(path: string, body: unknown) {
    return this.request<T>('POST', path, { body });
  }

  // Add token management for authenticated requests
  setAuthToken(token: string) {
    this.defaultHeaders = {
      ...this.defaultHeaders,
      Authorization: `Bearer ${token}`,
    };
  }
}

export class ApiError extends Error {
  constructor(public readonly envelope: ErrorEnvelope) {
    super(envelope.message);
  }
}

export const http = new HttpClient();
```

### 6.3 Create API Commands Pattern

Create `libs/stem/src/commands/artists.ts`:

```typescript
import { http } from '../http';
import type { Artist } from '../types';

export async function listArtists(): Promise<Artist[]> {
  const response = await http.get<{ data: Artist[] }>('/api/v1/artists');
  return response.data;
}

export async function getArtist(id: string): Promise<Artist> {
  const response = await http.get<{ data: Artist }>(`/api/v1/artists/${id}`);
  return response.data;
}
```

Create `libs/stem/src/index.ts`:

```typescript
export * from './http';
export * from './commands/artists';
// Export other commands
```

---

## 7. Create the Shared UI Kit (Petal Pattern)

### 7.1 Initialize Petal

```bash
mkdir -p libs/petal/src/{components,patterns,styles}
cd libs/petal
```

Create `package.json`:

```json
{
  "name": "@new-project/petal",
  "version": "0.0.1",
  "private": true,
  "type": "module",
  "scripts": {
    "check": "svelte-check --tsconfig ./tsconfig.json",
    "lint": "eslint src --ext .ts,.svelte"
  },
  "devDependencies": {
    "@sveltejs/vite-plugin-svelte": "^4.0.0",
    "svelte": "^5.0.0",
    "svelte-check": "^4.0.0",
    "typescript": "^5.0.0",
    "vitest": "^2.0.0"
  }
}
```

Create `svelte.config.js`:

```javascript
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

export default {
  preprocess: vitePreprocess(),
};
```

### 7.2 Create Shared Components

Create `libs/petal/src/components/Button.svelte`:

```svelte
<script lang="ts">
  interface Props {
    variant?: 'primary' | 'secondary' | 'danger';
    size?: 'sm' | 'md' | 'lg';
    disabled?: boolean;
    onclick?: () => void;
    children: import('svelte').Snippet;
  }

  let { variant = 'primary', size = 'md', disabled = false, onclick, children }: Props = $props();

  const base = 'inline-flex items-center justify-center font-medium rounded transition-colors';
  const variants = {
    primary: 'bg-blue-600 text-white hover:bg-blue-700',
    secondary: 'bg-gray-200 text-gray-900 hover:bg-gray-300',
    danger: 'bg-red-600 text-white hover:bg-red-700',
  };
  const sizes = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2 text-base',
    lg: 'px-6 py-3 text-lg',
  };
</script>

<button
  class="{base} {variants[variant]} {sizes[size]}"
  disabled={disabled}
  onclick={onclick}
>
  {@render children()}
</button>
```

---

## 8. Create the Artist Frontend (Bloom Pattern)

### 8.1 Initialize Bloom

```bash
mkdir -p apps/bloom/src/{routes,lib,components}
cd apps/bloom
```

Create `package.json`:

```json
{
  "name": "new-project-bloom",
  "private": true,
  "version": "0.0.1",
  "type": "module",
  "scripts": {
    "dev": "vite dev",
    "build": "vite build",
    "preview": "vite preview",
    "check": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json",
    "lint": "eslint src --ext .ts"
  },
  "dependencies": {
    "@decodelabs/underlay": "file:../../libs/underlay",
    "@new-project/petal": "file:../../libs/petal",
    "@new-project/stem": "file:../../libs/stem"
  },
  "devDependencies": {
    "@sveltejs/adapter-auto": "^7.0.0",
    "@sveltejs/kit": "^2.0.0",
    "@sveltejs/vite-plugin-svelte": "^4.0.0",
    "svelte": "^5.0.0",
    "svelte-check": "^4.0.0",
    "typescript": "^5.0.0",
    "vite": "^6.0.0"
  }
}
```

Create `src/app.html`:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <link rel="icon" href="%sveltekit.assets%/favicon.png" />
    <meta name="viewport" content="width=device-width" />
    %sveltekit.head%
  </head>
  <body data-sveltekit-preload-data="hover">
    <div style="display: contents">%sveltekit.body%</div>
  </body>
</html>
```

Create `src/routes/+layout.svelte`:

```svelte
<script lang="ts">
  import '@decodelabs/underlay/styles.css';
  let { children } = $props();
</script>

<nav class="p-4 bg-gray-100">
  <a href="/" class="mr-4">Home</a>
  <a href="/dashboard">Dashboard</a>
</nav>

<main class="p-4">
  {@render children()}
</main>
```

Create `src/routes/+page.svelte`:

```svelte
<script lang="ts">
  import { Button } from '@new-project/petal';
</script>

<h1 class="text-2xl font-bold mb-4">Welcome to New Project</h1>

<Button onclick={() => alert('Clicked!')}>
  Get Started
</Button>
```

### 8.2 Create API Integration

Create `src/lib/api.ts`:

```typescript
import { http } from '@new-project/stem';

http.setAuthToken(
  document.cookie
    .split('; ')
    .find(row => row.startsWith('auth_token='))
    ?.split('=')[1] || ''
);
```

---

## 9. Create the Admin Frontend (Greenhouse Pattern)

### 9.1 Initialize Greenhouse

```bash
mkdir -p apps/greenhouse/src/{routes,lib,components}
cd apps/greenhouse
```

Create `package.json` (similar to Bloom but with `@new-project/greenhouse` name):

```json
{
  "name": "new-project-greenhouse",
  "private": true,
  "version": "0.0.1",
  "type": "module",
  "scripts": {
    "dev": "vite dev",
    "build": "vite build",
    "check": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json"
  },
  "dependencies": {
    "@decodelabs/underlay": "file:../../libs/underlay",
    "@new-project/petal": "file:../../libs/petal",
    "@new-project/stem": "file:../../libs/stem"
  },
  "devDependencies": {
    "@sveltejs/adapter-auto": "^7.0.0",
    "@sveltejs/kit": "^2.0.0",
    "svelte": "^5.0.0",
    "vite": "^6.0.0"
  }
}
```

Create `src/routes/+layout.svelte`:

```svelte
<script lang="ts">
  import '@decodelabs/underlay/styles.css';
  let { children } = $props();
</script>

<nav class="p-4 bg-green-100 border-b border-green-200">
  <a href="/admin" class="mr-4 font-bold">Admin Panel</a>
  <a href="/admin/users">Users</a>
  <a href="/admin/settings">Settings</a>
</nav>

<main class="p-4">
  {@render children()}
</main>
```

---

## 10. Configuration Management

### 10.1 Create .env Files

Create `apps/nursery/.env`:

```bash
# Database
DATABASE_URL=postgres://user:pass@localhost:5432/new_project

# Auth (generate keys for production)
NURSERY_AUTH_ISSUER=new-project
NURSERY_AUTH_AUDIENCE=new-project-api
AUTH_JWT_PRIVATE_KEY=your-base64-encoded-private-key
AUTH_JWT_PUBLIC_KEY=your-base64-encoded-public-key
AUTH_JWT_LEEWAY_SECONDS=30

# Dev mode (only for local development)
NURSERY_DEV_AUTH=false
```

Create `apps/bloom/.env`:

```bash
VITE_API_URL=http://localhost:3000
```

Create `apps/greenhouse/.env`:

```bash
VITE_API_URL=http://localhost:3000
```

### 10.2 Environment Validation

Add startup validation in `apps/nursery/crates/api/src/main.rs`:

```rust
fn validate_config() {
    if std::env::var("DATABASE_URL").is_err() {
        panic!("DATABASE_URL must be set");
    }

    // Validate JWT keys if not in dev mode
    if std::env::var("NURSERY_DEV_AUTH").as_deref() != Ok("true") {
        if std::env::var("AUTH_JWT_PRIVATE_KEY").is_err() {
            panic!("AUTH_JWT_PRIVATE_KEY must be set (or set NURSERY_DEV_AUTH=true for dev)");
        }
    }
}
```

---

## 11. Testing Strategy

### 11.1 Rust Tests

Create `apps/nursery/crates/core/src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn artist_id_generates_v7_uuid() {
        let id = super::ArtistId::new();
        assert!(!id.0.as_uuid().is_nil());
    }
}
```

Run tests:

```bash
cd apps/nursery
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

### 11.2 TypeScript Tests

Create `libs/stem/src/http.test.ts`:

```typescript
import { describe, it, expect, vi } from 'vitest';
import { http } from './http';

describe('HttpClient', () => {
  it('makes GET requests', async () => {
    // Mock fetch
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ data: [] }),
    });

    const result = await http.get<{ data: [] }>('/test');
    expect(result).toEqual({ data: [] });
  });
});
```

Run tests:

```bash
cd libs/stem
bun test
```

### 11.3 Frontend Tests

Create `apps/bloom/src/routes/+page.test.ts`:

```typescript
import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import Page from './+page.svelte';

describe('Home page', () => {
  it('renders welcome message', () => {
    render(Page);
    expect(screen.getByText('Welcome to New Project')).toBeInTheDocument();
  });
});
```

Run frontend tests:

```bash
cd apps/bloom
bun test
```

---

## 12. Running Locally

### 12.1 Start Database

```bash
# Using Docker
docker run -d \
  --name new-project-db \
  -e POSTGRES_USER=user \
  -e POSTGRES_PASSWORD=pass \
  -e POSTGRES_DB=new_project \
  -p 5432:5432 \
  postgres:15

# Or use local PostgreSQL
```

### 12.2 Run Migrations

```bash
cd apps/nursery/crates/db
sqlx database create
sqlx migrate run
```

### 12.3 Start Backend

```bash
cd apps/nursery
cargo run -p nursery-api
```

### 12.4 Start Frontends

```bash
# Terminal 1: Bloom
cd apps/bloom
bun install
bun dev

# Terminal 2: Greenhouse
cd apps/greenhouse
bun install
bun dev
```

### 12.5 Verify

- API: `curl http://localhost:3000/health`
- Bloom: http://localhost:5173
- Greenhouse: http://localhost:5174

---

## 13. GitHub Actions CI (Optional)

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: |
          cd apps/nursery
          cargo test
          cargo clippy --all-targets --all-features -- -D warnings

  typescript:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: oven-sh/setup-bun@v2
        with:
          bun-version: latest
      - name: Install dependencies
        run: |
          cd libs/stem && bun install
          cd ../../
          cd apps/bloom && bun install
      - name: Run tests
        run: |
          cd libs/stem && bun test
          cd ../../
          cd apps/bloom && bun test
```

---

## 14. Checklist Summary

- [ ] Project structure created with apps/, libs/, trellis/
- [ ] Root AGENTS.md created
- [ ] Underlay linked as sibling or submodule
- [ ] Nursery workspace created with core/api/auth/db crates
- [ ] API compiles and connects to database
- [ ] Migrations created and run
- [ ] Stem client created with HTTP patterns
- [ ] Petal UI kit created with shared components
- [ ] Bloom frontend created (SvelteKit)
- [ ] Greenhouse frontend created (SvelteKit)
- [ ] Auth integrated (dev mode works)
- [ ] Configuration validated at startup
- [ ] Tests pass for Rust code
- [ ] Tests pass for TypeScript code
- [ ] Local development workflow verified

---

## 15. Common Issues and Solutions

### Issue: "Database URL must be set"

**Solution:** Ensure `.env` file exists in `apps/nursery/` with `DATABASE_URL`.

### Issue: "Module not found: Can't resolve '@decodelabs/underlay'"

**Solution:** Run `bun install` in the frontend directory, or ensure Underlay is properly linked.

### Issue: "Connection refused" when calling API

**Solution:** Ensure the Rust backend is running on port 3000.

### Issue: "CORS error" in browser

**Solution:** Configure CORS in the Rust API to allow localhost origins.

### Issue: "sqlx-cli not found"

**Solution:** Run `cargo install sqlx-cli --no-default-features --features postgres`.

---

## 16. Next Steps

After completing this guide:

1. **Add domain entities** specific to your project in `apps/nursery/crates/core_domain/`
2. **Implement CRUD handlers** for your domain entities
3. **Extend the API client** with domain-specific commands
4. **Build out UI components** in Petal for your domain
5. **Create feature pages** in Bloom and Greenhouse
6. **Add authentication flow** (login, registration, session management)
7. **Set up CI/CD** for automated testing and deployment
8. **Add deployment configuration** (Docker, Kubernetes, etc.)
