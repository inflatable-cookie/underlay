# Template API Reference

Status: active

Complete API reference for all template components.

For the shared `/system` route set in a new app, start with
[system-section-bootstrap.md](./system-section-bootstrap.md).

For agent-built admin resource families, start with
[admin-section-agent-protocol.md](./admin-section-agent-protocol.md).

## EntityListPage

[See entity-list-page.md](./entity-list-page.md)

## EntityList

[See entity-list-section.md](./entity-list-section.md)

## EntityDetailPage

[See entity-detail-page.md](./entity-detail-page.md)

## EntityDetail

[See entity-detail-section.md](./entity-detail-section.md)

## EntityInlineListModule

[See entity-inline-list-module.md](./entity-inline-list-module.md)

## EntityFormPage

[See entity-form-page.md](./entity-form-page.md)

## EntityTrashPage

[See entity-trash-page.md](./entity-trash-page.md)

## MediaUploadPage

[See media-upload-page.md](./media-upload-page.md)

## MediaDetailWorkflowPage

[See media-detail-workflow-page.md](./media-detail-workflow-page.md)

## SystemIndexPage

[See system-index-page.md](./system-index-page.md)

## AdminDashboardPage

[See admin-dashboard-page.md](./admin-dashboard-page.md)

## ErrorLogListPage

[See error-log-list-page.md](./error-log-list-page.md)

## Types

These types are exported from `@inflatable-cookie/underlay/templates`.

### FilterConfig

```typescript
interface FilterConfig {
  id: string;
  type: "search" | "select" | "date" | "number" | "sort";
  label: string;
  options?: { value: string; label: string }[];
  loadOptions?: () => Promise<{ value: string; label: string }[]>;
  placeholder?: string;
  sortFields?: { key: string; label: string; defaultDirection?: "asc" | "desc" }[];
}
```

### BatchActionConfig

```typescript
interface BatchActionConfig {
  id: string;
  label: string;
  tone?: "default" | "danger" | "warning";
  icon?: string;
  confirm?: boolean | { title: string; description: string | ((count: number) => string) };
  dialog?: BatchDialogConfig;
  handler: (ids: string[], values?: Record<string, unknown>) => Promise<void>;
}
```

### BatchDialogConfig

```typescript
interface BatchDialogConfig {
  title: string;
  content: Snippet<[BatchDialogContext]>;
}

interface BatchDialogContext {
  ids: string[];
  onSubmit: (values: Record<string, unknown>) => void;
  onCancel: () => void;
}
```

### InlineListDialogConfig

```typescript
interface InlineListDialogConfig {
  title: string;
  description?: string;
  width?: "sm" | "md" | "lg" | "xl" | "full";
  content: Snippet<[InlineListDialogContext]>;
}

interface InlineListDialogContext {
  close: () => void;
  refetch: () => Promise<void>;
}
```

### InlineListItemActionConfig

```typescript
interface InlineListItemActionConfig<T> {
  label: string;
  handler: (item: T) => void | Promise<void>;
  disabled?: boolean;
  destructive?: boolean;
  separator?: boolean;
}
```

### InlineListItemDeleteConfig

```typescript
interface InlineListItemDeleteConfig<T> {
  title: string;
  description: string;
  confirmLabel: string;
  entityLabel?: (item: T) => string | null;
  handler: (item: T) => void | Promise<void>;
}
```

### ReorderConfig

```typescript
interface ReorderConfig {
  enabled: boolean;
  handler: (orderedIds: string[]) => Promise<void>;
}
```

### DetailItemConfig

```typescript
interface DetailItemConfig {
  label: string;
  value: string | Snippet;
  description?: string;
  emptyText?: string;
  truncateValue?: boolean;
  layout?: "inline" | "stacked";
  presentation?: "simple" | "surface";
  span?: "full" | "half" | null;
}
```

### DetailMetaItemConfig

```typescript
interface DetailMetaItemConfig {
  label: string;
  value: string | Snippet;
  separator?: boolean;
}
```

### DetailTabConfig

```typescript
interface DetailTabConfig<T> {
  id: string;
  label: string;
  count?: number;
  content?: Snippet<[T]>;
  separator?: boolean;
}
```

### DetailActionConfig

```typescript
interface DetailActionConfig {
  label: string;
  tone?: "default" | "danger" | "warning";
  handler: () => void;
  confirm?: boolean | {
    title: string;
    description: string;
    confirmLabel?: string;
    cancelLabel?: string;
  };
}
```
