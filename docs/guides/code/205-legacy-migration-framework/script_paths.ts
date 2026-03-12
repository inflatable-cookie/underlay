import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const FRAMEWORK_DIR = dirname(fileURLToPath(import.meta.url));

export function frameworkDir(): string {
  return FRAMEWORK_DIR;
}

export function frameworkPath(...segments: string[]): string {
  return resolve(FRAMEWORK_DIR, ...segments);
}

export function frameworkScriptPath(scriptName: string): string {
  return frameworkPath(scriptName);
}
