# TypeScript API Client

The TS client layer is the bridge between the Rust API and SvelteKit apps.

## Principles

- Centralise HTTP concerns (base URL, headers, auth token, error parsing).
- Expose app-specific *commands* as standalone functions grouped by domain.
- Commands call `http.get()`/`http.post()` directly - no intermediate abstraction.
- Keep DTOs aligned with the API contract (ideally generated from OpenAPI).
- **Avoid duplicating client wrappers** across frontend apps.

## Underlay exports

- `@inflatable-cookie/underlay/client/types` provides DTO envelope types like
  `ListResponse`, `SingleResponse`, and `ErrorEnvelope`.
- `@inflatable-cookie/underlay/client/http` provides the small `HttpClient`
  abstraction used to build command modules.

## Command Pattern

Commands are standalone async functions that:
1. Accept `fetchFn` and `accessToken` as parameters
2. Call `getHttpClient()` to get a configured HttpClient
3. Make HTTP requests directly with `http.get()`/`http.post()`
4. Unwrap responses (return `T` not `SingleResponse<T>`)

### Parameter Conventions

- Functions **with payload**: `(payload, fetchFn, accessToken?)`
- Functions **without payload**: `(fetchFn, accessToken)`
- Functions **with ID + payload**: `(id, payload, fetchFn, accessToken)`
- Functions **with ID only**: `(id, fetchFn, accessToken)`

### Example Command

```ts
export async function login(
  payload: LoginRequest,
  fetchFn: typeof fetch,
): Promise<AuthSession> {
  const http = getHttpClient({ fetchFn });
  const response = await http.post<SingleResponse<AuthSession>>("/v1/auth/login", payload);
  return response.data;
}
```

## Client Factory Pattern

The recommended pattern for frontend integration:

1. **API client library** exports `configureStem()` and command namespaces.
2. **Each frontend** calls `configureStem()` once in `hooks.server.ts`.
3. **Routes** import commands directly from the shared library.

This avoids duplicate `$lib/api/client.ts` files in each frontend.

```ts
// In API client library (stem/src/index.ts)
export { configureStem } from "./utils/client-factory.js";
export * as authCommands from "./commands/auth-commands.js";
export * as coreCommands from "./commands/core-commands.js";

// In frontend hooks.server.ts
import { configureStem, authCommands } from "@stem";
configureStem({ baseUrl: env.PUBLIC_API_URL, apiVersion: env.PUBLIC_API_VERSION });

// In routes
import { coreCommands } from "@stem";
const artist = await coreCommands.getArtist(artistId, fetch, accessToken);
```

## Integration (in an app)

- Import `configureStem` and call it once in `hooks.server.ts`.
- Import command namespaces (`authCommands`, `coreCommands`, etc.) in routes and form actions.
- Keep Svelte components declarative: call commands in `load` functions and form actions, not inline in components.
- **Do not** create local `$lib/api/client.ts` wrappers that duplicate the shared library.

## File Structure

```
stem/
├── src/
│   ├── index.ts              # Exports commands, types, utilities
│   ├── utils/
│   │   ├── http-client.ts    # HttpClient wrapper around Underlay
│   │   └── client-factory.ts # configureStem + getHttpClient
│   ├── types/
│   │   ├── common-types.ts   # Domain DTOs
│   │   └── auth-types.ts     # Auth DTOs
│   └── commands/
│       ├── auth-commands.ts     # register, login, refresh, logout, me
│       ├── security-commands.ts # TOTP, sessions, passkeys, OAuth
│       ├── core-commands.ts     # Domain operations
│       ├── billing-commands.ts  # Billing operations
│       └── admin-commands.ts    # Admin operations
```
