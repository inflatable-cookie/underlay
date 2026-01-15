# 072 - Admin/Front Endpoint Separation

> **Reference Implementation**: This guide documents production patterns from Acowtancy's Farmyard API and cattle-grid TypeScript client.

This document covers the architectural pattern for separating admin and front-end API endpoints, ensuring clean boundaries between different client audiences.

## Overview

In applications with both a student/customer-facing frontend ("Cream") and an admin UI ("Dairy"), it's critical to maintain clear separation:

- **Admin UI** should only call admin endpoints
- **Front-end UI** should only call front-end endpoints
- **Shared endpoints** (like auth) can be called by both

This separation provides:
1. **Security clarity** - admin endpoints are grouped and easier to audit
2. **API evolution** - admin and front endpoints can diverge independently
3. **Performance** - admin endpoints may return more data (e.g., non-live items)
4. **Developer experience** - clear mental model of which endpoints serve which UI

## URL Path Convention

### Path Prefixes

| Prefix | Audience | Description |
|--------|----------|-------------|
| `/v1/admin/*` | Admin UI only | All write operations, admin-specific reads |
| `/v1/*` (non-admin) | Front-end UI | Student-facing reads, user-scoped writes |
| `/v1/auth/*` | Shared | Authentication (login, register, refresh, etc.) |
| `/health`, `/metrics` | Infrastructure | Health checks, monitoring |

### Examples

```
# Admin endpoints (Dairy only)
POST   /v1/admin/content/summaries           # Create summary
PUT    /v1/admin/content/summaries/:id       # Update summary
POST   /v1/admin/content/summaries/:id/soft-delete
GET    /v1/admin/content/summaries           # List all (including non-live)
GET    /v1/admin/learning/modules            # List all modules
PUT    /v1/admin/learning/modules/:id        # Update module

# Front endpoints (Cream only)
GET    /v1/content/summaries                 # List live summaries
GET    /v1/learning/modules                  # List live modules
GET    /v1/learning/modules/:id              # Get module (live check)
POST   /v1/assessment/sessions               # Create user's own session

# Shared endpoints (both)
POST   /v1/auth/login
POST   /v1/auth/register
POST   /v1/auth/refresh
GET    /v1/auth/me
```

## Backend Implementation (Rust)

### Router Structure

Organise routes into three sub-routers:

```
routes/
├── mod.rs              # Exports build_router
├── router.rs           # Merges sub-routers
├── admin/
│   ├── mod.rs          # Admin handler exports
│   ├── router.rs       # admin_routes() function
│   ├── content.rs      # Content admin handlers
│   └── learning.rs     # Learning admin handlers
├── front/
│   ├── mod.rs          # Front handler exports
│   ├── router.rs       # front_routes() function
│   ├── content.rs      # Content front handlers
│   └── learning.rs     # Learning front handlers
└── shared/
    ├── mod.rs          # Shared handler exports
    ├── router.rs       # shared_routes() function
    ├── auth.rs         # Auth handlers
    └── health.rs       # Health check handlers
```

### Router Builder Pattern

```rust
// routes/router.rs
use axum::{routing::MethodRouter, Router};
use crate::state::AppState;
use super::{admin, front, shared};

pub fn build_router(openapi_route: MethodRouter<AppState>) -> Router<AppState> {
    Router::new()
        .merge(shared::router::shared_routes(openapi_route))
        .merge(admin::router::admin_routes())
        .merge(front::router::front_routes())
}
```

### Admin Route Example

```rust
// routes/admin/router.rs
use axum::{routing::get, Router};
use crate::state::AppState;
use super::{content, learning};

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        // Content - all admin operations
        .route(
            "/v1/admin/content/summaries",
            get(content::list_summaries_for_admin).post(content::create_summary),
        )
        .route(
            "/v1/admin/content/summaries/:summary_id",
            axum::routing::put(content::update_summary),
        )
        // Learning - all admin operations  
        .route(
            "/v1/admin/learning/modules",
            get(learning::list_modules_admin),
        )
        .route(
            "/v1/admin/learning/modules/:module_id",
            get(learning::get_module_admin).put(learning::update_module),
        )
}
```

### Front Route Example

```rust
// routes/front/router.rs
use axum::{routing::get, Router};
use crate::state::AppState;
use super::{content, learning};

pub fn front_routes() -> Router<AppState> {
    Router::new()
        // Content - read-only, live items
        .route("/v1/content/summaries", get(content::list_summaries))
        .route("/v1/content/videos", get(content::list_videos))
        // Learning - read-only, live items
        .route("/v1/learning/modules", get(learning::list_modules))
        .route("/v1/learning/modules/:module_id", get(learning::get_module))
}
```

### Handler Differences

Admin and front handlers for the same resource may differ:

```rust
// Front handler - filters to live items only
pub async fn list_modules(State(state): State<AppState>) -> impl IntoResponse {
    match state.module_repo.list_modules_live().await {
        // ...
    }
}

// Admin handler - returns all items, requires admin role
pub async fn list_modules_admin(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> impl IntoResponse {
    if !user.has_role(UserRole::Admin) {
        return error_response(StatusCode::FORBIDDEN, 
            AppError::new("auth.forbidden", "Admin access required"));
    }
    
    match state.module_repo.list_modules_all().await {
        // ...
    }
}
```

## TypeScript Client Implementation

### Command Organisation

Organise commands by audience:

```
client/src/commands/
├── admin/
│   ├── content-commands.ts    # Admin content operations
│   └── learning-commands.ts   # Admin learning operations
├── front/
│   ├── content-commands.ts    # Front content operations
│   └── learning-commands.ts   # Front learning operations
└── shared/
    └── auth-commands.ts       # Shared auth operations
```

### Audience-Scoped HTTP Clients

Create separate HTTP client factories that enforce path prefixes:

```typescript
// utils/client-factory.ts

export type Audience = 'admin' | 'front' | 'shared';

interface AudienceClientConfig {
  fetchFn: typeof fetch;
  accessToken: string;
  audience: Audience;
}

const AUDIENCE_PREFIXES: Record<Audience, string[]> = {
  admin: ['/v1/admin/'],
  front: ['/v1/content/', '/v1/learning/', '/v1/assessment/', '/v1/nightfire/'],
  shared: ['/v1/auth/', '/health', '/metrics', '/openapi.json'],
};

function validatePath(path: string, audience: Audience): void {
  const allowedPrefixes = AUDIENCE_PREFIXES[audience];
  const isAllowed = allowedPrefixes.some(prefix => path.startsWith(prefix));
  
  if (!isAllowed) {
    throw new Error(
      `Path "${path}" is not allowed for audience "${audience}". ` +
      `Allowed prefixes: ${allowedPrefixes.join(', ')}`
    );
  }
}

export function getAdminHttpClient(options: Omit<AudienceClientConfig, 'audience'>) {
  return createGuardedClient({ ...options, audience: 'admin' });
}

export function getFrontHttpClient(options: Omit<AudienceClientConfig, 'audience'>) {
  return createGuardedClient({ ...options, audience: 'front' });
}

export function getSharedHttpClient(options: Omit<AudienceClientConfig, 'audience'>) {
  return createGuardedClient({ ...options, audience: 'shared' });
}

function createGuardedClient(config: AudienceClientConfig) {
  const baseClient = getHttpClient({
    fetchFn: config.fetchFn,
    accessToken: config.accessToken,
  });

  return {
    async get<T>(path: string): Promise<T> {
      validatePath(path, config.audience);
      return baseClient.get<T>(path);
    },
    async post<T>(path: string, body: unknown): Promise<T> {
      validatePath(path, config.audience);
      return baseClient.post<T>(path, body);
    },
    async put<T>(path: string, body: unknown): Promise<T> {
      validatePath(path, config.audience);
      return baseClient.put<T>(path, body);
    },
    async delete<T>(path: string): Promise<T> {
      validatePath(path, config.audience);
      return baseClient.delete<T>(path);
    },
  };
}
```

### Using Guarded Clients in Commands

```typescript
// commands/admin/content-commands.ts
import { getAdminHttpClient } from '../../utils/client-factory.js';

export async function createSummary(
  payload: SummaryItemPayload,
  fetchFn: typeof fetch,
  accessToken: string,
): Promise<SummaryItem> {
  const http = getAdminHttpClient({ fetchFn, accessToken });
  // This will throw if path doesn't start with /v1/admin/
  const response = await http.post<SingleResponse<SummaryItem>>(
    '/v1/admin/content/summaries',
    payload
  );
  return response.data;
}

// commands/front/content-commands.ts  
import { getFrontHttpClient } from '../../utils/client-factory.js';

export async function getSummaries(
  fetchFn: typeof fetch,
  accessToken: string,
): Promise<SummaryItem[]> {
  const http = getFrontHttpClient({ fetchFn, accessToken });
  // This will throw if path starts with /v1/admin/
  const response = await http.get<ListResponse<SummaryItem>>(
    '/v1/content/summaries'
  );
  return response.data;
}
```

### Frontend Integration

In the admin UI (Dairy), only import from admin commands:

```typescript
// dairy/src/routes/content/+page.server.ts
import { adminContentCommands } from '@myapp/client/admin';

export async function load({ fetch, locals }) {
  const summaries = await adminContentCommands.getSummariesForAdmin(
    fetch,
    locals.authToken
  );
  return { summaries };
}
```

In the front UI (Cream), only import from front commands:

```typescript
// cream/src/routes/content/+page.ts
import { frontContentCommands } from '@myapp/client/front';

export async function load({ fetch, data }) {
  const summaries = await frontContentCommands.getSummaries(
    fetch,
    data.accessToken
  );
  return { summaries };
}
```

## Development-Time Enforcement

### ESLint Rules (Optional)

Add ESLint rules to prevent cross-audience imports:

```javascript
// .eslintrc.js (in dairy/)
module.exports = {
  rules: {
    'no-restricted-imports': ['error', {
      patterns: [
        {
          group: ['@myapp/client/front/*'],
          message: 'Admin UI should not import front commands',
        },
      ],
    }],
  },
};
```

```javascript
// .eslintrc.js (in cream/)
module.exports = {
  rules: {
    'no-restricted-imports': ['error', {
      patterns: [
        {
          group: ['@myapp/client/admin/*'],
          message: 'Front UI should not import admin commands',
        },
      ],
    }],
  },
};
```

### Runtime Validation

The guarded HTTP clients throw errors at runtime if paths don't match:

```
Error: Path "/v1/admin/content/summaries" is not allowed for audience "front". 
Allowed prefixes: /v1/content/, /v1/learning/, /v1/assessment/
```

This catches mistakes during development and testing.

## Migration Guide

### Phase 1: Backend Route Reorganisation

1. Create `routes/admin/router.rs`, `routes/front/router.rs`, `routes/shared/router.rs`
2. Move handlers to appropriate domains
3. Update paths to follow convention:
   - Admin writes: `/v1/admin/...`
   - Front reads: `/v1/...`
4. Add missing admin GET endpoints (admin may need separate read endpoints)
5. Update OpenAPI annotations with new paths

### Phase 2: TypeScript Client Update

1. Update command paths to match new API paths
2. Implement guarded HTTP clients
3. Reorganise commands into `admin/`, `front/`, `shared/` directories
4. Update exports in `index.ts`

### Phase 3: Frontend Updates

1. Update imports to use audience-specific command modules
2. Add ESLint rules to prevent cross-audience imports
3. Test all flows

## Checklist

- [ ] All admin write operations use `/v1/admin/...` paths
- [ ] Admin read endpoints exist for resources admins need
- [ ] Front endpoints only return live/active items
- [ ] TypeScript client has guarded HTTP clients
- [ ] Frontend apps import from correct audience modules
- [ ] ESLint rules prevent cross-audience imports (optional)

## See Also

- **[070-api-handlers.md](./070-api-handlers.md)** - Handler patterns
- **[067-authorization.md](./067-authorization.md)** - Role-based access control
- **[080-typescript-client.md](./080-typescript-client.md)** - TypeScript client setup
- **[110-admin.md](./110-admin.md)** - Admin UI patterns
