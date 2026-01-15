# underlay-nightfire

A generic block-based content protocol for structured JSON content with typed blocks and validation strategies.

## Overview

Nightfire provides a flexible system for storing and validating structured content as JSON. Content is organized into:

- **Blocks**: Typed content units (e.g., paragraph, heading, image)
- **Values**: Collections of blocks with a schema identifier
- **Strategies**: Rules defining what blocks are allowed and how many

## Core Types

### NightfireValue

The top-level content structure persisted in JSONB columns:

```rust
use underlay_nightfire::{NightfireValue, BlockData, SchemaId};

// Single-block value
let value = NightfireValue::single("app:content/title@1", block);

// Multi-block value  
let value = NightfireValue::multi("app:content/body@1", vec![block1, block2]);
```

### Block Trait

Implement this trait for your typed block structs:

```rust
use underlay_nightfire::Block;
use serde_json::Value;

struct ParagraphBlock {
    text: String,
}

impl Block for ParagraphBlock {
    const TYPE_NAME: &'static str = "paragraph";
    
    fn to_data(&self) -> Value {
        serde_json::json!({ "text": self.text })
    }
}

// Export to BlockData for storage
let data = ParagraphBlock { text: "Hello".into() }.export();
```

### Strategies & Registries

Define validation rules using strategies and registries:

```rust
use underlay_nightfire::{
    NightfireStrategy, StrategyCardinality, MultiConfig,
    BlockRegistry, StrategyRegistry, BlockDescriptor,
    validate_nightfire_value, SchemaId,
};
use std::num::NonZeroUsize;

// Define your category enum
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum MyCategory { Text, Media }

// Build block registry
let mut blocks = BlockRegistry::new();
blocks.register(BlockDescriptor {
    type_name: "paragraph",
    label: "Paragraph",
    category: MyCategory::Text,
});

// Build strategy registry
let mut strategies = StrategyRegistry::new();
strategies.register(NightfireStrategy {
    id: SchemaId::from("app:content/body@1"),
    cardinality: StrategyCardinality::Multi(MultiConfig {
        min_blocks: NonZeroUsize::new(1).unwrap(),
        max_blocks: None,
    }),
    allowed_types: vec![],
    allowed_categories: vec![MyCategory::Text],
    default_type: "paragraph".to_string(),
});

// Validate content
let result = validate_nightfire_value(&value, &strategy, &blocks);
```

## JSON Wire Format

Single-block:
```json
{
  "schema": "app:content/title@1",
  "block": { "type": "heading", "version": "initial", "hash": "...", "data": {...} }
}
```

Multi-block:
```json
{
  "schema": "app:content/body@1",
  "blocks": [
    { "type": "paragraph", "version": "initial", "hash": "...", "data": {...} },
    { "type": "image", "version": "initial", "hash": "...", "data": {...} }
  ]
}
```

## Design Philosophy

1. **Generic over Category**: The `C` type parameter allows applications to define their own block categories without modifying this crate.

2. **Instance-based Registries**: Registries are instances rather than statics, allowing for testing and multiple registry configurations.

3. **Separation of Concerns**: Core protocol in `underlay-nightfire`, app-specific blocks and strategies in consuming crates.

## See Also

- `nightfire-acowtancy` - Acowtancy-specific blocks, categories, and registries built on this crate
