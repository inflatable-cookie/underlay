import type { PagedListResponse } from "@decodelabs/underlay/client/types";
import { toPagedListResult, type EntityListDataLoader } from "@decodelabs/underlay/templates";

interface TaskListItem {
  id: string;
  title: string;
  status: string;
}

interface AdminCommands {
  listProjectTasks(
    projectId: string,
    fetchFn: typeof fetch,
    token: string | null,
    query: unknown
  ): Promise<PagedListResponse<TaskListItem>>;
}

export function createProjectTasksTabLoader(
  adminCommands: AdminCommands,
  projectId: string
): EntityListDataLoader<TaskListItem> {
  return async (fetchFn, token, query) => {
    const response = await adminCommands.listProjectTasks(projectId, fetchFn, token, query);
    return toPagedListResult(response);
  };
}
