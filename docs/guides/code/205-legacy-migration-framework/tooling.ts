import { spawnSync } from "node:child_process";

import { type MigrationConfig, readString } from "./config.ts";

type CommandRunOptions = {
  env?: NodeJS.ProcessEnv;
};

function isBareCommand(commandText: string): boolean {
  return /^[A-Za-z0-9._/-]+$/.test(commandText.trim());
}

export function shellQuote(value: string): string {
  if (/^[A-Za-z0-9._/:=@+-]+$/.test(value)) {
    return value;
  }
  return `'${value.replace(/'/g, `'\\''`)}'`;
}

export function commandString(commandText: string, args: string[]): string {
  if (args.length === 0) {
    return commandText;
  }
  return `${commandText} ${args.map(shellQuote).join(" ")}`;
}

export function requireCommand(commandText: string): void {
  const trimmed = commandText.trim();
  const probe = isBareCommand(trimmed)
    ? trimmed
    : trimmed.split(/\s+/, 1)[0];
  const result = spawnSync("which", [probe], { stdio: "ignore" });
  if (result.status !== 0) {
    throw new Error(`${probe} is required in PATH`);
  }
}

export function spawnCommand(
  commandText: string,
  args: string[],
  options: CommandRunOptions = {},
) {
  const env = options.env ? { ...process.env, ...options.env } : process.env;
  if (isBareCommand(commandText)) {
    return spawnSync(commandText, args, {
      encoding: "utf-8",
      stdio: ["ignore", "pipe", "pipe"],
      env,
    });
  }

  return spawnSync("zsh", ["-lc", commandString(commandText, args)], {
    encoding: "utf-8",
    stdio: ["ignore", "pipe", "pipe"],
    env,
  });
}

export function runCommandText(
  commandText: string,
  args: string[],
  options: CommandRunOptions = {},
): string {
  const result = spawnCommand(commandText, args, options);

  if (result.stdout) process.stdout.write(result.stdout);
  if (result.stderr) process.stderr.write(result.stderr);

  if (result.status !== 0) {
    throw new Error(`command failed: ${commandString(commandText, args)}`);
  }

  return result.stdout ?? "";
}

export function underlayDevtoolsCommand(config: MigrationConfig): string {
  return readString(config, "UNDERLAY_DEVTOOLS_CMD", "underlay-devtools");
}
