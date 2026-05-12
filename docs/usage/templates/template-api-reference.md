# Template API Reference

Status: active

Complete API reference for all template components.

## EntityListPage

[See entity-list-page.md](./entity-list-page.md)

## EntityList

[See entity-list-section.md](./entity-list-section.md)

## EntityDetailPage

[See entity-detail-page.md](./entity-detail-page.md)

## EntityDetail

[See entity-detail-section.md](./entity-detail-section.md)

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

## Types

These types are exported from `@decodelabs/underlay/templates`.

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
