# 041 - Rust Module Splitting Conventions

This guide defines when and how to split large Rust source files in Underlay crates.

## Thresholds

1. **500 lines** — warning. Consider splitting.
2. **900 lines** — hard limit. Must split before merging.
3. Enforced by `scripts/check-file-length.sh`.

## When to Split

Split a file when:

1. It exceeds 500 lines and has distinct logical sections.
2. Tests comprise >30% of the file.
3. Feature-gated code can live in its own module.
4. A single struct's `impl` blocks span multiple unrelated concerns.

Do **not** split when:

1. The file is cohesive (all code serves one purpose).
2. Splitting would create circular dependencies.
3. The file is near the threshold and well-organized.

## Patterns

### Extract Test Suites

Use `#[path]` to move tests to a separate file without changing module structure.

```rust
// In source.rs — replace inline #[cfg(test)] mod tests { ... } with:
#[cfg(test)]
#[path = "source_tests.rs"]
mod tests;
```

The test file uses `use super::*;` to access the parent module.

### Extract Types (Row Types, DTOs)

Move struct definitions and their `From`/`Into` impls to a sibling module.

```rust
// In lib.rs:
mod postgres_rows;   // pub(crate) row types

// In postgres.rs:
use crate::postgres_rows::MyRow;
```

Use `pub(crate)` visibility for types only needed within the crate.

### Extract Feature-Gated Code

Move feature-gated implementations to dedicated modules.

```rust
// In lib.rs:
#[cfg(feature = "postgres")]
mod postgres_rows;
#[cfg(feature = "postgres")]
pub mod postgres;
```

### Preserve Public API

Always re-export moved items from their original path in `lib.rs`:

```rust
// Before: pub use crate::postgres::{Repo, Notifier};
// After:
pub use crate::postgres::Repo;
pub use crate::postgres_scheduled::Notifier;
// External consumers see no change.
```

## Naming Conventions

| Extracted content | File name |
|---|---|
| Tests from `foo.rs` | `foo_tests.rs` |
| Tests from `lib.rs` | `lib_tests.rs` |
| Row types from `postgres.rs` | `postgres_rows.rs` |
| Scheduled tasks from `postgres.rs` | `postgres_scheduled.rs` |
| Feature code from `lib.rs` | `feature_name.rs` (e.g., `google.rs`, `attested.rs`) |

## Workflow

1. Identify the section to extract.
2. Create the new file with the extracted code.
3. Replace the inline code with a module reference or import.
4. Update `lib.rs` with `mod` declarations and re-exports.
5. `cargo check -p <crate> --all-features` — fix compile errors.
6. `cargo test -p <crate> --all-features` — verify all tests pass.
7. Commit as a single change for easy revert.

## Reference

See `docs/roadmaps/g01/017-rust-module-splitting-roadmap.md` for the initial splitting work and per-file decisions.
