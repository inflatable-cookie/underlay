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
export { default as MediaUploadWorkflowPage } from "./MediaUploadWorkflowPage.svelte";
export { default as MediaUploadStatusPanel } from "./MediaUploadStatusPanel.svelte";
export { default as MediaReplaceFileForm } from "./MediaReplaceFileForm.svelte";
export { default as MediaBrowsePanel } from "./MediaBrowsePanel.svelte";
export { default as MediaActionsMenu } from "./MediaActionsMenu.svelte";
export { default as MediaListPage } from "./MediaListPage.svelte";
export { default as MediaListCard } from "./MediaListCard.svelte";
export { default as MediaFileDetailsCard } from "./MediaFileDetailsCard.svelte";
export { default as MediaEditDialog } from "./MediaEditDialog.svelte";
export { default as MediaPreviewTab } from "./MediaPreviewTab.svelte";
export { default as MediaRenditionsSection } from "./MediaRenditionsSection.svelte";
export { default as MediaVersionActionDialogs } from "./MediaVersionActionDialogs.svelte";
export { default as MediaVersionPreviewDialog } from "./MediaVersionPreviewDialog.svelte";
export { default as MediaVersionsList } from "./MediaVersionsList.svelte";
export { default as MediaUsageList } from "./MediaUsageList.svelte";
export { default as MediaPickerWorkflow } from "./MediaPickerWorkflow.svelte";
export { default as MediaDetailWorkflowPage } from "./MediaDetailWorkflowPage.svelte";
export { default as SystemIndexPage } from "./SystemIndexPage.svelte";
export { default as SystemAuditLogListPage } from "./SystemAuditLogListPage.svelte";
export { default as SystemJobDetailPage } from "./SystemJobDetailPage.svelte";
export { default as SystemJobListPage } from "./SystemJobListPage.svelte";
export { default as SystemMediaTrashListCard } from "./SystemMediaTrashListCard.svelte";
export { default as SystemMediaTrashListPage } from "./SystemMediaTrashListPage.svelte";
export { default as SystemScheduledTaskDetailPage } from "./SystemScheduledTaskDetailPage.svelte";
export { default as SystemScheduledTaskListCard } from "./SystemScheduledTaskListCard.svelte";
export { default as SystemScheduledTasksListPage } from "./SystemScheduledTasksListPage.svelte";
export { default as AdminDashboardPage } from "./AdminDashboardPage.svelte";
export { default as ErrorLogListPage } from "./ErrorLogListPage.svelte";
export { default as ErrorLogDetailPage } from "./ErrorLogDetailPage.svelte";
export { default as MetadataDialogTrigger } from "./MetadataDialogTrigger.svelte";
export { default as ContextActionBar } from "./ContextActionBar.svelte";
export { default as ContextActionDialog } from "./ContextActionDialog.svelte";
export { default as ContextActionList } from "./ContextActionList.svelte";
export { default as ContentCard } from "./ContentCard.svelte";
export { default as AdminPill } from "./AdminPill.svelte";
export { ADMIN_PILL_ACCENTS, type AdminPillKind } from "./admin-pill-accents";

// Level 2 — Sections
export { default as EntityList } from "./EntityList.svelte";
export { default as EntityDetail } from "./EntityDetail.svelte";
export { default as EntityDetailModule } from "./EntityDetailModule.svelte";
export { default as EntityAttributeList } from "./EntityAttributeList.svelte";
export { default as EntityInlineListModule } from "./EntityInlineListModule.svelte";

// Level 2.5 — Entity Compositions
export { default as EntityListCard } from "./EntityListCard.svelte";
export { default as EntityReorderControls } from "./EntityReorderControls.svelte";
export { default as EntityActionsMenu } from "./EntityActionsMenu.svelte";
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
  ErrorLogDetailItem,
  ErrorLogDetailLoader,
  ErrorLogListItem,
  ErrorLogListLoader,
  ErrorLogListRequest,
  ErrorLogStatsLoader,
  ErrorLogStatsSummary,
  EntityListCapabilitiesLoader,
  EntityListDataLoader,
  EntityListSharedProps,
  FilterConfig,
  ListCapabilities,
  ListFilterDefinition,
  ListVariantDefinition,
  InlineListDialogConfig,
  InlineListDialogContext,
  InlineListItemActionConfig,
  InlineListItemDeleteConfig,
  MediaActionsMenuItem,
  MediaListPageItem,
  MediaVersionListItem,
  MediaUsageListItem,
  MediaPickerBrowseItem,
  MediaPickerWorkflowItem,
  PagedListResult,
  ReorderConfig,
  SystemAuditActor,
  SystemAuditLogEntry,
  SystemAuditLogListLoader,
  SystemAuditLogListRequest,
  SystemJobAction,
  SystemJobDetailItem,
  SystemJobDetailLoader,
  SystemJobListItem,
  SystemJobListLoader,
  SystemJobListRequest,
  SystemJobStatsLoader,
  SystemJobStatsSummary,
  SystemJobStatus,
  SystemMediaTrashAction,
  SystemMediaTrashItem,
  SystemMediaTrashListLoader,
  SystemScheduledTaskAction,
  SystemScheduledTaskDetailItem,
  SystemScheduledTaskDetailLoader,
  SystemScheduledTaskJobRunsLoader,
  SystemScheduledTaskListItem,
  SystemScheduledTaskListLoader,
  SystemScheduledTaskListRequest,
  SystemIndexCardConfig,
  TemplateFilterOption,
  TemplateSortField,
  TemplateSurface
} from "./template.types";
export type { BreadcrumbItem } from "../patterns/types";

export type {
  ContextActionDefinition,
  ContextActionDialogForm,
  ContextActionDialogFormContext,
  ContextActionFieldOption,
  ContextActionFieldType,
  ContextActionInputField,
  ContextActionModelOption,
  ContextActionResultMode,
  ContextActionRunState,
  ContextActionSubmitDetail
} from "./contextual-action.types";
