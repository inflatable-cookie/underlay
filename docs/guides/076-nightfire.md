# 076 - Nightfire Integration

Nightfire is the standalone structured-content system at
`github.com/inflatable-cookie/nightfire`. That repository owns the Rust
protocol, TypeScript/Svelte editor and renderer, schemas, fixtures, generic
blocks, registries, validation, and package documentation.

Underlay owns integration around that package:

- Rust media traversal in `underlay-media`
- Nightfire validation-to-HTTP adaptation in `underlay-validation`
- the Svelte media-picker context and historical `media` block editor used by
  Underlay-based apps
- temporary compatibility facades for historical Underlay imports

Do not add generic Nightfire implementations here. Add them to Nightfire and
consume a released tag.

## Install Nightfire

Both language surfaces use the same Nightfire `0.2.0` release.

```toml
[dependencies]
nightfire = { git = "https://github.com/inflatable-cookie/nightfire.git", tag = "v0.2.0" }
```

```json
{
  "dependencies": {
    "@inflatable-cookie/nightfire": "0.2.0"
  }
}
```

Use Nightfire's repository documentation for generic setup and APIs. The Rust
crate uses the immutable `v0.2.0` Git tag; the TypeScript package uses the
released npm version `0.2.0`. New code imports `nightfire` in Rust and
`@inflatable-cookie/nightfire/*` in TypeScript.

## Compatibility Facades

Existing consumers may keep these deprecated TypeScript paths during the g12
migration:

- `@inflatable-cookie/underlay/nightfire`
- `@inflatable-cookie/underlay/nightfire/editor`
- `@inflatable-cookie/underlay/nightfire/renderer`
- `@inflatable-cookie/underlay/nightfire/block-editor`
- `@inflatable-cookie/underlay/nightfire/block-registration`
- `@inflatable-cookie/underlay/nightfire/markdown`
- `@inflatable-cookie/underlay/nightfire/editor-registry`
- `@inflatable-cookie/underlay/nightfire/render-registry`
- `@inflatable-cookie/underlay/nightfire/validator-registry`
- `@inflatable-cookie/underlay/nightfire/strategies`
- `@inflatable-cookie/underlay/nightfire/media-locator`
- `@inflatable-cookie/underlay/nightfire/block-ids`
- `@inflatable-cookie/underlay/nightfire/block-versions`
- `@inflatable-cookie/underlay/nightfire/utils`
- `@inflatable-cookie/underlay/nightfire/validation`

They re-export the corresponding Nightfire v0.2.0 subpaths. They do not own a
second implementation.

Rust consumers may keep the `underlay-nightfire` dependency name until the
facade retirement lane. That crate re-exports `nightfire` unchanged. New Rust
code should depend on `nightfire` directly.

## Underlay Media Picker Context

Nightfire v0.2.0 has no `./media` subpath. Underlay retains
`@inflatable-cookie/underlay/nightfire/media` because it is an Underlay media
library integration, not generic Nightfire runtime.

Provide the picker in an app layout:

```svelte
<script lang="ts">
  import {
    createNightfireMediaContext,
    type NightfireMediaPickResult,
  } from "@inflatable-cookie/underlay/nightfire/media";

  createNightfireMediaContext({
    async pickMedia(): Promise<NightfireMediaPickResult | null> {
      // Open the app's Underlay media-library picker.
      return null;
    },
  });
</script>
```

Consume it from an app-owned block editor:

```svelte
<script lang="ts">
  import { useNightfireMedia } from "@inflatable-cookie/underlay/nightfire/media";

  const media = useNightfireMedia();
</script>
```

Apps own their domain block editors and registration. Underlay keeps a wildcard
`media` editor adapter because it binds the historical Nightfire block payload
to the Underlay media-library picker. Strategy data still decides whether the
block is available for a field.

Nightfire's separate `./media-source` API is the generic source-registration
surface. Import it directly from `@inflatable-cookie/nightfire/media-source`.

## Rust Media Traversal

Enable `underlay-media`'s `nightfire` feature when an app needs media-reference
extraction from `NightfireValue`:

```toml
[dependencies]
underlay-media = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "vX.Y.Z", features = ["nightfire"] }
nightfire = { git = "https://github.com/inflatable-cookie/nightfire.git", tag = "v0.2.0" }
```

Underlay supplies:

- `NightfireBlockMediaUsageExtractor`
- `NightfireBlockMediaHandler`
- `NightfireBlockMediaHandlerRegistry`
- `NightfireMediaVisitContext`

The shared walker owns traversal. App-local handlers own the media semantics of
their block payloads and declare nested Nightfire values where needed. Prefer
registered handlers over field-name matching.

## Validation-To-HTTP Adaptation

Enable `underlay-validation`'s `nightfire` feature when an HTTP API needs to
map `nightfire::NightfireValidationError` into Underlay field-validation
responses:

```toml
[dependencies]
underlay-validation = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "vX.Y.Z", features = ["nightfire"] }
nightfire = { git = "https://github.com/inflatable-cookie/nightfire.git", tag = "v0.2.0" }
```

Nightfire remains the validation authority. Underlay only converts errors at
the API boundary.

## Payload Boundary

Nightfire values keep the canonical envelope:

```json
{
  "schema": "app:content/body",
  "blocks": [
    {
      "id": "nf_...",
      "type": "markdown",
      "version": "initial",
      "data": { "text": "Hello" }
    }
  ]
}
```

Outer DTO fields may be mapped to API `snake_case`. Keys inside block `data`
must remain verbatim. Underlay media handlers and Nightfire block logic inspect
that stored payload.

## Compatibility Posture

This is a deprecation rollout, not a breaking cutover. Existing Underlay paths
still resolve to the released implementation. Facade retirement requires a
fresh caller inventory and consumer proof under
[`023-release-and-compatibility-rollout.md`](../contracts/023-release-and-compatibility-rollout.md).

## See Also

- [`070-nightfire-and-migration-systems.md`](../contracts/070-nightfire-and-migration-systems.md)
- [`050-media-library-and-usage.md`](../contracts/050-media-library-and-usage.md)
- [`075-validation.md`](./075-validation.md)
- [`190-upgrade-compatibility.md`](./190-upgrade-compatibility.md)
