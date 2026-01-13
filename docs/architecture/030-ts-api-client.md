# TypeScript API Client

The TS client layer is the bridge between the Rust API and SvelteKit apps.

## Principles

- Centralise HTTP concerns (base URL, headers, auth token, error parsing).
- Expose app-specific *commands* as small functions grouped by domain.
- Keep DTOs aligned with the API contract (ideally generated from OpenAPI).
- **Avoid duplicating client wrappers** across frontend apps.

## Underlay exports

- `@decodelabs/underlay/client` provides:
  - DTO envelope types (`ListResponse`, `SingleResponse`, `ErrorEnvelope`).
  - A small `HttpClient` abstraction to build command modules on.

## Client Factory Pattern

The recommended pattern for frontend integration:

1. **API client library** exports `configureClient()` and `getClient()`.
2. **Each frontend** calls `configureClient()` once in `hooks.server.ts`.
3. **Routes** import `getClient()` directly from the shared library.

This avoids duplicate `$lib/api/client.ts` files in each frontend.

```ts
// In api-client library
export function configureClient(config: { baseUrl: string; apiVersion: string }): void;
export function getClient(options: { fetchFn: typeof fetch; accessToken: string | null }): ApiClient;

// In frontend hooks.server.ts
import { configureClient } from "@myorg/client";
configureClient({ baseUrl: env.PUBLIC_API_URL, apiVersion: env.PUBLIC_API_VERSION });

// In routes
import { getClient } from "@myorg/client";
const client = getClient({ fetchFn: fetch, accessToken: locals.authToken });
```

## Integration (in an app)

- Import `configureClient` and call it once in `hooks.server.ts`.
- Import `getClient` in routes and form actions.
- Keep Svelte components declarative: call commands in `load` functions and form actions, not inline in components.
- **Do not** create local `$lib/api/client.ts` wrappers that duplicate the shared library.
