# Backlog: Performance Benchmarks

**Status**: Backlog  
**Priority**: Low  
**Estimated Effort**: 4-6 hours  
**Source**: Deferred from roadmap 009 (Quick Wins)

---

## Problem Statement

Underlay lacks systematic performance benchmarks, making it difficult to:

- Detect performance regressions in PRs
- Compare implementation alternatives objectively
- Set performance budgets for critical paths
- Document expected performance characteristics
- Identify optimization opportunities

---

## Proposed Solution

Implement comprehensive benchmarks for both Rust and TypeScript codebases.

### 1. Rust Benchmarks (Criterion)

```rust
// benches/pagination.rs
use criterion::{criterion_group, criterion_main, Criterion};
use underlay_http::pagination::PaginationParams;

fn bench_pagination(c: &mut Criterion) {
    c.bench_function("pagination_wrap_1000_items", |b| {
        let params = PaginationParams::default();
        let items: Vec<i32> = (0..1000).collect();
        b.iter(|| params.wrap(items.clone(), 10000));
    });
}

criterion_group!(benches, bench_pagination);
criterion_main!(benches);
```

Key benchmarks:
- `underlay-http`: Pagination, context extraction, CORS
- `underlay-auth`: Token generation, verification, hashing
- `underlay-db`: Connection pool, query building
- `underlay-testing`: Test setup overhead

### 2. TypeScript Benchmarks (Vitest bench)

```typescript
// benches/storage.bench.ts
import { bench, describe } from 'vitest';
import { storage } from '../src/patterns/storage';

describe('storage benchmarks', () => {
  bench('local.get - miss', () => {
    storage.local.get('nonexistent', null);
  });

  bench('local.set - small value', () => {
    storage.local.set('key', { small: 'value' });
  });

  bench('local.set - large value', () => {
    storage.local.set('key', { data: 'x'.repeat(10000) });
  });
});
```

Key benchmarks:
- Storage operations (get/set/store)
- Form state updates
- Component render performance

### 3. CI Integration

```yaml
# .github/workflows/bench.yml
name: Benchmarks
on:
  pull_request:
    paths:
      - 'rust/**'
      - 'ts/**'

jobs:
  rust-bench:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo bench --workspace
      - uses: benchmark-action/github-action-benchmark@v1
        with:
          tool: 'cargo'
          output-file-path: target/criterion/*/new/estimates.json
          
  ts-bench:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: bun bench
```

### 4. Benchmark Reports

- Store historical results in `docs/benchmarks/`
- Generate comparison charts on PRs
- Alert on significant regressions (>10%)
- Track performance over time

---

## Dependencies

- `criterion` crate for Rust benchmarks
- `vitest` with bench support for TypeScript
- `github-action-benchmark` for CI integration
- GitHub Pages for hosting results (optional)

---

## Success Criteria

- [ ] Criterion benchmarks for all Rust crates
- [ ] Vitest benchmarks for TypeScript patterns
- [ ] CI runs benchmarks on every PR
- [ ] Regression alerts for >10% slowdowns
- [ ] Historical benchmark data stored
- [ ] Documentation of performance characteristics

---

## Risks & Considerations

- **CI time**: Benchmarks add build time
- **Noise**: Benchmark results can be noisy on shared CI runners
- **Maintenance**: Benchmarks need updating when APIs change
- **False positives**: Need to tune regression thresholds

---

## Benchmark Categories

### Critical Path (must be fast)
- Token verification
- Request context extraction
- Storage get/set
- Form state updates

### Bulk Operations (must scale)
- Pagination with large datasets
- Fixture loading in tests
- Batch storage operations

### Cold Start (affects UX)
- Test server initialization
- Component hydration
- Store initialization

---

## Performance Budgets

Initial targets (to be validated):

| Operation | Target | Max Acceptable |
|-----------|--------|----------------|
| Token verify | <1ms | 5ms |
| Context extract | <0.1ms | 1ms |
| Pagination wrap (1000) | <0.5ms | 2ms |
| Storage get | <0.1ms | 0.5ms |
| Storage set | <0.5ms | 2ms |

---

## Related

- `rust/crates/*/benches/` - Benchmark locations
- `ts/vitest.config.ts` - Vitest configuration
- [Criterion.rs](https://bheisler.github.io/criterion.rs/book/)
- [Vitest Benchmarking](https://vitest.dev/guide/features.html#benchmarking)

---

**Created**: 2026-01-12
