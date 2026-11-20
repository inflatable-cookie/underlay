import { realpathSync } from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

/**
 * Determine whether `import.meta.url` matches the invoked CLI script,
 * handling symlinks, relative paths, and paths with spaces or special chars.
 */
export function isCliEntrypoint(
	metaUrl: string,
	argvEntry: string | undefined = process.argv[1],
): boolean {
	if (!argvEntry) return false;

	const metaPath = fileURLToPath(metaUrl);
	const resolvedArgv = path.resolve(argvEntry);
	try {
		return realpathSync(metaPath) === realpathSync(resolvedArgv);
	} catch {
		return metaUrl === pathToFileURL(resolvedArgv).href;
	}
}
