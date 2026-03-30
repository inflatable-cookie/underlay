// Patterns are retained higher-level workflow and shell building blocks.
export { ForgotPasswordFlow, LoginPage, PasswordRequirements } from "./auth-workflows";
export { default as SpaFormShell } from "./SpaFormShell.svelte";
export type { SpaFormResult, SpaSubmitHandler, SpaNavigateFn } from "./spa-form-types";

// Detail header helpers
export {
  DetailMeta,
  DetailMetaItem,
  DetailMetaId,
  DetailMetaStatus,
  DetailMetaSeparator
} from "./DetailPageShell/index.js";
