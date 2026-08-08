# 052 - Soft Delete

> **Reference Implementation**: This guide includes patterns from production applications built with Underlay, demonstrating soft delete with batch restore capabilities.

Soft delete is a pattern where records are marked as deleted rather than physically removed from the database. This enables undo functionality, audit trails, and trash/recycle bin UIs.

## Conventions

Underlay standardizes soft delete with two columns:

| Column | Type | Description |
|--------|------|-------------|
| `deleted_at` | `TIMESTAMPTZ NULL` | When the record was soft-deleted (NULL = active) |
| `delete_batch_id` | `UUID NULL` | Groups related deletions for batch restore |

### Why `delete_batch_id`?

When deleting a parent entity with children (e.g., a Module with Sections, Areas, and Activities), all related records share the same `delete_batch_id`. This enables:

- **Batch restore**: Restore the entire hierarchy with one operation
- **Trash UI**: Group related deletions in a single trash entry
- **Audit trail**: Track which deletions happened together

## Database Schema

Add soft delete columns to your tables:

```sql
CREATE TABLE content.summary_item (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    -- ... other columns ...
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,              -- NULL = active
    delete_batch_id UUID                 -- Groups related deletions
);

-- Index for filtering active records efficiently
CREATE INDEX idx_summary_item_deleted
    ON content.summary_item(deleted_at)
    WHERE deleted_at IS NOT NULL;
```

### Querying Active Records

Always filter out soft-deleted records in normal queries:

```rust
// List active items only
sqlx::query_as::<_, SummaryRow>(
    r#"
    SELECT id, slug, title, created_at, updated_at
    FROM content.summary_item
    WHERE deleted_at IS NULL
    ORDER BY created_at DESC
    "#
)
.fetch_all(pool)
.await
```

### Querying Deleted Records (Trash)

For trash/recycle bin views:

```rust
// List deleted items
sqlx::query_as::<_, TrashRow>(
    r#"
    SELECT id, slug, title, deleted_at, delete_batch_id
    FROM content.summary_item
    WHERE deleted_at IS NOT NULL
    ORDER BY deleted_at DESC
    "#
)
.fetch_all(pool)
.await
```

## The `SoftDeletable` Trait

The `underlay-soft-delete` crate provides a trait-based approach for implementing soft delete operations.

### Trait Definition

```rust
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

#[async_trait]
pub trait SoftDeletable: Send + Sync {
    /// Soft delete an entity within an existing transaction.
    /// Returns `true` if deleted, `false` if not found or already deleted.
    async fn soft_delete_in_tx(
        tx: &mut Transaction<'_, Postgres>,
        id: uuid::Uuid,
        batch_id: uuid::Uuid,
    ) -> Result<bool, sqlx::Error>;
}
```

### Generic Functions

Once you implement `SoftDeletable`, you get these generic functions:

```rust
use underlay_soft_delete::{soft_delete, batch_soft_delete};

// Single item delete
let result = soft_delete::<SummaryItem>(&pool, item_id).await?;
// Returns: SoftDeleteResult::Deleted { batch_id } or SoftDeleteResult::NotFound

// Batch delete (all items share same batch_id for grouped restore)
let (count, batch_id) = batch_soft_delete::<SummaryItem>(&pool, &ids).await?;
```

### Variants with Custom Batch ID

If you need to control the batch ID (e.g., grouping across entity types):

```rust
use underlay_soft_delete::{soft_delete_with_batch_id, batch_soft_delete_with_batch_id};

let batch_id = new_delete_batch_id();

// All these deletions share the same batch_id
soft_delete_with_batch_id::<SummaryItem>(&pool, id1, batch_id).await?;
soft_delete_with_batch_id::<VideoItem>(&pool, id2, batch_id).await?;
```

## Simple Case: Single Table

For entities without cascading relationships, use the `impl_soft_deletable!` macro:

```rust
use underlay_soft_delete::impl_soft_deletable;

/// Marker type for soft-deleting summary items.
pub struct SummaryItem;

impl_soft_deletable!(SummaryItem, "content.summary_item");
```

This generates an implementation that executes:

```sql
UPDATE content.summary_item
SET deleted_at = NOW(), delete_batch_id = $2
WHERE id = $1 AND deleted_at IS NULL
```

### Usage

```rust
use underlay_soft_delete::{soft_delete, batch_soft_delete};
use crate::db::SummaryItem;

// Single delete
let result = soft_delete::<SummaryItem>(&pool, summary_id).await?;

// Batch delete
let (deleted_count, batch_id) = batch_soft_delete::<SummaryItem>(&pool, &ids).await?;
```

## Cascade Case: Parent with Children

When deleting a parent entity requires cascading to children, implement the trait manually:

### Example: Module with Cascade

A Module contains Sections, which contain Areas, which contain Outcomes, which have Activities. Deleting a Module must cascade to all descendants.

```rust
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};
use underlay_soft_delete::SoftDeletable;

/// Marker type for soft-deleting modules with cascade.
pub struct ModuleCascade;

#[async_trait]
impl SoftDeletable for ModuleCascade {
    async fn soft_delete_in_tx(
        tx: &mut Transaction<'_, Postgres>,
        id: uuid::Uuid,
        batch_id: uuid::Uuid,
    ) -> Result<bool, sqlx::Error> {
        // Delegate to internal cascade function
        soft_delete_module_cascade_inner(tx, id, batch_id).await
    }
}

/// Internal cascade logic.
async fn soft_delete_module_cascade_inner(
    tx: &mut Transaction<'_, Postgres>,
    module_id: uuid::Uuid,
    batch_id: uuid::Uuid,
) -> Result<bool, sqlx::Error> {
    // 1. Mark the module itself
    let result = sqlx::query(
        r#"
        UPDATE learning.module
        SET deleted_at = NOW(), delete_batch_id = $2
        WHERE id = $1 AND deleted_at IS NULL
        "#,
    )
    .bind(module_id)
    .bind(batch_id)
    .execute(&mut **tx)
    .await?;

    if result.rows_affected() == 0 {
        return Ok(false); // Not found or already deleted
    }

    // 2. Cascade to sections
    sqlx::query(
        r#"
        UPDATE learning.section
        SET deleted_at = NOW(), delete_batch_id = $2
        WHERE module_id = $1 AND deleted_at IS NULL
        "#,
    )
    .bind(module_id)
    .bind(batch_id)
    .execute(&mut **tx)
    .await?;

    // 3. Cascade to areas (via sections)
    sqlx::query(
        r#"
        UPDATE learning.area a
        SET deleted_at = NOW(), delete_batch_id = $2
        FROM learning.section s
        WHERE a.section_id = s.id
          AND s.module_id = $1
          AND a.deleted_at IS NULL
        "#,
    )
    .bind(module_id)
    .bind(batch_id)
    .execute(&mut **tx)
    .await?;

    // 4. Cascade to outcomes (via areas via sections)
    sqlx::query(
        r#"
        UPDATE learning.outcome o
        SET deleted_at = NOW(), delete_batch_id = $2
        FROM learning.area a
        JOIN learning.section s ON s.id = a.section_id
        WHERE o.area_id = a.id
          AND s.module_id = $1
          AND o.deleted_at IS NULL
        "#,
    )
    .bind(module_id)
    .bind(batch_id)
    .execute(&mut **tx)
    .await?;

    // 5. Cascade to activities (via outcomes)
    sqlx::query(
        r#"
        UPDATE learning.activity act
        SET deleted_at = NOW(), delete_batch_id = $2
        WHERE act.domain = 'Outcome'::learning.activity_domain
          AND act.domain_id IN (
            SELECT o.id FROM learning.outcome o
            JOIN learning.area a ON a.id = o.area_id
            JOIN learning.section s ON s.id = a.section_id
            WHERE s.module_id = $1
          )
          AND act.deleted_at IS NULL
        "#,
    )
    .bind(module_id)
    .bind(batch_id)
    .execute(&mut **tx)
    .await?;

    Ok(true)
}
```

### Usage

```rust
use underlay_soft_delete::{soft_delete, batch_soft_delete};
use crate::db::ModuleCascade;

// Single module delete (cascades to children)
let result = soft_delete::<ModuleCascade>(&pool, module_id).await?;

// Batch delete multiple modules (all share same batch_id)
let (count, batch_id) = batch_soft_delete::<ModuleCascade>(&pool, &module_ids).await?;
```

## Restore Operations

Restore clears `deleted_at` and `delete_batch_id` for all records in a batch:

```rust
/// Restore all records in a delete batch.
pub async fn restore_batch(
    pool: &PgPool,
    batch_id: Uuid,
) -> Result<RestoreBatchResult, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut restored = false;

    // Restore each table that uses soft delete
    for table in ["learning.module", "learning.section", "learning.area",
                  "learning.outcome", "learning.activity"] {
        let result = sqlx::query(&format!(
            "UPDATE {} SET deleted_at = NULL, delete_batch_id = NULL
             WHERE delete_batch_id = $1",
            table
        ))
        .bind(batch_id)
        .execute(&mut *tx)
        .await?;

        if result.rows_affected() > 0 {
            restored = true;
        }
    }

    tx.commit().await?;

    if restored {
        Ok(RestoreBatchResult::Restored)
    } else {
        Ok(RestoreBatchResult::NotFound)
    }
}
```

### Macro for Single-Table Restore

For simple cases:

```rust
use underlay_soft_delete::impl_restore_single;

impl_restore_single!(restore_summary, "content.summary_item");

// Usage:
let rows_affected = restore_summary(&pool, item_id).await?;
```

## Purge Operations

Permanently delete soft-deleted records:

```rust
use underlay_soft_delete::impl_purge_single;

impl_purge_single!(purge_summary, "content.summary_item");

// Usage: permanently delete a soft-deleted item
let rows_affected = purge_summary(&pool, item_id).await?;
```

For batch purge, implement similar logic to restore but with `DELETE` instead of `UPDATE`.

## API Layer

### Soft Delete Endpoint

```rust
pub async fn soft_delete_module(
    State(state): State<AppState>,
    Path(module_id): Path<String>,
) -> impl IntoResponse {
    let id = parse_uuid(&module_id)?;

    match soft_delete::<ModuleCascade>(&state.pool, id).await {
        Ok(SoftDeleteResult::Deleted { batch_id }) => {
            Json(json!({ "deleteBatchId": batch_id.to_string() }))
        }
        Ok(SoftDeleteResult::NotFound) => {
            (StatusCode::NOT_FOUND, "Module not found")
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Delete failed")
        }
    }
}
```

### Batch Delete Endpoint

```rust
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDeleteRequest {
    pub ids: Vec<Uuid>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDeleteResponse {
    pub deleted: i64,
    pub delete_batch_id: String,
}

pub async fn batch_delete_modules(
    State(state): State<AppState>,
    Json(req): Json<BatchDeleteRequest>,
) -> impl IntoResponse {
    if req.ids.is_empty() {
        return (StatusCode::BAD_REQUEST, "No IDs provided");
    }

    let ids: Vec<uuid::Uuid> = req.ids.iter().map(|id| id.into_inner()).collect();

    match batch_soft_delete::<ModuleCascade>(&state.pool, &ids).await {
        Ok((deleted, batch_id)) => {
            Json(BatchDeleteResponse {
                deleted,
                delete_batch_id: batch_id.to_string(),
            })
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Batch delete failed")
        }
    }
}
```

## Frontend Integration

### TypeScript Types

```typescript
interface BatchSoftDeleteRequest {
  ids: string[];
}

interface BatchSoftDeleteResponse {
  deleted: number;
  deleteBatchId: string;
}
```

### Batch Selection UI Pattern

Use `useBatchSelection` from Underlay runtime data helpers with Poodle `BulkActionBar` plus an explicit confirmation dialog:

```svelte
<script lang="ts">
  import { useBatchSelection } from "@inflatable-cookie/underlay/runtime/data";
  import { AlertDialog, BulkActionBar } from "@inflatable-cookie/poodle-svelte";

  const selection = useBatchSelection<string>();
  let isSelectionMode = $state(false);
  let showBatchDeleteConfirm = $state(false);

  async function handleBatchDelete() {
    const result = await api.batchSoftDelete({ ids: selection.selectedIds });
    toastStore.push({
      variant: "success",
      message: `Moved ${result.deleted} items to trash`
    });
    selection.clear();
    isSelectionMode = false;
    await refresh();
  }
</script>

<BulkActionBar
  selectionCount={selection.count}
  totalCount={items.length}
  actions={[{ id: "delete", label: "Delete", icon: "trash-2", tone: "danger" }]}
  showSelectAll
  on:clear={() => { selection.clear(); isSelectionMode = false; }}
  on:selectAll={() => selection.selectAll(items.map(i => i.id))}
  on:action={() => { showBatchDeleteConfirm = true; }}
/>

<AlertDialog
  open={showBatchDeleteConfirm}
  title="Delete selected modules"
  description={`Delete ${selection.count} selected module${selection.count === 1 ? "" : "s"}?`}
  confirmLabel={`Delete ${selection.count} module${selection.count === 1 ? "" : "s"}`}
  tone="danger"
  onConfirm={handleBatchDelete}
  onCancel={() => { showBatchDeleteConfirm = false; }}
/>
```

## When to Use Each Approach

| Scenario | Approach |
|----------|----------|
| Simple entity, no children | `impl_soft_deletable!` macro |
| Entity with cascade relationships | Manual `SoftDeletable` implementation |
| Batch operations on simple entities | Keep existing `WHERE id = ANY($1)` for efficiency |
| Batch operations with cascade | Use `batch_soft_delete::<T>()` with manual impl |

### Performance Note

For simple entities without cascade, the `WHERE id = ANY($1)` pattern is more efficient than looping:

```rust
// More efficient for simple batch operations (no cascade)
sqlx::query(
    "UPDATE content.summary_item
     SET deleted_at = NOW(), delete_batch_id = $2
     WHERE id = ANY($1) AND deleted_at IS NULL"
)
.bind(&ids)
.bind(batch_id)
.execute(pool)
.await?;
```

The trait-based `batch_soft_delete` loops through items, which is necessary for cascade scenarios but less efficient for simple cases. Choose the appropriate approach based on your entity's complexity.

## See Also

- **[050-database.md](./050-database.md)** - Database setup and migrations
- **[070-api-handlers.md](./070-api-handlers.md)** - API endpoint patterns
- **[097-autonomous-list-components.md](./097-autonomous-list-components.md)** - Retained selection and batch-action runtime patterns
- **Poodle list and dialog guides** - Generic visible list, bulk-action, and confirmation UI composition
