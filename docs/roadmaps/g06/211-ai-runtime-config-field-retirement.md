# g06.211 - AI Runtime Config Field Retirement

## Status

Complete.

## Scope

Close public-field compatibility boundaries for AI runtime resilience config
structs:

- `underlay_ai_runtime::RetryConfig`
- `underlay_ai_runtime::CircuitBreakerConfig`
- `underlay_ai_runtime::RouteChainConfig`

## Change

- Made AI runtime resilience config fields private.
- Added read-only accessors for retry, circuit-breaker, and route-chain policy
  values.
- Added builder-style setters for retained policy values.
- Updated Underlay AI runtime internals and tests.
- Migrated the known Farmyard circuit-breaker config literal.

## Compatibility

Impact: coordinated breaking change.

Known consumer struct literals were migrated. New apps must use defaults,
builders, and accessors instead of direct field reads or struct literals.

## Validation

- `cargo test -p underlay-ai-runtime`
- `cargo check -p farmyard-infra`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
