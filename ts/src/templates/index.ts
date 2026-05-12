// Underlay Template System
// Higher-order Svelte components for common admin page shapes.
//
// Three-level composition:
// - Level 1: Page shells (EntityListPage, EntityDetailPage, EntityFormPage)
// - Level 2: Sections (EntityList, EntityDetail) — reusable in tabs/dialogs
// - Level 3: Primitives (Poodle: PageHeader, DataTable, DetailSection, etc.)
//
// Forms are intentionally NOT templated. Real forms have arbitrary layout,
// custom fields, conditional logic, complex validation. Use EntityFormPage
// as a wrapper (header + loading + error states) and bring your own form.

// Level 1 — Page Shells
export { default as EntityListPage } from "./EntityListPage.svelte";
export { default as EntityDetailPage } from "./EntityDetailPage.svelte";
export { default as EntityFormPage } from "./EntityFormPage.svelte";
export { default as EntityTrashPage } from "./EntityTrashPage.svelte";
export { default as MediaUploadPage } from "./MediaUploadPage.svelte";
export { default as MediaDetailWorkflowPage } from "./MediaDetailWorkflowPage.svelte";
export { default as SystemIndexPage } from "./SystemIndexPage.svelte";
export { default as AdminDashboardPage } from "./AdminDashboardPage.svelte";

// Level 2 — Sections
export { default as EntityList } from "./EntityList.svelte";
export { default as EntityDetail } from "./EntityDetail.svelte";
export { default as EntityDetailModule } from "./EntityDetailModule.svelte";
export { default as EntityAttributeList } from "./EntityAttributeList.svelte";
export { default as EntityInlineListModule } from "./EntityInlineListModule.svelte";

// Level 2.5 — Entity Compositions
export { default as EntityListCard } from "./EntityListCard.svelte";
export { toPagedListResult } from "./paged-list";

export type {
  EntityListCardBadge,
  EntityListCardCounter,
  EntityListCardMenuTrigger,
  EntityListCardModeDisplay,
  EntityListCardProps
} from "./entity-list-card.types";
export type {
  BatchActionConfig,
  BatchActionConfirm,
  BatchDialogConfig,
  BatchDialogContext,
  AdminDashboardSectionConfig,
  DetailActionConfig,
  DetailActionConfirm,
  DetailItemConfig,
  DetailMetaItemConfig,
  DetailTabConfig,
  EntityListDataLoader,
  EntityListSharedProps,
  FilterConfig,
  PagedListResult,
  ReorderConfig,
  SystemIndexCardConfig,
  TemplateFilterOption,
  TemplateSortField,
  TemplateSurface
} from "./template.types";
