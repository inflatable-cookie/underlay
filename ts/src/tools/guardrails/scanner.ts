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

/**
 * Prefix checks look for `window.`, `document.` and friends. Require the
 * preceding character to be a value boundary so member access on another
 * object (`obj.window.foo`, `el.location.href`) is not reported as a browser
 * global.
 */
function hasPrefixAt(text: string, index: number, prefix: string): boolean {
	if (!text.startsWith(prefix, index)) return false;

	const before = index === 0 ? "" : text[index - 1];
	if (before && (isIdentChar(before) || before === ".")) return false;

	return true;
}

function isGuardedContext(text: string, boundaryIndex: number, matchIndex: number): boolean {
	const windowStart = Math.max(boundaryIndex, matchIndex - 140);
	const context = text.slice(windowStart, matchIndex);

	// Guardrails are intentionally strict: `if (browser)` is not considered a safe
	// module-scope guard; prefer explicit `typeof` checks or `onMount()`.
	return (
		/\btypeof\s+$/.test(context) ||
		/typeof\s+(window|document|navigator|location|history|localStorage|sessionStorage)\b/.test(
			context,
		)
	);
}

const CONTROL_KEYWORDS = new Set(["if", "for", "while", "switch", "catch", "with"]);
const REGEX_PREFIX_KEYWORDS = new Set([
	"await",
	"case",
	"delete",
	"do",
	"else",
	"in",
	"instanceof",
	"new",
	"return",
	"throw",
	"typeof",
	"void",
	"yield",
]);

type Parenthesis =
	"control" | "for" | "function" | "function-declaration" | "method" | "expression";
type Brace = "block" | "function" | "function-declaration" | "object" | "interpolation" | "type";
const RETURN_TYPE_OBJECT_PREFIXES = new Set([":", "|", "&", "<", "(", "[", ",", "=>", "?"]);

function regexLiteralEnd(text: string, start: number): number | undefined {
	let inClass = false;
	for (let i = start + 1; i < text.length; i++) {
		const char = text[i];
		if (char === "\n" || char === "\r") return undefined;
		if (char === "\\") {
			const escaped = text[++i];
			if (escaped === "\n" || escaped === "\r") return undefined;
		} else if (char === "[") {
			inClass = true;
		} else if (char === "]") {
			inClass = false;
		} else if (char === "/" && !inClass) {
			while (/[a-z]/i.test(text[i + 1] ?? "")) i++;
			return i;
		}
	}
	return undefined;
}

export function scanModuleScopeBrowserApis(
	text: string,
	baseIndex: number,
	checks: ModuleScopeCheck[],
): Issue[] {
	const issues: Issue[] = [];
	const braces: Brace[] = [];
	const parentheses: Parenthesis[] = [];
	let functionDepth = 0;
	let boundaryIndex = 0;
	let mode:
		"code" | "line_comment" | "block_comment" | "single_quote" | "double_quote" | "template" =
		"code";
	let previous = "";
	let beforePrevious = "";
	let canStartRegex = true;
	let closedParen: Parenthesis | undefined;
	let functionHead: "function" | "function-declaration" | undefined;
	let functionTypeParameters = 0;
	let functionReturnType: typeof functionHead;
	let returnTypeAngleDepth = 0;
	let returnTypeBraceDepth = 0;
	let returnTypeParenDepth = 0;
	let returnTypeBracketDepth = 0;

	function resetReturnType(): void {
		functionReturnType = undefined;
		returnTypeAngleDepth = 0;
		returnTypeBraceDepth = 0;
		returnTypeParenDepth = 0;
		returnTypeBracketDepth = 0;
	}

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

		if (mode === "single_quote" || mode === "double_quote" || mode === "template") {
			if (char === "\\") {
				i++;
				continue;
			}
			if (mode === "template" && char === "$" && next === "{") {
				// The interpolation marker stays under any braces opened by its
				// expression, including interpolations in nested templates.
				braces.push("interpolation");
				mode = "code";
				previous = "${";
				canStartRegex = true;
				i++;
			} else if (char === (mode === "single_quote" ? "'" : mode === "double_quote" ? '"' : "`")) {
				mode = "code";
				previous = "literal";
				canStartRegex = false;
			}
			continue;
		}

		if (/\s/.test(char)) continue;
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
		if (char === "/" && canStartRegex) {
			const end = regexLiteralEnd(text, i);
			if (end !== undefined) {
				i = end;
				previous = "literal";
				canStartRegex = false;
				continue;
			}
		}
		if (char === "'" || char === '"' || char === "`") {
			mode = char === "'" ? "single_quote" : char === '"' ? "double_quote" : "template";
			continue;
		}

		if (char === ";" && functionDepth === 0 && !functionReturnType) boundaryIndex = i + 1;

		if (functionDepth === 0 && !functionReturnType) {
			for (const check of checks) {
				let matched = false;

				if (check.kind === "prefix") {
					matched = hasPrefixAt(text, i, check.value);
				} else if (check.kind === "identifier") {
					matched = hasIdentifierAt(text, i, check.value);
				} else if (check.kind === "call") {
					matched = hasCallAt(text, i, check.value);
				}

				if (matched && !isGuardedContext(text, boundaryIndex, i)) {
					issues.push({
						index: baseIndex + i,
						name: check.name,
						message: check.message,
					});
					break;
				}
			}
		}

		if (isIdentChar(char)) {
			let end = i + 1;
			while (end < text.length && isIdentChar(text[end])) end++;
			const word = text.slice(i, end);
			if (word === "function" && previous !== "." && functionTypeParameters === 0) {
				const beforeFunction = previous === "async" ? beforePrevious : previous;
				functionHead =
					!beforeFunction || ["{", "}", ";", "export", "default"].includes(beforeFunction)
						? "function-declaration"
						: "function";
			} else if (functionTypeParameters === 0 && previous !== "function" && previous !== "*") {
				functionHead = undefined;
			}
			canStartRegex =
				previous !== "." &&
				(REGEX_PREFIX_KEYWORDS.has(word) || (word === "of" && parentheses.at(-1) === "for"));
			beforePrevious = previous;
			previous = word;
			i = end - 1;
			continue;
		}

		if (functionHead && char === "<") functionTypeParameters++;
		else if (functionTypeParameters > 0 && char === ">") functionTypeParameters--;
		else if (functionTypeParameters === 0 && char !== "(" && char !== "*") functionHead = undefined;

		if (functionReturnType) {
			if (char === "<") returnTypeAngleDepth++;
			else if (char === ">" && returnTypeAngleDepth > 0) returnTypeAngleDepth--;
			else if (char === "[") returnTypeBracketDepth++;
			else if (char === "]" && returnTypeBracketDepth > 0) returnTypeBracketDepth--;
		}

		switch (char) {
			case "(":
				if (functionReturnType) returnTypeParenDepth++;
				if (functionHead && functionTypeParameters === 0) {
					parentheses.push(functionHead);
					functionHead = undefined;
				} else if (previous === "for" || (previous === "await" && beforePrevious === "for")) {
					parentheses.push("for");
				} else if (CONTROL_KEYWORDS.has(previous)) {
					parentheses.push("control");
				} else {
					parentheses.push(
						isIdentChar(previous[0] ?? "") && !canStartRegex ? "method" : "expression",
					);
				}
				canStartRegex = true;
				break;
			case ")":
				if (functionReturnType && returnTypeParenDepth > 0) returnTypeParenDepth--;
				closedParen = parentheses.pop();
				canStartRegex = closedParen === "control" || closedParen === "for";
				break;
			case ":":
				if (
					!functionReturnType &&
					previous === ")" &&
					(closedParen === "function" || closedParen === "function-declaration")
				) {
					functionReturnType = closedParen;
				}
				canStartRegex = true;
				break;
			case "{": {
				const inReturnTypeObject =
					functionReturnType !== undefined &&
					(returnTypeAngleDepth > 0 ||
						returnTypeBraceDepth > 0 ||
						returnTypeParenDepth > 0 ||
						returnTypeBracketDepth > 0 ||
						RETURN_TYPE_OBJECT_PREFIXES.has(previous));
				if (inReturnTypeObject) {
					braces.push("type");
					returnTypeBraceDepth++;
					canStartRegex = true;
					break;
				}
				const functionBody = functionReturnType ?? (previous === ")" ? closedParen : undefined);
				const isFunction =
					previous === "=>" ||
					functionBody === "function" ||
					functionBody === "function-declaration" ||
					functionBody === "method";
				const isBlock =
					previous === "" ||
					previous === ";" ||
					previous === "else" ||
					previous === "try" ||
					previous === "finally" ||
					previous === "do" ||
					previous === "catch" ||
					(previous === ")" && (closedParen === "control" || closedParen === "for"));
				braces.push(
					functionBody === "function-declaration"
						? functionBody
						: isFunction
							? "function"
							: isBlock
								? "block"
								: "object",
				);
				if (isFunction) functionDepth++;
				resetReturnType();
				canStartRegex = true;
				break;
			}
			case "}": {
				const closed = braces.pop();
				if (closed === "type" && returnTypeBraceDepth > 0) returnTypeBraceDepth--;
				if (closed === "function" || closed === "function-declaration") functionDepth--;
				if (closed === "interpolation") mode = "template";
				canStartRegex = closed === "block" || closed === "function-declaration";
				break;
			}
			case "=":
				if (next === ">") {
					previous = "=>";
					canStartRegex = true;
					i++;
					continue;
				}
				canStartRegex = true;
				break;
			case "+":
			case "-":
				if (next === char) i++;
				else canStartRegex = true;
				break;
			case ";":
				if (
					functionReturnType &&
					returnTypeBraceDepth === 0 &&
					returnTypeAngleDepth === 0 &&
					returnTypeParenDepth === 0 &&
					returnTypeBracketDepth === 0
				) {
					resetReturnType();
				}
				canStartRegex = true;
				break;
			default:
				canStartRegex = char !== "]" && char !== ".";
		}
		beforePrevious = previous;
		previous = char;
	}

	return issues;
}
