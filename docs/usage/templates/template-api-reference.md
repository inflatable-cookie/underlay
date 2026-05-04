# Template API Reference

**Status:** In development

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

## EntityForm

[See entity-form-section.md](./entity-form-section.md)

## Types

### FilterConfig

```typescript
interface FilterConfig {
  id: string;
  type: "search" | "select" | "date" | "number";
  label: string;
  options?: { value: string; label: string }[];
  loadOptions?: () => Promise<{ value: string; label: string }[]>;
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

### DetailSectionConfig

```typescript
interface DetailSectionConfig {
  title: string;
  columns?: number;
  items: DetailItemConfig[];
}
```

### DetailItemConfig

```typescript
interface DetailItemConfig {
  label: string;
  value: string | Snippet;
  emptyText?: string;
}
```
