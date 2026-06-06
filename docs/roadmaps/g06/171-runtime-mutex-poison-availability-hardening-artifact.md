# g06.171 Artifact - Runtime Mutex Poison Availability Hardening

## Result

Runtime helper mutex poison no longer panics in the audited production paths.

The affected state is cache, circuit-breaker coordination, and dev email
capture state. Those are availability-sensitive helper surfaces, so recovering
the poisoned guard is preferable to taking down the caller path.

## Change

- `underlay-http::MicroCache` now recovers poisoned entry guards.
- `underlay-ai-runtime::CircuitBreakerMiddleware` now recovers poisoned provider
  state guards.
- `underlay-email::InMemoryEmailStore` now recovers poisoned capture-store
  guards.
- Added focused poison-regression tests for all three surfaces.

## Consumer Impact

Classification: additive hardening.

No public API changes were made. Consumers only see fewer panic paths after an
internal panic poisons one of these helper locks.

## Validation

- `cargo test -p underlay-http -p underlay-ai-runtime -p underlay-email`:
  passed.
- Production poison-panic scan across the three affected crate source trees:
  clean.

## Next Lane

Move to `g06.172`: auth cookie construction surface tightening.
