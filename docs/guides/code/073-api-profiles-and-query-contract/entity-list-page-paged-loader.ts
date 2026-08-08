import type { PagedListResponse } from "@inflatable-cookie/underlay/client/types";
import { toPagedListResult, type EntityListDataLoader } from "@inflatable-cookie/underlay/templates";

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
    return toPagedListResult(response);
  };
}
