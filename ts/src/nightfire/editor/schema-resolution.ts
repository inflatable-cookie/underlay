import { getSchemaDefinition } from "../editor-registry";

export interface ResolvedNightfireSchema {
	editorSchema: string;
	registryDef: {
		schema: string;
		mode: "single" | "multi";
		defaultType: string;
	};
}

export function resolveSchemaDefinition(
	schemaId: string,
	fallbackSchemaId: string
): ResolvedNightfireSchema {
	const definition = getSchemaDefinition(schemaId);
	if (definition) {
		return {
			editorSchema: schemaId,
			registryDef: {
				schema: definition.schema,
				mode: definition.mode,
				defaultType: definition.defaultType
			}
		};
	}

	const fallbackDefinition = getSchemaDefinition(fallbackSchemaId);
	if (fallbackDefinition) {
		return {
			editorSchema: fallbackSchemaId,
			registryDef: {
				schema: fallbackDefinition.schema,
				mode: fallbackDefinition.mode,
				defaultType: fallbackDefinition.defaultType
			}
		};
	}

	return {
		editorSchema: schemaId,
		registryDef: {
			schema: schemaId,
			mode: "single",
			defaultType: "markdown"
		}
	};
}
