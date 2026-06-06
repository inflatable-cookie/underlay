import type {
  LoadedMediaBrowsePage,
  LoadMediaBrowsePageInput,
  MediaBrowseState,
} from "./types";

export async function loadMediaBrowsePage<TItem>({
  listPage,
  cursor,
  limit = 12,
}: LoadMediaBrowsePageInput<TItem>): Promise<LoadedMediaBrowsePage<TItem>> {
  const response = await listPage({
    cursor: cursor ?? undefined,
    limit,
  });

  return {
    items: response.data,
    nextCursor: response.nextCursor ?? null,
    hasMore: response.hasMore ?? false,
  };
}

export function mergeMediaBrowseItems<TItem>(
  existingItems: TItem[],
  nextItems: TItem[],
  cursor?: string,
): TItem[] {
  return cursor ? [...existingItems, ...nextItems] : nextItems;
}

export function createResetMediaBrowseState<TItem>(): MediaBrowseState<TItem> {
  return {
    items: [],
    nextCursor: null,
    hasMore: false,
  };
}
