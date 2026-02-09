import type { CspSource, ResolvedCspConfig } from "./csp-types";

export function buildCspHeader(config: ResolvedCspConfig, nonce?: string): string {
  const directives: string[] = [];

  const addDirective = (name: string, sources: CspSource[]) => {
    if (sources.length > 0) {
      directives.push(`${name} ${sources.join(" ")}`);
    }
  };

  addDirective("default-src", config.defaultSrc);

  const scriptSources = nonce
    ? [...config.scriptSrc, `'nonce-${nonce}'`]
    : config.scriptSrc;
  addDirective("script-src", scriptSources);

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

export function getCspHeaderName(config: ResolvedCspConfig): string {
  return config.reportOnly
    ? "Content-Security-Policy-Report-Only"
    : "Content-Security-Policy";
}
