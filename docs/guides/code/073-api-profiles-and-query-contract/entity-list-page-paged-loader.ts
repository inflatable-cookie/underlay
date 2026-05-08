import type { PagedListResponse } from "@decodelabs/underlay/client/types";
import type { EntityListDataLoader } from "@decodelabs/underlay/templates";

interface ProjectListItem {
  id: string;
  title: string;
  status: string;
}

interface AdminCommands {
  listProjects(
    fetchFn: typeof fetch,
    token: string | null,
    query: unknown
  ): Promise<PagedListResponse<ProjectListItem>>;
}

export function createProjectsListLoader(
  adminCommands: AdminCommands
): EntityListDataLoader<ProjectListItem> {
  return async (fetchFn, token, query) => {
    const response = await adminCommands.listProjects(fetchFn, token, query);

    return {
      data: response.data,
      total: response.total,
      hasMore: response.hasMore
    };
  };
}
