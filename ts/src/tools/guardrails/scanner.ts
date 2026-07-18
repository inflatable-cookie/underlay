interface ModuleScopeCheck {
  name: string;
  kind: "prefix" | "identifier" | "call";
  value: string;
  message: string;
}

interface Issue {
  index: number;
  name: string;
  message: string;
}

function isIdentChar(char: string): boolean {
  return /[a-zA-Z0-9_$]/.test(char);
}

function hasIdentifierAt(text: string, index: number, identifier: string): boolean {
  if (!text.startsWith(identifier, index)) return false;

  const before = index === 0 ? "" : text[index - 1];
  const after = text[index + identifier.length] ?? "";

  if (before && isIdentChar(before)) return false;
  if (after && isIdentChar(after)) return false;

  return true;
}

function hasCallAt(text: string, index: number, identifier: string): boolean {
  if (!hasIdentifierAt(text, index, identifier)) return false;

  // Skip declarations: `function foo(`, `foo(` in a `function foo(` position.
  // Look back past whitespace for the `function` keyword so we don't flag the
  // definition of a guarded helper as a module-scope call of it.
  let b = index - 1;
  while (b >= 0 && /\s/.test(text[b])) b--;
  let wordEnd = b;
  while (b >= 0 && isIdentChar(text[b])) b--;
  const precedingWord = text.slice(b + 1, wordEnd + 1);
  if (precedingWord === "function") return false;

  let i = index + identifier.length;
  while (i < text.length && /\s/.test(text[i])) i++;

  return text[i] === "(";
}

function isGuardedContext(text: string, boundaryIndex: number, matchIndex: number): boolean {
  const windowStart = Math.max(boundaryIndex, matchIndex - 140);
  const context = text.slice(windowStart, matchIndex);

  // Guardrails are intentionally strict: `if (browser)` is not considered a safe
  // module-scope guard; prefer explicit `typeof` checks or `onMount()`.
  return /typeof\s+(window|document|navigator|location|history)\b/.test(context);
}

function isFunctionBodyStart(text: string, braceIndex: number): boolean {
  let j = braceIndex - 1;
  while (j >= 0 && /\s/.test(text[j])) j--;

  // Arrow function body: `(...) => {`
  if (text[j] === ">") {
    let k = j - 1;
    while (k >= 0 && /\s/.test(text[k])) k--;
    if (text[k] === "=") return true;
  }

  // Function declaration/expression: `function ... {`
  const prefixStart = Math.max(0, braceIndex - 120);
  const prefix = text.slice(prefixStart, braceIndex);
  if (/\bfunction\b/.test(prefix)) return true;

  // Method body: `method(...) {` (but not `if (...) {`, `for (...) {`, etc.)
  if (text[j] === ")") {
    let depth = 0;
    for (let p = j; p >= 0; p--) {
      const c = text[p];
      if (c === ")") depth++;
      if (c === "(") {
        depth--;
        if (depth === 0) {
          let q = p - 1;
          while (q >= 0 && /\s/.test(text[q])) q--;

          let start = q;
          while (start >= 0 && isIdentChar(text[start])) start--;
          const word = text.slice(start + 1, q + 1);

          if (["if", "for", "while", "switch", "catch", "with"].includes(word)) {
            return false;
          }

          return word.length > 0;
        }
      }
    }
  }

  return false;
}

export function scanModuleScopeBrowserApis(
  text: string,
  baseIndex: number,
  checks: ModuleScopeCheck[]
): Issue[] {
  const issues: Issue[] = [];

  /**
   * Modes:
   * - code
   * - line_comment
   * - block_comment
   * - single_quote
   * - double_quote
   * - template
   * - template_expr
   */
  let mode:
    | "code"
    | "line_comment"
    | "block_comment"
    | "single_quote"
    | "double_quote"
    | "template"
    | "template_expr" = "code";

  const braceStack: boolean[] = [];
  let functionDepth = 0;

  let boundaryIndex = 0;
  let templateExprBraceBalance = 0;

  for (let i = 0; i < text.length; i++) {
    const char = text[i];
    const next = text[i + 1] ?? "";

    if (char === "\n") {
      if (mode === "line_comment") mode = "code";
      if (functionDepth === 0) boundaryIndex = i + 1;
      continue;
    }

    if (mode === "line_comment") continue;

    if (mode === "block_comment") {
      if (char === "*" && next === "/") {
        mode = "code";
        i++;
      }
      continue;
    }

    if (mode === "single_quote") {
      if (char === "\\") {
        i++;
        continue;
      }

      if (char === "'") mode = "code";
      continue;
    }

    if (mode === "double_quote") {
      if (char === "\\") {
        i++;
        continue;
      }

      if (char === "\"") mode = "code";
      continue;
    }

    if (mode === "template") {
      if (char === "\\") {
        i++;
        continue;
      }

      if (char === "`") {
        mode = "code";
        continue;
      }

      if (char === "$" && next === "{") {
        mode = "template_expr";
        templateExprBraceBalance = 0;
        i++;
      }

      continue;
    }

    // mode: code | template_expr
    if (char === "/" && next === "/") {
      mode = "line_comment";
      i++;
      continue;
    }

    if (char === "/" && next === "*") {
      mode = "block_comment";
      i++;
      continue;
    }

    if (char === "'") {
      mode = "single_quote";
      continue;
    }

    if (char === "\"") {
      mode = "double_quote";
      continue;
    }

    if (char === "`") {
      mode = "template";
      continue;
    }

    if (char === ";" && functionDepth === 0) {
      boundaryIndex = i + 1;
    }

    if (functionDepth === 0) {
      for (const check of checks) {
        let matched = false;

        if (check.kind === "prefix") {
          matched = text.startsWith(check.value, i);
        } else if (check.kind === "identifier") {
          matched = hasIdentifierAt(text, i, check.value);
        } else if (check.kind === "call") {
          matched = hasCallAt(text, i, check.value);
        }

        if (matched && !isGuardedContext(text, boundaryIndex, i)) {
          issues.push({
            index: baseIndex + i,
            name: check.name,
            message: check.message
          });
          break;
        }
      }
    }

    if (mode === "template_expr") {
      if (char === "{") {
        templateExprBraceBalance++;
        const isFunction = isFunctionBodyStart(text, i);
        braceStack.push(isFunction);
        if (isFunction) functionDepth++;
      } else if (char === "}") {
        if (templateExprBraceBalance === 0) {
          mode = "template";
        } else {
          templateExprBraceBalance--;
          const wasFunction = braceStack.pop();
          if (wasFunction) functionDepth = Math.max(0, functionDepth - 1);
        }
      }

      continue;
    }

    if (char === "{") {
      const isFunction = isFunctionBodyStart(text, i);
      braceStack.push(isFunction);
      if (isFunction) functionDepth++;
    } else if (char === "}") {
      const wasFunction = braceStack.pop();
      if (wasFunction) functionDepth = Math.max(0, functionDepth - 1);
    }
  }

  return issues;
}
