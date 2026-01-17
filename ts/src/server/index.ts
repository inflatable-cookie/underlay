/**
 * Server-only utilities for SvelteKit applications.
 *
 * This module contains utilities that use Node.js APIs and should
 * only be imported in server-side code (e.g., hooks.server.ts, +page.server.ts).
 *
 * @example
 * ```typescript
 * // In hooks.server.ts
 * import { generateNonce, createCspConfig, applyCspHeaders } from "@decodelabs/underlay/server";
 * ```
 */
export * from "./csp";
