# 076 - Nightfire Integration

Nightfire is the standalone structured-content system at
`github.com/inflatable-cookie/nightfire`. That repository owns the Rust
protocol, TypeScript/Svelte editor and renderer, schemas, fixtures, generic
blocks, registries, validation, and package documentation.

Underlay owns integration around that package:

- Rust media traversal in `underlay-media`
- Nightfire validation-to-HTTP adaptation in `underlay-validation`
- the Svelte media-picker context (`@inflatable-cookie/underlay/nightfire/media`)
  and historical `media` block editor (`@inflatable-cookie/underlay/nightfire/editor`)
  used by Underlay-based apps

Do not add generic Nightfire implementations here. Add them to Nightfire and
consume a released tag.

## Install Nightfire

Both language surfaces use the same Nightfire `0.6.0` release. Underlay
declares Nightfire and Poodle as dependency ranges (`>=0.6.0 <0.7` and
`>=0.4.4 <0.5`), so a consumer resolving a Nightfire version inside that range
shares Underlay's copy instead of carrying a second one. A consumer pinned
below the range floor still carries two; see
[contract 023](../knowledge/contracts/023-release-and-compatibility-rollout.md).

```toml
[dependencies]
nightfire = { git = "https://github.com/inflatable-cookie/nightfire.git", tag = "v0.6.0" }
```

```json
{
  "dependencies": {
    "@inflatable-cookie/nightfire": "0.6.0"
  }
}
```

Use Nightfire's repository documentation for generic setup and APIs. The Rust
crate uses the immutable `v0.6.0` Git tag; the TypeScript package uses the
released npm version `0.6.0`. New code imports `nightfire` in Rust and
`@inflatable-cookie/nightfire/*` in TypeScript.

The historical `underlay-nightfire` Rust facade and the 14 pass-through
`@inflatable-cookie/underlay/nightfire/*` TypeScript shims are retired. Only
`@inflatable-cookie/underlay/nightfire/editor` and
`@inflatable-cookie/underlay/nightfire/media` remain exported by Underlay.

## Underlay Media Picker Context

Nightfire `v0.6.0` has no `./media` subpath. Underlay retains
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
nightfire = { git = "https://github.com/inflatable-cookie/nightfire.git", tag = "v0.6.0" }
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
nightfire = { git = "https://github.com/inflatable-cookie/nightfire.git", tag = "v0.6.0" }
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

The legacy `underlay-nightfire` Rust crate and the 14 pass-through
`@inflatable-cookie/underlay/nightfire/*` TypeScript shims are retired.
Consumers depend on `nightfire` and `@inflatable-cookie/nightfire/*` directly
(matching Underlay's pinned release) and use Underlay only for
`@inflatable-cookie/underlay/nightfire/editor` and
`@inflatable-cookie/underlay/nightfire/media`. Further surface changes follow
[`023-release-and-compatibility-rollout.md`](../knowledge/contracts/023-release-and-compatibility-rollout.md).

## See Also

- [`070-nightfire-and-migration-systems.md`](../knowledge/contracts/070-nightfire-and-migration-systems.md)
- [`050-media-library-and-usage.md`](../knowledge/contracts/050-media-library-and-usage.md)
- [`075-validation.md`](./075-validation.md)
- [`190-upgrade-compatibility.md`](./190-upgrade-compatibility.md)
