import type {
  MediaSummary,
  PaginatedResponse,
  PaginationParams
} from "../../patterns/index.js";

interface LoadMediaBrowsePageInput {
  listMediaPaginated: (
    params?: PaginationParams
  ) => Promise<PaginatedResponse<MediaSummary>>;
  cursor?: string;
}

export interface LoadedMediaBrowsePage {
  items: MediaSummary[];
  nextCursor: string | null;
  hasMore: boolean;
}

export async function loadMediaBrowsePage({
  listMediaPaginated,
  cursor
}: LoadMediaBrowsePageInput): Promise<LoadedMediaBrowsePage> {
  const response = await listMediaPaginated({
    cursor: cursor ?? undefined,
    limit: 12
  });

  return {
    items: response.data,
    nextCursor: response.nextCursor,
    hasMore: response.hasMore
  };
}

export function mergeMediaBrowseItems(
  existingItems: MediaSummary[],
  nextItems: MediaSummary[],
  cursor?: string
): MediaSummary[] {
  return cursor ? [...existingItems, ...nextItems] : nextItems;
}
