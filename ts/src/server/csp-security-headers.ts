import type { SecurityHeadersConfig } from "./csp-types";

export function applySecurityHeaders(
  response: Response,
  securityHeaders: SecurityHeadersConfig
): void {
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
