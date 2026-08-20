# 076 - Nightfire Structured Content

> **Reference Implementation**: This guide includes patterns from production applications built with Underlay. These serve as working examples of best practices.

Nightfire is a block-based structured content system for storing, validating, and rendering typed JSON content. It provides a flexible foundation for rich content fields like descriptions, summaries, and document bodies.

## Overview

Nightfire solves the problem of storing and validating structured content in database fields. Instead of storing plain text or unvalidated JSON, Nightfire provides:

- **Typed blocks** with versioned schemas
- **Validation strategies** that define what blocks are allowed
- **Block-level versioning** with registry-declared coercion
- **Generic design** so applications define their own block types

For the full consumer assembly pattern across Rust and TS, use:

- [`docs/guides/code/076-nightfire/nightfire-block-module-pattern.md`](/Users/tom/Dev/projects/underlay/docs/guides/code/076-nightfire/nightfire-block-module-pattern.md)

Use `@inflatable-cookie/underlay/nightfire/block-registration` as the primary public
home for TS block registration helpers. `nightfire/editor` still re-exports
them for compatibility, but new consumer code should prefer the explicit
block-registration subpath.

### When to Use Nightfire vs Plain Markdown

Not all rich text fields need Nightfire. Follow this convention based on database column type:

| Column Type | Content Format | Frontend Editor | Use Case |
|-------------|----------------|-----------------|----------|
| `TEXT` | Plain Markdown | `MarkdownEditor` | Simple text: learning aims, key takeaways, notes |
| `JSONB` | Nightfire JSON | `NightfireEditor` | Complex content: descriptions, article bodies, multi-block content |

**Rule of thumb**: If the field is fundamentally simple text with basic formatting (bold, italic, lists), use `TEXT` and Markdown. If it requires structured blocks, validation strategies, or complex editing, use `JSONB` and Nightfire.

See **[050-database.md](./050-database.md#rich-text-field-conventions)** for detailed guidance.

### Core Concepts

| Concept | Description |
|---------|-------------|
| **Block** | A typed content unit (e.g., paragraph, heading, image) |
| **BlockData** | The serialized form of a block with id, type, version, and data |
| **NightfireValue** | A collection of blocks with a schema identifier |
| **Strategy** | Rules defining what blocks are allowed and how many |
| **Registry** | Collections of known blocks and strategies |

### Architecture

```
                    ┌─────────────────────────────────────────┐
                    │         underlay-nightfire              │
                    │  (Generic protocol - reusable)          │
                    │                                         │
                    │  BlockData, NightfireValue, SchemaId    │
                    │  Block trait                            │
                    │  NightfireStrategy<C>, registries       │
                    │  validate_nightfire_value()             │
                    └─────────────────────────────────────────┘
                                      ▲
                                      │ depends on
                                      │
                    ┌─────────────────────────────────────────┐
                    │         Your Application                │
                    │  (App-specific blocks & categories)     │
                    │                                         │
                    │  BlockCategory enum                     │
                    │  BLOCK_REGISTRY, STRATEGY_REGISTRY      │
                    │  ParagraphBlock, ImageBlock, etc.       │
                    └─────────────────────────────────────────┘
```

---

## Installation

Add `underlay-nightfire` to your `Cargo.toml`:

```toml
[dependencies]
underlay-nightfire = { path = "../underlay/rust/crates/underlay-nightfire" }
```

---

## Core Types

### BlockData

The raw, serializable representation of a Nightfire block:

```rust
use underlay_nightfire::BlockData;

// BlockData is what gets stored in JSONB columns
let block = BlockData {
    id: "nf_...".to_string(),              // Stable block id
    r#type: "paragraph".to_string(),      // Block type identifier
    version: "initial".to_string(),        // Block implementation version
    data: serde_json::json!({              // Opaque JSON payload
        "text": "Hello world"
    }),
};
```

### SchemaId

A schema identifier following the convention `<namespace>:<context>/<field>`:

```rust
use underlay_nightfire::SchemaId;

// Create from string
let id = SchemaId::from("acow:content/summary");

// Access the string
assert_eq!(id.as_str(), "acow:content/summary");
```

Schema ID examples:
- `acow:content/summary` - Summary content for Acowtancy
- `myapp:article/body` - Article body content
- `cms:page/description` - Page description

Schema IDs are unversioned. Version lives on each block.

### NightfireValue

The top-level structure persisted in JSONB columns:

```rust
use underlay_nightfire::{NightfireValue, BlockData};

// Always `{ schema, blocks }`. Cardinality is a strategy rule.
let value = NightfireValue::single("app:content/title", block);
assert_eq!(value.blocks.len(), 1);

let value = NightfireValue::multi("app:content/body", vec![block1, block2]);
assert_eq!(value.blocks.len(), 2);
```

---

## Implementing Blocks

### The Block Trait

Implement the `Block` trait for your typed block structs:

```rust
use underlay_nightfire::Block;
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParagraphBlock {
    pub text: String,
}

impl Block for ParagraphBlock {
    const TYPE_NAME: &'static str = "paragraph";
    
    // Optional: override default version list
    // const VERSIONS: &'static [&'static str] = &["initial", "v2"];
    
    fn to_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or(Value::Null)
    }
}
```

### Exporting Blocks

Use `.export()` to convert a typed block to `BlockData`:

```rust
let block = ParagraphBlock {
    text: "Hello world".to_string(),
};

// Export generates BlockData with a stable id
let data: BlockData = block.export();

assert_eq!(data.r#type, "paragraph");
assert_eq!(data.version, "initial");
assert!(data.id.starts_with("nf_"));
```

### Block Versioning

Blocks support versioning for schema evolution:

```rust
impl Block for ParagraphBlock {
    const TYPE_NAME: &'static str = "paragraph";
    const VERSIONS: &'static [&'static str] = &["v2", "initial"];
    
    fn to_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or(Value::Null)
    }
}

// Current implementation is the first in the list; older entries stay readable
assert_eq!(ParagraphBlock::active_version(), "v2");
assert_eq!(ParagraphBlock::versions().coerce("initial"), Some("v2"));
assert_eq!(ParagraphBlock::versions().coerce("9"), None);
```

### Block Module Rule

Keep each real block family in one module set that owns:

- the Rust payload type
- block registration bundle
- strategy participation
- TS block registration object for editor, renderer, validator, and empty-state
  behavior
- Rust media registration when the block references media

Do not let one of those live in a separate app-wide heuristics file if it is
really block-specific behavior. Use the block-module pattern:

- [`docs/guides/code/076-nightfire/nightfire-block-module-pattern.md`](/Users/tom/Dev/projects/underlay/docs/guides/code/076-nightfire/nightfire-block-module-pattern.md)

---

## Categories and Registries

### Defining Block Categories

Define your application's block categories as an enum:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockCategory {
    Text,
    Media,
    Layout,
    Interactive,
}
```

### Block Registry

Register your block types with their categories:

```rust
use underlay_nightfire::{BlockRegistry, BlockDescriptor};

pub static BLOCK_REGISTRY: Lazy<BlockRegistry<BlockCategory>> = Lazy::new(|| {
    let mut registry = BlockRegistry::new();
    
    registry.register(BlockDescriptor::new(
        "paragraph",
        "Paragraph",
        BlockCategory::Text,
    ));
    
    registry.register(BlockDescriptor::new(
        "heading",
        "Heading",
        BlockCategory::Text,
    ));
    
    registry.register(BlockDescriptor::new(
        "image",
        "Image",
        BlockCategory::Media,
    ));
    
    registry
});

// Look up a block descriptor
let desc = BLOCK_REGISTRY.get("paragraph").unwrap();
assert_eq!(desc.label, "Paragraph");
assert_eq!(desc.category, BlockCategory::Text);

// Iterate all blocks
for block in BLOCK_REGISTRY.all() {
    println!("{}: {:?}", block.type_name, block.category);
}
```

### Strategy Registry

Register validation strategies for your schemas:

```rust
use underlay_nightfire::{
    StrategyRegistry, NightfireStrategy, StrategyCardinality, MultiConfig, SchemaId,
};
use std::num::NonZeroUsize;

pub static STRATEGY_REGISTRY: Lazy<StrategyRegistry<BlockCategory>> = Lazy::new(|| {
    let mut registry = StrategyRegistry::new();
    
    // Single-block strategy (exactly one block)
    registry.register(NightfireStrategy {
        id: SchemaId::from("myapp:content/title@1"),
        cardinality: StrategyCardinality::Single,
        allowed_types: vec![],
        allowed_categories: vec![BlockCategory::Text],
        default_type: "heading".to_string(),
    });
    
    // Multi-block strategy (1 or more blocks)
    registry.register(NightfireStrategy {
        id: SchemaId::from("myapp:content/body@1"),
        cardinality: StrategyCardinality::Multi(MultiConfig {
            min_blocks: NonZeroUsize::new(1).unwrap(),
            max_blocks: None,  // No upper limit
        }),
        allowed_types: vec![],
        allowed_categories: vec![BlockCategory::Text, BlockCategory::Media],
        default_type: "paragraph".to_string(),
    });
    
    // Bounded multi-block (1-5 blocks)
    registry.register(NightfireStrategy {
        id: SchemaId::from("myapp:content/gallery@1"),
        cardinality: StrategyCardinality::Multi(MultiConfig {
            min_blocks: NonZeroUsize::new(1).unwrap(),
            max_blocks: Some(5),
        }),
        allowed_types: vec!["image".to_string()],  // Explicit type list
        allowed_categories: vec![],
        default_type: "image".to_string(),
    });
    
    registry
});

// Look up a strategy
let strategy = STRATEGY_REGISTRY.get_by_str("myapp:content/body@1").unwrap();
```

---

## Validation

### Basic Validation

Validate a `NightfireValue` against a strategy:

```rust
use underlay_nightfire::{validate_nightfire_value, NightfireValidationError};

let value = NightfireValue::single("myapp:content/title@1", block);
let strategy = STRATEGY_REGISTRY.get_by_str("myapp:content/title@1").unwrap();

match validate_nightfire_value(&value, strategy, &BLOCK_REGISTRY) {
    Ok(()) => println!("Valid!"),
    Err(NightfireValidationError::CardinalityMismatch { .. }) => {
        println!("Wrong number of blocks");
    }
    Err(NightfireValidationError::DisallowedBlockType { block_type, .. }) => {
        println!("Block type {} not allowed", block_type);
    }
    Err(NightfireValidationError::UnknownBlockType { block_type, .. }) => {
        println!("Unknown block type: {}", block_type);
    }
    Err(NightfireValidationError::UnknownStrategy { schema }) => {
        println!("No strategy for schema: {}", schema);
    }
}
```

### Registry-Based Validation

Use `StrategyRegistry::validate()` to automatically look up the strategy:

```rust
// Validates using the value's schema to find the strategy
match STRATEGY_REGISTRY.validate(&value, &BLOCK_REGISTRY) {
    Ok(()) => println!("Valid!"),
    Err(e) => println!("Validation failed: {:?}", e),
}
```

### Validation Errors

| Error | Description |
|-------|-------------|
| `CardinalityMismatch` | Wrong number of blocks or single vs multi mismatch |
| `DisallowedBlockType` | Block type not in allowed_types or allowed_categories |
| `UnknownBlockType` | Block type not registered in BlockRegistry |
| `UnknownStrategy` | No strategy registered for the schema |

### Creating Convenience Wrappers

For cleaner API usage, create application-specific wrappers:

```rust
/// Validate using our static registries.
pub fn validate_nightfire_value(
    value: &NightfireValue,
    strategy: &NightfireStrategy<BlockCategory>,
) -> Result<(), NightfireValidationError> {
    underlay_nightfire::validate_nightfire_value(value, strategy, &BLOCK_REGISTRY)
}

/// Validate by looking up the strategy from the value's schema.
pub fn validate_nightfire_value_by_schema(
    value: &NightfireValue,
) -> Result<(), NightfireValidationError> {
    STRATEGY_REGISTRY.validate(value, &BLOCK_REGISTRY)
}
```

---

## Content Hashing

Hash is not part of the Nightfire envelope. `compute_block_hash` remains as a
utility for app-specific change detection (for example exam-question pairing)
when a payload contract needs it.

```rust
use underlay_nightfire::compute_block_hash;
use serde_json::json;

let data = json!({"text": "Hello world"});
let hash = compute_block_hash(&data);

let hash2 = compute_block_hash(&data);
assert_eq!(hash, hash2);
```

---

## JSON Wire Format

Every Nightfire value uses one envelope. Cardinality is a strategy rule.

```json
{
  "schema": "myapp:content/body",
  "blocks": [
    {
      "id": "nf_...",
      "type": "paragraph",
      "version": "initial",
      "data": {
        "text": "First paragraph..."
      }
    },
    {
      "id": "nf_...",
      "type": "image",
      "version": "initial",
      "data": {
        "src": "https://example.com/image.jpg",
        "alt": "An example image"
      }
    }
  ]
}
```

A single-block strategy still uses `blocks` with `len == 1`. The v1 `{ block }`
field and envelope `hash` are rejected.

---

## API Integration

### Handler Example

```rust
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use underlay_core::AppError;

#[derive(Deserialize)]
struct UpdateContentRequest {
    content: NightfireValue,
}

async fn update_content(
    State(state): State<AppState>,
    Json(payload): Json<UpdateContentRequest>,
) -> impl IntoResponse {
    // Validate content
    if let Err(e) = validate_nightfire_value_by_schema(&payload.content) {
        return match e {
            NightfireValidationError::CardinalityMismatch { expected, actual_blocks, .. } => {
                underlay_http::error_response(
                    StatusCode::BAD_REQUEST,
                    AppError::new("content.cardinality", format!(
                        "Expected {:?}, got {} blocks", expected, actual_blocks
                    )),
                )
            }
            NightfireValidationError::DisallowedBlockType { block_type, .. } => {
                underlay_http::error_response(
                    StatusCode::BAD_REQUEST,
                    AppError::new("content.invalid_block", format!(
                        "Block type '{}' not allowed", block_type
                    )),
                )
            }
            NightfireValidationError::UnknownBlockType { block_type, .. } => {
                underlay_http::error_response(
                    StatusCode::BAD_REQUEST,
                    AppError::new("content.unknown_block", format!(
                        "Unknown block type: {}", block_type
                    )),
                )
            }
            NightfireValidationError::UnknownStrategy { schema } => {
                underlay_http::error_response(
                    StatusCode::BAD_REQUEST,
                    AppError::new("content.unknown_schema", format!(
                        "Unknown schema: {}", schema
                    )),
                )
            }
        };
    }
    
    // Store content...
    (StatusCode::OK, Json(serde_json::json!({"status": "ok"}))).into_response()
}
```

### DTO Example

```rust
use underlay_nightfire::{NightfireValue, BlockData, SchemaId};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleDto {
    pub id: String,
    pub title: String,
    pub body: NightfireValue,
}

// NightfireValue serializes directly to JSON
let dto = ArticleDto {
    id: "article-123".to_string(),
    title: "My Article".to_string(),
    body: NightfireValue::multi("myapp:content/body@1", vec![
        ParagraphBlock { text: "Hello".to_string() }.export(),
    ]),
};
```

---

## Database Storage

### JSONB Column

Store `NightfireValue` in PostgreSQL JSONB columns:

```sql
CREATE TABLE articles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    body JSONB NOT NULL,  -- Stores NightfireValue
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

### Reading and Writing

```rust
// Writing
let body = NightfireValue::multi("myapp:content/body@1", vec![...]);
let body_json = serde_json::to_value(&body)?;

sqlx::query!(
    "INSERT INTO articles (title, body) VALUES ($1, $2)",
    title,
    body_json
)
.execute(&pool)
.await?;

// Reading
let row = sqlx::query!(
    "SELECT id, title, body FROM articles WHERE id = $1",
    article_id
)
.fetch_one(&pool)
.await?;

let body: NightfireValue = serde_json::from_value(row.body)?;
```

---

## Helper Functions

### Creating Markdown Content

```rust
use underlay_nightfire::{NightfireValue, SchemaId};

/// Create a multi-block value with a single markdown block.
pub fn markdown_blocks<S: Into<SchemaId>, T: Into<String>>(
    schema: S, 
    text: T
) -> NightfireValue {
    let block = MarkdownBlock { text: text.into() }.export();
    NightfireValue::multi(schema, vec![block])
}

// Usage
let description = markdown_blocks(
    "myapp:content/description@1", 
    "This is a **markdown** description."
);
```

---

## Design Philosophy

### 1. Generic Over Category

The `C` type parameter allows applications to define their own block categories:

```rust
// Your app's categories
enum MyCategory { Text, Media, Custom }

// Use with Nightfire types
BlockRegistry<MyCategory>
StrategyRegistry<MyCategory>
NightfireStrategy<MyCategory>
```

### 2. Instance-Based Registries

Registries are instances rather than globals, enabling:
- Testing with isolated registries
- Multiple registry configurations
- Dependency injection

```rust
// Production: static registries
static BLOCK_REGISTRY: Lazy<BlockRegistry<...>> = ...;

// Testing: fresh registries
let mut registry = BlockRegistry::new();
registry.register(...);
```

### 3. Separation of Concerns

- **underlay-nightfire**: Generic protocol (blocks, values, validation)
- **Your app**: Specific block types, categories, registries

This allows multiple applications to use Nightfire with different content models.

### 4. Block Version Coercion

The registry declares supported versions per block type. Readers resolve any
supported stored version to the current implementation. Unknown versions fail
closed.

---

## Complete Example

Here's a complete example showing all the pieces together:

```rust
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::num::NonZeroUsize;
use underlay_nightfire::{
    Block, BlockData, BlockDescriptor, BlockRegistry, MultiConfig,
    NightfireStrategy, NightfireValue, NightfireValidationError,
    SchemaId, StrategyCardinality, StrategyRegistry,
};

// 1. Define your categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockCategory {
    Text,
    Media,
}

// 2. Define your block types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParagraphBlock {
    pub text: String,
}

impl Block for ParagraphBlock {
    const TYPE_NAME: &'static str = "paragraph";
    fn to_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or(Value::Null)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageBlock {
    pub src: String,
    pub alt: Option<String>,
}

impl Block for ImageBlock {
    const TYPE_NAME: &'static str = "image";
    fn to_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or(Value::Null)
    }
}

// 3. Create registries
pub static BLOCK_REGISTRY: Lazy<BlockRegistry<BlockCategory>> = Lazy::new(|| {
    let mut r = BlockRegistry::new();
    r.register(BlockDescriptor::new(
        "paragraph",
        "Paragraph",
        BlockCategory::Text,
    ));
    r.register(BlockDescriptor::new(
        "image",
        "Image",
        BlockCategory::Media,
    ));
    r
});

pub static STRATEGY_REGISTRY: Lazy<StrategyRegistry<BlockCategory>> = Lazy::new(|| {
    let mut r = StrategyRegistry::new();
    r.register(NightfireStrategy {
        id: SchemaId::from("myapp:content/body@1"),
        cardinality: StrategyCardinality::Multi(MultiConfig {
            min_blocks: NonZeroUsize::new(1).unwrap(),
            max_blocks: None,
        }),
        allowed_types: vec![],
        allowed_categories: vec![BlockCategory::Text, BlockCategory::Media],
        default_type: "paragraph".to_string(),
    });
    r
});

// 4. Create and validate content
fn main() {
    let content = NightfireValue::multi(
        "myapp:content/body@1",
        vec![
            ParagraphBlock { text: "Hello world!".to_string() }.export(),
            ImageBlock { 
                src: "https://example.com/image.jpg".to_string(),
                alt: Some("Example".to_string()),
            }.export(),
        ],
    );
    
    match STRATEGY_REGISTRY.validate(&content, &BLOCK_REGISTRY) {
        Ok(()) => println!("Content is valid!"),
        Err(e) => println!("Validation error: {:?}", e),
    }
    
    // Serialize for storage/transmission
    let json = serde_json::to_string_pretty(&content).unwrap();
    println!("{}", json);
}
```

---

## Frontend Implementation (TypeScript/Svelte)

Underlay provides TypeScript components for editing and rendering Nightfire content in Svelte applications.

### Installation

Import Nightfire components from explicit Nightfire subpaths:

```typescript
import { NightfireEditor, type NightfireValue } from "@inflatable-cookie/underlay/nightfire/editor";
import { NightfireRenderer } from "@inflatable-cookie/underlay/nightfire/renderer";
import {
  configureNightfireStrategies,
  createNightfireStrategiesContext,
  useNightfireStrategies
} from "@inflatable-cookie/underlay/nightfire/strategies";
```

The public Nightfire subpaths are:

- `@inflatable-cookie/underlay/nightfire/editor`
- `@inflatable-cookie/underlay/nightfire/renderer`
- `@inflatable-cookie/underlay/nightfire/block-editor`
- `@inflatable-cookie/underlay/nightfire/block-registration`
- `@inflatable-cookie/underlay/nightfire/markdown`
- `@inflatable-cookie/underlay/nightfire/editor-registry`
- `@inflatable-cookie/underlay/nightfire/render-registry`
- `@inflatable-cookie/underlay/nightfire/validator-registry`
- `@inflatable-cookie/underlay/nightfire/strategies`
- `@inflatable-cookie/underlay/nightfire/media`
- `@inflatable-cookie/underlay/nightfire/utils`
- `@inflatable-cookie/underlay/nightfire/validation`

---

## Strategies: Lazy Loading and Caching

Nightfire strategies define what blocks are allowed in each schema. Rather than loading strategies on every page, Underlay provides a lazy-loading system that:

1. **Loads on demand** - Strategies are only fetched when a `NightfireEditor` is rendered
2. **Caches automatically** - Once loaded, strategies are cached for 1 hour
3. **Deduplicates requests** - Multiple concurrent editors share a single fetch request
4. **Supports invalidation** - Cache can be manually cleared when strategies change

### Configuration

Configure the strategies fetcher once in your app's root layout:

```svelte
<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import {
    configureNightfireStrategies,
    createNightfireStrategiesContext
  } from "@inflatable-cookie/underlay/nightfire/strategies";
  import { nightfireCommands } from "@my-app/api";
  import { auth } from "$lib/stores/auth";

  // Configure how strategies are fetched (called once at app startup)
  configureNightfireStrategies({
    fetchStrategies: async () => {
      const token = auth.getToken();
      if (!token) return [];
      return nightfireCommands.listStrategies(fetch, token, { includeOptions: true });
    },
    cacheTtl: 60 * 60 * 1000  // Optional: 1 hour (default)
  });

  // Create the context (makes strategies available to child components)
  createNightfireStrategiesContext();

  let { children } = $props();
</script>

{@render children()}
```

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `fetchStrategies` | `() => Promise<NightfireStrategy[]>` | **Required** | Async function that fetches strategies from your API |
| `cacheTtl` | `number` | `3600000` (1 hour) | Cache time-to-live in milliseconds |

### Strategy Types

```typescript
interface NightfireStrategy {
  id: string;                           // Schema ID (e.g., "myapp:content/body@1")
  cardinality: NightfireStrategyCardinality;
  allowedTypes: string[];               // Explicit block type whitelist
  allowedCategories: string[];          // Category-based whitelist
  defaultType: string;                  // Default block type for new blocks
  blockOptions?: NightfireBlockOption[]; // Pre-computed block options for UI
}

interface NightfireStrategyCardinality {
  mode: "single" | "multi";
  minBlocks?: number;
  maxBlocks?: number | null;
}

interface NightfireBlockOption {
  type: string;
  label: string;
  category?: string;
}
```

---

## NightfireEditor Component

The `NightfireEditor` component provides a block-based editor for Nightfire content. It automatically loads strategies from context and handles normalisation.

### Basic Usage

```svelte
<script lang="ts">
  import { NightfireEditor, type NightfireValue } from "@inflatable-cookie/underlay/nightfire/editor";

  let description = $state<NightfireValue>({ schema: "myapp:content/description@1" });
  let prepare = $state<(formData: FormData) => void>(() => {});
</script>

<form>
  <NightfireEditor
    name="description"
    schema="myapp:content/description@1"
    bind:value={description}
    bind:prepare
  />
</form>
```

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `name` | `string` | **Required** | Form field name for the hidden input |
| `schema` | `string` | **Required** | Strategy ID to use (e.g., `"myapp:content/body@1"`) |
| `value` | `NightfireValue` | **Required** | Bindable value containing the content |
| `prepare` | `(formData: FormData) => void` | **Required** | Bindable function to write the prepared Nightfire payload before form submission |
| `required` | `boolean` | `false` | Whether the field is required |
| `disabled` | `boolean` | `false` | Whether the editor is disabled |
| `modeOverride` | `"single" \| "multi"` | `null` | Override the strategy's cardinality mode |
| `blockOptionsOverride` | `NightfireBlockOption[]` | `null` | Override the available block types |
| `onSchemaMismatch` | `(info: SchemaMismatchInfo) => void` | `null` | Callback when loaded content has a different schema |
| `slashCommands` | `NightfireSlashCommandsConfig` | `null` | Opt-in slash-command palette for multi-block editors |

### Opt-In Slash Commands

Slash commands are available for multi-block `NightfireEditor` fields, but the first Underlay release keeps them disabled by default so existing editors do not change keyboard behavior on upgrade.

When enabled:

1. Type `/` inside a markdown block to open the palette.
2. Pick a command with arrow keys, Enter, or click.
3. Underlay removes the slash token from the markdown block and inserts the chosen block immediately below it.

```svelte
<script lang="ts">
  import {
    NightfireEditor,
    type NightfireSlashCommandsConfig,
    type NightfireValue
  } from "@inflatable-cookie/underlay/nightfire/editor";

  let body = $state<NightfireValue>({ schema: "myapp:content/body@1" });

  const slashCommands: NightfireSlashCommandsConfig = {
    enabled: true,
    commands: [
      {
        type: "media",
        aliases: ["image", "photo"],
        description: "Insert a reusable media block."
      }
    ]
  };
</script>

<NightfireEditor
  name="body"
  schema="myapp:content/body@1"
  bind:value={body}
  {slashCommands}
/>
```

Notes:

- The default command list is derived from the block types registered for the active schema.
- Custom commands are metadata overrides for existing registered block types; this batch does not add arbitrary app-specific command actions.
- The palette is only shown for markdown blocks in multi-block editors, because single-block fields do not support block insertion and non-text blocks do not have slash-entry semantics.

### Form Integration

The `prepare` function must be called before form submission to write the
canonical save payload:

- validated Nightfire JSON
- stable block ids
- verbatim inner block `data` keys

```svelte
<script lang="ts">
  import { NightfireEditor, type NightfireValue } from "@inflatable-cookie/underlay/nightfire/editor";

  let body = $state<NightfireValue>({ schema: "myapp:content/body@1" });
  let bodyPrepare = $state<(formData: FormData) => void>(() => {});

  async function handleSubmit(formData: FormData) {
    // Call prepare to write the prepared Nightfire payload to the form.
    bodyPrepare(formData);

    // Now formData contains the validated, block-id-stable JSON.
    const bodyJson = formData.get("body");
    // ...submit to API
  }
</script>

<form onsubmit={handleSubmit}>
  <NightfireEditor
    name="body"
    schema="myapp:content/body@1"
    bind:value={body}
    bind:prepare={bodyPrepare}
  />
  <button type="submit">Save</button>
</form>
```

### Multiple Editors

When a form has multiple Nightfire fields, combine the prepare functions:

```svelte
<script lang="ts">
  let description = $state<NightfireValue>({ schema: "myapp:content/markup@1" });
  let body = $state<NightfireValue>({ schema: "myapp:content/body@1" });

  let prepareDescription = $state<(formData: FormData) => void>(() => {});
  let prepareBody = $state<(formData: FormData) => void>(() => {});

  // Combined prepare function
  let prepare = $derived(() => (formData: FormData) => {
    prepareDescription(formData);
    prepareBody(formData);
  });
</script>

<NightfireEditor
  name="description"
  schema="myapp:content/markup@1"
  bind:value={description}
  bind:prepare={prepareDescription}
/>

<NightfireEditor
  name="body"
  schema="myapp:content/body@1"
  bind:value={body}
  bind:prepare={prepareBody}
/>
```

Boundary rule for all of these form patterns:

- map outer DTO field names to API `snake_case` if needed
- do not remap keys inside the Nightfire JSON itself
- fields like `image_id` and `attachment_id` must reach the API unchanged or
  shared media extractors will stop matching them

---

## Schema Normalisation

When editing existing content, the stored schema may differ from the expected schema (e.g., after a schema version upgrade). `NightfireEditor` handles this automatically:

1. Loads the content with its original schema
2. Coerces it to match the strategy's cardinality (single vs multi)
3. Updates the schema to the expected version
4. Optionally notifies via the `onSchemaMismatch` callback

### Schema Mismatch Callback

```svelte
<script lang="ts">
  import { NightfireEditor, type NightfireValue, type SchemaMismatchInfo } from "@inflatable-cookie/underlay/nightfire/editor";

  let description = $state<NightfireValue>({ schema: "myapp:content/markup@1" });
  let schemaMismatch = $state<SchemaMismatchInfo | null>(null);

  function handleSchemaMismatch(info: SchemaMismatchInfo) {
    schemaMismatch = info;
    console.log(`Schema mismatch: ${info.actualSchema} → ${info.expectedSchema}`);
  }
</script>

{#if schemaMismatch}
  <p class="warning">
    This content uses legacy schema
    <code>{schemaMismatch.actualSchema ?? "unspecified"}</code>
    and will be saved using <code>{schemaMismatch.expectedSchema}</code>.
  </p>
{/if}

<NightfireEditor
  name="description"
  schema="myapp:content/markup@1"
  bind:value={description}
  bind:prepare={prepare}
  onSchemaMismatch={handleSchemaMismatch}
/>
```

### SchemaMismatchInfo Type

```typescript
interface SchemaMismatchInfo {
  actualSchema: string | null;   // The schema found in the content (null if missing)
  expectedSchema: string;        // The schema specified in the editor's props
}
```

---

## Manual Strategy Access

For advanced use cases, you can access the strategies store directly:

```svelte
<script lang="ts">
  import { useNightfireStrategies } from "@inflatable-cookie/underlay/nightfire/strategies";
  import { onMount } from "svelte";

  const strategiesStore = useNightfireStrategies();

  onMount(async () => {
    if (strategiesStore) {
      // Ensure strategies are loaded
      await strategiesStore.ensure();

      // Find a specific strategy
      const bodyStrategy = strategiesStore.findById("myapp:content/body@1");
      console.log("Body strategy:", bodyStrategy);
    }
  });
</script>
```

### Store API

```typescript
interface NightfireStrategiesStore {
  strategies: Readable<NightfireStrategy[]>;  // Svelte store of all strategies
  loading: Readable<boolean>;                  // Whether currently fetching
  error: Readable<string | null>;              // Last error message

  ensure(): Promise<NightfireStrategy[]>;      // Load if not cached, return strategies
  refresh(): Promise<NightfireStrategy[]>;     // Force reload, ignoring cache
  invalidate(): void;                          // Clear cache (next ensure() will fetch)
  findById(id: string): NightfireStrategy | null;  // Find strategy by schema ID
}
```

### Cache Invalidation

When strategies are updated (e.g., via an admin interface), invalidate the cache:

```typescript
const strategiesStore = useNightfireStrategies();

async function handleStrategyUpdated() {
  // Invalidate cache so next editor load fetches fresh data
  strategiesStore?.invalidate();

  // Or force immediate refresh
  await strategiesStore?.refresh();
}
```

---

## NightfireRenderer Component

For read-only display of Nightfire content:

```svelte
<script lang="ts">
  import { NightfireRenderer, type NightfireValue } from "@inflatable-cookie/underlay/nightfire/renderer";

  interface Props {
    content: NightfireValue;
  }

  let { content }: Props = $props();
</script>

<NightfireRenderer value={content} />
```

---

## NightfireValue Utilities

### Checking for Empty Content

```typescript
import { isEmptyNightfire, type NightfireValue } from "@inflatable-cookie/underlay/nightfire/utils";

const value: NightfireValue = { schema: "myapp:content/body", blocks: [] };

if (isEmptyNightfire(value)) {
  console.log("No content");
}
```

### Normalising Values

```typescript
import { normaliseNightfireValue, type NightfireValue } from "@inflatable-cookie/underlay/nightfire/utils";

// Normalise a stored or raw value onto `{ schema, blocks }`
const normalised = normaliseNightfireValue(
  existingValue,
  "myapp:content/body"
);
```

### Preparing for Save

```typescript
import {
  prepareNightfireForSave,
  writePreparedNightfireToFormData,
  type NightfireValue
} from "@inflatable-cookie/underlay/nightfire/validation";

// Validates blocks, coerces supported versions, assigns missing ids
const prepared = prepareNightfireForSave(value);

const formData = new FormData();
writePreparedNightfireToFormData(formData, "body", value);
```

### Writing to FormData

```typescript
import { writeNightfireToFormData, type NightfireValue } from "@inflatable-cookie/underlay/nightfire/utils";

const formData = new FormData();
writeNightfireToFormData(formData, "body", value);
```

Use `writeNightfireToFormData()` only when you deliberately want to serialize
the current editor-local draft value as-is. For normal save paths, prefer
`writePreparedNightfireToFormData()` so block ids are stable before persistence.

Important boundary rule:

- outer DTO field names may still be mapped to API `snake_case`
- inner Nightfire JSON must stay verbatim
- do not rename keys inside block `data` objects on the way to the API.
  Payloads are snake_case (`image_id`); extractors and locators match the
  stored JSON exactly

---

## Complete Frontend Example

Here's a complete example of a form with Nightfire content:

```svelte
<!-- ArticleForm.svelte -->
<script lang="ts">
  import { Field } from "@inflatable-cookie/poodle-svelte";
  import { NightfireEditor, type NightfireValue } from "@inflatable-cookie/underlay/nightfire/editor";

  interface Props {
    values: { title: string; body: NightfireValue };
    errors?: Record<string, string> | null;
    onSubmit: (data: { title: string; body: NightfireValue }) => Promise<void>;
  }

  let { values, errors = null, onSubmit }: Props = $props();

  let title = $state(values.title);
  let body = $state<NightfireValue>(values.body);
  let bodyPrepare = $state<(formData: FormData) => void>(() => {});
  let submitting = $state(false);

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    submitting = true;

    try {
      const formData = new FormData(e.target as HTMLFormElement);
      bodyPrepare(formData);

      const bodyJson = formData.get("body") as string;
      const bodyValue = JSON.parse(bodyJson) as NightfireValue;

      await onSubmit({ title, body: bodyValue });
    } finally {
      submitting = false;
    }
  }
</script>

<form onsubmit={handleSubmit}>
  <Field label="Title" error={errors?.title} required>
    <input type="text" name="title" bind:value={title} required />
  </Field>

  <Field label="Body" error={errors?.body} required>
    <NightfireEditor
      name="body"
      schema="myapp:content/body@1"
      bind:value={body}
      bind:prepare={bodyPrepare}
      required
    />
  </Field>

  <button type="submit" disabled={submitting}>
    {submitting ? "Saving..." : "Save"}
  </button>
</form>
```

---

## See Also

**Related Guides:**
- **[070-api-handlers.md](./070-api-handlers.md)** - Using Nightfire in API handlers
- **[075-validation.md](./075-validation.md)** - Request validation patterns
- **[050-database.md](./050-database.md)** - Storing JSONB content
- **Poodle guides** - Generic UI composition outside retained Nightfire-specific runtime/editor behavior

**Crate Documentation:**
- `underlay-nightfire/README.md` - Quick reference
- `underlay-nightfire/src/lib.rs` - Module documentation
