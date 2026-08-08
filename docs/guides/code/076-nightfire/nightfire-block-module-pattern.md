# Nightfire block module pattern

Use this when a consumer app wants one predictable place for:

- block payload shape
- Rust validation and strategy participation
- TS editor and renderer registration
- media extraction registration

## Rule

Keep one block family together as one module set.

Do not spread one block across:

- one Rust payload file
- one unrelated API extractor file
- one random TS editor registration list
- one separate media handler map

The app should be able to answer "what does the `hero` block mean?" by opening
one block module set.

## Recommended shape

```text
myapp/
├── api/
│   └── src/nightfire/
│       ├── blocks/
│       │   ├── hero.rs
│       │   ├── hero_media.rs
│       │   ├── gallery.rs
│       │   └── gallery_media.rs
│       ├── block_registry.rs
│       ├── strategy_registry.rs
│       └── media_registry.rs
└── admin/
    └── src/lib/nightfire/
        ├── blocks/
        │   ├── HeroEditor.svelte
        │   ├── HeroRenderer.svelte
        │   ├── hero.validation.ts
        │   └── hero.registration.ts
        ├── editor-registrations.ts
        └── render-registrations.ts
```

## Rust block module

```rust
// api/src/nightfire/blocks/hero.rs

use serde::{Deserialize, Serialize};
use serde_json::Value;
use underlay_nightfire::Block;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroBlock {
    pub title: String,
    #[serde(rename = "imageId")]
    pub image_id: Option<String>,
}

impl Block for HeroBlock {
    const TYPE_NAME: &'static str = "hero";

    fn to_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or(Value::Null)
    }
}
```

```rust
// api/src/nightfire/blocks/hero_media.rs

use underlay_media::{
    MediaId, MediaUsageRole, NightfireBlockMediaHandler,
    NightfireBlockMediaReference, NightfireBlockMediaRegistration,
    NightfireMediaVisitContext,
};
use uuid::Uuid;

pub struct HeroBlockMediaHandler;

impl NightfireBlockMediaHandler for HeroBlockMediaHandler {
    fn extract_media_references(
        &self,
        context: &NightfireMediaVisitContext<'_>,
    ) -> underlay_media::MediaResult<Vec<NightfireBlockMediaReference>> {
        let Some(media_id) = context
            .resolve_relative_pointer("/imageId")
            .and_then(|value| value.as_str())
            .and_then(|value| Uuid::parse_str(value).ok())
            .map(MediaId::from_uuid)
        else {
            return Ok(Vec::new());
        };

        Ok(vec![NightfireBlockMediaReference::new(
            media_id,
            MediaUsageRole::Embedded,
            "/imageId",
        )])
    }
}

pub fn hero_media_registration() -> NightfireBlockMediaRegistration {
    NightfireBlockMediaRegistration::new("hero", HeroBlockMediaHandler)
}

pub fn hero_block_registration() -> underlay_nightfire::BlockRegistration<
    crate::nightfire::block_registry::BlockCategory,
    NightfireBlockMediaRegistration,
> {
    underlay_nightfire::BlockRegistration::new(
        underlay_nightfire::BlockDescriptor {
            type_name: "hero",
            label: "Hero",
            category: crate::nightfire::block_registry::BlockCategory::Content,
        },
        hero_media_registration(),
    )
}
```

## Rust registry assembly

```rust
// api/src/nightfire/media_registry.rs

use underlay_media::NightfireBlockMediaHandlerMap;

use crate::nightfire::blocks::gallery::gallery_block_registration;
use crate::nightfire::blocks::hero::hero_block_registration;

pub fn build_media_registry() -> NightfireBlockMediaHandlerMap {
    NightfireBlockMediaHandlerMap::from_block_registrations([
        hero_block_registration(),
        gallery_block_registration(),
    ])
}
```

```rust
// api/src/nightfire/block_registry.rs

use once_cell::sync::Lazy;
use underlay_nightfire::BlockRegistry;

use crate::nightfire::blocks::gallery::gallery_block_registration;
use crate::nightfire::blocks::hero::hero_block_registration;

pub static BLOCK_REGISTRY: Lazy<BlockRegistry<BlockCategory>> = Lazy::new(|| {
    let mut registry = BlockRegistry::new();
    registry.extend_registrations([
        hero_block_registration(),
        gallery_block_registration(),
    ]);
    registry
});
```

The app-level registry files should only assemble block-module exports. They
should not contain block-specific JSON heuristics.

## TS block module

```ts
// admin/src/lib/nightfire/blocks/hero.registration.ts

import HeroEditor from "./HeroEditor.svelte";
import HeroRenderer from "./HeroRenderer.svelte";
import {
  type NightfireBlockRegistration,
} from "@inflatable-cookie/underlay/nightfire/block-registration";

export function heroBlockRegistration(schema: string): NightfireBlockRegistration {
  return {
    schema: {
      schema,
      mode: "multi",
      defaultType: "hero",
    },
    type: "hero",
    label: "Hero",
    editor: HeroEditor,
    renderer: HeroRenderer,
    validator: (block) => {
      const title = block?.data?.title;
      return typeof title === "string" && title.trim().length > 0 ? block : null;
    },
  };
}
```

## TS app assembly

```ts
// admin/src/lib/nightfire/editor-registrations.ts

import { registerNightfireBlocks } from "@inflatable-cookie/underlay/nightfire/block-registration";
import { galleryBlockRegistration } from "./blocks/gallery.registration";
import { heroBlockRegistration } from "./blocks/hero.registration";

const BODY_SCHEMA = "myapp:content/body@1";

registerNightfireBlocks([
  heroBlockRegistration(BODY_SCHEMA),
  galleryBlockRegistration(BODY_SCHEMA),
]);
```

If the consumer splits package entrypoints into separate side-effect modules like
`@myapp/ui/editor`, `@myapp/ui/render`, and `@myapp/ui/validation`, keep one
shared registration list and install the needed slices from it:

```ts
import {
  registerNightfireEditors,
  registerNightfireRenderers,
  registerNightfireValidators,
} from "@inflatable-cookie/underlay/nightfire/block-registration";

const registrations = [
  heroBlockRegistration(BODY_SCHEMA),
  galleryBlockRegistration(BODY_SCHEMA),
];

registerNightfireEditors(registrations);
registerNightfireRenderers(registrations);
registerNightfireValidators(registrations);
```

## Media sync usage

At save time:

1. TS writes prepared Nightfire JSON with `writePreparedNightfireToFormData(...)`
2. API persists the exact Nightfire JSON
3. API builds the block and media registries from block-module registrations
4. API runs `NightfireBlockMediaUsageExtractor`

```rust
let registry = build_media_registry();

let extractor = NightfireBlockMediaUsageExtractor::new(
    "lesson",
    Some(lesson_id),
    "body_blocks",
    MediaUsageProvenanceKind::ContentSync,
    registry,
);
```

## Good smell

- adding a new block means touching one block module set
- TS and Rust both know the same block type name
- validation, rendering, and media extraction evolve together
- the app registry files only assemble exports; they do not contain block logic

## Bad smell

- route code knows block-specific JSON field names
- one media extractor file hard-codes every block payload in the app
- TS editor registrations and Rust media handlers drift on block type names
- block meaning lives in multiple unrelated places
