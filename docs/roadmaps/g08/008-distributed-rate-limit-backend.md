# g08.008 - Distributed Rate-Limit Backend

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Make rate limiting real in multi-instance deployments. Only `InMemoryBackend`
ships (process-local DashMap, `Instant` windows), while docs advertise Redis/DB
backends that do not exist in-tree. Attackers spread attempts across replicas,
and a restart wipes all counters. This is the enforcement layer behind the
login, second-factor, and reset limits.

## Evidence

- `rust/crates/underlay-ratelimit/src/memory.rs:25,84,181-204`
- backlog item `docs/roadmaps/backlog/rate-limiting.md`

## Governing References

- [020 HTTP transport and server boundary](../../contracts/020-http-transport-and-server-boundary.md)
- [030 Auth and session systems](../../contracts/030-auth-and-session-systems.md)

## Planned Changes

- [x] Add a distributed atomic backend (Redis `INCR`+`EXPIRE` or Postgres) as the
  documented prod path.
- [x] Label `InMemoryBackend` non-prod with a `single_instance()` semantic
  constructor. *(Scope note, g08 audit: the gate is documentation + naming, not
  a hard config gate — `new()`/`Default` remain constructible. Hard-gating would
  break every test/dev call site for no prod win; the prod path is the
  documented Postgres backend.)*
- [x] Reconcile docs so advertised backends match what ships.

## Consumer Upgrade Impact

Impact class: `configuration`. Prod consumers must configure a distributed
backend. Requires six-consumer proof per `023`.

## Validation

- [x] test: distributed backend enforces a shared window across simulated
  instances — `src/tests/postgres_integration.rs`
  (`two_instances_enforce_one_shared_window`, two `PostgresBackend` instances
  over one database; runs in CI against the `postgres:16` service). *(Audit
  note: this test landed at generation close once `g08.019` provisioned the
  Postgres test path; before that the box overstated in-tree coverage.)*
- [x] `cargo test -p underlay-ratelimit`
- [x] `effigy validate`

## Stop Conditions

Stop if the chosen backend adds a dependency consumers cannot uniformly provide;
surface the trade-off before committing.

## Completion Notes

Completed 2026-07-17. New `PostgresBackend` (feature `postgres`) in
`underlay-ratelimit`: fixed-window counters via a single atomic
`INSERT ... ON CONFLICT` upsert so replicas cannot race a window reset -
this is the documented production path. Migration
`0001__rate_limit_counters.sql` ships with the crate. `InMemoryBackend`
docs now state single-instance/non-prod explicitly, with a greppable
`single_instance()` constructor for deliberate prod use. Guide `068`
reconciled: the advertised `RedisBackend` never existed - documented as
"implement the trait if you want Redis" instead. Chose Postgres over Redis
so consumers already running the app DB need no new dependency (stop
condition satisfied). Unit tests cover table-name validation and count ->
result mapping; live multi-instance enforcement is an integration test
against real Postgres. `cargo test -p underlay-ratelimit --features postgres`
green.

## Next Task

`g08.009` http-client SSRF and timeout defaults.
