import { getLineNumberFromIndex, getLineText } from "./line-utils.js";

function hasSuppression(lineText: string, directive: "line" | "next-line", ruleIds: string[]): boolean {
  const pattern = new RegExp(`\\bguardrails-disable-${directive}\\b([^\\n]*)`);
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
  ruleIds: string[]
): boolean {
  const lineNumber = getLineNumberFromIndex(lineStarts, index);

  const currentLine = getLineText(text, lineStarts, lineNumber);
  if (hasSuppression(currentLine, "line", ruleIds)) return true;

  const prevLine = lineNumber > 1 ? getLineText(text, lineStarts, lineNumber - 1) : "";
  if (hasSuppression(prevLine, "next-line", ruleIds)) return true;

  return false;
}
