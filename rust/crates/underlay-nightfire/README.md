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

// Always `{ schema, blocks }`. Cardinality is a strategy rule.
let value = NightfireValue::single("app:content/title", block);
let value = NightfireValue::multi("app:content/body", vec![block1, block2]);
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

// Define your category enum
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum MyCategory { Text, Media }

// Build block registry
let mut blocks = BlockRegistry::new();
blocks.register(BlockDescriptor::new(
    "paragraph",
    "Paragraph",
    MyCategory::Text,
));

// Build strategy registry
let mut strategies = StrategyRegistry::new();
strategies.register(NightfireStrategy {
    id: SchemaId::from("app:content/body"),
    cardinality: StrategyCardinality::Multi(MultiConfig::one_or_more()),
    allowed_types: vec![],
    allowed_categories: vec![MyCategory::Text],
    default_type: "paragraph".to_string(),
});

// Validate content
let result = validate_nightfire_value(&value, &strategy, &blocks);
```

## JSON Wire Format

```json
{
  "schema": "app:content/body",
  "blocks": [
    { "id": "nf_...", "type": "paragraph", "version": "initial", "data": {} },
    { "id": "nf_...", "type": "image", "version": "initial", "data": {} }
  ]
}
```

Single-block strategies still use `blocks` with `len == 1`. Schema IDs are
unversioned. Version lives on each block. There is no envelope `hash`.

## Stable block ids and media locators

Nightfire blocks carry a stable `id`. Underlay uses those ids as the
preferred anchor for media-usage references inside structured content.

Canonical Nightfire media locator format:

- `<block-id>#<json-pointer-relative-to-block.data>`

Examples:

- `hero_01#/image_id`
- `gallery_02#/pages/1/image_id`

Use `ensure_block_ids()` on the Rust side and `prepareNightfireForSave()` on
the TS side so stored values have stable top-level block ids before media usage
extraction or audit.

Shared media lifecycle with Underlay:

1. persist a `NightfireValue` with block ids
2. extract usage edges with `underlay-media`'s registry-backed
   `NightfireBlockMediaUsageExtractor`
3. sync those edges into `media_usage`
4. later resolve stored `block_id` or `path` locators back into the current
   Nightfire JSON

For new block work, keep the block payload type, validation, TS registrations,
and Rust media handler registration in the same block module set. Underlay's
media side now supports exporting one `NightfireBlockMediaRegistration` per
block module and assembling the app registry from those exports.

For cleaner Rust-side assembly, `underlay-nightfire` also provides a generic
`BlockRegistration<C, M>` bundle so one block-module export can contribute:

- the Nightfire block descriptor
- optional higher-layer metadata such as media registration

See also:

- `docs/guides/code/076-nightfire/nightfire-block-module-pattern.md`

## Design Philosophy

1. **Generic over Category**: The `C` type parameter allows applications to define their own block categories without modifying this crate.

2. **Instance-based Registries**: Registries are instances rather than statics, allowing for testing and multiple registry configurations.

3. **Separation of Concerns**: Core protocol in `underlay-nightfire`, app-specific blocks and strategies in consuming crates.

## See Also

- `nightfire-acowtancy` - Acowtancy-specific blocks, categories, and registries built on this crate
