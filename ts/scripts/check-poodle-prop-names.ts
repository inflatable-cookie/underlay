import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, "..", "..");

const targets = [
	path.join(repoRoot, "ts", "src"),
	path.join(repoRoot, "docs", "guides"),
];

const retiredPropNames = [
	"isDisabled",
	"isLoading",
	"isReadOnly",
	"isPressed",
	"isChecked",
	"isMixed",
	"isCollapsible",
	"isCollapsed",
	"isPrimaryCollapsed",
	"isSecondaryCollapsed",
	"isInteractive",
	"isSelected",
	"hasMedia",
	"isDismissible",
	"isFocusable",
	"isAnimated",
	"isElevated",
	"isMuted",
	"isIndeterminate",
	"isVisible",
	"isDragRegion",
	"isSeparated",
	"isSticky",
	"isReorderable",
	"isClosable",
	"isCurrent",
	"isSortable",
	"isHideable",
	"hasChildren",
	"isRequired",
	"isModal",
	"isNotLive",
];

const poodleImportPattern =
	/import\s*\{([^}]*)\}\s*from\s*["']@poodle\/svelte(?:-(?:primitives|composites))?["']/g;

// Poodle components take camelCase callback props (onClick, onChange, ...).
// A lowercase native handler on a Poodle component compiles but is overridden
// by the component's own internal handler - a silent dead handler (the
// LoginGoogleTab "Continue with Google" bug).
const lowercaseEventProps = [
	"onclick",
	"onchange",
	"oninput",
	"onsubmit",
	"onblur",
	"onfocus",
	"onkeydown",
	"onkeyup",
	"onmousedown",
	"onmouseup",
];

const optionModelTypeNames = new Set([
	"TabItem",
	"MenuItem",
	"CommandActionItem",
	"TableColumn",
	"BreadcrumbItem",
	"DrillDownItem",
	"CardRadioItem",
	"PanelTabItem",
]);

type Violation = {
	file: string;
	line: number;
	prop: string;
};

function listFiles(dir: string): string[] {
	const out: string[] = [];
	for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
		if (entry.name.startsWith(".")) continue;
		const full = path.join(dir, entry.name);
		if (entry.isDirectory()) {
			out.push(...listFiles(full));
			continue;
		}
		if (entry.isFile() && (full.endsWith(".ts") || full.endsWith(".svelte") || full.endsWith(".md"))) {
			out.push(full);
		}
	}
	return out;
}

function getLineNumber(text: string, index: number): number {
	let line = 1;
	for (let i = 0; i < index; i += 1) {
		if (text.charCodeAt(i) === 10) line += 1;
	}
	return line;
}

function parsePoodleImports(text: string): { components: string[]; optionTypes: string[] } {
	const components = new Set<string>();
	const optionTypes = new Set<string>();

	poodleImportPattern.lastIndex = 0;
	let match: RegExpExecArray | null;
	while ((match = poodleImportPattern.exec(text))) {
		const specifiers = match[1]
			.split(",")
			.map((item) => item.trim())
			.filter(Boolean);

		for (const specifier of specifiers) {
			const withoutType = specifier.replace(/^type\s+/, "").trim();
			const aliasMatch = withoutType.match(/^([A-Za-z0-9_]+)\s+as\s+([A-Za-z0-9_]+)$/);
			const importedName = aliasMatch ? aliasMatch[1] : withoutType;
			const localName = aliasMatch ? aliasMatch[2] : withoutType;

			if (!localName) continue;

			if (optionModelTypeNames.has(importedName)) {
				optionTypes.add(localName);
			} else if (/^[A-Z]/.test(localName)) {
				components.add(localName);
			}
		}
	}

	return {
		components: [...components],
		optionTypes: [...optionTypes],
	};
}

function scanMarkupContext(text: string, importedComponents: string[], file: string): Violation[] {
	if (importedComponents.length === 0) return [];

	const tagPattern = new RegExp(
		`<(${importedComponents.join("|")})\\b[^>]*\\b(${retiredPropNames.join("|")})\\b(?=(\\s*=|\\s|>|\\/))`,
		"g",
	);

	const violations: Violation[] = [];
	let match: RegExpExecArray | null;
	while ((match = tagPattern.exec(text))) {
		violations.push({
			file,
			line: getLineNumber(text, match.index),
			prop: match[2],
		});
	}

	return violations;
}

function scanLowercaseEventContext(
	text: string,
	importedComponents: string[],
	file: string,
): Violation[] {
	if (importedComponents.length === 0) return [];

	const tagPattern = new RegExp(
		`<(${importedComponents.join("|")})\\b[^>]*\\b(${lowercaseEventProps.join("|")})\\b(?=(\\s*=|\\s|>|\\/))`,
		"g",
	);

	const violations: Violation[] = [];
	let match: RegExpExecArray | null;
	while ((match = tagPattern.exec(text))) {
		violations.push({
			file,
			line: getLineNumber(text, match.index),
			prop: match[2],
		});
	}

	return violations;
}

function scanOptionModelContext(text: string, importedOptionTypes: string[], file: string): Violation[] {
	if (importedOptionTypes.length === 0) return [];

	const typeMentionPattern = new RegExp(`\\b(${importedOptionTypes.join("|")})\\b`);
	if (!typeMentionPattern.test(text)) return [];

	const keyPattern = new RegExp(`\\b(${retiredPropNames.join("|")})\\s*:`, "g");
	const violations: Violation[] = [];
	let match: RegExpExecArray | null;
	while ((match = keyPattern.exec(text))) {
		violations.push({
			file,
			line: getLineNumber(text, match.index),
			prop: match[1],
		});
	}

	return violations;
}

function getMarkdownCodeBlocks(text: string): string[] {
	const blocks: string[] = [];
	const codeBlockPattern = /```[a-zA-Z0-9_-]*\n([\s\S]*?)```/g;
	let match: RegExpExecArray | null;
	while ((match = codeBlockPattern.exec(text))) {
		blocks.push(match[1]);
	}
	return blocks;
}

function scanFile(file: string): Violation[] {
	const rel = path.relative(repoRoot, file);
	const text = fs.readFileSync(file, "utf8");

	if (file.endsWith(".md")) {
		return getMarkdownCodeBlocks(text).flatMap((block) => {
			const imports = parsePoodleImports(block);
			if (imports.components.length === 0 && imports.optionTypes.length === 0) {
				return [];
			}
			return [
				...scanMarkupContext(block, imports.components, rel),
				...scanLowercaseEventContext(block, imports.components, rel),
				...scanOptionModelContext(block, imports.optionTypes, rel),
			];
		});
	}

	const imports = parsePoodleImports(text);
	if (imports.components.length === 0 && imports.optionTypes.length === 0) {
		return [];
	}

	return [
		...scanMarkupContext(text, imports.components, rel),
		...scanLowercaseEventContext(text, imports.components, rel),
		...scanOptionModelContext(text, imports.optionTypes, rel),
	];
}

const violations = targets.flatMap((target) => listFiles(target).flatMap(scanFile));

if (violations.length > 0) {
	console.error("Poodle prop-name violations found in Underlay shared source or active guides:");
	for (const violation of violations) {
		const kind = lowercaseEventProps.includes(violation.prop)
			? "lowercase event handler (use camelCase, e.g. onClick)"
			: "retired prop";
		console.error(`- ${violation.file}:${violation.line} uses ${kind} ${violation.prop}`);
	}
	process.exit(1);
}

console.log("Poodle prop-name guardrail check passed.");
