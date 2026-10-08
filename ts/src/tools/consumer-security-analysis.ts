import { spawnSync } from "node:child_process";
import { readFileSync, statSync } from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

type TokenKind = "identifier" | "string" | "punctuation" | "comment";

interface Token {
	kind: TokenKind;
	text: string;
	value: string;
	start: number;
	end: number;
}

interface FunctionBody {
	name: string;
	start: number;
	end: number;
	parameters: string[];
	publicAsync: boolean;
}

interface RustModule {
	path: string;
	source: string;
	tokens: Token[];
	braces: Map<number, number>;
	parens: Map<number, number>;
	brackets: Map<number, number>;
	functions: FunctionBody[];
}

interface Finding {
	path: string;
	line: number;
	message: string;
}

function rustFiles(root: string): string[] {
	const result = spawnSync("rg", ["--files", "--type", "rust", "-g", "!target", "--null"], {
		cwd: root,
		encoding: "utf8",
		maxBuffer: 32 * 1024 * 1024,
	});

	if (result.error) throw result.error;
	if (result.status === 1) return [];
	if (result.status !== 0) {
		throw new Error(result.stderr.trim() || `rg exited with status ${result.status}`);
	}

	return result.stdout.split("\0").filter(Boolean);
}

function stringValue(source: string, start: number, end: number): string {
	return source
		.slice(start, end)
		.replace(/\\([\\"'nrt0])/g, (_match, escaped: string) => {
			const replacements: Record<string, string> = {
				"\\": "\\",
				'"': '"',
				"'": "'",
				n: "\n",
				r: "\r",
				t: "\t",
				0: "\0",
			};
			return replacements[escaped] ?? escaped;
		});
}

function tokenizeRust(source: string): Token[] {
	const tokens: Token[] = [];
	let index = 0;

	while (index < source.length) {
		const char = source[index]!;
		if (/\s/.test(char)) {
			index += 1;
			continue;
		}

		if (source.startsWith("//", index)) {
			const end = source.indexOf("\n", index);
			const final = end === -1 ? source.length : end;
			tokens.push({
				kind: "comment",
				text: source.slice(index, final),
				value: source.slice(index, final),
				start: index,
				end: final,
			});
			index = final;
			continue;
		}

		if (source.startsWith("/*", index)) {
			const start = index;
			let depth = 1;
			index += 2;
			while (index < source.length && depth > 0) {
				if (source.startsWith("/*", index)) {
					depth += 1;
					index += 2;
				} else if (source.startsWith("*/", index)) {
					depth -= 1;
					index += 2;
				} else {
					index += 1;
				}
			}
			const text = source.slice(start, index);
			tokens.push({ kind: "comment", text, value: text, start, end: index });
			continue;
		}

		const rawPrefix = /^(?:br|r)(#*)"/.exec(source.slice(index));
		if (rawPrefix) {
			const start = index;
			const hashes = rawPrefix[1]!;
			const contentStart = index + rawPrefix[0].length;
			const closing = `"${hashes}`;
			const closingIndex = source.indexOf(closing, contentStart);
			const contentEnd = closingIndex === -1 ? source.length : closingIndex;
			const end = closingIndex === -1 ? source.length : closingIndex + closing.length;
			const text = source.slice(start, end);
			tokens.push({
				kind: "string",
				text,
				value: source.slice(contentStart, contentEnd),
				start,
				end,
			});
			index = end;
			continue;
		}

		if ((char === "b" && source[index + 1] === '"') || char === '"') {
			const start = index;
			const quote = char === "b" ? index + 1 : index;
			index = quote + 1;
			const contentStart = index;
			while (index < source.length) {
				if (source[index] === "\\") {
					index += 2;
					continue;
				}
				if (source[index] === '"') break;
				index += 1;
			}
			const contentEnd = index;
			if (source[index] === '"') index += 1;
			const text = source.slice(start, index);
			tokens.push({
				kind: "string",
				text,
				value: stringValue(source, contentStart, contentEnd),
				start,
				end: index,
			});
			continue;
		}

		if (/[A-Za-z_]/.test(char)) {
			const start = index;
			index += 1;
			while (index < source.length && /[A-Za-z0-9_]/.test(source[index]!)) index += 1;
			const text = source.slice(start, index);
			tokens.push({ kind: "identifier", text, value: text, start, end: index });
			continue;
		}

		const start = index;
		const operator = ["::", "=>", "&&", "||", "==", "!=", "<=", ">=", "->", ".."]
			.find((candidate) => source.startsWith(candidate, index));
		if (operator) index += operator.length;
		else index += 1;
		const text = source.slice(start, index);
		tokens.push({ kind: "punctuation", text, value: text, start, end: index });
	}

	return tokens;
}

function codeTokens(tokens: Token[]): Token[] {
	return tokens.filter((token) => token.kind !== "comment");
}

function pairTokens(tokens: Token[], open: string, close: string): Map<number, number> {
	const stack: number[] = [];
	const pairs = new Map<number, number>();
	for (let index = 0; index < tokens.length; index += 1) {
		if (tokens[index]!.value === open) stack.push(index);
		if (tokens[index]!.value === close) {
			const start = stack.pop();
			if (start !== undefined) {
				pairs.set(start, index);
				pairs.set(index, start);
			}
		}
	}
	return pairs;
}

function splitTopLevel(
	tokens: Token[],
	start: number,
	end: number,
	braces: Map<number, number>,
	parens: Map<number, number>,
	brackets: Map<number, number>,
): Token[][] {
	const parts: Token[][] = [];
	let partStart = start;
	let angleDepth = 0;
	for (let index = start; index < end; index += 1) {
		const value = tokens[index]!.value;
		if (value === "(") {
			const closing = parens.get(index);
			if (closing !== undefined && closing < end) index = closing;
			continue;
		}
		if (value === "[") {
			const closing = brackets.get(index);
			if (closing !== undefined && closing < end) index = closing;
			continue;
		}
		if (value === "{") {
			const closing = braces.get(index);
			if (closing !== undefined && closing < end) index = closing;
			continue;
		}
		if (value === "<") angleDepth += 1;
		if (value === ">" && angleDepth > 0) angleDepth -= 1;
		if (value === "," && angleDepth === 0) {
			parts.push(tokens.slice(partStart, index));
			partStart = index + 1;
		}
	}
	if (partStart < end) parts.push(tokens.slice(partStart, end));
	return parts;
}

function lineAt(source: string, offset: number): number {
	let line = 1;
	for (let index = 0; index < offset; index += 1) {
		if (source[index] === "\n") line += 1;
	}
	return line;
}

type GateKind = "environment" | "docs";

const DOCS_GATE_NAMES = new Set([
		"include_docs",
		"include_api_docs",
		"with_docs",
		"show_docs",
		"enable_docs",
		"docs_enabled",
		"openapi_enabled",
		"swagger_enabled",
	]);

const ENVIRONMENT_GATE_NAMES = new Set(["is_development", "is_local", "is_dev"]);

function gateKindAtomic(condition: Token[], negated: boolean): GateKind | undefined {
	let environmentPositive = false;
	let environmentNegative = false;
	let docsPositive = false;
	let docsNegative = false;

	for (let index = 0; index < condition.length; index += 1) {
		const token = condition[index]!;
		let kind: GateKind | undefined;
		let environmentVariant = false;
		if (token.kind === "identifier" && ENVIRONMENT_GATE_NAMES.has(token.value)) kind = "environment";
		if (token.kind === "identifier" && DOCS_GATE_NAMES.has(token.value)) kind = "docs";
		if (
			token.value === "Local" ||
			token.value === "Development" ||
			token.value === "Dev"
		) {
			if (
				condition[index - 1]?.value === "::" &&
				condition[index - 2]?.value === "Environment"
			) {
				kind = "environment";
				environmentVariant = true;
			}
		}
		if (!kind) continue;

		let atomStart = index;
		while (
			atomStart > 0 &&
			condition[atomStart - 1]!.value !== "&&" &&
			condition[atomStart - 1]!.value !== "||"
		) {
			atomStart -= 1;
		}
		let atomEnd = index + 1;
		while (
			atomEnd < condition.length &&
			condition[atomEnd]!.value !== "&&" &&
			condition[atomEnd]!.value !== "||"
		) {
			atomEnd += 1;
		}
		const atom = condition.slice(atomStart, atomEnd);
		const firstExpressionToken = atom.find((part) => part.value !== "(");
		const expressionNegated =
			firstExpressionToken?.value === "!" ||
			(environmentVariant && atom.some((part) => part.value === "!="));
		if (kind === "environment" && expressionNegated) environmentNegative = true;
		if (kind === "environment" && !expressionNegated) environmentPositive = true;
		if (kind === "docs" && expressionNegated) docsNegative = true;
		if (kind === "docs" && !expressionNegated) docsPositive = true;
	}

	if (negated) {
		if (environmentPositive || docsPositive) return undefined;
		if (environmentNegative) return "environment";
		if (docsNegative) return "docs";
		return undefined;
	}
	if (environmentPositive && !environmentNegative) return "environment";
	if (environmentNegative) return undefined;
	if (docsPositive && !docsNegative) return "docs";
	return undefined;
}

function splitCondition(condition: Token[], operator: "&&" | "||"): Token[][] {
	const parts: Token[][] = [];
	let start = 0;
	for (let index = 0; index < condition.length; index += 1) {
		if (condition[index]!.value !== operator) continue;
		parts.push(condition.slice(start, index));
		start = index + 1;
	}
	parts.push(condition.slice(start));
	return parts;
}

function gateKind(condition: Token[], negated: boolean): GateKind | undefined {
	if (negated) {
		const clauses = condition.reduce<Token[][]>((parts, token) => {
			if (token.value === "&&" || token.value === "||") parts.push([]);
			else parts[parts.length - 1]!.push(token);
			return parts;
		}, [[]]);
		const kinds = clauses.map((clause) => gateKindAtomic(clause, true));
		if (kinds.some((kind) => kind === undefined)) return undefined;
		return kinds.includes("environment") ? "environment" : "docs";
	}

	const disjuncts = splitCondition(condition, "||");
	if (disjuncts.length === 1) return gateKindAtomic(condition, false);
	const kinds = disjuncts.map((clause) => gateKindAtomic(clause, false));
	return kinds.every((kind) => kind === "environment") ? "environment" : undefined;
}

function ifBodyOpen(
	tokens: Token[],
	ifIndex: number,
	parenPairs: Map<number, number>,
	bracketPairs: Map<number, number>,
): number | undefined {
	for (let index = ifIndex + 1; index < tokens.length; index += 1) {
		const value = tokens[index]!.value;
		if (value === "(") {
			const closing = parenPairs.get(index);
			if (closing !== undefined) index = closing;
			continue;
		}
		if (value === "[") {
			const closing = bracketPairs.get(index);
			if (closing !== undefined) index = closing;
			continue;
		}
		if (value === "{") return index;
		if (value === ";" || value === "=>") return undefined;
	}
	return undefined;
}

function functionBodies(
	tokens: Token[],
	braces: Map<number, number>,
	parens: Map<number, number>,
	brackets: Map<number, number>,
): FunctionBody[] {
	const functions: FunctionBody[] = [];
	for (let index = 0; index < tokens.length; index += 1) {
		if (tokens[index]!.value !== "fn") continue;
		const prior = tokens.slice(Math.max(0, index - 10), index);
		const name = tokens[index + 1];
		if (!name || name.kind !== "identifier") continue;

		let paramsOpen = index + 2;
		while (paramsOpen < tokens.length && tokens[paramsOpen]!.value !== "(") paramsOpen += 1;
		const paramsClose = parens.get(paramsOpen);
		if (paramsClose === undefined) continue;

		let bodyOpen: number | undefined;
		for (let cursor = paramsClose + 1; cursor < tokens.length; cursor += 1) {
			if (tokens[cursor]!.value === ";") break;
			if (tokens[cursor]!.value === "{") {
				bodyOpen = cursor;
				break;
			}
			if (tokens[cursor]!.value === "(" && parens.has(cursor)) cursor = parens.get(cursor)!;
		}
		if (bodyOpen === undefined) continue;
		const bodyEnd = braces.get(bodyOpen);
		if (bodyEnd === undefined) continue;

		const parameterTokens = splitTopLevel(tokens, paramsOpen + 1, paramsClose, braces, parens, brackets);
		const parameters = parameterTokens.flatMap((parameter) => {
			const colon = parameter.findIndex((token) => token.value === ":");
			if (colon < 0) return [];
			const nameToken = parameter.slice(0, colon).find((token) => token.kind === "identifier");
			return nameToken ? [nameToken.value] : [];
		});
		functions.push({
			name: name.value,
			start: bodyOpen,
			end: bodyEnd,
			parameters,
			publicAsync: prior.some((token) => token.value === "pub") && prior.some((token) => token.value === "async"),
		});
	}
	return functions;
}

function enclosingFunction(functions: FunctionBody[], tokenIndex: number): FunctionBody | undefined {
	return functions
		.filter((fn) => fn.start < tokenIndex && tokenIndex < fn.end)
		.sort((left, right) => left.end - left.start - (right.end - right.start))[0];
}

function hasTopLevelReturn(tokens: Token[], bodyStart: number, bodyEnd: number, braces: Map<number, number>): boolean {
	for (let index = bodyStart + 1; index < bodyEnd; index += 1) {
		if (tokens[index]!.value === "return") return true;
		if (tokens[index]!.value === "{" && braces.has(index)) index = braces.get(index)!;
	}
	return false;
}

function openApiMounts(tokens: Token[]): number[] {
	const mounts = new Set<number>();
	for (let index = 0; index < tokens.length; index += 1) {
		if (
			tokens[index]!.value === "SwaggerUi" &&
			tokens[index + 1]?.value === "::" &&
			tokens[index + 2]?.value === "new"
		) {
			mounts.add(index);
		}
		if (tokens[index]!.kind !== "string" || tokens[index]!.value !== "/openapi.json") continue;
		const window = tokens.slice(Math.max(0, index - 10), index + 11);
		if (
			!window.some((token) => token.value === "SwaggerUi") &&
			window.some((token) => ["route", "route_service", "url", "openapi"].includes(token.value))
		) {
			mounts.add(index);
		}
	}
	return [...mounts].sort((left, right) => left - right);
}

function relativePath(root: string, file: string): string {
	return path.relative(root, file).split(path.sep).join("/");
}

function parseRustModule(root: string, file: string): RustModule {
	const source = readFileSync(path.join(root, file), "utf8");
	const tokens = codeTokens(tokenizeRust(source));
	const braces = pairTokens(tokens, "{", "}");
	const parens = pairTokens(tokens, "(", ")");
	const brackets = pairTokens(tokens, "[", "]");
	return {
		path: relativePath(root, path.resolve(root, file)),
		source,
		tokens,
		braces,
		parens,
		brackets,
		functions: functionBodies(tokens, braces, parens, brackets),
	};
}

function isOpenApiSourcePath(relative: string): boolean {
	const segments = relative.split("/");
	const file = segments.at(-1) ?? "";
	return !segments.some((segment) => segment === "underlay" || segment === "test" || segment === "tests") &&
		!file.endsWith("_test.rs");
}

function isEnvironmentGatedAt(module: RustModule, tokenIndex: number): boolean {
	for (let index = 0; index < tokenIndex; index += 1) {
		if (module.tokens[index]!.value !== "if") continue;
		const bodyOpen = ifBodyOpen(module.tokens, index, module.parens, module.brackets);
		if (bodyOpen === undefined) continue;
		const bodyEnd = module.braces.get(bodyOpen);
		if (
			bodyEnd !== undefined &&
			tokenIndex > bodyOpen &&
			tokenIndex < bodyEnd &&
			gateKind(module.tokens.slice(index + 1, bodyOpen), false) === "environment"
		) {
			return true;
		}
	}

	const fn = enclosingFunction(module.functions, tokenIndex);
	if (!fn) return false;
	for (let index = fn.start + 1; index < tokenIndex; index += 1) {
		if (module.tokens[index]!.value !== "if") continue;
		const bodyOpen = ifBodyOpen(module.tokens, index, module.parens, module.brackets);
		if (bodyOpen === undefined) continue;
		const bodyEnd = module.braces.get(bodyOpen);
		if (bodyEnd === undefined || bodyEnd >= tokenIndex || bodyEnd >= fn.end) continue;
		if (
			gateKind(module.tokens.slice(index + 1, bodyOpen), true) === "environment" &&
			hasTopLevelReturn(module.tokens, bodyOpen, bodyEnd, module.braces)
		) {
			return true;
		}
	}
	return false;
}

function docsParameterIndex(fn: FunctionBody, condition: Token[]): number | undefined {
	for (let index = 0; index < condition.length; index += 1) {
		if (!DOCS_GATE_NAMES.has(condition[index]!.value)) continue;
		const directName = condition[index]!.value;
		const directIndex = fn.parameters.indexOf(directName);
		if (directIndex >= 0) return directIndex;
		if (condition[index - 1]?.value === ".") {
			const receiver = condition[index - 2]?.value;
			if (receiver) {
				const receiverIndex = fn.parameters.indexOf(receiver);
				if (receiverIndex >= 0) return receiverIndex;
			}
		}
	}
	return undefined;
}

function callArguments(module: RustModule, callIndex: number): Token[][] | undefined {
	const open = callIndex + 1;
	const close = module.parens.get(open);
	if (close === undefined) return undefined;
	return splitTopLevel(module.tokens, open + 1, close, module.braces, module.parens, module.brackets);
}

function localBindingExpression(
	module: RustModule,
	fn: FunctionBody,
	name: string,
	beforeIndex: number,
): Token[] | undefined {
	let binding: { expression: Token[]; end: number } | undefined;

	for (let index = fn.start + 1; index < beforeIndex; index += 1) {
		if (module.tokens[index]!.value !== "let") continue;
		let nameIndex = index + 1;
		if (module.tokens[nameIndex]?.value === "mut") nameIndex += 1;
		if (module.tokens[nameIndex]?.value !== name) continue;
		let visibleAtCall = true;
		for (let open = fn.start + 1; open < index; open += 1) {
			if (module.tokens[open]!.value !== "{") continue;
			const scopeEnd = module.braces.get(open);
			if (scopeEnd !== undefined && scopeEnd > index && scopeEnd < beforeIndex) {
				visibleAtCall = false;
				break;
			}
		}
		if (!visibleAtCall) continue;

		let equalsIndex = nameIndex + 1;
		while (
			equalsIndex < beforeIndex &&
			module.tokens[equalsIndex]!.value !== "=" &&
			module.tokens[equalsIndex]!.value !== ";"
		) {
			equalsIndex += 1;
		}
		if (module.tokens[equalsIndex]?.value !== "=") continue;

		let end = equalsIndex + 1;
		while (end < beforeIndex) {
			const value = module.tokens[end]!.value;
			const pairedEnd = value === "("
				? module.parens.get(end)
				: value === "["
					? module.brackets.get(end)
					: value === "{"
						? module.braces.get(end)
						: undefined;
			if (pairedEnd !== undefined && pairedEnd < beforeIndex) {
				end = pairedEnd + 1;
				continue;
			}
			if (value === ";") break;
			end += 1;
		}
		if (module.tokens[end]?.value === ";") {
			binding = { expression: module.tokens.slice(equalsIndex + 1, end), end };
		}
	}

	if (!binding) return undefined;
	for (let index = binding.end + 1; index + 1 < beforeIndex; index += 1) {
		const previous = module.tokens[index - 1]?.value;
		const previousPrevious = module.tokens[index - 2]?.value;
		const isDeclaration = previous === "let" || (previous === "mut" && previousPrevious === "let");
		if (
			!isDeclaration &&
			module.tokens[index]!.value === name &&
			module.tokens[index + 1]!.value === "="
		) {
			return undefined;
		}
	}
	return binding.expression;
}

function simpleIdentifier(expression: Token[]): string | undefined {
	const identifiers = expression.filter((token) => token.kind === "identifier");
	if (
		identifiers.length !== 1 ||
		expression.some((token) => token !== identifiers[0] && token.value !== "(" && token.value !== ")")
	) {
		return undefined;
	}
	return identifiers[0]!.value;
}

function localBindingIsDevelopment(
	module: RustModule,
	fn: FunctionBody,
	name: string,
	beforeIndex: number,
	seen = new Set<string>(),
): boolean {
	if (seen.has(name)) return false;
	seen.add(name);
	const expression = localBindingExpression(module, fn, name, beforeIndex);
	if (!expression) return false;
	if (gateKind(expression, false) === "environment") return true;
	const alias = simpleIdentifier(expression);
	return alias !== undefined && localBindingIsDevelopment(module, fn, alias, beforeIndex, seen);
}

function argumentHasLocallyDerivedDocsFlag(
	module: RustModule,
	fn: FunctionBody | undefined,
	argument: Token[],
	callIndex: number,
): boolean {
	if (!fn) return false;
	const names = new Set<string>();
	for (let index = 0; index < argument.length; index += 1) {
		const token = argument[index]!;
		if (!DOCS_GATE_NAMES.has(token.value)) continue;
		if (argument[index + 1]?.value === ":") {
			const value = argument[index + 2];
			if (
				value?.kind === "identifier" &&
				[",", "}"].includes(argument[index + 3]?.value ?? "}")
			) {
				names.add(value.value);
			}
		} else if ([",", "}"].includes(argument[index + 1]?.value ?? "}")) {
			names.add(token.value);
		}
	}
	const directName = simpleIdentifier(argument);
	if (directName) names.add(directName);
	return [...names].some((name) => localBindingIsDevelopment(module, fn, name, callIndex));
}

function callersProveDevelopment(
	target: FunctionBody,
	parameterIndex: number,
	modules: RustModule[],
): boolean {
	let callCount = 0;
	for (const module of modules) {
		if (!isOpenApiSourcePath(module.path)) continue;
		for (let index = 0; index < module.tokens.length - 1; index += 1) {
			if (
				module.tokens[index]!.value !== target.name ||
				module.tokens[index + 1]!.value !== "(" ||
				module.tokens[index - 1]?.value === "fn"
			) {
				continue;
			}
			const args = callArguments(module, index);
			if (!args) continue;
			callCount += 1;
			const argument = args[parameterIndex] ?? [];
			const caller = enclosingFunction(module.functions, index);
			if (
				gateKind(argument, false) !== "environment" &&
				!isEnvironmentGatedAt(module, index) &&
				!argumentHasLocallyDerivedDocsFlag(module, caller, argument, index)
			) {
				return false;
			}
		}
	}
	return callCount > 0;
}

function localDocsFlagIsDevelopment(
	module: RustModule,
	fn: FunctionBody,
	condition: Token[],
	mountIndex: number,
): boolean {
	const directFlags = condition
		.filter((token) => DOCS_GATE_NAMES.has(token.value))
		.map((token) => token.value)
		.filter((flag) => !fn.parameters.includes(flag));
	for (const flag of directFlags) {
		for (let index = fn.start + 1; index < mountIndex; index += 1) {
			if (module.tokens[index]!.value !== "let") continue;
			let nameIndex = index + 1;
			if (module.tokens[nameIndex]?.value === "mut") nameIndex += 1;
			if (module.tokens[nameIndex]?.value !== flag || module.tokens[nameIndex + 1]?.value !== "=") continue;
			let end = nameIndex + 2;
			while (end < fn.end && module.tokens[end]!.value !== ";") end += 1;
			if (gateKind(module.tokens.slice(nameIndex + 2, end), false) === "environment") return true;
		}
	}
	return false;
}

function docsGateIsDevelopment(
	module: RustModule,
	fn: FunctionBody | undefined,
	condition: Token[],
	mountIndex: number,
	modules: RustModule[],
): boolean {
	if (!fn) return false;
	if (localDocsFlagIsDevelopment(module, fn, condition, mountIndex)) return true;
	const parameterIndex = docsParameterIndex(fn, condition);
	return parameterIndex !== undefined && callersProveDevelopment(fn, parameterIndex, modules);
}

function analyzeOpenApiModule(module: RustModule, modules: RustModule[]): Finding[] {
	if (!isOpenApiSourcePath(module.path)) return [];
	const mounts = openApiMounts(module.tokens);
	if (mounts.length === 0) return [];
	const findings: Finding[] = [];

	for (const mount of mounts) {
		let guarded = false;
		for (let index = 0; index < mount; index += 1) {
			if (module.tokens[index]!.value !== "if") continue;
			const bodyOpen = ifBodyOpen(module.tokens, index, module.parens, module.brackets);
			if (bodyOpen === undefined) continue;
			const bodyEnd = module.braces.get(bodyOpen);
			if (bodyEnd === undefined || mount <= bodyOpen || mount >= bodyEnd) continue;
			const condition = module.tokens.slice(index + 1, bodyOpen);
			const gate = gateKind(condition, false);
			if (gate === "environment") {
				guarded = true;
				break;
			}
			if (gate === "docs" && docsGateIsDevelopment(module, enclosingFunction(module.functions, mount), condition, mount, modules)) {
				guarded = true;
				break;
			}
		}

		if (!guarded) {
			const fn = enclosingFunction(module.functions, mount);
			if (fn) {
				for (let index = fn.start + 1; index < mount; index += 1) {
					if (module.tokens[index]!.value !== "if") continue;
					const bodyOpen = ifBodyOpen(module.tokens, index, module.parens, module.brackets);
					if (bodyOpen === undefined) continue;
					const bodyEnd = module.braces.get(bodyOpen);
					if (bodyEnd === undefined || bodyEnd >= mount || bodyEnd >= fn.end) continue;
					const condition = module.tokens.slice(index + 1, bodyOpen);
					const gate = gateKind(condition, true);
					if (
						gate === "environment" &&
						hasTopLevelReturn(module.tokens, bodyOpen, bodyEnd, module.braces)
					) {
						guarded = true;
						break;
					}
					if (
						gate === "docs" &&
						hasTopLevelReturn(module.tokens, bodyOpen, bodyEnd, module.braces) &&
						docsGateIsDevelopment(module, fn, condition, mount, modules)
					) {
						guarded = true;
						break;
					}
				}

				const lowerBound = Math.max(fn.start, mount - 14);
				const prefix = module.tokens.slice(lowerBound, mount);
				const thenIndex = prefix.findIndex((token) => token.value === "then" || token.value === "then_some");
				if (thenIndex !== -1) {
					const condition = prefix.slice(0, thenIndex);
					const gate = gateKind(condition, false);
					if (
						gate === "environment" ||
						(gate === "docs" && docsGateIsDevelopment(module, fn, condition, mount, modules))
					) {
						guarded = true;
					}
				}
			}
		}

		if (!guarded) {
			findings.push({
				path: module.path,
				line: lineAt(module.source, module.tokens[mount]!.start),
				message: "mount must be inside a development-only gate or follow a guarded early return",
			});
		}
	}
	return findings;
}

function blankSqlRange(characters: string[], start: number, end: number): void {
	for (let index = start; index < end; index += 1) {
		if (characters[index] !== "\n" && characters[index] !== "\r") characters[index] = " ";
	}
}

function sqlCode(sql: string): string {
	const characters = sql.split("");
	let index = 0;
	while (index < sql.length) {
		if (sql.startsWith("--", index)) {
			let end = sql.indexOf("\n", index);
			if (end === -1) end = sql.length;
			blankSqlRange(characters, index, end);
			index = end;
			continue;
		}
		if (sql.startsWith("/*", index)) {
			const start = index;
			let depth = 1;
			index += 2;
			while (index < sql.length && depth > 0) {
				if (sql.startsWith("/*", index)) {
					depth += 1;
					index += 2;
				} else if (sql.startsWith("*/", index)) {
					depth -= 1;
					index += 2;
				} else {
					index += 1;
				}
			}
			blankSqlRange(characters, start, index);
			continue;
		}

		const dollarQuote = sql[index] === "$"
			? /^\$[A-Za-z_][A-Za-z0-9_]*\$|^\$\$/.exec(sql.slice(index))?.[0]
			: undefined;
		if (dollarQuote) {
			const start = index;
			const close = sql.indexOf(dollarQuote, index + dollarQuote.length);
			index = close === -1 ? sql.length : close + dollarQuote.length;
			blankSqlRange(characters, start, index);
			continue;
		}

		if (sql[index] === "'" || sql[index] === '"') {
			const quote = sql[index]!;
			const start = index;
			let end = index + 1;
			let content = "";
			while (end < sql.length) {
				if (sql[end] === "\\") {
					content += sql.slice(end, end + 2);
					end += 2;
					continue;
				}
				if (sql[end] === quote && sql[end + 1] === quote) {
					content += quote;
					end += 2;
					continue;
				}
				if (sql[end] === quote) {
					end += 1;
					break;
				}
				content += sql[end]!;
				end += 1;
			}
			blankSqlRange(characters, start, end);
			if (quote === '"' && /^(?:id|[A-Za-z_]\w*_id)$/i.test(content)) {
				for (let offset = 0; offset < content.length; offset += 1) {
					characters[start + offset] = content[offset]!;
				}
			}
			index = end;
			continue;
		}
		index += 1;
	}
	return characters.join("");
}

interface SqlWord {
	value: string;
	start: number;
	end: number;
	depth: number;
}

function topLevelSqlWords(sql: string): SqlWord[] {
	const words: SqlWord[] = [];
	let depth = 0;
	let index = 0;
	while (index < sql.length) {
		if (sql[index] === "(") {
			depth += 1;
			index += 1;
			continue;
		}
		if (sql[index] === ")") {
			depth = Math.max(0, depth - 1);
			index += 1;
			continue;
		}
		if (/[A-Za-z_]/.test(sql[index]!)) {
			const start = index;
			index += 1;
			while (index < sql.length && /[A-Za-z0-9_$]/.test(sql[index]!)) index += 1;
			words.push({ value: sql.slice(start, index).toUpperCase(), start, end: index, depth });
			continue;
		}
		index += 1;
	}
	return words;
}

function startsWithSelect(sql: string): boolean {
	return /^(?:SELECT|WITH)\b/i.test(sqlCode(sql).trim());
}

function hasSqlLimit(sql: string): boolean {
	const clean = sqlCode(sql);
	const words = topLevelSqlWords(clean);
	for (const word of words) {
		if (word.depth !== 0) continue;
		const remainder = clean.slice(word.end);
		if (
			word.value === "LIMIT" &&
			/^\s*(?:\$\d+|\?|:[A-Za-z_]\w*|\d+)(?=\s|,|;|::|$)/i.test(remainder)
		) {
			return true;
		}
		if (word.value === "FETCH" && /^\s*(?:FIRST|NEXT)\s+(?:\$\d+|\?|\d+)\s+ROWS?\s+ONLY\b/i.test(remainder)) {
			return true;
		}
	}
	return false;
}

function hasExplicitIdBound(sql: string): boolean {
	const clean = sqlCode(sql);
	const words = topLevelSqlWords(clean);
	if (words.some((word) => word.depth === 0 && word.value === "UNION")) return false;
	let whereWord: SqlWord | undefined;
	for (const word of words) {
		if (word.depth === 0 && word.value === "WHERE") whereWord = word;
	}
	if (!whereWord) return false;
	const endWord = words.find(
		(word) =>
			word.depth === 0 &&
			word.start > whereWord.end &&
			["GROUP", "ORDER", "LIMIT", "FETCH", "OFFSET", "UNION"].includes(word.value),
	);
	let where = clean.slice(whereWord.end, endWord?.start ?? clean.length).trim();
	while (where.startsWith("(") && where.endsWith(")")) {
		let depth = 0;
		let wrapsWholePredicate = true;
		for (let index = 0; index < where.length; index += 1) {
			if (where[index] === "(") depth += 1;
			if (where[index] === ")") depth -= 1;
			if (depth === 0 && index < where.length - 1) {
				wrapsWholePredicate = false;
				break;
			}
		}
		if (!wrapsWholePredicate) break;
		where = where.slice(1, -1).trim();
	}
	const whereWords = topLevelSqlWords(where);
	if (whereWords.some((word) => word.depth === 0 && word.value === "OR")) return false;

	const column = String.raw`(?:[A-Za-z_]\w*\s*\.\s*)?(?:id|[A-Za-z_]\w*_id)`;
	const value = String.raw`(?:\$\d+|\?|:[A-Za-z_]\w*)`;
	const cast = String.raw`(?:\s*::\s*[A-Za-z_]\w*(?:\s*\[\])?)?`;
	const any = new RegExp(String.raw`\b${column}\s*=\s*ANY\s*\(\s*${value}${cast}\s*\)`, "i");
	const inList = new RegExp(
		String.raw`\b${column}\s+IN\s*\(\s*${value}${cast}(?:\s*,\s*${value}${cast})*\s*\)`,
		"i",
	);
	return whereWords.some(
		(word) =>
			word.depth === 0 &&
			/^(?:id|[A-Za-z_]\w*_id)$/i.test(word.value) &&
			(any.test(where.slice(word.start)) || inList.test(where.slice(word.start))),
	);
}

function queryAllowanceImmediatelyBefore(source: string, statementOffset: number): boolean {
	const lineStart = source.lastIndexOf("\n", statementOffset - 1) + 1;
	const priorLines = source.slice(0, lineStart).split(/\r?\n/);
	let skippedBlank = false;
	const comments: string[] = [];
	for (let index = priorLines.length - 1; index >= 0; index -= 1) {
		const line = priorLines[index]!.trim();
		if (!line) {
			if (comments.length === 0) continue;
			if (skippedBlank) break;
			skippedBlank = true;
			continue;
		}
		if (!line.startsWith("//")) break;
		comments.push(line.replace(/^\/\/+\s?/, ""));
	}
	const text = comments.reverse().join(" ");
	const marker = /conformance:\s*allow(?:\s+bounded-queries)?\s*(?::|[-—])\s*(.+)$/i.exec(text);
	return (marker?.[1]?.trim().length ?? 0) >= 12;
}

function statementStartIndex(tokens: Token[], bodyStart: number, before: number): number {
	let start = bodyStart + 1;
	for (let index = bodyStart + 1; index < before; index += 1) {
		if ([";", "{", "}"].includes(tokens[index]!.value)) start = index + 1;
	}
	return Math.min(start, before);
}

function moduleSqlConstants(
	tokens: Token[],
	functions: FunctionBody[],
	braces: Map<number, number>,
	parens: Map<number, number>,
	brackets: Map<number, number>,
): Map<string, string> {
	const constants = new Map<string, string>();
	for (let index = 0; index < tokens.length - 1; index += 1) {
		if (tokens[index]!.value !== "const" || enclosingFunction(functions, index)) continue;
		const name = tokens[index + 1];
		if (name?.kind !== "identifier") continue;

		let equalsIndex = index + 2;
		while (equalsIndex < tokens.length && !["=", ";"].includes(tokens[equalsIndex]!.value)) {
			equalsIndex += 1;
		}
		if (tokens[equalsIndex]?.value !== "=") continue;

		let end = equalsIndex + 1;
		while (end < tokens.length) {
			const value = tokens[end]!.value;
			const pairedEnd = value === "("
				? parens.get(end)
				: value === "["
					? brackets.get(end)
					: value === "{"
						? braces.get(end)
						: undefined;
			if (pairedEnd !== undefined) {
				end = pairedEnd + 1;
				continue;
			}
			if (value === ";") break;
			end += 1;
		}
		const sql = tokens
			.slice(equalsIndex + 1, end)
			.find((token) => token.kind === "string" && startsWithSelect(token.value));
		if (sql) constants.set(name.value, sql.value);
	}
	return constants;
}

function queryCallClose(
	tokens: Token[],
	fn: FunctionBody,
	argumentIndex: number,
	parens: Map<number, number>,
): number | undefined {
	for (let open = argumentIndex - 1; open > fn.start; open -= 1) {
		if (tokens[open]!.value !== "(") continue;
		const close = parens.get(open);
		if (close === undefined || close < argumentIndex) continue;

		let callee = open - 1;
		if (tokens[callee]?.value === ">") {
			let genericDepth = 0;
			for (; callee > fn.start; callee -= 1) {
				if (tokens[callee]!.value === ">") genericDepth += 1;
				else if (tokens[callee]!.value === "<") {
					genericDepth -= 1;
					if (genericDepth === 0) {
						callee -= 1;
						break;
					}
				}
			}
			if (genericDepth !== 0) continue;
		}
		if (tokens[callee]?.value === "::") callee -= 1;
		if (tokens[callee]?.value === "!") callee -= 1;
		if (["query", "query_as", "query_scalar"].includes(tokens[callee]?.value ?? "")) return close;
	}
	return undefined;
}

function analyzeBoundedQueriesFile(root: string, file: string): Finding[] {
	const relative = relativePath(root, path.resolve(root, file));
	if (
		relative.includes("underlay/") ||
		relative.startsWith("underlay/") ||
		relative.includes("/tests/") ||
		relative.startsWith("tests/")
	) {
		return [];
	}

	const source = readFileSync(path.join(root, file), "utf8");
	const allTokens = tokenizeRust(source);
	const tokens = allTokens.filter((token) => token.kind !== "comment");
	const braces = pairTokens(tokens, "{", "}");
	const parens = pairTokens(tokens, "(", ")");
	const brackets = pairTokens(tokens, "[", "]");
	const functions = functionBodies(tokens, braces, parens, brackets);
	const sqlConstants = moduleSqlConstants(tokens, functions, braces, parens, brackets);
	const findings: Finding[] = [];

	for (const fn of functions) {
		if (!fn.publicAsync || !/^(?:list|search)_/.test(fn.name)) continue;
		for (let index = fn.start + 1; index < fn.end; index += 1) {
			const token = tokens[index]!;
			const sql = token.kind === "string"
				? token.value
				: token.kind === "identifier"
					? sqlConstants.get(token.value)
					: undefined;
			if (!sql || !startsWithSelect(sql)) continue;

			let searchFrom = index + 1;
			if (token.kind === "identifier") {
				const queryClose = queryCallClose(tokens, fn, index, parens);
				if (queryClose === undefined) continue;
				searchFrom = queryClose + 1;
			}

			let executionIndex: number | undefined;
			for (let cursor = searchFrom; cursor < fn.end; cursor += 1) {
				if (tokens[cursor]!.value === ";" || tokens[cursor]!.value === "}") break;
				if (tokens[cursor]!.value === "fetch_all" || tokens[cursor]!.value === "fetch_optional") {
					executionIndex = cursor;
					break;
				}
			}
			if (executionIndex === undefined) continue;

			const statementIndex = statementStartIndex(tokens, fn.start, index);
			const statementStart = tokens[statementIndex] ?? token;
			const statementTokens = tokens.slice(statementIndex, executionIndex + 1);
			const statementText = statementTokens.map((part) => part.text).join(" ");
			const isBounded =
				hasSqlLimit(sql) ||
				hasExplicitIdBound(sql) ||
				/\.\s*(?:limit|take)\s*\(/i.test(statementText);
			const isDocumentedException = queryAllowanceImmediatelyBefore(source, statementStart.start);
			if (isBounded || isDocumentedException) continue;

			findings.push({
				path: relative,
				line: lineAt(source, token.start),
				message: `${fn.name} query uses ${tokens[executionIndex]!.value} without a query-local bound or adjacent reasoned allowance`,
			});
			index = executionIndex;
		}
	}

	return findings;
}

function analyze(rootArgument: string, rule: "openapi-gated" | "bounded-queries"): Finding[] {
	const root = path.resolve(rootArgument);
	if (!statSync(root).isDirectory()) throw new Error(`not a directory: ${root}`);
	const files = rustFiles(root);
	const findings = rule === "openapi-gated"
		? (() => {
				const modules = files.map((file) => parseRustModule(root, file));
				return modules.flatMap((module) => analyzeOpenApiModule(module, modules));
			})()
		: files.flatMap((file) => analyzeBoundedQueriesFile(root, file));
	return findings.sort((left, right) => left.path.localeCompare(right.path) || left.line - right.line);
}

function main(): void {
	const [, , rule, root = "."] = process.argv;
	if (rule !== "openapi-gated" && rule !== "bounded-queries") {
		console.error("usage: consumer-security-analysis.ts <openapi-gated|bounded-queries> [consumer-root]");
		process.exitCode = 2;
		return;
	}

	try {
		const findings = analyze(root, rule);
		for (const finding of findings) {
			console.log(`${finding.path}:${finding.line}: ${finding.message}`);
		}
		process.exitCode = findings.length > 0 ? 1 : 0;
	} catch (error) {
		console.error(error instanceof Error ? error.message : String(error));
		process.exitCode = 2;
	}
}

if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) main();
