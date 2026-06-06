export type MediaWorkflowPaginationParams = {
  cursor?: string;
  limit?: number;
};

export type MediaWorkflowPageResponse<TItem> = {
  data: TItem[];
  nextCursor?: string | null;
  hasMore?: boolean;
};

export type MediaBrowseState<TItem> = {
  items: TItem[];
  nextCursor: string | null;
  hasMore: boolean;
};

export type LoadedMediaBrowsePage<TItem> = {
  items: TItem[];
  nextCursor: string | null;
  hasMore: boolean;
};

export type LoadMediaBrowsePageInput<TItem> = {
  listPage: (
    params?: MediaWorkflowPaginationParams,
  ) => Promise<MediaWorkflowPageResponse<TItem>>;
  cursor?: string;
  limit?: number;
};
