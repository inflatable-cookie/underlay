/// <reference types="node" />
import { randomBytes } from "node:crypto";
import {
  DEFAULT_SECURITY_HEADERS,
  resolveCspConfig,
  resolveSecurityHeadersConfig,
} from "./csp-config";
export { createCspResolveOptions } from "./csp-resolve";
export { buildCspHeader, getCspHeaderName } from "./csp-header";
export { applySecurityHeaders } from "./csp-security-headers";
export type {
  CspSource,
  CspConfig,
  ResolvedCspConfig,
  SecurityHeadersConfig,
  CspHandleOptions,
} from "./csp-types";
import type {
  CspConfig,
  ResolvedCspConfig,
  SecurityHeadersConfig,
} from "./csp-types";
import { buildCspHeader, getCspHeaderName } from "./csp-header";
import { applySecurityHeaders } from "./csp-security-headers";

/**
 * Generate a cryptographically secure nonce for CSP.
 */
export function generateNonce(): string {
  return randomBytes(16).toString("base64");
}

/** Create a CSP configuration by merging with defaults. */
export function createCspConfig(config: CspConfig = {}): ResolvedCspConfig {
  return resolveCspConfig(config);
}

/** Create security headers configuration by merging with defaults. */
export function createSecurityHeadersConfig(
  config: SecurityHeadersConfig = {},
): SecurityHeadersConfig {
  return resolveSecurityHeadersConfig(config);
}

/** Apply CSP and security headers to a Response object. */
export function applyCspHeaders(
  response: Response,
  cspConfig: ResolvedCspConfig,
  nonce?: string,
  securityHeaders: SecurityHeadersConfig = DEFAULT_SECURITY_HEADERS,
): void {
  response.headers.set(
    getCspHeaderName(cspConfig),
    buildCspHeader(cspConfig, nonce),
  );

  applySecurityHeaders(response, securityHeaders);
}

// ============================================================================
// SvelteKit Integration
// ============================================================================
