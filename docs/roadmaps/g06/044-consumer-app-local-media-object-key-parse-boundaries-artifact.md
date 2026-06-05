# g06.044 Artifact - Consumer App-Local Media Object-Key Parse Boundaries

## Summary

Rolled the stored media object-key parse boundary through the six current
consumer apps.

Consumer DB/domain media rows now carry `BlobObjectKey` where the app owns the
stored media row boundary. SQL columns and JSON DTOs remain string edges.

## Consumer Changes

Updated app-local media DB rows and query mappings in:

- `underlay-reference/acme-api`
- `contact-patch/cp-api`
- `compli-me/api`
- `loophole/composer/composer-api`
- `songsprout/nursery`
- `acowtancy/farmyard`

Common shape:

- exported media version/rendition/summary rows expose `BlobObjectKey`
- private SQLx raw rows keep database strings
- query functions parse raw rows with `TryFrom`
- invalid stored keys fail during row mapping before DTO, URL, delete, or
  download paths
- JSON DTOs convert typed keys back to strings at the edge
- blob delete/download/public URL paths use typed keys or `as_str()` at the
  adapter boundary

Farmyard also carries typed keys through its app domain layer because it has an
explicit domain crate between DB and API.

## Compatibility

Impact: breaking source change inside the six consumer repos.

This is acceptable under the active g06 posture because the known apps are not
in production and were rolled in one batch.

Persisted object-key values and database column types did not change.

## Validation

Validation passed:

- `underlay-reference/acme-api`: `cargo check --workspace`
- `contact-patch/cp-api`: `cargo check --workspace`
- `compli-me/api`: `cargo check --workspace`
- `loophole/composer/composer-api`: `cargo check --workspace`
- `songsprout/nursery`: `cargo check --workspace`
- `acowtancy/farmyard`: `cargo check --workspace`

Farmyard still reports its pre-existing unused-function warning in
`farmyard-migration`.

## Residual Gap

Run a closeout audit for remaining raw object-key uses outside the media
repository/model boundary. Some raw string object keys are still expected for
non-media systems, direct SQL edges, or JSON/API boundaries.
