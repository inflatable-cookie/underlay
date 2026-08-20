import {
  registerBlockEditor,
  registerBlockEmptyChecker,
  registerSchema,
  type BlockEditorComponent,
  type BlockEmptyChecker,
  type SchemaDefinition
} from "./editor-registry";
import {
  registerBlockRenderer,
  type BlockRendererComponent
} from "./render-registry";
import {
  registerBlockValidator,
  type BlockValidator
} from "./validator-registry";
import {
  registerBlockVersions,
  type BlockVersionSpec
} from "./block-versions";

export interface NightfireBlockRegistration {
  schema: SchemaDefinition;
  type: string;
  label: string;
  editor: BlockEditorComponent;
  renderer?: BlockRendererComponent | null;
  rendererSchema?: string | null;
  validator?: BlockValidator | null;
  emptyChecker?: BlockEmptyChecker | null;
  versions?: BlockVersionSpec | null;
}

export function registerNightfireEditor(
  registration: NightfireBlockRegistration
): void {
  registerSchema(registration.schema);
  registerBlockEditor(
    registration.schema.schema,
    registration.type,
    registration.label,
    registration.editor
  );
}

export function registerNightfireEditors(
  registrations: Iterable<NightfireBlockRegistration>
): void {
  for (const registration of registrations) {
    registerNightfireEditor(registration);
  }
}

export function registerNightfireRenderer(
  registration: NightfireBlockRegistration
): void {
  if (!registration.renderer) {
    return;
  }

  registerBlockRenderer(
    registration.rendererSchema ?? registration.schema.schema,
    registration.type,
    registration.renderer
  );
}

export function registerNightfireRenderers(
  registrations: Iterable<NightfireBlockRegistration>
): void {
  for (const registration of registrations) {
    registerNightfireRenderer(registration);
  }
}

export function registerNightfireValidator(
  registration: NightfireBlockRegistration
): void {
  if (!registration.validator) {
    return;
  }

  registerBlockValidator(
    registration.schema.schema,
    registration.type,
    registration.validator
  );
}

export function registerNightfireValidators(
  registrations: Iterable<NightfireBlockRegistration>
): void {
  for (const registration of registrations) {
    registerNightfireValidator(registration);
  }
}

export function registerNightfireEmptyChecker(
  registration: NightfireBlockRegistration
): void {
  if (!registration.emptyChecker) {
    return;
  }

  registerBlockEmptyChecker(registration.type, registration.emptyChecker);
}

export function registerNightfireEmptyCheckers(
  registrations: Iterable<NightfireBlockRegistration>
): void {
  for (const registration of registrations) {
    registerNightfireEmptyChecker(registration);
  }
}

export function registerNightfireVersions(
  registration: NightfireBlockRegistration
): void {
  if (!registration.versions) {
    return;
  }

  registerBlockVersions(registration.type, registration.versions);
}

export function registerNightfireBlock(
  registration: NightfireBlockRegistration
): void {
  registerNightfireEditor(registration);
  registerNightfireRenderer(registration);
  registerNightfireValidator(registration);
  registerNightfireEmptyChecker(registration);
  registerNightfireVersions(registration);
}

export function registerNightfireBlocks(
  registrations: Iterable<NightfireBlockRegistration>
): void {
  for (const registration of registrations) {
    registerNightfireBlock(registration);
  }
}
