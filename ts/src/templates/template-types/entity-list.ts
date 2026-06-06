import type {
  LogActionType,
  LogActor,
  LogEntry,
  TableColumn,
  TableRow,
  TableRowAction,
} from "@poodle/svelte";
import type { QueryParams } from "../../client/query";
import type {
  ReorderController,
  ReorderableItem,
} from "../../patterns/reorder-controller.svelte";
import type {
  BatchActionConfig,
  ReorderConfig,
  ReorderErrorResult,
} from "./actions";
import type {
  EntityListCapabilitiesLoader,
  EntityListDataLoader,
  FilterConfig,
  ListVariantDefinition,
} from "./list";
import type { TemplateSurface } from "./primitives";

export type TableRowActionFactory<TItem> = (
  row: TableRow<TItem>,
) => { value: string; label: string }[];

export type LogEntryMapper<TItem> = (items: TItem[]) => LogEntry[];

export type LogActionTypeResolver = (action: string) => LogActionType;

export type LogActionFormatter = (action: string) => string;

export type LogResourceTypeFormatter = (resourceType: string) => string;

export type LogActorHrefResolver = (actor: LogActor) => string;

export type LogResourceHrefResolver = (
  resourceType: string,
  resourceId: string,
  action: string,
) => string | null;

export interface EntityListSharedProps<TItem> {
  dataLoader: EntityListDataLoader<TItem>;
  reloadKey?: string | number;
  idField?: string;
  presentation: "cards" | "table" | "log";
  renderItem?: TemplateSurface;
  renderReorderItem?: TemplateSurface;
  columns?: TableColumn[];
  rowActions?: TableRowActionFactory<TItem>;
  showRowActions?: boolean;
  renderCell?: TemplateSurface;
  renderExpandedRow?: TemplateSurface;
  expandedRowIds?: string[];
  onRowActionSelect?: (row: TableRow<TItem>, action: TableRowAction) => void;
  toLogEntries?: LogEntryMapper<TItem>;
  actionIcon?: TemplateSurface;
  entryDetails?: TemplateSurface;
  getActionType?: LogActionTypeResolver;
  formatAction?: LogActionFormatter;
  formatResourceType?: LogResourceTypeFormatter;
  getActorHref?: LogActorHrefResolver;
  getResourceHref?: LogResourceHrefResolver;
  filters?: FilterConfig[];
  queryVariants?: ListVariantDefinition[];
  defaultVariantId?: string;
  capabilitiesLoader?: EntityListCapabilitiesLoader;
  batchActions?: BatchActionConfig[];
  reorder?: ReorderConfig<TItem>;
  customReorderContent?: TemplateSurface;
  onAdd?: () => void;
  addLabel?: string;
  onDataChange?: () => void;
  onSelectedIdsChange?: (ids: string[]) => void;
  query?: QueryParams;
  onQueryChange?: (query: QueryParams) => void;
  onReorderError?: (context: {
    error: unknown;
    controller: ReorderController<ReorderableItem & TItem>;
    items: Array<ReorderableItem & TItem>;
  }) =>
    | Promise<string | ReorderErrorResult | void>
    | string
    | ReorderErrorResult
    | void;
}
