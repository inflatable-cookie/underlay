# g06.025 Artifact - Six-Consumer Rollout And Compatibility Retirement Proof

## Summary

The six known consumers now use explicit Rust contract crates plus concrete
adapter crates for the extracted Postgres runtime surfaces.

No compatibility aliases remain for the jobs Postgres extraction. The old
`underlay-jobs` adapter feature flags and concrete adapter imports are retired.

## Retired Surface Scan

Scanned:

- `underlay`
- `underlay-reference/acme-api`
- `contact-patch/cp-api`
- `compli-me/api`
- `acowtancy/farmyard`
- `songsprout/nursery`
- `loophole/composer/composer-api`

Patterns scanned:

- `underlay-jobs` with `postgres`, `scheduler`, `outbox`, or `full` features
- concrete jobs adapter imports from `underlay_jobs`
- concrete media Postgres imports from `underlay_media`
- auth-state Postgres imports from `underlay_auth`

Result:

- no consumer match for retired jobs adapter features or imports
- no consumer match for retired media Postgres imports
- no consumer match for retired auth-state Postgres imports
- active Underlay guides were updated to teach `underlay-jobs-postgres`

The only remaining active code match is `underlay_jobs::SchedulerConfig`, which
is an intended core contract export, not a retired adapter symbol.

Historical roadmap artifacts still mention old paths as evidence. They are not
active guidance.

## Current Consumer Dependency Shape

| Consumer | Core contract crate | Adapter crate |
| --- | --- | --- |
| `underlay-reference/acme-api` | `underlay-jobs` | `underlay-jobs-postgres` |
| `contact-patch/cp-api` | `underlay-jobs` | `underlay-jobs-postgres` |
| `compli-me/api` | `underlay-jobs` | `underlay-jobs-postgres` |
| `acowtancy/farmyard` | `underlay-jobs` | `underlay-jobs-postgres` |
| `songsprout/nursery` | `underlay-jobs` | `underlay-jobs-postgres` |
| `loophole/composer/composer-api` | `underlay-jobs` | `underlay-jobs-postgres` |

Auth-state adapter usage is explicit where needed:

- `underlay-reference/acme-api`
- `contact-patch/cp-api`
- `compli-me/api`
- `acowtancy/farmyard`

Media Postgres adapter usage remains explicit where needed from the prior media
adapter extraction line.

## Validation Evidence

Underlay:

- `effigy rust:check`
- `effigy qa:docs`

Consumers:

- `underlay-reference/acme-api`: `cargo check -p acme-jobs -p acme-api`
- `contact-patch/cp-api`: `cargo check -p cp-jobs -p cp-api`
- `compli-me/api`: `cargo check -p compli-me-jobs -p compli-me-api`
- `songsprout/nursery`: `cargo check -p nursery-jobs -p nursery-api`
- `acowtancy/farmyard`: `cargo check -p farmyard-jobs -p farmyard-api`
- `loophole/composer/composer-api`: `cargo check -p composer-api`

## Consumer Impact

Impact: breaking change already absorbed by the six known consumers.

Required upgrade pattern for any other consumer:

1. keep `underlay-jobs` for core job contracts
2. add `underlay-jobs-postgres` for concrete Postgres storage/runtime
3. replace `underlay_jobs::JobRepository` with
   `underlay_jobs_postgres::JobRepository`
4. replace `underlay_jobs::tasks::*` with
   `underlay_jobs_postgres::tasks::*`
5. replace `underlay_jobs::outbox::*` with
   `underlay_jobs_postgres::outbox::*`
6. import `PostgresJobRunnerExt` where `run_with_notifier` is called

## Decision

The adapter compatibility surface is retired for the known consumer family.

Proceed to `g06.026`: reference-grade docs and upgrade-note closeout.
