import type { NavigationContextConfig } from "./navigation-types";

const DEFAULT_STORAGE_KEY = "underlay:nav-context";
const DEFAULT_MAX_DEPTH = 3;

let config: Required<NavigationContextConfig> = {
  storageKey: DEFAULT_STORAGE_KEY,
  maxDepth: DEFAULT_MAX_DEPTH
};

export function setNavigationContextConfig(options: NavigationContextConfig): void {
  config = {
    storageKey: options.storageKey ?? DEFAULT_STORAGE_KEY,
    maxDepth: options.maxDepth ?? DEFAULT_MAX_DEPTH
  };
}

export function getNavigationContextConfig(): Required<NavigationContextConfig> {
  return config;
}
