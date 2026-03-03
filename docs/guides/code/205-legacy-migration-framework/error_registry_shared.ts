import { readFileSync } from "node:fs";

export type ErrorCodeCategory = "cli" | "config" | "evidence" | "refresh" | "promotion";

export type RegistryEntry = {
  code: string;
  category: ErrorCodeCategory | string;
  scripts: string[];
  meaning: string;
  remediation: string;
};

export type Registry = {
  schema: string;
  schema_version: number;
  generated_at?: string;
  codes: RegistryEntry[];
};

export const DEFAULT_ERROR_CODE_SCRIPTS = [
  "00_config_lint.ts",
  "00_doctor.ts",
  "00_preflight.ts",
  "04_evidence_manifest.ts",
  "05_evidence_verify.ts",
  "06_promotion_check.ts",
  "07_promotion_decision_lint.ts",
  "09_promotion_ci_guard.ts",
  "11_promotion_ci_guard_lint.ts",
  "12_decision_reuse_summary_lint.ts",
  "14_refresh_drift_gate_lint.ts",
] as const;

export function collectCodesFromScript(scriptPath: string): Set<string> {
  const content = readFileSync(scriptPath, "utf-8");
  const codes = new Set<string>();
  const pattern = /(?:fail|withCode)\(\s*"([A-Z0-9_]+)"/g;
  let match: RegExpExecArray | null = pattern.exec(content);
  while (match) {
    const code = match[1];
    if (code.startsWith("MIG_")) {
      codes.add(code);
    }
    match = pattern.exec(content);
  }
  return codes;
}

export function inferCategory(code: string): ErrorCodeCategory {
  if (code.startsWith("MIG_CLI_")) return "cli";
  if (code.startsWith("MIG_CFG_")) return "config";
  if (code.startsWith("MIG_EVID_")) return "evidence";
  if (code.startsWith("MIG_REFRESH_")) return "refresh";
  return "promotion";
}
