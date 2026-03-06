# 012 – Nightfire Extraction to Underlay

**Status**: Complete  
**Priority**: Medium  
**Estimated Duration**: 2-3 hours  
**Owner**: TBD  
**Created**: 2026-01-15  
**Completed**: 2026-01-15

---

## Overview

Extract the generic Nightfire structured content system from Farmyard into Underlay as `underlay-nightfire`. Nightfire is a block-based content protocol for storing and validating JSON content with typed blocks and validation strategies.

**Key Goals**:
1. Move the generic protocol/engine to Underlay for reuse across projects
2. Keep Acowtancy-specific blocks, categories, and registrations in Farmyard
3. Generalize the category type so consuming apps can define their own block categories
4. Convert static registries to instance-based pattern for flexibility

**Success Metrics**:
- [x] `underlay-nightfire` crate created with generic engine
- [x] `nightfire` updated to depend on and re-export from `underlay-nightfire` (renamed from `nightfire-acowtancy`)
- [x] Original generic `nightfire` types merged into `underlay-nightfire`
- [x] All Farmyard tests passing
- [x] No breaking changes to API consumers

---

## Background

### Current State

| Crate | Location | Purpose |
|-------|----------|---------|
| `nightfire` | `farmyard/crates/nightfire/` | Generic protocol (BlockData, NightfireValue, hashing) |
| `nightfire-acowtancy` | `farmyard/crates/nightfire-acowtancy/` | Acowtancy blocks + generic strategy/registry code mixed together |

### The Problem

The `nightfire` crate is already generic and portable. However, `nightfire-acowtancy` contains both:
- **Generic infrastructure** (strategies, validation, registries) that should be in Underlay
- **Acowtancy-specific** blocks and registrations that should stay in Farmyard

This mixing prevents other Underlay consumers from using the Nightfire system.

---

## Phase 1 — Create underlay-nightfire Crate

**Priority**: High  
**Estimated Duration**: ~1 hour  

### 1.1 — Set Up Crate Structure

- [x] 1.1.1 Create `/underlay/rust/crates/underlay-nightfire/`
- [x] 1.1.2 Create `Cargo.toml` with dependencies
- [x] 1.1.3 Add to `/underlay/rust/Cargo.toml` workspace members

### 1.2 — Move Core Protocol Types

From `farmyard/crates/nightfire/src/lib.rs`:

- [x] 1.2.1 Move `BlockData` struct
- [x] 1.2.2 Move `SchemaId` struct
- [x] 1.2.3 Move `NightfireValue` struct
- [x] 1.2.4 Move `compute_block_hash()` function

### 1.3 — Extract Generic Strategy Types

From `farmyard/crates/nightfire-acowtancy/`:

- [x] 1.3.1 Move `StrategyCardinality` enum
- [x] 1.3.2 Move `MultiConfig` struct
- [x] 1.3.3 Create generic `BlockDescriptor<C>` (category as type parameter)
- [x] 1.3.4 Create generic `NightfireStrategy<C>`

### 1.4 — Extract Validation Logic

- [x] 1.4.1 Move `NightfireValidationError` enum (including `UnknownStrategy` variant)
- [x] 1.4.2 Move `validate_nightfire_value()` function (genericized over category)

### 1.5 — Extract Block Trait

- [x] 1.5.1 Move `AcowBlock` trait, rename to `Block`
- [x] 1.5.2 Keep generic: `TYPE_NAME`, `VERSIONS`, `active_version()`, `to_data()`, `export()`

### 1.6 — Create Generic Registries

- [x] 1.6.1 Create `BlockRegistry<C>` struct (instance-based, not static)
- [x] 1.6.2 Create `StrategyRegistry<C>` struct with `validate()` method

### 1.7 — Organize Module Structure

- [x] 1.7.1 Create module structure:
  ```
  underlay-nightfire/src/
  ├── lib.rs          # Module root, public API
  ├── block.rs        # BlockData, Block trait
  ├── value.rs        # NightfireValue, SchemaId
  ├── strategy.rs     # StrategyCardinality, NightfireStrategy<C>
  ├── registry.rs     # BlockRegistry<C>, StrategyRegistry<C>
  ├── validation.rs   # validate_nightfire_value(), error types
  └── hash.rs         # compute_block_hash()
  ```

### 1.8 — Build and Test

- [x] 1.8.1 Run `cargo build -p underlay-nightfire`
- [x] 1.8.2 Run `cargo test -p underlay-nightfire` (15 tests pass)
- [x] 1.8.3 Run `cargo clippy -p underlay-nightfire`

---

## Phase 2 — Update nightfire-acowtancy (renamed to nightfire)

**Priority**: High  
**Estimated Duration**: ~45 minutes  

### 2.1 — Add Underlay Dependency

- [x] 2.1.1 Update `Cargo.toml` to depend on `underlay-nightfire`
- [x] 2.1.2 Remove old `nightfire` dependency (merged into underlay-nightfire)

### 2.2 — Re-export Core Types

- [x] 2.2.1 Add re-exports to `lib.rs`:
  ```rust
  pub use underlay_nightfire::{
      compute_block_hash, Block, BlockData, BlockDescriptor, BlockRegistry, MultiConfig,
      NightfireStrategy, NightfireValidationError, NightfireValue, SchemaId, StrategyCardinality,
      StrategyRegistry,
  };
  ```

### 2.3 — Keep Acowtancy-Specific Code

- [x] 2.3.1 `BlockCategory` enum (Acowtancy-specific categories)
- [x] 2.3.2 All concrete block types (MarkdownBlock, Summary blocks, etc.)
- [x] 2.3.3 Helper types: `SummaryPage`, `SummaryImagePage`, `SummaryStep`, `SummaryTarget`
- [x] 2.3.4 Static registries using generic registry types (`BLOCK_REGISTRY`, `STRATEGY_REGISTRY`)
- [x] 2.3.5 Helper functions: `markdown_blocks()`, `lookup_strategy()`
- [x] 2.3.6 `validate_nightfire_value_by_schema()` (uses static registry)

### 2.4 — Remove Moved Code

- [x] 2.4.1 Remove generic types now in underlay-nightfire
- [x] 2.4.2 Update imports throughout the crate
- [x] 2.4.3 Rename crate from `nightfire-acowtancy` to `nightfire`
- [x] 2.4.4 Remove unused `BLOCK_REGISTRY_MAP` static
- [x] 2.4.5 Remove `AcowtancyValidationError` (merged `UnknownStrategy` into core error)

### 2.5 — Build and Test

- [x] 2.5.1 Run `cargo build -p nightfire`
- [x] 2.5.2 Run `cargo test -p nightfire` (4 tests pass)

---

## Phase 3 — Delete Original nightfire Crate

**Priority**: Medium  
**Estimated Duration**: ~15 minutes  

- [x] 3.1 Remove `farmyard/crates/nightfire/` directory (original generic crate)
- [x] 3.2 Remove from `farmyard/Cargo.toml` workspace members
- [x] 3.3 Update any direct imports to use `nightfire` or `underlay-nightfire`

---

## Phase 4 — Update Consumer Crates

**Priority**: High  
**Estimated Duration**: ~30 minutes  

### 4.1 — Update Import Paths

- [x] 4.1.1 `learning` crate - updated dependency and imports
- [x] 4.1.2 `content` crate - updated dependency and imports
- [x] 4.1.3 `api` crate - updated DTOs, validation, registries; simplified error handling
- [x] 4.1.4 `assessment` crate - updated dependency
- [x] 4.1.5 `platform` crate - removed unused nightfire dependency

### 4.2 — Verify Builds

- [x] 4.2.1 Run `cargo build --workspace` in Farmyard
- [x] 4.2.2 Run `cargo test --workspace` in Farmyard
- [x] 4.2.3 Run `cargo clippy --all-targets --all-features` in Farmyard

---

## Phase 5 — Documentation

**Priority**: Low  
**Estimated Duration**: ~30 minutes  

- [x] 5.1 Add `underlay-nightfire/README.md` with usage examples
- [x] 5.2 Add `076-nightfire.md` guide to `underlay/docs/guides/`
- [x] 5.3 Add doc comments to all public types in underlay-nightfire

---

## File Structure After Migration

### Underlay

```
underlay/rust/crates/underlay-nightfire/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs          # Module root, public API
    ├── block.rs        # BlockData, Block trait
    ├── value.rs        # NightfireValue, SchemaId
    ├── strategy.rs     # StrategyCardinality, NightfireStrategy<C>
    ├── registry.rs     # BlockRegistry<C>, StrategyRegistry<C>
    ├── validation.rs   # validate_nightfire_value(), error types
    └── hash.rs         # compute_block_hash()
```

### Farmyard

```
farmyard/crates/nightfire/
├── Cargo.toml
└── src/
    └── lib.rs          # BlockCategory, blocks, static registries, re-exports
```

### Deleted

```
farmyard/crates/nightfire/          # Original generic crate - merged into underlay-nightfire
farmyard/crates/nightfire-acowtancy/ # Renamed to nightfire
```

---

## Risks & Mitigation

### Risk: Breaking API Changes
**Probability**: Low  
**Impact**: Medium  
**Mitigation**: 
- Re-export all types from `nightfire` so existing imports continue to work
- Only internal structure changes, public API remains compatible

### Risk: Generic Category Complexity
**Probability**: Low  
**Impact**: Low  
**Mitigation**:
- Keep type bounds simple: `C: Clone + Eq + Hash + Send + Sync`
- Acowtancy continues using concrete `BlockCategory` type

---

## Benefits

1. **Reusability**: Other Underlay consumers get the Nightfire protocol without Acowtancy baggage
2. **Clean Boundaries**: Protocol vs. domain logic clearly separated
3. **Extensibility**: New apps can define their own `BlockCategory` and strategies
4. **Testing**: Generic engine testable in isolation
5. **Consistency**: Follows the pattern of `underlay-jobs` (generic registry + app handlers)

---

## References

- Farmyard nightfire crate: `farmyard/crates/nightfire/`
- [Underlay nightfire crate](../../../rust/crates/underlay-nightfire/)
