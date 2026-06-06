export type RestoreBlockerKind = "conflict" | "parent_state" | "invalid_state";

export interface RestoreReference {
	kind: string;
	id: string;
	displayName?: string | null;
}

export interface RestoreFieldConflict {
	fieldName: string;
	candidateValue?: string | null;
	activeOccupant?: RestoreReference | null;
	resolutionHints: string[];
}

export interface RestoreBlocker {
	kind: RestoreBlockerKind;
	entity: RestoreReference;
	parent?: RestoreReference | null;
	parentState?: string | null;
	message?: string | null;
	fieldConflicts: RestoreFieldConflict[];
}

export interface RestoreBlockedResult {
	status: "blocked";
	scope: string;
	deleteBatchId: string;
	blockerKinds: string[];
	blockers: RestoreBlocker[];
}

export type RestoreReferenceFormatter = (input: {
	blocker: RestoreBlocker;
	reference: RestoreReference;
	role: "entity" | "parent" | "active_occupant";
	conflict?: RestoreFieldConflict | null;
}) => string | null;

function isRecord(value: unknown): value is Record<string, unknown> {
	return typeof value === "object" && value !== null;
}

function isRestoreReference(value: unknown): value is RestoreReference {
	return (
		isRecord(value) &&
		typeof value.kind === "string" &&
		typeof value.id === "string" &&
		(value.displayName === undefined ||
			value.displayName === null ||
			typeof value.displayName === "string")
	);
}

function isRestoreFieldConflict(value: unknown): value is RestoreFieldConflict {
	return (
		isRecord(value) &&
		typeof value.fieldName === "string" &&
		(value.candidateValue === undefined ||
			value.candidateValue === null ||
			typeof value.candidateValue === "string") &&
		(value.activeOccupant === undefined ||
			value.activeOccupant === null ||
			isRestoreReference(value.activeOccupant)) &&
		Array.isArray(value.resolutionHints) &&
		value.resolutionHints.every((hint) => typeof hint === "string")
	);
}

function isRestoreBlocker(value: unknown): value is RestoreBlocker {
	return (
		isRecord(value) &&
		(value.kind === "conflict" ||
			value.kind === "parent_state" ||
			value.kind === "invalid_state") &&
		isRestoreReference(value.entity) &&
		(value.parent === undefined ||
			value.parent === null ||
			isRestoreReference(value.parent)) &&
		(value.parentState === undefined ||
			value.parentState === null ||
			typeof value.parentState === "string") &&
		(value.message === undefined ||
			value.message === null ||
			typeof value.message === "string") &&
		Array.isArray(value.fieldConflicts) &&
		value.fieldConflicts.every((conflict) => isRestoreFieldConflict(conflict))
	);
}

export function isRestoreBlockedResult(value: unknown): value is RestoreBlockedResult {
	return (
		isRecord(value) &&
		value.status === "blocked" &&
		typeof value.scope === "string" &&
		typeof value.deleteBatchId === "string" &&
		Array.isArray(value.blockerKinds) &&
		value.blockerKinds.every((kind) => typeof kind === "string") &&
		Array.isArray(value.blockers) &&
		value.blockers.every((blocker) => isRestoreBlocker(blocker))
	);
}
