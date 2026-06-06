# g06.169 Artifact - Rust Runtime Security Surface Re-audit

## Result

The current Rust runtime surface is broadly sound after the g06 reset, but one
security-relevant construction boundary should be hardened next.

`underlay-config::ConfigStack` still accepts raw environment and local overlay
names and turns them into `{name}.toml` paths under the config directory. The
current consumer family uses valid names, but `with_environment_from_env()` reads
`UNDERLAY_ENV` directly, so a path-like value can select files outside the
intended overlay namespace.

## Findings

- Priority: harden config overlay names before continuing lower-risk runtime
  cleanup.
- Scope: validate environment and local overlay names before file resolution.
- Consumer impact: expected additive for normal consumers, breaking only for
  invalid names such as path separators, `.` components, or control characters.
- Current callers: `underlay-reference`, `contact-patch`, `compli-me`,
  `acowtancy`, and `loophole/composer` use `with_environment_from_env()` with
  `"local"` overlays.

## Retained Strengths

- JWT config redacts secret material and validates keypair compatibility before
  service use.
- WebAuthn keeps state serialization behind the explicit
  `danger-allow-state-serialisation` feature and checks counter regressions.
- Blob and media object-key helpers keep typed parse boundaries available.
- Config env overrides remain explicit allowlist entries, not broad env
  interpolation.
- Auth cookie builders validate names, paths, domains, secure flags, and
  `SameSite=None` before producing headers.

## Deferred Cleanup

- `underlay-http::MicroCache`, `underlay-ai-runtime::CircuitBreakerMiddleware`,
  and `underlay-email` dev capture helpers still panic on poisoned synchronous
  mutexes. This is an availability hardening item, not the next security card.
- `AuthCookieConfig` retains raw public string fields and unchecked convenience
  setters. Runtime validation makes this acceptable for now, but the ideal
  shape is typed fields or fallible-only construction.

## Evidence

- Production Rust panic scan excluded tests and highlighted only known
  invariant panics, mutex poison panics, regex/HMAC invariants, and inline-test
  residue.
- Public crate-root review covered auth, JWT, password, TOTP, WebAuthn, HTTP,
  config, blob, media, jobs, audit, security alerts, and rate limit crates.
- Consumer search found no use of invalid config overlay names.

## Next Lane

Move to `g06.170`: config environment filename boundary hardening.
