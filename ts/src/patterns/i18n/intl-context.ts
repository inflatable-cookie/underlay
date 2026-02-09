export interface I18nFormatConfig {
  locale?: string;
  timezone?: string;
}

let globalConfig: I18nFormatConfig = {};

export function mergeI18nConfig(config: I18nFormatConfig): void {
  globalConfig = { ...globalConfig, ...config };
}

export function resolveLocale(override?: string): string {
  return override ?? globalConfig.locale ?? (typeof navigator !== "undefined" ? navigator.language : "en-GB");
}

export function resolveTimezone(override?: string): string | undefined {
  return override ?? globalConfig.timezone;
}
