/**
 * Media Library types for Underlay.
 *
 * Generic types for media items, versions, usage tracking, and upload operations.
 * These types are designed to be shared across consuming applications.
 */

export {
  MediaKind,
  MediaVisibility,
  MediaVersionState
} from "./media-types/enums";

export type {
  MediaSummary,
  MediaDetail,
  MediaVersion,
  MediaRendition,
  MediaUsage
} from "./media-types/dto";

export type {
  CreateMediaRequest,
  UpdateMediaRequest,
  CheckDuplicateRequest,
  CheckDuplicateResponse,
  InitiateUploadRequest,
  InitiateUploadResponse,
  MediaUploadPlan,
  FinaliseUploadRequest,
  FinaliseUploadResponse,
  MediaListQuery
} from "./media-types/requests";

export {
  getMediaKindLabel,
  getMediaKindAccent,
  getMediaVisibilityLabel,
  getMediaVisibilityAccent,
  getMediaVersionStateLabel,
  getMediaVersionStateAccent,
  detectMediaKindFromMimeType,
  isMediaDeleted,
  getMediaDisplayName
} from "./media-types/labels";

export { getMediaKindIcon, type IconComponent } from "./media-types/icons";
