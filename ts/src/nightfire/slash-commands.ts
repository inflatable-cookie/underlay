import type { MarkdownEditorContext } from "./markup/markdown-editor-context";

export interface NightfireSlashCommandInput {
  type: string;
  id?: string;
  label?: string;
  description?: string;
  aliases?: string[];
  keywords?: string[];
}

export interface NightfireSlashCommand {
  id: string;
  type: string;
  label: string;
  description: string | null;
  aliases: string[];
  keywords: string[];
}

export interface NightfireSlashCommandsConfig {
  enabled?: boolean;
  includeDefaults?: boolean;
  commands?: NightfireSlashCommandInput[];
}

export interface NightfireSlashMatch {
  start: number;
  end: number;
  query: string;
}

interface BlockTypeOptionLike {
  type: string;
  label: string;
}

function dedupe(values: string[]): string[] {
  return Array.from(new Set(values.map((value) => value.trim()).filter((value) => value.length > 0)));
}

function normaliseSearchValue(value: string): string {
  return value.toLowerCase().trim();
}

function buildDefaultCommand(option: BlockTypeOptionLike): NightfireSlashCommand {
  return {
    id: `insert-${option.type}`,
    type: option.type,
    label: option.label,
    description: `Insert a ${option.label.toLowerCase()} block.`,
    aliases: [option.type],
    keywords: []
  };
}

export function buildNightfireSlashCommands(
  typeOptions: BlockTypeOptionLike[],
  config: NightfireSlashCommandsConfig | null | undefined
): NightfireSlashCommand[] {
  const allowedTypes = new Map(typeOptions.map((option) => [option.type, option.label]));
  const commands = new Map<string, NightfireSlashCommand>();

  if (config?.includeDefaults !== false) {
    for (const option of typeOptions) {
      commands.set(option.type, buildDefaultCommand(option));
    }
  }

  for (const input of config?.commands ?? []) {
    const defaultLabel = allowedTypes.get(input.type);
    if (!defaultLabel) {
      continue;
    }

    const existing = commands.get(input.type) ?? buildDefaultCommand({
      type: input.type,
      label: defaultLabel
    });

    commands.set(input.type, {
      id: input.id ?? existing.id,
      type: input.type,
      label: input.label ?? existing.label,
      description: input.description ?? existing.description,
      aliases: dedupe([...(existing.aliases ?? []), ...(input.aliases ?? [])]),
      keywords: dedupe([...(existing.keywords ?? []), ...(input.keywords ?? [])])
    });
  }

  return Array.from(commands.values()).sort((left, right) => left.label.localeCompare(right.label));
}

export function filterNightfireSlashCommands(
  commands: NightfireSlashCommand[],
  query: string
): NightfireSlashCommand[] {
  const normalisedQuery = normaliseSearchValue(query);
  if (normalisedQuery.length === 0) {
    return commands;
  }

  return commands.filter((command) => {
    const haystack = dedupe([
      command.label,
      command.type,
      command.description ?? "",
      ...command.aliases,
      ...command.keywords
    ]).map(normaliseSearchValue);

    return haystack.some((value) => value.includes(normalisedQuery));
  });
}

export function findNightfireSlashMatch(
  context: MarkdownEditorContext
): NightfireSlashMatch | null {
  if (context.selectionStart !== context.selectionEnd) {
    return null;
  }

  const beforeCursor = context.value.slice(0, context.selectionStart);
  const lineStart = beforeCursor.lastIndexOf("\n") + 1;
  const linePrefix = beforeCursor.slice(lineStart);
  const match = /(?:^|\s)\/([^\s/]*)$/.exec(linePrefix);

  if (!match) {
    return null;
  }

  const token = match[0];
  const query = match[1] ?? "";
  const offset = token.startsWith("/") ? 0 : 1;
  const start = context.selectionStart - token.length + offset;

  return {
    start,
    end: context.selectionStart,
    query
  };
}

export function removeNightfireSlashText(
  value: string,
  match: Pick<NightfireSlashMatch, "start" | "end">
): string {
  return `${value.slice(0, match.start)}${value.slice(match.end)}`;
}
