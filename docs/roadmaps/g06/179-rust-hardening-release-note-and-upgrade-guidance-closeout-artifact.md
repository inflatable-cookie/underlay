# g06.179 Artifact - Rust Hardening Release-note And Upgrade-guidance Closeout

## Result

Rust hardening release and upgrade guidance is complete.

The durable upgrade note now lives in
[`docs/guides/190-upgrade-compatibility.md`](../../guides/190-upgrade-compatibility.md)
under `Rust Hardening API Tightening (2026-06-06)`.

## Upgrade Note Coverage

The note covers:

- retired `AuthCookieConfig` public fields
- retired unchecked auth cookie string setters
- retained auth cookie accessors, checked builders, and typed setters
- retired `PostgresMediaConfig::with_schema`
- retained `PostgresMediaConfig::try_with_schema`
- config overlay name validation
- runtime helper mutex poison recovery
- six-consumer validation proof

## Contract Alignment

[`docs/contracts/122-rust-public-api-inventory.md`](../../contracts/122-rust-public-api-inventory.md)
now records the final cookie config surface:

- read-only accessors
- checked builders
- typed setters
- no raw string setter compatibility
- no public raw fields

## Validation

Consumer proof is retained from `g06.178`:

- `underlay-reference`: `effigy health` passed
- `contact-patch`: `effigy health` passed
- `compli-me`: `effigy health` passed
- `acowtancy`: `effigy health` passed with the known non-failing
  `farmyard-migration` dead-code warning
- `songsprout`: `effigy health` passed
- `loophole/composer`: `effigy health` passed

## Next Lane

Move to `g06.180`: g06 closeout readiness checkpoint.
