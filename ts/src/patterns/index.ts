// Patterns are higher-level, app-agnostic UI building blocks.

export * from "./toasts";
export * from "./clipboard";
export * from "./dom";
export * from "./auth";
export * from "./forms";
export * from "./storage";
export * from "./navigation";
export * from "./i18n";
export * from "./optimistic";
export * from "./banner";
export * from "./slugify";
export * from "./selection-history";
export {
  createLocalSearchFns,
  type LocalSearchOptions,
  type LocalSearchFns
} from "./local-search";

// Reorder utilities
export {
  createReorderController,
  type ReorderController,
  type ReorderableItem
} from "./reorder-controller.svelte";

export { default as FilterBar } from "./FilterBar.svelte";
export { default as FormShell } from "./FormShell.svelte";
export { default as PageHeader } from "./PageHeader.svelte";
export type { PageHeaderLevel, BreadcrumbItem } from "./types";
export { default as CopyActionsMenu } from "./CopyActionsMenu.svelte";
export { default as CardActions } from "./CardActions.svelte";
export { default as SubmitButton } from "./SubmitButton.svelte";
export { default as NavCard } from "./NavCard.svelte";
export { default as NavCardGrid } from "./NavCardGrid.svelte";
export { default as Banner } from "./Banner.svelte";
export { default as ReorderableList } from "./ReorderableList.svelte";
export { default as SlugField } from "./SlugField.svelte";
export { default as SpaFormShell } from "./SpaFormShell.svelte";
export type { SpaFormResult, SpaSubmitHandler, SpaNavigateFn } from "./spa-form-types";

// FormDialog pattern
export { FormDialog } from "./FormDialog/index.js";

// RelationSelector pattern
export {
  RelationSelector,
  RelationSelectorTrigger,
  RelationSelectorModal,
  createRelationSelectorContext,
  useRelationSelector,
  type RelationSelectorContext,
  type SelectableRelation,
  type SearchResult,
  type SearchOptions,
  type RelationSearchFn,
  type RelationSuggestionsFn,
  type SuggestionOptions,
  type RelationSelectorProps,
  type RelationSelectorState,
  type FilterOption,
  type FilterConfig
} from "./RelationSelector/index.js";

// RelationPickerDialog - item picker dialog (base component for relation selection)
export { default as RelationPickerDialog } from "./RelationPickerDialog.svelte";
export type { PickableItem, PickerSection } from "./relation-picker-types.js";

// Explicit export avoids dev-time prebundle staleness.
export { useToasts } from "./useToasts";

// Authenticated data fetching pattern
export {
  useAuthenticatedData,
  type AuthenticatedDataOptions,
  type AuthenticatedDataResult
} from "./authenticated-data.svelte";

// Selection state management
export {
  useSyncedSelection,
  type SyncedSelectionResult
} from "./synced-selection.svelte";

// Batch selection state management (for multi-select lists)
export {
  useBatchSelection,
  type BatchSelectionResult
} from "./batch-selection.svelte";

// Timezone management
export {
  timezoneStore,
  initTimezone,
  resolveTimezoneConflict,
  setEffectiveTimezone,
  resetTimezone,
  detectBrowserTimezone,
  formatInTimezone,
  formatDate,
  type TimezoneState,
  type InitTimezoneOptions
} from "./timezone.svelte";

// Account / user profile types (identity & personalization)
export {
  deriveDisplayName,
  getEffectiveDisplayName,
  type UserProfileBase,
  type UserProfileUpdateBase
} from "./account-types";

// Pagination types and utilities
export {
  buildPaginationQuery,
  appendPaginationParams,
  DEFAULT_PAGE_SIZE,
  MAX_PAGE_SIZE,
  type PaginationParams,
  type PaginatedResponse,
  type PaginationController
} from "./pagination-types";

// Pagination controllers (Svelte 5 runes)
export {
  createPaginationController,
  createClientPagination,
  type ServerPaginationOptions,
  type ServerPaginationResult,
  type ClientPaginationOptions
} from "./pagination.svelte";

// Blob upload utilities (direct-to-S3 uploads)
export {
  uploadToBlob,
  computeFileHash,
  validateFileType,
  validateFileSize,
  validateFile,
  isVideoFile,
  formatFileSize,
  getFileTypeDescription,
} from "./blob-upload";

export {
  BlobUploadError,
  ALLOWED_IMAGE_TYPES,
  ALLOWED_PDF_TYPES,
  ALLOWED_MEDIA_TYPES,
  REJECTED_VIDEO_TYPES,
  type UploadPlan,
  type UploadProgress,
  type UploadResult,
  type UploadOptions,
  type StoredObject,
  type BlobErrorCode,
  type AllowedImageType,
  type AllowedPdfType,
  type AllowedMediaType,
} from "./blob-types";

// Media upload flow pattern
export {
  createMediaUploadFlow,
  type MediaUploadStep,
  type MediaUploadFlowOptions,
  type MediaUploadFlowController,
} from "./media-upload-flow.svelte";

// Media library types and utilities
export {
  // Enums
  MediaKind,
  MediaVisibility,
  MediaVersionState,
  // Utility functions
  getMediaKindLabel,
  getMediaKindAccent,
  getMediaVisibilityLabel,
  getMediaVisibilityAccent,
  getMediaVersionStateLabel,
  getMediaVersionStateAccent,
  detectMediaKindFromMimeType,
  isMediaDeleted,
  getMediaDisplayName,
  // Types
  type MediaSummary,
  type MediaDetail,
  type MediaVersion,
  type MediaRendition,
  type MediaUsage,
  type CreateMediaRequest,
  type UpdateMediaRequest,
  type CheckDuplicateRequest,
  type CheckDuplicateResponse,
  type InitiateUploadRequest,
  type InitiateUploadResponse,
  type MediaUploadPlan,
  type FinaliseUploadRequest,
  type FinaliseUploadResponse,
  type MediaListQuery,
} from "./media-types";
