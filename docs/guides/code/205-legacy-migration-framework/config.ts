import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

export type ConfigKey =
  | "SOURCE_SYSTEM"
  | "TARGET_SCHEMA_VERSION"
  | "MEDIA_DIR"
  | "BUNDLE_FILE"
  | "OCI_REF_TAG"
  | "BUNDLE_REF"
  | "OUTPUT_DIR"
  | "RUN_REPORT"
  | "GOVERNANCE_POLICY_FILE"
  | "REUSE_FROM_DIGEST_REF"
  | "APP_MIGRATION_RUNNER_CMD"
  | "DECISION_INDEX_FILE"
  | "DECISION_JOURNAL_FILE"
  | "DECISION_REUSE_SUMMARY_FILE"
  | "DECISION_REUSE_SUMMARY_SCHEMA_FILE"
  | "DECISION_REUSE_SUMMARY_LINT_FILE"
  | "REFRESH_DRIFT_GATE_FILE"
  | "REFRESH_DRIFT_GATE_SCHEMA_FILE"
  | "REFRESH_DRIFT_GATE_LINT_FILE"
  | "PROJECT_NAME"
  | "RUN_SCOPE"
  | "RUN_DATE_UTC"
  | "DOCTOR_REPORT"
  | "VERIFY_ARTIFACT_FILE"
  | "AUDIT_ARTIFACT_FILE"
  | "ARTIFACT_MANIFEST_FILE"
  | "PROMOTION_DECISION_FILE"
  | "PROMOTION_DECISION_SCHEMA_FILE"
  | "PROMOTION_DECISION_LINT_FILE"
  | "PROMOTION_RELEASE_NOTE_JSON_FILE"
  | "PROMOTION_RELEASE_NOTE_MD_FILE"
  | "PROMOTION_CI_GUARD_FILE"
  | "PROMOTION_CI_GUARD_SCHEMA_FILE"
  | "PROMOTION_CI_GUARD_LINT_FILE"
  | "DRIFT_MAX_UNRESOLVED"
  | "DRIFT_MAX_GOVERNANCE"
  | "DRIFT_MAX_LINEAGE";

export type MigrationConfig = Partial<Record<ConfigKey, string>>;

function readConfigFile(configFilePath: string): MigrationConfig {
  if (!existsSync(configFilePath)) {
    return {};
  }

  const raw = readFileSync(configFilePath, "utf-8");
  const parsed = JSON.parse(raw) as unknown;
  if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) {
    throw new Error(`config file must be a JSON object: ${configFilePath}`);
  }

  const result: MigrationConfig = {};
  for (const [key, value] of Object.entries(parsed)) {
    if (typeof value === "string") {
      result[key as ConfigKey] = value;
      continue;
    }
    if (value !== null && value !== undefined) {
      throw new Error(`config key ${key} must be a string`);
    }
  }

  return result;
}

export function loadConfig(): { filePath: string; values: MigrationConfig } {
  const requested = process.env.MIGRATION_CONFIG_FILE?.trim();
  const filePath = resolve(requested && requested.length > 0 ? requested : "./migration.config.json");
  const values = readConfigFile(filePath);
  return { filePath, values };
}

export function readString(
  config: MigrationConfig,
  key: ConfigKey,
  fallback?: string,
): string {
  const fileValue = config[key];
  if (fileValue && fileValue.trim().length > 0) {
    return fileValue;
  }

  const envValue = process.env[key];
  if (envValue && envValue.trim().length > 0) {
    return envValue;
  }

  if (fallback !== undefined) {
    return fallback;
  }

  throw new Error(`missing required configuration value: ${key}`);
}

export function readOptional(config: MigrationConfig, key: ConfigKey): string {
  const fileValue = config[key];
  if (fileValue && fileValue.trim().length > 0) {
    return fileValue;
  }
  const envValue = process.env[key];
  if (envValue && envValue.trim().length > 0) {
    return envValue;
  }
  return "";
}

export function validateDigestRef(value: string, key: ConfigKey): void {
  if (!/@sha256:[0-9a-f]{64}$/.test(value)) {
    throw new Error(`${key} must be digest-pinned (<repo>@sha256:<64 hex>)`);
  }
}
