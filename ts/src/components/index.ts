/// <reference path="../svelte.d.ts" />

export { default as Badge } from "./Badge.svelte";
export { default as Breadcrumbs } from "./Breadcrumbs.svelte";
export { default as Code } from "./Code.svelte";
export { default as Card } from "./Card.svelte";
export { default as CardRadioGroup } from "./CardRadioGroup.svelte";
export { default as CheckboxChip } from "./CheckboxChip.svelte";
export { default as FilterToolbar } from "./FilterToolbar.svelte";
export { default as Pill } from "./Pill.svelte";
export {
	default as DateRange,
	formatAdaptiveDateRange,
	formatDateWithOrdinal
} from "./DateRange.svelte";
export { default as DurationInput } from "./DurationInput.svelte";
export { default as RangeSlider } from "./RangeSlider.svelte";
export type { RangeSliderOption } from "./RangeSlider.svelte";
export { default as ColorPicker } from "./ColorPicker.svelte";
export { default as TextArea } from "./TextArea.svelte";
export { default as MarkdownEditor } from "./MarkdownEditor.svelte";
export { default as FormActions } from "./FormActions.svelte";
export { default as SegmentedControl } from "./SegmentedControl.svelte";
export { default as Switch } from "./Switch.svelte";
export { default as ListGrid } from "./ListGrid.svelte";
export { default as ListCard } from "./ListCard.svelte";
export { default as VideoPlayer } from "./VideoPlayer.svelte";
export { default as Select } from "./Select.svelte";
export { default as Drawer } from "./Drawer.svelte";
export { default as ToastHost } from "./ToastHost.svelte";
export { default as DropdownMenu } from "./DropdownMenu.svelte";
export { default as TimeAgo } from "./TimeAgo.svelte";
export { default as Skeleton } from "./Skeleton.svelte";
export { default as DataSkeleton } from "./DataSkeleton.svelte";
export { default as EmptyState } from "./EmptyState.svelte";
export { default as ErrorBoundary } from "./ErrorBoundary.svelte";
export { default as PageLoading } from "./PageLoading.svelte";
export { default as StatusBadge } from "./StatusBadge.svelte";
export { default as DataTable } from "./DataTable.svelte";
export { default as DetailsCard } from "./DetailsCard.svelte";
export { default as ContainerGrid } from "./ContainerGrid.svelte";
export { default as DetailsItem } from "./DetailsItem.svelte";
export { default as DetailsSection } from "./DetailsSection.svelte";
export { default as DetailList } from "./DetailList.svelte";
export { default as DetailItem } from "./DetailItem.svelte";
export { default as InlineListCard } from "./InlineListCard.svelte";
export { default as InlineListItem } from "./InlineListItem.svelte";
export { default as InlineActionGroup } from "./InlineActionGroup.svelte";
export type {
	DataTableColumn,
	DataTableAction,
	DataTablePagination,
	DataTableSort,
	DataTableFilters
} from "./DataTable.svelte";
export { DEFAULT_LIMIT_OPTIONS, exportToCsv } from "./DataTable.svelte";
export { default as Pagination } from "./Pagination.svelte";
export type { PaginationState } from "./Pagination.svelte";
export { default as ProgressBar } from "./ProgressBar.svelte";
export { OrderBy } from "./OrderBy";
export type { OrderByFieldDefinition, OrderByField, OrderByValue } from "./OrderBy";

// Media library components
export { default as MediaPicker } from "./MediaPicker.svelte";
export { default as MediaActionsMenu } from "./MediaActionsMenu.svelte";

export { default as AudioPlayer } from "./AudioPlayer.svelte";
export { default as AudioEmbed } from "./AudioEmbed.svelte";

// Log list (audit/activity logs with filtering, pagination, export)
export { default as LogList } from "./LogList.svelte";
export type { LogEntry, LogActor, LogFilter, LogActionType } from "./LogList.svelte";

// Batch actions
export { default as BatchActionBar } from "./BatchActionBar.svelte";

// Dashboard stats
export { default as StatCard } from "./StatCard.svelte";
export type { StatVariant } from "./StatCard.svelte";
export { default as StatGrid } from "./StatGrid.svelte";
export {
	registerDataSkeletonPreset,
	unregisterDataSkeletonPreset,
	getDataSkeletonPreset,
	type DataSkeletonType,
	type DataSkeletonListPattern,
	type DataSkeletonGridPattern,
	type DataSkeletonDetailSection,
	type DataSkeletonPreset
} from "./data-skeleton";

export { default as TotpInput } from "./auth/TotpInput.svelte";
export { default as PasswordRequirements } from "./auth/PasswordRequirements.svelte";

// Auth building blocks (Phase 1 of auth consolidation)
export { default as AuthLayout } from "./auth/AuthLayout.svelte";

// Auth composite pages (Phase 2 of auth consolidation)
export { default as ForgotPasswordFlow } from "./auth/ForgotPasswordFlow.svelte";
export { default as LoginPage } from "./auth/LoginPage.svelte";

export * from "./auth/types";
