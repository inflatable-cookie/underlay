/**
 * Content Security Policy (CSP) utilities for SvelteKit applications.
 *
 * Provides nonce-based CSP header generation with sensible defaults
 * that work well with SvelteKit's hydration model.
 *
 * @example
 * ```typescript
 * // In hooks.server.ts
 * import { createCspConfig, generateNonce, buildCspHeader, applyCspHeaders } from "@decodelabs/underlay/client";
 *
 * const cspConfig = createCspConfig({
 *   connectSrc: ["https://api.example.com"],
 *   frameSrc: ["https://www.youtube.com"]
 * });
 *
 * export const handle: Handle = async ({ event, resolve }) => {
 *   const nonce = generateNonce();
 *
 *   const response = await resolve(event, {
 *     transformPageChunk: ({ html }) => html.replace("%sveltekit.nonce%", nonce)
 *   });
 *
 *   applyCspHeaders(response, cspConfig, nonce);
 *   return response;
 * };
 * ```
 */

/// <reference types="node" />
import { randomBytes } from "crypto";
import {
  DEFAULT_SECURITY_HEADERS,
  resolveCspConfig,
  resolveSecurityHeadersConfig
} from "./csp-config";
export { createCspResolveOptions } from "./csp-resolve";
export type {
  CspSource,
  CspConfig,
  ResolvedCspConfig,
  SecurityHeadersConfig,
  CspHandleOptions
} from "./csp-types";
import type {
  CspSource,
  CspConfig,
  ResolvedCspConfig,
  SecurityHeadersConfig
} from "./csp-types";

// ============================================================================
// Nonce Generation
// ============================================================================

/**
 * Generate a cryptographically secure nonce for CSP.
 *
 * Uses Web Crypto API for secure random generation.
 * Returns a base64-encoded 16-byte random value.
 *
 * @returns A unique nonce string for this request
 */
export function generateNonce(): string {
  return randomBytes(16).toString("base64");
}

// ============================================================================
// Configuration
// ============================================================================

/**
 * Create a CSP configuration by merging with defaults.
 *
 * @param config - Partial configuration to merge with defaults
 * @returns Resolved configuration with all values set
 *
 * @example
 * ```typescript
 * const config = createCspConfig({
 *   connectSrc: ["https://api.example.com"],
 *   frameSrc: ["https://www.youtube.com", "https://player.vimeo.com"],
 *   reportOnly: true // Start in report-only mode
 * });
 * ```
 */
export function createCspConfig(config: CspConfig = {}): ResolvedCspConfig {
  return resolveCspConfig(config);
}

/**
 * Create security headers configuration by merging with defaults.
 *
 * @param config - Partial configuration to merge with defaults
 * @returns Resolved configuration with all values set
 */
export function createSecurityHeadersConfig(
  config: SecurityHeadersConfig = {}
): SecurityHeadersConfig {
  return resolveSecurityHeadersConfig(config);
}

// ============================================================================
// Header Building
// ============================================================================

/**
 * Build a CSP header value from configuration.
 *
 * @param config - Resolved CSP configuration
 * @param nonce - Optional nonce to include in script-src
 * @returns The CSP header value string
 *
 * @example
 * ```typescript
 * const nonce = generateNonce();
 * const headerValue = buildCspHeader(config, nonce);
 * response.headers.set("Content-Security-Policy", headerValue);
 * ```
 */
export function buildCspHeader(config: ResolvedCspConfig, nonce?: string): string {
  const directives: string[] = [];

  const addDirective = (name: string, sources: CspSource[]) => {
    if (sources.length > 0) {
      directives.push(`${name} ${sources.join(" ")}`);
    }
  };

  addDirective("default-src", config.defaultSrc);

  // Add nonce to script-src if provided
  const scriptSources = nonce
    ? [...config.scriptSrc, `'nonce-${nonce}'`]
    : config.scriptSrc;
  addDirective("script-src", scriptSources);

  // Note: Do NOT add nonce to style-src. SvelteKit component styles use inline
  // styles that rely on 'unsafe-inline'. Adding a nonce makes the browser ignore
  // 'unsafe-inline' per CSP spec, breaking all component styles.
  addDirective("style-src", config.styleSrc);

  addDirective("img-src", config.imgSrc);
  addDirective("font-src", config.fontSrc);
  addDirective("connect-src", config.connectSrc);
  addDirective("frame-src", config.frameSrc);
  addDirective("media-src", config.mediaSrc);
  addDirective("object-src", config.objectSrc);
  addDirective("form-action", config.formAction);
  addDirective("base-uri", config.baseUri);
  addDirective("frame-ancestors", config.frameAncestors);

  if (config.reportUri) {
    directives.push(`report-uri ${config.reportUri}`);
  }

  return directives.join("; ");
}

/**
 * Get the appropriate CSP header name based on configuration.
 *
 * @param config - CSP configuration
 * @returns "Content-Security-Policy-Report-Only" or "Content-Security-Policy"
 */
export function getCspHeaderName(config: ResolvedCspConfig): string {
  return config.reportOnly
    ? "Content-Security-Policy-Report-Only"
    : "Content-Security-Policy";
}

/**
 * Apply CSP and security headers to a Response object.
 *
 * @param response - The Response to add headers to
 * @param cspConfig - CSP configuration
 * @param nonce - Optional nonce for script-src
 * @param securityHeaders - Additional security headers configuration
 *
 * @example
 * ```typescript
 * const response = await resolve(event);
 * applyCspHeaders(response, cspConfig, nonce);
 * return response;
 * ```
 */
export function applyCspHeaders(
  response: Response,
  cspConfig: ResolvedCspConfig,
  nonce?: string,
  securityHeaders: SecurityHeadersConfig = DEFAULT_SECURITY_HEADERS
): void {
  // CSP header
  response.headers.set(
    getCspHeaderName(cspConfig),
    buildCspHeader(cspConfig, nonce)
  );

  // Additional security headers
  if (securityHeaders.contentTypeOptions) {
    response.headers.set("X-Content-Type-Options", securityHeaders.contentTypeOptions);
  }
  if (securityHeaders.frameOptions) {
    response.headers.set("X-Frame-Options", securityHeaders.frameOptions);
  }
  if (securityHeaders.referrerPolicy) {
    response.headers.set("Referrer-Policy", securityHeaders.referrerPolicy);
  }
  if (securityHeaders.xssProtection) {
    response.headers.set("X-XSS-Protection", securityHeaders.xssProtection);
  }
  if (securityHeaders.permissionsPolicy) {
    response.headers.set("Permissions-Policy", securityHeaders.permissionsPolicy);
  }
}

// ============================================================================
// SvelteKit Integration
// ============================================================================
