export type CspSource = string;

export interface CspConfig {
  defaultSrc?: CspSource[] | false;
  scriptSrc?: CspSource[] | false;
  styleSrc?: CspSource[] | false;
  imgSrc?: CspSource[] | false;
  fontSrc?: CspSource[] | false;
  connectSrc?: CspSource[] | false;
  frameSrc?: CspSource[] | false;
  mediaSrc?: CspSource[] | false;
  objectSrc?: CspSource[] | false;
  formAction?: CspSource[] | false;
  baseUri?: CspSource[] | false;
  frameAncestors?: CspSource[] | false;
  reportOnly?: boolean;
  reportUri?: string;
}

export interface ResolvedCspConfig {
  defaultSrc: CspSource[];
  scriptSrc: CspSource[];
  styleSrc: CspSource[];
  imgSrc: CspSource[];
  fontSrc: CspSource[];
  connectSrc: CspSource[];
  frameSrc: CspSource[];
  mediaSrc: CspSource[];
  objectSrc: CspSource[];
  formAction: CspSource[];
  baseUri: CspSource[];
  frameAncestors: CspSource[];
  reportOnly: boolean;
  reportUri?: string;
}

export interface SecurityHeadersConfig {
  contentTypeOptions?: string | false;
  frameOptions?: string | false;
  referrerPolicy?: string | false;
  xssProtection?: string | false;
  permissionsPolicy?: string | false;
}

export interface CspHandleOptions {
  csp: ResolvedCspConfig;
  securityHeaders?: SecurityHeadersConfig;
}
