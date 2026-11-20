import { getLineNumberFromIndex, getLineText } from "./line-utils.js";

function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function hasSuppression(
  lineText: string,
  directive: "line" | "next-line",
  ruleIds: string[],
  suppressionPrefix: string
): boolean {
  if (!suppressionPrefix) return false;

  const escapedPrefix = escapeRegExp(suppressionPrefix);
  const pattern = new RegExp(`(?<![\\w-])${escapedPrefix}-${directive}\\b([^\\n]*)`);
  const match = lineText.match(pattern);
  if (!match) return false;

  const raw = (match[1] ?? "").trim();

  // If no rule is specified, treat it as disabling everything for that line.
  if (!raw) return true;

  const tokens = raw.split(/[\s,]+/).filter(Boolean);

  for (const token of tokens) {
    if (token === "all") return true;
    if (ruleIds.includes(token)) return true;
  }

  return false;
}

export function isSuppressed(
  text: string,
  lineStarts: number[],
  index: number,
  ruleIds: string[],
  suppressionPrefix = "guardrails-disable"
): boolean {
  const lineNumber = getLineNumberFromIndex(lineStarts, index);

  const currentLine = getLineText(text, lineStarts, lineNumber);
  if (hasSuppression(currentLine, "line", ruleIds, suppressionPrefix)) return true;

  const prevLine = lineNumber > 1 ? getLineText(text, lineStarts, lineNumber - 1) : "";
  if (hasSuppression(prevLine, "next-line", ruleIds, suppressionPrefix)) return true;

  return false;
}
