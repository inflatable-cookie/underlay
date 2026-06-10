import type { QueryParams } from "../../client/query";
import type { FetchFn, PagedListResult } from "./primitives";

export interface SystemMediaTrashItem {
  id: string;
  kind: string;
  title?: string | null;
  originalFilename?: string | null;
  mimeType?: string | null;
  thumbnailUrl?: string | null;
  originalUrl?: string | null;
  byteSize?: number | null;
  deletedAt?: string | null;
}

export interface MediaPickerWorkflowItem {
  id: string;
  kind?: string | null;
  visibility?: string | null;
  title?: string | null;
  originalFilename?: string | null;
  currentVersionId?: string | null;
  createdAt?: string | null;
  updatedAt?: string | null;
  deletedAt?: string | null;
  byteSize?: number | null;
  mimeType?: string | null;
  thumbnailUrl?: string | null;
  originalUrl?: string | null;
}

export interface MediaPickerBrowseItem {
  id: string;
  label: string;
  thumbnailUrl?: string | null;
  originalUrl?: string | null;
  mimeType?: string | null;
  kind?: string | null;
  meta?: string | null;
}

export interface MediaActionsMenuItem {
  id: string;
  title?: string | null;
  originalFilename?: string | null;
  deletedAt?: string | null;
}

export interface MediaListPageItem {
  id: string;
  kind: string;
  visibility?: string | null;
  title?: string | null;
  originalFilename?: string | null;
  mimeType?: string | null;
  thumbnailUrl?: string | null;
  originalUrl?: string | null;
  byteSize?: number | null;
  updatedAt?: string | null;
}

export interface MediaVersionListItem {
  id: string;
  state: string;
  sha256?: string | null;
  byteSize?: number | null;
  mimeType?: string | null;
  createdAt?: string | null;
}

export interface MediaUsageListItem {
  id?: string | null;
  usedByType: string;
  usedById?: string | null;
  ownerField?: string | null;
  usageRole?: string | null;
  locatorKind?: string | null;
  locatorKey?: string | null;
}

export type SystemMediaTrashListLoader<
  TMedia extends SystemMediaTrashItem = SystemMediaTrashItem,
> = (
  fetch: FetchFn,
  token: string,
  query: QueryParams,
) => Promise<PagedListResult<TMedia>>;

export type SystemMediaTrashAction<
  TMedia extends SystemMediaTrashItem = SystemMediaTrashItem,
> = (media: TMedia, fetch: FetchFn, token: string) => Promise<void>;
