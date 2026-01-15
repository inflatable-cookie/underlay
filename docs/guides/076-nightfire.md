# 076 - Nightfire Structured Content

> **Reference Implementation**: This guide includes patterns from production applications built with Underlay. These serve as working examples of best practices.

Nightfire is a block-based structured content system for storing, validating, and rendering typed JSON content. It provides a flexible foundation for rich content fields like descriptions, summaries, and document bodies.

## Overview

Nightfire solves the problem of storing and validating structured content in database fields. Instead of storing plain text or unvalidated JSON, Nightfire provides:

- **Typed blocks** with versioned schemas
- **Validation strategies** that define what blocks are allowed
- **Content hashing** for change detection
- **Generic design** so applications define their own block types

### Core Concepts

| Concept | Description |
|---------|-------------|
| **Block** | A typed content unit (e.g., paragraph, heading, image) |
| **BlockData** | The serialized form of a block with type, version, hash, and data |
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
    r#type: "paragraph".to_string(),      // Block type identifier
    version: "initial".to_string(),        // Schema version
    hash: "abc123...".to_string(),         // Content hash
    data: serde_json::json!({              // Opaque JSON payload
        "text": "Hello world"
    }),
};
```

### SchemaId

A schema identifier following the convention `<namespace>:<context>/<field>@<version>`:

```rust
use underlay_nightfire::SchemaId;

// Create from string
let id = SchemaId::from("acow:content/summary@1");

// Access the string
assert_eq!(id.as_str(), "acow:content/summary@1");
```

Schema ID examples:
- `acow:content/summary@1` - Summary content for Acowtancy
- `myapp:article/body@1` - Article body content
- `cms:page/description@2` - Page description (version 2)

### NightfireValue

The top-level structure persisted in JSONB columns:

```rust
use underlay_nightfire::{NightfireValue, BlockData};

// Single-block value
let value = NightfireValue::single("app:content/title@1", block);
assert!(value.is_single());

// Multi-block value
let value = NightfireValue::multi("app:content/body@1", vec![block1, block2]);
assert!(value.is_multi());
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

// Export generates BlockData with computed hash
let data: BlockData = block.export();

assert_eq!(data.r#type, "paragraph");
assert_eq!(data.version, "initial");
assert!(!data.hash.is_empty());  // Hash computed from data
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

// Active version is the first in the list
assert_eq!(ParagraphBlock::active_version(), "v2");
```

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
    
    registry.register(BlockDescriptor {
        type_name: "paragraph",
        label: "Paragraph",
        category: BlockCategory::Text,
    });
    
    registry.register(BlockDescriptor {
        type_name: "heading",
        label: "Heading",
        category: BlockCategory::Text,
    });
    
    registry.register(BlockDescriptor {
        type_name: "image",
        label: "Image",
        category: BlockCategory::Media,
    });
    
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

Nightfire computes content hashes for change detection:

```rust
use underlay_nightfire::compute_block_hash;
use serde_json::json;

let data = json!({"text": "Hello world"});
let hash = compute_block_hash(&data);

// Hash is deterministic
let hash2 = compute_block_hash(&data);
assert_eq!(hash, hash2);

// Different content produces different hash
let different = json!({"text": "Goodbye"});
let hash3 = compute_block_hash(&different);
assert_ne!(hash, hash3);
```

Use cases for content hashing:
- Detecting when content has changed during a user session
- Cache invalidation
- Audit trails and versioning
- Optimistic concurrency control

---

## JSON Wire Format

### Single-Block Format

```json
{
  "schema": "myapp:content/title@1",
  "block": {
    "type": "heading",
    "version": "initial",
    "hash": "3b8a7d2f...",
    "data": {
      "text": "My Article Title",
      "level": 1
    }
  }
}
```

### Multi-Block Format

```json
{
  "schema": "myapp:content/body@1",
  "blocks": [
    {
      "type": "paragraph",
      "version": "initial",
      "hash": "a1b2c3d4...",
      "data": {
        "text": "First paragraph..."
      }
    },
    {
      "type": "image",
      "version": "initial",
      "hash": "e5f6g7h8...",
      "data": {
        "src": "https://example.com/image.jpg",
        "alt": "An example image"
      }
    }
  ]
}
```

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

### 4. Content Hashing for Change Detection

Each block includes a hash computed from its data, enabling:
- Detecting content changes during user sessions
- Optimistic concurrency control
- Cache invalidation without deep comparison

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
    r.register(BlockDescriptor {
        type_name: "paragraph",
        label: "Paragraph",
        category: BlockCategory::Text,
    });
    r.register(BlockDescriptor {
        type_name: "image",
        label: "Image",
        category: BlockCategory::Media,
    });
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

## See Also

**Related Guides:**
- **[070-api-handlers.md](./070-api-handlers.md)** - Using Nightfire in API handlers
- **[075-validation.md](./075-validation.md)** - Request validation patterns
- **[050-database.md](./050-database.md)** - Storing JSONB content

**Crate Documentation:**
- `underlay-nightfire/README.md` - Quick reference
- `underlay-nightfire/src/lib.rs` - Module documentation
