// Underlay Template System
// Higher-order Svelte components for common admin page shapes.
//
// Three-level composition:
// - Level 1: Page shells (EntityListPage, EntityDetailPage, EntityFormPage)
// - Level 2: Sections (EntityList, EntityDetail, EntityForm) — reusable in tabs/dialogs
// - Level 3: Primitives (Poodle: PageHeader, DataTable, DetailSection, etc.)

// Level 1 — Page Shells
export { default as EntityListPage } from "./EntityListPage.svelte";
export { default as EntityDetailPage } from "./EntityDetailPage.svelte";
// export { default as EntityFormPage } from "./EntityFormPage.svelte";

// Level 2 — Sections
export { default as EntityList } from "./EntityList.svelte";
export { default as EntityDetail } from "./EntityDetail.svelte";
// export { default as EntityForm } from "./EntityForm.svelte";
