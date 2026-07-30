import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

import { parse, type TomlTable, type TomlValue } from "smol-toml";

export const DEFAULT_CONFIG_DIR = "config";
export const DEFAULT_ENVIRONMENT = "dev";
export const DEFAULT_ENV_VAR = "ENVIRONMENT_NAME";

export type ConfigValue = TomlValue;
export type ConfigTable = TomlTable;

export interface LoadConfigStackOptions {
  projectRoot?: string;
  configDir?: string;
  environment?: string;
  environmentVar?: string;
  localOverlay?: string | false;
  envOverrides?: Record<string, ConfigValue | undefined>;
}

export function environmentName(options: LoadConfigStackOptions = {}): string {
  const raw =
    options.environment ??
    process.env[options.environmentVar ?? DEFAULT_ENV_VAR] ??
    DEFAULT_ENVIRONMENT;

  const trimmed = raw.trim();
  if (trimmed.length === 0) {
    return DEFAULT_ENVIRONMENT;
  }
  assertSafeConfigName(trimmed, "environment");
  return trimmed;
}

function assertSafeConfigName(name: string, kind: string): void {
  if (name === ".." || name.includes("/") || name.includes("\\") || name.includes("\0")) {
    throw new Error(`Config ${kind} name contains path separators or traversal: ${name}`);
  }
}

export function loadConfigStack(
  options: LoadConfigStackOptions = {},
): ConfigTable {
  const configDir =
    options.configDir ?? join(options.projectRoot ?? process.cwd(), DEFAULT_CONFIG_DIR);
  const environment = environmentName(options);
  const localOverlay = options.localOverlay === false ? null : (options.localOverlay ?? "local");
  const merged: ConfigTable = {};

  mergeConfigFile(merged, join(configDir, "default.toml"), true);
  mergeConfigFile(merged, join(configDir, `${environment}.toml`), false);

  if (localOverlay) {
    assertSafeConfigName(localOverlay, "localOverlay");
    mergeConfigFile(merged, join(configDir, `${localOverlay}.toml`), false);
  }

  for (const [key, value] of Object.entries(options.envOverrides ?? {})) {
    if (value !== undefined) {
      setDottedValue(merged, key, value);
    }
  }

  return merged;
}

export function readDottedValue<T = ConfigValue>(
  config: ConfigTable,
  dottedKey: string,
): T | undefined {
  let current: ConfigValue | undefined = config;

  for (const part of dottedKey.split(".")) {
    if (!isTable(current)) {
      return undefined;
    }

    current = current[part];
  }

  return current as T | undefined;
}

function mergeConfigFile(target: ConfigTable, path: string, required: boolean): void {
  if (!existsSync(path)) {
    if (required) {
      throw new Error(`Missing config file: ${path}`);
    }
    return;
  }

  const parsed = parse(readFileSync(path, "utf8"));
  if (!isTable(parsed)) {
    throw new Error(`Config file must parse to a TOML table: ${path}`);
  }

  mergeTables(target, parsed);
}

function mergeTables(target: ConfigTable, overlay: ConfigTable): void {
  for (const [key, value] of Object.entries(overlay)) {
    if (isUnsafeKey(key)) {
      continue;
    }
    const current = target[key];
    if (isTable(current) && isTable(value)) {
      mergeTables(current, value);
    } else {
      target[key] = value;
    }
  }
}

function setDottedValue(target: ConfigTable, dottedKey: string, value: ConfigValue): void {
  const parts = dottedKey.split(".");
  if (parts.some((part) => part.trim().length === 0)) {
    throw new Error(`Config override key is empty: ${dottedKey}`);
  }
  if (parts.some(isUnsafeKey)) {
    throw new Error(`Config override key contains a forbidden segment: ${dottedKey}`);
  }

  let current = target;
  for (const part of parts.slice(0, -1)) {
    const next = current[part];
    if (next === undefined) {
      const table: ConfigTable = {};
      current[part] = table;
      current = table;
    } else if (isTable(next)) {
      current = next;
    } else {
      throw new Error(`Config override cannot descend through non-table value: ${dottedKey}`);
    }
  }

  current[parts[parts.length - 1]!] = value;
}

function isUnsafeKey(key: string): boolean {
  return key === "__proto__" || key === "constructor" || key === "prototype";
}

function isTable(value: unknown): value is ConfigTable {
  return (
    value !== null &&
    typeof value === "object" &&
    !Array.isArray(value) &&
    !(value instanceof Date)
  );
}
