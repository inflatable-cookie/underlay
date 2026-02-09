import type {
  CspConfig,
  CspSource,
  ResolvedCspConfig,
  SecurityHeadersConfig
} from "./csp";

export const DEFAULT_CSP_CONFIG: ResolvedCspConfig = {
  defaultSrc: ["'self'"],
  scriptSrc: ["'self'"],
  styleSrc: ["'self'", "'unsafe-inline'"],
  imgSrc: ["'self'", "data:", "https:"],
  fontSrc: ["'self'"],
  connectSrc: ["'self'"],
  frameSrc: ["'self'"],
  mediaSrc: ["'self'"],
  objectSrc: ["'none'"],
  formAction: ["'self'"],
  baseUri: ["'self'"],
  frameAncestors: ["'none'"],
  reportOnly: false
};

export const DEFAULT_SECURITY_HEADERS: Required<Omit<SecurityHeadersConfig, "permissionsPolicy">> &
  Pick<SecurityHeadersConfig, "permissionsPolicy"> = {
  contentTypeOptions: "nosniff",
  frameOptions: "DENY",
  referrerPolicy: "strict-origin-when-cross-origin",
  xssProtection: false,
  permissionsPolicy: false
};

function mergeDirectiveSources(
  defaults: CspSource[],
  custom: CspSource[] | false | undefined
): CspSource[] {
  if (custom === false) return [];
  if (!custom) return defaults;
  return [...new Set([...defaults, ...custom])];
}

export function resolveCspConfig(config: CspConfig = {}): ResolvedCspConfig {
  return {
    defaultSrc: mergeDirectiveSources(DEFAULT_CSP_CONFIG.defaultSrc, config.defaultSrc),
    scriptSrc: mergeDirectiveSources(DEFAULT_CSP_CONFIG.scriptSrc, config.scriptSrc),
    styleSrc: mergeDirectiveSources(DEFAULT_CSP_CONFIG.styleSrc, config.styleSrc),
    imgSrc: mergeDirectiveSources(DEFAULT_CSP_CONFIG.imgSrc, config.imgSrc),
    fontSrc: mergeDirectiveSources(DEFAULT_CSP_CONFIG.fontSrc, config.fontSrc),
    connectSrc: mergeDirectiveSources(DEFAULT_CSP_CONFIG.connectSrc, config.connectSrc),
    frameSrc: mergeDirectiveSources(DEFAULT_CSP_CONFIG.frameSrc, config.frameSrc),
    mediaSrc: mergeDirectiveSources(DEFAULT_CSP_CONFIG.mediaSrc, config.mediaSrc),
    objectSrc: mergeDirectiveSources(DEFAULT_CSP_CONFIG.objectSrc, config.objectSrc),
    formAction: mergeDirectiveSources(DEFAULT_CSP_CONFIG.formAction, config.formAction),
    baseUri: mergeDirectiveSources(DEFAULT_CSP_CONFIG.baseUri, config.baseUri),
    frameAncestors: mergeDirectiveSources(DEFAULT_CSP_CONFIG.frameAncestors, config.frameAncestors),
    reportOnly: config.reportOnly ?? DEFAULT_CSP_CONFIG.reportOnly,
    reportUri: config.reportUri
  };
}

export function resolveSecurityHeadersConfig(
  config: SecurityHeadersConfig = {}
): SecurityHeadersConfig {
  return {
    contentTypeOptions: config.contentTypeOptions ?? DEFAULT_SECURITY_HEADERS.contentTypeOptions,
    frameOptions: config.frameOptions ?? DEFAULT_SECURITY_HEADERS.frameOptions,
    referrerPolicy: config.referrerPolicy ?? DEFAULT_SECURITY_HEADERS.referrerPolicy,
    xssProtection: config.xssProtection ?? DEFAULT_SECURITY_HEADERS.xssProtection,
    permissionsPolicy: config.permissionsPolicy ?? DEFAULT_SECURITY_HEADERS.permissionsPolicy
  };
}
